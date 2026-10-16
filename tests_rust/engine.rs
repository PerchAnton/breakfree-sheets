use breakfree_sheets::{
    cells::{Address, Cell, Kind, Value},
    history::Editor,
    interaction::{GridMetrics, InputKind, Session, Surface},
    number::Decimal,
    office,
    persistence::{self, SaveFault},
    workbook::Workbook,
};
use serde_json::{Value as Json, json};
use std::{fs, process::Command};
fn addr(s: &str) -> Address {
    s.parse().unwrap()
}
fn cell(kind: Kind, source: &str) -> Cell {
    Cell::new(kind, source).unwrap()
}
fn set(editor: &mut Editor, name: &str, kind: Kind, source: &str) {
    editor.set(addr(name), Some(cell(kind, source))).unwrap();
}
fn display(editor: &Editor, name: &str) -> String {
    editor.values[&addr(name)].to_string()
}
#[test]
fn preserved_python_v1_conformance() {
    let fixtures: Vec<Json> =
        serde_json::from_str(include_str!("fixtures/python-v1-conformance.json")).unwrap();
    assert!(fixtures.len() >= 30);
    for fixture in fixtures {
        let book = persistence::decode(&serde_json::to_vec(&fixture["book"]).unwrap()).unwrap();
        let editor = Editor::new(book);
        for (name, value) in fixture["expected"].as_object().unwrap() {
            assert_eq!(
                display(&editor, name),
                value.as_str().unwrap(),
                "scenario {} cell {name}",
                fixture["name"]
            );
        }
    }
}
#[test]
fn required_loop_undo_save_reopen_and_continue() {
    let mut e = Editor::default();
    set(&mut e, "A1", Kind::Number, "2");
    set(&mut e, "B1", Kind::Number, "3");
    set(&mut e, "C1", Kind::Formula, "=A1+B1");
    assert_eq!(display(&e, "C1"), "5");
    set(&mut e, "A1", Kind::Number, "4");
    assert_eq!(display(&e, "C1"), "7");
    e.undo();
    assert_eq!(display(&e, "C1"), "5");
    e.redo();
    assert_eq!(display(&e, "C1"), "7");
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("Україна with spaces.bfsheet");
    persistence::save(&e.book, &path).unwrap();
    let mut e = Editor::new(persistence::load(&path).unwrap());
    assert_eq!(e.book.get(addr("C1")).unwrap().source, "=A1+B1");
    set(&mut e, "A1", Kind::Number, "6");
    assert_eq!(display(&e, "C1"), "9");
}
#[test]
fn whole_grid_chain_is_iterative() {
    let mut book = Workbook::default();
    let names: Vec<_> = (0..26)
        .flat_map(|col| (0..100).map(move |row| Address { col, row }))
        .collect();
    book.set(names[0], Some(cell(Kind::Number, "1"))).unwrap();
    for pair in names.windows(2) {
        book.set(
            pair[1],
            Some(cell(Kind::Formula, &format!("={}+1", pair[0]))),
        )
        .unwrap();
    }
    let mut editor = Editor::new(book);
    assert_eq!(editor.values[&names[2599]].to_string(), "2600");
    editor.set(names[0], Some(cell(Kind::Number, "2"))).unwrap();
    assert_eq!(editor.values[&names[2599]].to_string(), "2601");
}
#[test]
fn cycle_recovery_and_dependency_replacement() {
    let mut e = Editor::default();
    set(&mut e, "A1", Kind::Formula, "=B1");
    set(&mut e, "B1", Kind::Formula, "=A1");
    set(&mut e, "C1", Kind::Formula, "=B1*2");
    assert_eq!(display(&e, "C1"), "#CYCLE!");
    set(&mut e, "A1", Kind::Number, "3");
    assert_eq!(display(&e, "C1"), "6");
    e.undo();
    assert_eq!(display(&e, "A1"), "#CYCLE!");
    set(&mut e, "B1", Kind::Number, "5");
    assert_eq!(display(&e, "A1"), "5");
}
#[test]
fn parser_limits_and_nested_parentheses() {
    let mut e = Editor::default();
    set(
        &mut e,
        "A1",
        Kind::Formula,
        &format!("={}2{}", "(".repeat(200), ")".repeat(200)),
    );
    assert_eq!(display(&e, "A1"), "2");
    set(
        &mut e,
        "A1",
        Kind::Formula,
        &format!("={}1", "1+".repeat(300)),
    );
    assert_eq!(display(&e, "A1"), "#LIMIT!");
    for formula in [
        "=",
        "=1+",
        "=()",
        "=(1",
        "=1)",
        "=1 2",
        "=WEBSERVICE(A1)",
        "=[book]A1",
        "=https://example.org",
        "=+2",
    ] {
        set(&mut e, "A1", Kind::Formula, formula);
        assert_eq!(display(&e, "A1"), "#SYNTAX!", "{formula}");
    }
}
#[test]
fn decimal_bounds_and_exact_half_even() {
    for text in [
        "NaN",
        "Infinity",
        "1,5",
        "1e4",
        " 2 ",
        "12345678901234567890123456789",
    ] {
        assert!(Decimal::parse(text).is_err(), "{text}");
    }
    assert_eq!(
        Decimal::parse("0.1")
            .unwrap()
            .operation('+', &Decimal::parse("0.2").unwrap())
            .unwrap()
            .to_string(),
        "0.3"
    );
    assert_eq!(
        Decimal::parse("1")
            .unwrap()
            .operation('/', &Decimal::parse("3").unwrap())
            .unwrap()
            .to_string(),
        "0.3333333333333333333333333333"
    );
}
#[test]
fn typed_sources_and_display_are_separate() {
    let mut e = Editor::default();
    set(&mut e, "A1", Kind::Text, "00123");
    set(&mut e, "B1", Kind::Formula, "=A1+1");
    assert_eq!(display(&e, "A1"), "00123");
    assert_eq!(display(&e, "B1"), "#VALUE!");
    set(&mut e, "A1", Kind::Number, "1.2300");
    assert_eq!(display(&e, "A1"), "1.23");
    assert_eq!(e.book.get(addr("A1")).unwrap().source, "1.2300");
    e.set(addr("A1"), None).unwrap();
    assert_eq!(display(&e, "B1"), "1");
}
#[test]
fn saved_state_and_redo_branch() {
    let mut e = Editor::default();
    set(&mut e, "A1", Kind::Number, "2");
    e.mark_saved();
    set(&mut e, "A1", Kind::Number, "4");
    assert!(e.dirty());
    e.undo();
    assert!(!e.dirty());
    set(&mut e, "B1", Kind::Text, "new branch");
    assert!(!e.can_redo());
    e.undo();
    assert!(!e.dirty());
}
#[test]
fn malformed_foreign_future_duplicate_and_unknown_fields_are_rejected_without_writes() {
    let base: Json =
        serde_json::from_slice(&persistence::encode(&Workbook::default()).unwrap()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("bad.bfsheet");
    let mut inputs = vec![
        b"bad".to_vec(),
        b"[]".to_vec(),
        b"null".to_vec(),
        vec![255],
        b"[".repeat(2000),
    ];
    for (key, value) in [
        ("app_id", json!("breakfree.docs")),
        ("format_id", json!("other")),
        ("file_version", json!(2)),
        ("file_version", json!(true)),
        ("extra", json!("unknown")),
    ] {
        let mut doc = base.clone();
        doc[key] = value;
        inputs.push(serde_json::to_vec(&doc).unwrap());
    }
    let valid = String::from_utf8(persistence::encode(&Workbook::default()).unwrap()).unwrap();
    inputs.push(
        valid
            .replace(
                "\"app_id\": \"breakfree.sheets\"",
                "\"app_id\": \"breakfree.sheets\", \"app_id\": \"breakfree.sheets\"",
            )
            .into_bytes(),
    );
    inputs.push(valid.replace("\"cells\": {}","\"cells\": {\"A1\": {\"kind\":\"text\",\"source\":\"a\"},\"A1\":{\"kind\":\"text\",\"source\":\"b\"}}").into_bytes());
    for bytes in inputs {
        fs::write(&path, &bytes).unwrap();
        assert!(persistence::load(&path).is_err());
        assert_eq!(fs::read(&path).unwrap(), bytes);
    }
}
#[test]
fn invalid_cells_and_unknown_formulas_have_different_policies() {
    let mut base: Json =
        serde_json::from_slice(&persistence::encode(&Workbook::default()).unwrap()).unwrap();
    for (key, value) in [
        ("a1", json!({"kind":"text","source":"a"})),
        ("A101", json!({"kind":"text","source":"a"})),
        ("A1", json!({"kind":"number","source":"NaN"})),
        ("A1", json!({"kind":"macro","source":"run"})),
        ("A1", json!({"kind":"text","source":"a","extra":true})),
    ] {
        base["worksheet"]["cells"] = json!({key:value});
        assert!(persistence::decode(&serde_json::to_vec(&base).unwrap()).is_err());
    }
    base["worksheet"]["cells"] = json!({"A1":{"kind":"formula","source":"=UNSUPPORTED(A1)"}});
    let e = Editor::new(persistence::decode(&serde_json::to_vec(&base).unwrap()).unwrap());
    assert_eq!(display(&e, "A1"), "#SYNTAX!");
}
#[test]
fn failed_writes_flushes_and_replacements_keep_previous_file() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("book.bfsheet");
    let book = Workbook::default();
    persistence::save(&book, &path).unwrap();
    let previous = fs::read(&path).unwrap();
    let mut e = Editor::default();
    set(&mut e, "A1", Kind::Number, "4");
    for fault in [SaveFault::Write, SaveFault::Flush, SaveFault::Replace] {
        assert!(persistence::save_transaction(&e.book, &path, Some(fault)).is_err());
        assert_eq!(fs::read(&path).unwrap(), previous);
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 1);
    }
}
#[test]
fn cell_and_formula_entry_share_one_source_and_escape_discards() {
    let mut s = Session::default();
    s.begin_edit(Surface::Grid, Some("2".into()));
    s.commit().unwrap();
    assert_eq!(s.editor.book.get(addr("A1")).unwrap().kind, Kind::Number);
    s.begin_edit(Surface::Formula, None);
    s.draft.source = "4".into();
    assert!(s.dirty());
    s.reset_draft();
    assert_eq!(s.draft.source, "2");
    s.begin_edit(Surface::Formula, None);
    s.draft.source = "4".into();
    s.commit().unwrap();
    assert_eq!(display(&s.editor, "A1"), "4");
}
#[test]
fn automatic_entry_preserves_identifiers_and_explicit_number_conversion() {
    let mut s = Session::default();
    s.begin_edit(Surface::Grid, Some("00123".into()));
    s.commit().unwrap();
    assert_eq!(s.editor.book.get(addr("A1")).unwrap().kind, Kind::Text);
    s.draft.kind = InputKind::Number;
    s.commit().unwrap();
    assert_eq!(display(&s.editor, "A1"), "123");
}
#[test]
fn range_clear_is_one_undoable_model_edit() {
    let mut s = Session::default();
    set(&mut s.editor, "A1", Kind::Number, "2");
    set(&mut s.editor, "B1", Kind::Number, "3");
    s.go_to("A1:B1").unwrap();
    s.clear().unwrap();
    assert!(s.editor.book.cells().is_empty());
    s.undo();
    assert_eq!(display(&s.editor, "A1"), "2");
    assert_eq!(display(&s.editor, "B1"), "3");
}
#[test]
fn address_navigation_and_resizing_do_not_mutate_values() {
    let mut s = Session::default();
    s.begin_edit(Surface::Grid, Some("2".into()));
    s.move_by(1, 0, false).unwrap();
    s.move_by(1, 1, true).unwrap();
    assert_eq!(s.selection.label(), "B1:C2");
    let book = s.editor.book.clone();
    let mut grid = GridMetrics::default();
    grid.resize_column(0, 150.0);
    grid.resize_row(0, 36.0);
    grid.zoom = 1.5;
    assert_eq!(grid.hit(151.0, 20.0), Some(addr("A1")));
    assert_eq!(grid.hit(-1.0, 2.0), None);
    assert_eq!(s.editor.book, book);
    assert!(grid.visible_rows(0.0, 300.0).len() * grid.visible_columns(0.0, 600.0).len() < 300);
}
#[test]
fn rejected_save_and_open_keep_session_and_draft() {
    let mut s = Session::default();
    s.begin_edit(Surface::Grid, Some("2".into()));
    s.commit().unwrap();
    let directory = tempfile::tempdir().unwrap();
    let bad = directory.path().join("bad.bfsheet");
    fs::write(&bad, "bad").unwrap();
    s.draft.source = "4".into();
    assert!(s.open(&bad).is_err());
    assert_eq!(s.draft.source, "4");
    assert_eq!(display(&s.editor, "A1"), "2");
    assert!(
        s.save_to(&directory.path().join("missing/book.bfsheet"))
            .is_err()
    );
    assert!(s.editor.dirty());
    assert!(s.path.is_none());
}
#[test]
fn old_native_fixture_and_office_contract() {
    let book = persistence::decode(include_bytes!("../examples/first-workbook.bfsheet")).unwrap();
    assert_eq!(display(&Editor::new(book), "C1"), "5");
    let report = office::capabilities();
    assert_eq!(report["app_id"], "breakfree.sheets");
    assert_eq!(report["protocol_version"], 1);
    assert_eq!(report["app_version"], env!("CARGO_PKG_VERSION"));
    let fixture: Json =
        serde_json::from_str(include_str!("../contracts/v1/capabilities.example.json")).unwrap();
    assert_eq!(report, fixture);
}
#[test]
fn executable_headless_contract_and_failures() {
    let binary = env!("CARGO_BIN_EXE_breakfree-sheets");
    let temp = tempfile::tempdir().unwrap();
    let result = Command::new(binary)
        .arg("--capabilities")
        .current_dir(temp.path())
        .env_remove("DISPLAY")
        .output()
        .unwrap();
    assert!(result.status.success());
    assert!(result.stderr.is_empty());
    assert_eq!(String::from_utf8_lossy(&result.stdout).lines().count(), 1);
    assert_eq!(
        serde_json::from_slice::<Json>(&result.stdout).unwrap(),
        office::capabilities()
    );
    let version = Command::new(binary).arg("--version").output().unwrap();
    assert!(String::from_utf8_lossy(&version.stdout).contains(office::VERSION));
    for args in [
        vec!["--open", "relative.bfsheet"],
        vec!["--new", "--capabilities"],
    ] {
        let output = Command::new(binary).args(args).output().unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
    }
    let missing = temp.path().join("missing.bfsheet");
    let output = Command::new(binary)
        .arg("--open")
        .arg(missing)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
}
#[test]
fn first_error_propagation_and_unrelated_cells() {
    let mut e = Editor::default();
    set(&mut e, "A1", Kind::Formula, "=1/0");
    set(&mut e, "B1", Kind::Formula, "=SUM(1)");
    set(&mut e, "C1", Kind::Formula, "=A1+B1");
    set(&mut e, "D1", Kind::Number, "7");
    assert_eq!(display(&e, "C1"), "#DIV/0!");
    assert!(matches!(e.values[&addr("C1")], Value::Error(_)));
    assert_eq!(display(&e, "D1"), "7");
}

#[test]
fn oversized_input_and_out_of_grid_api_are_rejected() {
    assert!(persistence::decode(&vec![b' '; persistence::MAX_FILE as usize + 1]).is_err());
    let mut book = Workbook::default();
    assert!(
        book.set(Address { col: 26, row: 0 }, Some(cell(Kind::Text, "x")))
            .is_err()
    );
    assert!(book.cells().is_empty());
    for text in ["A0", "A01", "A101", "AA1", "ſ1", " A1"] {
        assert!(text.parse::<Address>().is_err());
    }
}
