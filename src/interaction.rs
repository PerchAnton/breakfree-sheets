//! Desktop interaction state, shared by the grid and formula bar; no rendering or calculation here.
use crate::{
    cells::{Address, COLS, Cell, Kind, ROWS},
    history::Editor,
    persistence,
    workbook::Workbook,
};
use std::path::{Path, PathBuf};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Surface {
    Selection,
    Grid,
    Formula,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputKind {
    Auto,
    Text,
    Number,
    Formula,
}
impl InputKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Auto => "Auto (safe)",
            Self::Text => "Text",
            Self::Number => "Number",
            Self::Formula => "Formula",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Selection {
    pub anchor: Address,
    pub active: Address,
}
impl Default for Selection {
    fn default() -> Self {
        let a = Address::new(0, 0).unwrap();
        Self {
            anchor: a,
            active: a,
        }
    }
}
impl Selection {
    pub fn bounds(&self) -> (Address, Address) {
        (
            Address {
                col: self.anchor.col.min(self.active.col),
                row: self.anchor.row.min(self.active.row),
            },
            Address {
                col: self.anchor.col.max(self.active.col),
                row: self.anchor.row.max(self.active.row),
            },
        )
    }
    pub fn cells(&self) -> Vec<Address> {
        let (min, max) = self.bounds();
        (min.row..=max.row)
            .flat_map(|row| (min.col..=max.col).map(move |col| Address { col, row }))
            .collect()
    }
    pub fn is_single(&self) -> bool {
        self.anchor == self.active
    }
    pub fn label(&self) -> String {
        let (min, max) = self.bounds();
        if min == max {
            min.to_string()
        } else {
            format!("{min}:{max}")
        }
    }
}
#[derive(Clone)]
pub struct Draft {
    pub source: String,
    pub kind: InputKind,
}
pub struct Session {
    pub editor: Editor,
    pub path: Option<PathBuf>,
    pub selection: Selection,
    pub surface: Surface,
    pub draft: Draft,
    original: Draft,
}
fn input_kind(cell: Option<&Cell>) -> InputKind {
    match cell.map(|c| c.kind) {
        None => InputKind::Auto,
        Some(Kind::Text) => InputKind::Text,
        Some(Kind::Number) => InputKind::Number,
        Some(Kind::Formula) => InputKind::Formula,
    }
}
fn inferred(source: &str) -> Kind {
    if source.starts_with('=') {
        return Kind::Formula;
    }
    let unsigned = source.strip_prefix(['-', '+']).unwrap_or(source);
    let integer = unsigned.split('.').next().unwrap_or("");
    let leading_zero = integer.len() > 1 && integer.starts_with('0');
    if !leading_zero && crate::number::Decimal::parse(source).is_ok() {
        Kind::Number
    } else {
        Kind::Text
    }
}
impl Default for Session {
    fn default() -> Self {
        Self::new(Workbook::default(), None)
    }
}
impl Session {
    pub fn new(book: Workbook, path: Option<PathBuf>) -> Self {
        let draft = Draft {
            source: String::new(),
            kind: InputKind::Auto,
        };
        let mut session = Self {
            editor: Editor::new(book),
            path,
            selection: Selection::default(),
            surface: Surface::Selection,
            draft: draft.clone(),
            original: draft,
        };
        session.reset_draft();
        session
    }
    pub fn reset_draft(&mut self) {
        let cell = self.editor.book.get(self.selection.active);
        self.draft = Draft {
            source: cell.map(|c| c.source.clone()).unwrap_or_default(),
            kind: input_kind(cell),
        };
        self.original = self.draft.clone();
        self.surface = Surface::Selection;
    }
    pub fn draft_dirty(&self) -> bool {
        self.draft.source != self.original.source || self.draft.kind != self.original.kind
    }
    pub fn dirty(&self) -> bool {
        self.editor.dirty() || self.draft_dirty()
    }
    pub fn begin_edit(&mut self, surface: Surface, replace: Option<String>) {
        self.surface = surface;
        if let Some(text) = replace {
            self.draft.source = text;
            self.draft.kind = InputKind::Auto;
        }
    }
    pub fn commit(&mut self) -> Result<(), String> {
        if self.draft_dirty() {
            let kind = match self.draft.kind {
                InputKind::Auto => inferred(&self.draft.source),
                InputKind::Text => Kind::Text,
                InputKind::Number => Kind::Number,
                InputKind::Formula => Kind::Formula,
            };
            self.editor.set(
                self.selection.active,
                Some(Cell::new(kind, self.draft.source.clone())?),
            )?;
        }
        self.reset_draft();
        Ok(())
    }
    pub fn select(&mut self, target: Address, extend: bool) -> Result<(), String> {
        self.commit()?;
        if !extend {
            self.selection.anchor = target;
        }
        self.selection.active = target;
        self.reset_draft();
        Ok(())
    }
    pub fn move_by(&mut self, dx: i32, dy: i32, extend: bool) -> Result<(), String> {
        let active = self.selection.active;
        let target = Address {
            col: (active.col as i32 + dx).clamp(0, COLS as i32 - 1) as usize,
            row: (active.row as i32 + dy).clamp(0, ROWS as i32 - 1) as usize,
        };
        self.select(target, extend)
    }
    pub fn go_to(&mut self, text: &str) -> Result<(), String> {
        let parts: Vec<_> = text.split(':').collect();
        if parts.is_empty() || parts.len() > 2 {
            return Err("Use an address or a rectangular range such as A1:C4".into());
        }
        let anchor: Address = parts[0].parse()?;
        let active = if parts.len() == 2 {
            parts[1].parse()?
        } else {
            anchor
        };
        self.commit()?;
        self.selection = Selection { anchor, active };
        self.reset_draft();
        Ok(())
    }
    pub fn clear(&mut self) -> Result<(), String> {
        self.editor.apply(
            self.selection
                .cells()
                .into_iter()
                .map(|a| (a, None))
                .collect(),
        )?;
        self.reset_draft();
        Ok(())
    }
    pub fn undo(&mut self) {
        if self.draft_dirty() {
            self.reset_draft();
        } else {
            self.editor.undo();
            self.reset_draft();
        }
    }
    pub fn redo(&mut self) {
        if !self.draft_dirty() {
            self.editor.redo();
            self.reset_draft();
        }
    }
    pub fn save_to(&mut self, path: &Path) -> Result<(), String> {
        self.commit()?;
        persistence::save(&self.editor.book, path)?;
        self.path = Some(path.to_owned());
        self.editor.mark_saved();
        Ok(())
    }
    pub fn open(&mut self, path: &Path) -> Result<(), String> {
        let book = persistence::load(path)?;
        *self = Self::new(book, Some(path.to_owned()));
        Ok(())
    }
    pub fn title(&self) -> String {
        format!(
            "{}{} — BreakFree Sheets",
            if self.dirty() { "* " } else { "" },
            self.path
                .as_ref()
                .and_then(|p| p.file_name())
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "Book1".into())
        )
    }
}

