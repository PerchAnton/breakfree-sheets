use breakfree_sheets::{
    cells::{Address, Kind},
    gui::Application,
    interaction::{Session, Surface},
    persistence,
};
use eframe::egui::{
    Context, Event, Key, Modifiers, PointerButton, Pos2, RawInput, Rect, pos2, vec2,
};
struct Desktop {
    app: Application,
    context: Context,
    time: f64,
}
impl Desktop {
    fn new(scale: f32) -> Self {
        let context = Context::default();
        context.set_pixels_per_point(scale);
        let mut this = Self {
            app: Application::new(Session::default()),
            context,
            time: 0.0,
        };
        this.frame(vec![]);
        this.frame(vec![]);
        this
    }
    fn frame(&mut self, events: Vec<Event>) {
        self.time += 0.05;
        let input = RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1366.0, 768.0))),
            time: Some(self.time),
            events,
            ..Default::default()
        };
        let _ = self.context.run(input, |ctx| self.app.ui(ctx));
    }
    fn click(&mut self, p: Pos2, modifiers: Modifiers) {
        self.frame(vec![
            Event::PointerMoved(p),
            Event::PointerButton {
                pos: p,
                button: PointerButton::Primary,
                pressed: true,
                modifiers,
            },
        ]);
        self.frame(vec![Event::PointerButton {
            pos: p,
            button: PointerButton::Primary,
            pressed: false,
            modifiers,
        }]);
        self.frame(vec![]);
    }
    fn cell_position(&self, name: &str) -> Pos2 {
        let a: Address = name.parse().unwrap();
        self.app.probes.cell_origin
            + vec2(
                self.app.metrics.column(a.col)
                    + self.app.metrics.widths[a.col] * self.app.metrics.zoom / 2.0,
                self.app.metrics.row(a.row)
                    + self.app.metrics.heights[a.row] * self.app.metrics.zoom / 2.0,
            )
    }
    fn cell(&mut self, name: &str) {
        self.click(self.cell_position(name), Modifiers::NONE);
    }
    fn key(&mut self, key: Key, modifiers: Modifiers) {
        self.frame(vec![Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers,
        }]);
        self.frame(vec![Event::Key {
            key,
            physical_key: Some(key),
            pressed: false,
            repeat: false,
            modifiers,
        }]);
        self.frame(vec![]);
    }
    fn text(&mut self, text: &str) {
        self.frame(vec![Event::Text(text.into())]);
        self.frame(vec![]);
    }
    fn grid_enter(&mut self, name: &str, text: &str) {
        self.cell(name);
        self.text(text);
        self.key(Key::Enter, Modifiers::NONE);
    }
    fn formula_enter(&mut self, name: &str, text: &str) {
        self.cell(name);
        self.click(self.app.probes.formula.center(), Modifiers::NONE);
        self.key(Key::A, command());
        self.text(text);
        self.key(Key::Enter, Modifiers::NONE);
    }
    fn value(&self, name: &str) -> String {
        self.app.session.editor.values[&name.parse().unwrap()].to_string()
    }
    fn drag(&mut self, from: Pos2, to: Pos2) {
        self.frame(vec![
            Event::PointerMoved(from),
            Event::PointerButton {
                pos: from,
                button: PointerButton::Primary,
                pressed: true,
                modifiers: Modifiers::NONE,
            },
        ]);
        self.frame(vec![Event::PointerMoved(to)]);
        self.frame(vec![Event::PointerButton {
            pos: to,
            button: PointerButton::Primary,
            pressed: false,
            modifiers: Modifiers::NONE,
        }]);
        self.frame(vec![]);
    }
}
fn command() -> Modifiers {
    Modifiers {
        ctrl: true,
        command: true,
        ..Modifiers::NONE
    }
}
#[test]
fn actual_grid_and_formula_events_calculate_undo_save_open() {
    let mut h = Desktop::new(1.0);
    h.grid_enter("A1", "2");
    assert_eq!(h.value("A1"), "2");
    assert_eq!(
        h.app
            .session
            .editor
            .book
            .get("A1".parse().unwrap())
            .unwrap()
            .kind,
        Kind::Number
    );
    h.grid_enter("B1", "3");
    h.formula_enter("C1", "=A1+B1");
    assert_eq!(h.value("C1"), "5");
    h.grid_enter("A1", "4");
    assert_eq!(h.value("C1"), "7");
    h.key(Key::Z, command());
    assert_eq!(h.value("C1"), "5");
    h.key(Key::Y, command());
    assert_eq!(h.value("C1"), "7");
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("loop.bfsheet");
    assert!(h.app.save_to(&path));
    h.app.session.open(&path).unwrap();
    h.frame(vec![]);
    assert_eq!(
        h.app
            .session
            .editor
            .book
            .get("C1".parse().unwrap())
            .unwrap()
            .source,
        "=A1+B1"
    );
    h.formula_enter("A1", "6");
    assert_eq!(h.value("C1"), "9");
    assert_eq!(
        persistence::load(&path)
            .unwrap()
            .get("A1".parse().unwrap())
            .unwrap()
            .source,
        "4"
    );
}
#[test]
fn one_shared_draft_f2_escape_and_formula_bar() {
    let mut h = Desktop::new(1.0);
    h.grid_enter("A1", "2");
    h.cell("A1");
    h.key(Key::F2, Modifiers::NONE);
    assert_eq!(h.app.session.surface, Surface::Grid);
    assert_eq!(h.app.session.draft.source, "2");
    h.key(Key::A, command());
    h.text("4");
    assert_eq!(h.app.session.draft.source, "4");
    h.key(Key::Escape, Modifiers::NONE);
    assert_eq!(h.value("A1"), "2");
    assert_eq!(h.app.session.draft.source, "2");
}
#[test]
fn pointer_and_shift_keyboard_select_real_ranges() {
    let mut h = Desktop::new(1.0);
    h.cell("A1");
    h.key(Key::ArrowRight, Modifiers::SHIFT);
    assert_eq!(h.app.session.selection.label(), "A1:B1");
    h.key(Key::ArrowDown, Modifiers::SHIFT);
    assert_eq!(h.app.session.selection.label(), "A1:B2");
    let from = h.cell_position("A1");
    let to = h.cell_position("C3");
    h.drag(from, to);
    assert_eq!(h.app.session.selection.label(), "A1:C3");
}
#[test]
fn header_drag_resizes_without_editing_content() {
    let mut h = Desktop::new(1.0);
    h.grid_enter("A1", "00123");
    let book = h.app.session.editor.book.clone();
    let edge = pos2(
        h.app.probes.cell_origin.x + h.app.metrics.column(1),
        h.app.probes.grid.top() - 12.0,
    );
    h.drag(edge, edge + vec2(30.0, 0.0));
    assert!(
        h.app.metrics.widths[0] >= 109.0,
        "width {}",
        h.app.metrics.widths[0]
    );
    let edge = pos2(
        h.app.probes.grid.left() - 22.0,
        h.app.probes.cell_origin.y + h.app.metrics.row(1),
    );
    h.drag(edge, edge + vec2(0.0, 12.0));
    assert!(h.app.metrics.heights[0] >= 31.0);
    assert_eq!(h.app.session.editor.book, book);
    assert_eq!(h.value("A1"), "00123");
}
#[test]
fn headless_layout_at_three_simulated_dpi_values_is_virtualized() {
    for scale in [1.0, 1.25, 1.5] {
        let h = Desktop::new(scale);
        assert!(h.app.probes.painted_cells > 0);
        assert!(
            h.app.probes.painted_cells < 1000,
            "{} cells",
            h.app.probes.painted_cells
        );
        assert!(h.app.probes.name.right() < h.app.probes.formula.left());
        assert!(h.app.probes.formula.bottom() <= h.app.probes.grid.top());
        assert!(h.app.probes.grid.width() > 900.0);
        assert!(h.app.probes.grid.height() > 350.0);
    }
}
#[test]
fn approved_logo_has_transparent_corners() {
    let image = image::load_from_memory(include_bytes!("../assets/breakfree-sheets.webp"))
        .unwrap()
        .into_rgba8();
    for (x, y) in [(0, 0), (191, 0), (0, 191), (191, 191)] {
        assert_eq!(image.get_pixel(x, y)[3], 0);
    }
    assert!(image.get_pixel(96, 96)[3] > 0);
}

