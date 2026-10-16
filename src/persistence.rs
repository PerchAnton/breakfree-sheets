use crate::{
    cells::{Address, Cell},
    workbook::Workbook,
};
use serde::{
    Deserialize, Serialize,
    de::{self, MapAccess, Visitor},
};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};
pub const APP_ID: &str = "breakfree.sheets";
pub const FORMAT_ID: &str = "breakfree.sheets.native";
pub const MAX_FILE: u64 = 16 * 1024 * 1024;
#[derive(Serialize, Default)]
struct Cells(BTreeMap<String, Cell>);
impl<'de> Deserialize<'de> for Cells {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Unique;
        impl<'de> Visitor<'de> for Unique {
            type Value = Cells;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("unique cell records")
            }
            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Cells, M::Error> {
                let mut cells = BTreeMap::new();
                while let Some((key, value)) = map.next_entry::<String, Cell>()? {
                    if cells.insert(key, value).is_some() {
                        return Err(de::Error::custom("Duplicate cell key"));
                    }
                    if cells.len() > 2600 {
                        return Err(de::Error::custom("Too many cells"));
                    }
                }
                Ok(Cells(cells))
            }
        }
        d.deserialize_map(Unique)
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Sheet {
    name: String,
    cells: Cells,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    app_id: String,
    format_id: String,
    file_version: u32,
    worksheet: Sheet,
}
pub fn encode(book: &Workbook) -> Result<Vec<u8>, String> {
    let cells = book
        .cells()
        .iter()
        .map(|(key, value)| (key.to_string(), value.clone()))
        .collect();
    let document = Document {
        app_id: APP_ID.into(),
        format_id: FORMAT_ID.into(),
        file_version: 1,
        worksheet: Sheet {
            name: "Sheet1".into(),
            cells: Cells(cells),
        },
    };
    let mut bytes = serde_json::to_vec_pretty(&document).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    if bytes.len() as u64 > MAX_FILE {
        return Err("Workbook exceeds 16 MiB".into());
    }
    Ok(bytes)
}
pub fn decode(bytes: &[u8]) -> Result<Workbook, String> {
    if bytes.len() as u64 > MAX_FILE {
        return Err("Workbook exceeds 16 MiB".into());
    }
    let doc: Document =
        serde_json::from_slice(bytes).map_err(|e| format!("Invalid workbook: {e}"))?;
    if doc.app_id != APP_ID || doc.format_id != FORMAT_ID {
        return Err("Document belongs to another application or format".into());
    }
    if doc.file_version != 1 {
        return Err("Unsupported workbook content version".into());
    }
    if doc.worksheet.name != "Sheet1" {
        return Err("Only Sheet1 is supported".into());
    }
    let mut book = Workbook::default();
    for (key, cell) in doc.worksheet.cells.0 {
        let address: Address = key.parse()?;
        if address.to_string() != key {
            return Err("Persisted addresses must be canonical uppercase".into());
        }
        book.set(address, Some(cell))?;
    }
    Ok(book)
}
pub fn load(path: &Path) -> Result<Workbook, String> {
    let mut bytes = Vec::new();
    File::open(path)
        .map_err(|e| e.to_string())?
        .take(MAX_FILE + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    decode(&bytes)
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub enum SaveFault {
    Write,
    Flush,
    Replace,
}
pub fn save(book: &Workbook, path: &Path) -> Result<(), String> {
    save_transaction(book, path, None)
}
#[doc(hidden)]
pub fn save_transaction(
    book: &Workbook,
    path: &Path,
    fault: Option<SaveFault>,
) -> Result<(), String> {
    let bytes = encode(book)?;
    if fs::symlink_metadata(path).is_ok_and(|m| m.file_type().is_symlink()) {
        return Err("Saving through a symbolic link is unsupported".into());
    }
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut temporary = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
    if fault == Some(SaveFault::Write) {
        temporary
            .write_all(&bytes[..3.min(bytes.len())])
            .map_err(|e| e.to_string())?;
        return Err("Injected partial write failure".into());
    }
    temporary.write_all(&bytes).map_err(|e| e.to_string())?;
    temporary.flush().map_err(|e| e.to_string())?;
    if fault == Some(SaveFault::Flush) {
        return Err("Injected flush failure".into());
    }
    temporary.as_file().sync_all().map_err(|e| e.to_string())?;
    if fault == Some(SaveFault::Replace) {
        return Err("Injected replace failure".into());
    }
    temporary.persist(path).map_err(|e| e.error.to_string())?;
    Ok(())
}