#[derive(Clone, Debug)]
pub struct GridMetrics {
    pub widths: Vec<f32>,
    pub heights: Vec<f32>,
    pub zoom: f32,
}
impl Default for GridMetrics {
    fn default() -> Self {
        Self {
            widths: vec![80.0; COLS],
            heights: vec![20.0; ROWS],
            zoom: 1.0,
        }
    }
}
impl GridMetrics {
    pub fn column(&self, index: usize) -> f32 {
        self.widths[..index].iter().sum::<f32>() * self.zoom
    }
    pub fn row(&self, index: usize) -> f32 {
        self.heights[..index].iter().sum::<f32>() * self.zoom
    }
    fn visible(values: &[f32], zoom: f32, start: f32, size: f32) -> std::ops::Range<usize> {
        let mut position = 0.0;
        let mut first = values.len();
        let mut last = values.len();
        for (i, value) in values.iter().enumerate() {
            let next = position + value * zoom;
            if next > start && first == values.len() {
                first = i;
            }
            if position > start + size {
                last = i;
                break;
            }
            position = next;
        }
        first..last
    }
    pub fn visible_columns(&self, start: f32, size: f32) -> std::ops::Range<usize> {
        Self::visible(&self.widths, self.zoom, start, size)
    }
    pub fn visible_rows(&self, start: f32, size: f32) -> std::ops::Range<usize> {
        Self::visible(&self.heights, self.zoom, start, size)
    }
    pub fn hit(&self, x: f32, y: f32) -> Option<Address> {
        if x < 0.0 || y < 0.0 {
            return None;
        }
        let col = Self::visible(&self.widths, self.zoom, x, 0.0).start;
        let row = Self::visible(&self.heights, self.zoom, y, 0.0).start;
        Address::new(col, row)
    }
    pub fn resize_column(&mut self, index: usize, width: f32) {
        self.widths[index] = width.clamp(32.0, 400.0);
    }
    pub fn resize_row(&mut self, index: usize, height: f32) {
        self.heights[index] = height.clamp(16.0, 100.0);
    }
}