#[test]
fn actual_unsaved_dialog_cancel_discard_and_save_include_pending_entry() {
    let mut h = Desktop::new(1.0);
    h.cell("A1");
    h.text("pending");
    h.key(Key::N, command());
    let cancel = h.app.probes.confirm_cancel.center();
    h.click(cancel, Modifiers::NONE);
    assert_eq!(h.app.session.draft.source, "pending");
    assert!(h.app.session.dirty());
    h.key(Key::N, command());
    let discard = h.app.probes.confirm_discard.center();
    h.click(discard, Modifiers::NONE);
    assert!(h.app.session.editor.book.cells().is_empty());
    assert!(!h.app.session.dirty());
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("draft.bfsheet");
    h.app.session.path = Some(path.clone());
    h.cell("A1");
    h.text("saved draft");
    h.key(Key::N, command());
    let save = h.app.probes.confirm_save.center();
    h.click(save, Modifiers::NONE);
    assert_eq!(
        persistence::load(&path)
            .unwrap()
            .get("A1".parse().unwrap())
            .unwrap()
            .source,
        "saved draft"
    );
    assert!(h.app.session.editor.book.cells().is_empty());
    assert!(!h.app.session.dirty());
}

#[test]
fn failed_save_does_not_complete_destructive_new_action() {
    let mut h = Desktop::new(1.0);
    let directory = tempfile::tempdir().unwrap();
    h.app.session.path = Some(directory.path().join("missing/book.bfsheet"));
    h.cell("A1");
    h.text("2");
    h.key(Key::N, command());
    let save = h.app.probes.confirm_save.center();
    h.click(save, Modifiers::NONE);
    assert!(h.app.error.is_some());
    assert!(h.app.session.dirty());
    assert_eq!(h.value("A1"), "2");
    let cancel = h.app.probes.confirm_cancel.center();
    h.click(cancel, Modifiers::NONE);
    assert_eq!(h.value("A1"), "2");
}

