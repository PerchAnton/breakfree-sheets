//! Native Rust desktop application. Painting and interaction never evaluate formulas.
mod chrome;
mod grid;
mod icons;
mod ribbon;
use crate::{
    cells::Cell,
    interaction::{GridMetrics, InputKind, Session, Surface},
    persistence,
};
use eframe::egui::{self, *};
use std::path::PathBuf;
pub const TEAL: Color32 = Color32::from_rgb(36, 138, 131);
pub const INK: Color32 = Color32::from_rgb(36, 19, 55);
#[derive(Clone)]
pub struct Probes {
    pub commands: std::collections::BTreeMap<&'static str, Rect>,
    pub ribbon_tabs: std::collections::BTreeMap<&'static str, Rect>,
    pub ribbon_groups: Vec<Rect>,
    pub ribbon_viewport: Rect,
    pub formula: Rect,
    pub name: Rect,
    pub grid: Rect,
    pub active: Rect,
    pub cell_origin: Pos2,
    pub painted_cells: usize,
    pub confirm_save: Rect,
    pub confirm_discard: Rect,
    pub confirm_cancel: Rect,
}
impl Default for Probes {
    fn default() -> Self {
        Self {
            commands: Default::default(),
            ribbon_tabs: Default::default(),
            ribbon_groups: Vec::new(),
            ribbon_viewport: Rect::NOTHING,
            formula: Rect::NOTHING,
            name: Rect::NOTHING,
            grid: Rect::NOTHING,
            active: Rect::NOTHING,
            cell_origin: Pos2::ZERO,
            painted_cells: 0,
            confirm_save: Rect::NOTHING,
            confirm_discard: Rect::NOTHING,
            confirm_cancel: Rect::NOTHING,
        }
    }
}
#[derive(Clone)]
enum Action {
    New,
    Open(PathBuf),
    Close,
}
pub struct Application {
    pub session: Session,
    pub metrics: GridMetrics,
    pub probes: Probes,
    tab: String,
    backstage: bool,
    ribbon_collapsed: bool,
    ribbon_before_tab_click: Option<(&'static str, bool)>,
    show_gridlines: bool,
    show_headings: bool,
    show_formula_bar: bool,
    name: String,
    logo: Option<TextureHandle>,
    pending: Option<Action>,
    pub error: Option<String>,
    allow_close: bool,
    focus_editor: bool,
    reveal: bool,
    clipboard: Option<Cell>,
    #[cfg(feature = "capture")]
    capture_requested: bool,
}
impl Application {
    pub fn new(session: Session) -> Self {
        let name = session.selection.label();
        Self {
            session,
            metrics: GridMetrics::default(),
            probes: Probes::default(),
            tab: "Home".into(),
            backstage: false,
            ribbon_collapsed: false,
            ribbon_before_tab_click: None,
            show_gridlines: true,
            show_headings: true,
            show_formula_bar: true,
            name,
            logo: None,
            pending: None,
            error: None,
            allow_close: false,
            focus_editor: false,
            reveal: false,
            clipboard: None,
            #[cfg(feature = "capture")]
            capture_requested: false,
        }
    }
    fn configure(&mut self, ctx: &Context) {
        if self.logo.is_some() {
            return;
        }
        ctx.set_visuals(Visuals::light());
        ctx.style_mut(|style| {
            style.spacing.item_spacing = vec2(6.0, 3.0);
            style.spacing.button_padding = vec2(5.0, 2.0);
            style.spacing.interact_size.y = 22.0;
            style.spacing.scroll = egui::style::ScrollStyle::solid();
            style.spacing.scroll.bar_width = 12.0;
            style.visuals.selection.bg_fill = TEAL;
            style.visuals.selection.stroke = Stroke::new(1.0_f32, Color32::WHITE);
            style.visuals.panel_fill = Color32::WHITE;
            style.visuals.disabled_alpha = 0.72;
            style.visuals.extreme_bg_color = Color32::WHITE;
            style.visuals.widgets.inactive.bg_fill = Color32::from_gray(222);
            style.visuals.widgets.inactive.weak_bg_fill = Color32::WHITE;
            style.visuals.widgets.inactive.bg_stroke =
                Stroke::new(1.0_f32, Color32::from_gray(208));
            style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(230, 241, 239);
            style.visuals.widgets.active.bg_fill = Color32::from_rgb(220, 239, 235);
            style.visuals.widgets.inactive.corner_radius = CornerRadius::same(2);
            style
                .text_styles
                .insert(TextStyle::Body, FontId::proportional(13.0));
            style
                .text_styles
                .insert(TextStyle::Button, FontId::proportional(13.0));
            style
                .text_styles
                .insert(TextStyle::Small, FontId::proportional(11.0));
        });
        let rgba = image::load_from_memory(include_bytes!("../assets/breakfree-sheets.webp"))
            .expect("approved embedded icon")
            .into_rgba8();
        self.logo = Some(ctx.load_texture(
            "BreakFree Sheets",
            ColorImage::from_rgba_unmultiplied(
                [rgba.width() as usize, rgba.height() as usize],
                rgba.as_raw(),
            ),
            TextureOptions::LINEAR,
        ));
    }
    fn report(&mut self, result: Result<(), String>) -> bool {
        match result {
            Ok(()) => {
                self.error = None;
                true
            }
            Err(error) => {
                self.error = Some(error);
                false
            }
        }
    }
    fn reset_focus(&self, ctx: &Context) {
        ctx.memory_mut(|m| {
            m.surrender_focus(Id::new("formula-source"));
            m.surrender_focus(Id::new("grid-source"));
            m.surrender_focus(Id::new("name-box"));
        });
    }
    fn selected(&mut self) {
        self.name = self.session.selection.label();
        self.reveal = true;
    }
    fn commit_move(&mut self, ctx: &Context, dx: i32, dy: i32, extend: bool) {
        let result = self.session.move_by(dx, dy, extend);
        if self.report(result) {
            self.selected();
            self.reset_focus(ctx);
        }
    }
    fn keyboard(&mut self, ctx: &Context) {
        if self.pending.is_some() {
            return;
        }
        if self.backstage {
            if ctx.input(|i| i.key_pressed(Key::Escape)) {
                self.backstage = false;
            }
            if ctx.input(|i| i.modifiers.command && i.key_pressed(Key::S)) {
                self.save(false);
            }
            if ctx.input(|i| i.modifiers.command && i.key_pressed(Key::N)) {
                self.request(Action::New, ctx);
            }
            if ctx.input(|i| i.modifiers.command && i.key_pressed(Key::O)) {
                self.choose_open(ctx);
            }
            return;
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::F1)) {
            self.ribbon_collapsed = !self.ribbon_collapsed;
            self.ribbon_before_tab_click = None;
        }
        let events = ctx.input(|i| i.events.clone());
        for event in events {
            match event {
                Event::Copy if self.session.surface == Surface::Selection => self.copy(ctx, false),
                Event::Cut if self.session.surface == Surface::Selection => self.copy(ctx, true),
                Event::Key {
                    key,
                    pressed: true,
                    modifiers,
                    ..
                } if modifiers.command => match key {
                    Key::S => {
                        self.save(modifiers.shift);
                    }
                    Key::N => self.request(Action::New, ctx),
                    Key::O => self.choose_open(ctx),
                    Key::Z => {
                        if modifiers.shift {
                            self.session.redo();
                        } else {
                            self.session.undo();
                        }
                        self.selected();
                        self.reset_focus(ctx);
                    }
                    Key::Y => {
                        self.session.redo();
                        self.selected();
                        self.reset_focus(ctx);
                    }

                    _ => {}
                },
                Event::Key {
                    key: Key::Escape,
                    pressed: true,
                    ..
                } => {
                    self.session.reset_draft();
                    self.name = self.session.selection.label();
                    self.reset_focus(ctx);
                }
                Event::Key {
                    key: Key::F2,
                    pressed: true,
                    ..
                } => {
                    self.session.begin_edit(Surface::Grid, None);
                    self.focus_editor = true;
                }
                Event::Key {
                    key: Key::Enter,
                    pressed: true,
                    modifiers,
                    ..
                } if self.session.surface != Surface::Selection => {
                    self.commit_move(ctx, 0, if modifiers.shift { -1 } else { 1 }, false)
                }
                Event::Key {
                    key: Key::Tab,
                    pressed: true,
                    modifiers,
                    ..
                } if self.session.surface != Surface::Selection => {
                    self.commit_move(ctx, if modifiers.shift { -1 } else { 1 }, 0, false)
                }
                Event::Key {
                    key,
                    pressed: true,
                    modifiers,
                    ..
                } if !ctx.wants_keyboard_input() && self.session.surface == Surface::Selection => {
                    match key {
                        Key::ArrowLeft => self.commit_move(ctx, -1, 0, modifiers.shift),
                        Key::ArrowRight => self.commit_move(ctx, 1, 0, modifiers.shift),
                        Key::ArrowUp => self.commit_move(ctx, 0, -1, modifiers.shift),
                        Key::ArrowDown | Key::Enter => self.commit_move(ctx, 0, 1, modifiers.shift),
                        Key::Tab => {
                            self.commit_move(ctx, if modifiers.shift { -1 } else { 1 }, 0, false)
                        }
                        Key::Delete | Key::Backspace => {
                            let result = self.session.clear();
                            self.report(result);
                        }
                        _ => {}
                    }
                }
                Event::Text(text) | Event::Paste(text)
                    if self.session.surface == Surface::Selection
                        && !ctx.wants_keyboard_input() =>
                {
                    if text.contains(['\t', '\n', '\r']) {
                        self.error =
                            Some("Range paste is not available; paste one cell at a time".into());
                        continue;
                    }
                    let copied_kind = self
                        .clipboard
                        .as_ref()
                        .filter(|cell| cell.source == text)
                        .map(|cell| cell.kind);
                    self.session.begin_edit(Surface::Grid, Some(text));
                    if let Some(kind) = copied_kind {
                        self.session.draft.kind = match kind {
                            crate::cells::Kind::Text => InputKind::Text,
                            crate::cells::Kind::Number => InputKind::Number,
                            crate::cells::Kind::Formula => InputKind::Formula,
                        };
                    }
                    self.focus_editor = true;
                    // The draft already contains these characters; do not insert them twice in TextEdit.
                    ctx.input_mut(|i| {
                        i.events
                            .retain(|e| !matches!(e, Event::Text(_) | Event::Paste(_)))
                    });
                }
                _ => {}
            }
        }
    }
    fn choose_open(&mut self, ctx: &Context) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("BreakFree Sheets", &["bfsheet"])
            .pick_file()
            && self.report(persistence::load(&path).map(|_| ()))
        {
            self.request(Action::Open(path), ctx);
        }
    }
    pub fn save_to(&mut self, path: &std::path::Path) -> bool {
        let result = self.session.save_to(path);
        self.report(result)
    }
    fn save(&mut self, choose: bool) -> bool {
        let path = if choose || self.session.path.is_none() {
            rfd::FileDialog::new()
                .add_filter("BreakFree Sheets", &["bfsheet"])
                .set_file_name("Book1.bfsheet")
                .save_file()
        } else {
            self.session.path.clone()
        };
        path.is_some_and(|path| self.save_to(&path))
    }
    fn request(&mut self, action: Action, ctx: &Context) {
        if self.session.dirty() {
            self.pending = Some(action);
        } else {
            self.execute(action, ctx);
        }
    }
    fn execute(&mut self, action: Action, ctx: &Context) {
        self.backstage = false;
        match action {
            Action::New => {
                self.session = Session::default();
                self.metrics = GridMetrics::default();
                self.selected();
            }
            Action::Open(path) => {
                let result = self.session.open(&path);
                if self.report(result) {
                    self.metrics = GridMetrics::default();
                    self.selected();
                }
            }
            Action::Close => {
                self.allow_close = true;
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
        }
    }
    fn copy(&mut self, ctx: &Context, cut: bool) {
        if !self.session.selection.is_single() {
            self.error = Some("Range copy/cut is not available in this iteration".into());
            return;
        }
        self.clipboard = self
            .session
            .editor
            .book
            .get(self.session.selection.active)
            .cloned();
        ctx.copy_text(
            self.clipboard
                .as_ref()
                .map(|c| c.source.clone())
                .unwrap_or_default(),
        );
        if cut {
            let result = self.session.clear();
            self.report(result);
        }
    }
    fn paste(&mut self) {
        if let Some(cell) = self.clipboard.clone() {
            let result = self
                .session
                .editor
                .set(self.session.selection.active, Some(cell));
            self.report(result);
            self.session.reset_draft();
        }
    }
    pub fn ui(&mut self, ctx: &Context) {
        self.configure(ctx);
        #[cfg(feature = "capture")]
        self.capture(ctx);
        if ctx.input(|i| i.viewport().close_requested())
            && !self.allow_close
            && self.session.dirty()
        {
            ctx.send_viewport_cmd(ViewportCommand::CancelClose);
            self.pending = Some(Action::Close);
        }
        self.keyboard(ctx);
        self.probes = Probes::default();
        ctx.send_viewport_cmd(ViewportCommand::Title(self.session.title()));
        self.title_strip(ctx);
        if self.backstage {
            self.file_page(ctx);
        } else {
            self.ribbon_chrome(ctx);
            if self.show_formula_bar {
                self.formula_bar(ctx);
            }
            self.message_bar(ctx);
            self.bottom_chrome(ctx);
            CentralPanel::default()
                .frame(Frame::NONE.fill(Color32::WHITE))
                .show(ctx, |ui| {
                    ui.add_enabled_ui(self.pending.is_none(), |ui| self.grid(ui, ctx));
                });
        }
        if ctx.input(|i| i.pointer.any_click())
            && !ctx.input(|i| i.pointer.interact_pos()).is_some_and(|p| {
                self.probes
                    .ribbon_tabs
                    .values()
                    .any(|rect| rect.contains(p))
            })
        {
            self.ribbon_before_tab_click = None;
        }
        if let Some(action) = self.pending.clone() {
            Window::new("Unsaved changes")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label("Save changes before continuing? The pending cell entry is included.");
                    ui.horizontal(|ui| {
                        let save = ui.button("Save");
                        self.probes.confirm_save = save.rect;
                        if save.clicked() && self.save(false) {
                            self.pending = None;
                            self.execute(action.clone(), ctx);
                        }
                        let discard = ui.button("Discard");
                        self.probes.confirm_discard = discard.rect;
                        if discard.clicked() {
                            self.pending = None;
                            self.execute(action.clone(), ctx);
                        }
                        let cancel = ui.button("Cancel");
                        self.probes.confirm_cancel = cancel.rect;
                        if cancel.clicked() {
                            self.pending = None;
                        }
                    });
                });
        }
    }
}
#[cfg(feature = "capture")]
impl Application {
    fn capture(&mut self, ctx: &Context) {
        let Ok(path) = std::env::var("BREAKFREE_CAPTURE_TO") else {
            return;
        };
        for event in ctx.input(|i| i.events.clone()) {
            if let Event::Screenshot { image, .. } = event {
                let pixels: Vec<u8> = image.pixels.iter().flat_map(|c| c.to_array()).collect();
                image::save_buffer(
                    &path,
                    &pixels,
                    image.width() as u32,
                    image.height() as u32,
                    image::ColorType::Rgba8,
                )
                .expect("save requested own-app capture");
                self.allow_close = true;
                ctx.send_viewport_cmd(ViewportCommand::Close);
                return;
            }
        }
        if !self.capture_requested && ctx.cumulative_frame_nr() >= 10 {
            ctx.send_viewport_cmd(ViewportCommand::Screenshot(Default::default()));
            self.capture_requested = true;
        }
        ctx.request_repaint();
    }
}
impl eframe::App for Application {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.ui(ctx);
    }
}
pub fn run(session: Session) -> eframe::Result {
    let rgba = image::load_from_memory(include_bytes!("../assets/breakfree-sheets.webp"))
        .expect("embedded logo")
        .into_rgba8();
    let icon = IconData {
        width: rgba.width(),
        height: rgba.height(),
        rgba: rgba.into_raw(),
    };
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1366.0, 768.0])
            .with_min_inner_size([980.0, 600.0])
            .with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        "BreakFree Sheets",
        options,
        Box::new(|cc| {
            #[cfg(feature = "capture")]
            if let Ok(value) = std::env::var("BREAKFREE_CAPTURE_SCALE")
                && let Ok(scale) = value.parse::<f32>()
            {
                cc.egui_ctx.set_zoom_factor(scale.clamp(1.0, 1.5));
            }
            let _ = cc;
            let mut app = Application::new(session);
            #[cfg(feature = "capture")]
            if let Ok(address) = std::env::var("BREAKFREE_CAPTURE_CELL") {
                let _ = app.session.go_to(&address);
                app.selected();
            }
            #[cfg(feature = "capture")]
            match std::env::var("BREAKFREE_CAPTURE_VIEW").as_deref() {
                Ok("file") => app.backstage = true,
                Ok("view") => app.tab = "View".into(),
                Ok("collapsed") => app.ribbon_collapsed = true,
                _ => {}
            }
            let _ = &mut app;
            Ok(Box::new(app))
        }),
    )
}