#[test]
fn native_copy_cut_paste_events_preserve_typed_identifiers() {
    let mut h = Desktop::new(1.0);
    h.grid_enter("A1", "00123");
    h.cell("A1");
    h.frame(vec![Event::Copy]);
    h.cell("B1");
    h.frame(vec![Event::Paste("00123".into())]);
    h.key(Key::Enter, Modifiers::NONE);
    assert_eq!(h.value("B1"), "00123");
    assert_eq!(
        h.app
            .session
            .editor
            .book
            .get("B1".parse().unwrap())
            .unwrap()
            .kind,
        Kind::Text
    );
    h.cell("A1");
    h.frame(vec![Event::Cut]);
    assert!(
        h.app
            .session
            .editor
            .book
            .get("A1".parse().unwrap())
            .is_none()
    );
    h.cell("C1");
    h.frame(vec![Event::Paste("00123".into())]);
    h.key(Key::Enter, Modifiers::NONE);
    assert_eq!(h.value("C1"), "00123");
}

#[test]
fn actual_name_box_navigates_to_a_rectangular_range() {
    let mut h = Desktop::new(1.0);
    h.click(h.app.probes.name.center(), Modifiers::NONE);
    h.key(Key::A, command());
    h.text("A1:C4");
    h.key(Key::Enter, Modifiers::NONE);
    assert_eq!(h.app.session.selection.label(), "A1:C4");
}

#[test]
fn wheel_scroll_moves_data_but_keeps_headers_and_bounded_painting() {
    let mut h = Desktop::new(1.0);
    let before = h.app.probes.cell_origin;
    let top = h.app.probes.grid.top();
    h.frame(vec![
        Event::PointerMoved(h.app.probes.grid.center()),
        Event::MouseWheel {
            unit: eframe::egui::MouseWheelUnit::Point,
            delta: vec2(0.0, -500.0),
            modifiers: Modifiers::NONE,
        },
    ]);
    for _ in 0..15 {
        h.frame(vec![]);
    }
    assert!(h.app.probes.cell_origin.y < before.y);
    assert_eq!(h.app.probes.grid.top(), top);
    assert!(h.app.probes.painted_cells < 1000);
}
