use super::{
    icons::{Icon, command},
    *,
};
const UNAVAILABLE: &str = "Not available in this iteration";
fn small(ui: &mut Ui, icon: Icon, label: &str, width: f32) {
    command(ui, icon, label, vec2(width, 23.0), false, false).on_disabled_hover_text(UNAVAILABLE);
}
fn large(ui: &mut Ui, icon: Icon, label: &str, width: f32) {
    command(ui, icon, label, vec2(width, 70.0), false, true).on_disabled_hover_text(UNAVAILABLE);
}
fn group(ui: &mut Ui, width: f32, title: &str, content: impl FnOnce(&mut Ui)) -> Rect {
    let (rect, _) = ui.allocate_exact_size(vec2(width, 94.0), Sense::hover());
    let body = Rect::from_min_max(rect.min + vec2(4.0, 2.0), rect.max - vec2(4.0, 16.0));
    let mut child = ui.new_child(
        UiBuilder::new()
            .max_rect(body)
            .layout(Layout::top_down(Align::Min)),
    );
    child.set_clip_rect(body.intersect(ui.clip_rect()));
    child.spacing_mut().item_spacing = vec2(2.0, 3.0);
    content(&mut child);
    ui.painter().text(
        pos2(rect.center().x, rect.bottom() - 8.0),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(11.0),
        Color32::from_gray(90),
    );
    ui.painter().line_segment(
        [
            rect.right_top() + vec2(0.0, 4.0),
            rect.right_bottom() - vec2(0.0, 3.0),
        ],
        Stroke::new(1.0_f32, Color32::from_gray(222)),
    );
    rect
}
impl Application {
    pub(super) fn home(&mut self, ui: &mut Ui, ctx: &Context) {
        self.probes.ribbon_groups.clear();
        let output = ScrollArea::horizontal().id_salt("ribbon-scroll").show(ui, |ui| {
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 2.0;
                let rect = group(ui, 168.0, "Clipboard", |ui| {
                    ui.horizontal_top(|ui| {
                        let paste = command(ui, Icon::Paste, "Paste", vec2(47.0,70.0), self.clipboard.is_some() && self.session.selection.is_single(), true)
                            .on_hover_text("Paste one copied cell; formula references stay literal")
                            .on_disabled_hover_text("Copy one cell first; range paste is unavailable");
                        self.probes.commands.insert("paste", paste.rect);
                        if paste.clicked() { self.paste(); }
                        ui.vertical(|ui| {
                            let cut = command(ui, Icon::Cut,"Cut",vec2(108.0,23.0),self.session.selection.is_single(),false).on_disabled_hover_text("Range cut is unavailable");
                            self.probes.commands.insert("cut",cut.rect);
                            if cut.clicked() { self.copy(ctx,true); }
                            let copy = command(ui,Icon::Copy,"Copy",vec2(108.0,23.0),self.session.selection.is_single(),false).on_disabled_hover_text("Range copy is unavailable");
                            self.probes.commands.insert("copy",copy.rect);
                            if copy.clicked() { self.copy(ctx,false); }
                            small(ui,Icon::Brush,"Format Painter",108.0);
                        });
                    });
                });
                self.probes.ribbon_groups.push(rect);
                let rect = group(ui, 181.0, "Font", |ui| {
                    ui.horizontal(|ui| {
                        ui.add_enabled(false, Button::new("Default font").min_size(vec2(123.0,23.0))).on_disabled_hover_text("Font selection is not available; using bundled fonts");
                        ui.add_enabled(false, Button::new("13").min_size(vec2(43.0,23.0))).on_disabled_hover_text("Font size is not available");
                    });
                    ui.add_space(9.0);
                    ui.horizontal(|ui| {
                        for (icon,label) in [(Icon::Bold,"Bold"),(Icon::Italic,"Italic"),(Icon::Underline,"Underline"),(Icon::Borders,"Borders"),(Icon::Fill,"Fill color"),(Icon::FontColor,"Font color")] {
                            command(ui,icon,"",vec2(26.0,25.0),false,false).on_disabled_hover_text(format!("{label}: {UNAVAILABLE}"));
                        }
                    });
                });
                self.probes.ribbon_groups.push(rect);
                let rect = group(ui, 222.0, "Alignment", |ui| {
                    ui.horizontal(|ui| {
                        for kind in 0..3 { small(ui,Icon::Align(kind),"",23.0); }
                        small(ui,Icon::Wrap,"Wrap Text",128.0);
                    });
                    ui.add_space(9.0);
                    ui.horizontal(|ui| {
                        for kind in 3..6 { small(ui,Icon::Align(kind),"",23.0); }
                        small(ui,Icon::Merge,"Merge & Center",128.0);
                    });
                });
                self.probes.ribbon_groups.push(rect);
                let rect = group(ui, 135.0, "Number", |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Cell type").size(10.0).color(Color32::from_gray(100)));
                    });
                    ComboBox::from_id_salt("source-kind").selected_text(self.session.draft.kind.label()).width(121.0).height(120.0).show_ui(ui, |ui| {
                        for kind in [InputKind::Auto,InputKind::Text,InputKind::Number,InputKind::Formula] {
                            if ui.selectable_value(&mut self.session.draft.kind,kind,kind.label()).changed() { self.session.surface=Surface::Formula; }
                        }
                    }).response.on_hover_text("Stored content type. Changing it requires applying the cell entry; this is not a display format.");
                    ui.horizontal(|ui| {
                        for text in ["$", "%", ",", ".00"] { ui.add_enabled(false,Button::new(text).frame(false).min_size(vec2(25.0,22.0))).on_disabled_hover_text("Display number formatting is unavailable"); }
                    });
                });
                self.probes.ribbon_groups.push(rect);
                let rect = group(ui, 182.0, "Styles", |ui| {
                    ui.horizontal_top(|ui| {
                        large(ui,Icon::Conditional,"Conditional\nFormatting",65.0);
                        large(ui,Icon::Table,"Format as\nTable",57.0);
                        large(ui,Icon::Styles,"Cell\nStyles",46.0);
                    });
                });
                self.probes.ribbon_groups.push(rect);
                let rect = group(ui, 144.0, "Cells", |ui| {
                    ui.horizontal_top(|ui| {
                        large(ui,Icon::Insert,"Insert",42.0);
                        large(ui,Icon::Delete,"Delete",42.0);
                        let response = command(ui,Icon::Format,"Format",vec2(46.0,70.0),true,true).on_hover_text("Column width and row height (view only)");
                        self.probes.commands.insert("format",response.rect);
                        Popup::menu(&response).show(|ui| {
                            let active = self.session.selection.active;
                            ui.label("Cell size (view only)");
                            ui.horizontal(|ui| { ui.label("Column width"); ui.add(DragValue::new(&mut self.metrics.widths[active.col]).range(32.0..=400.0).speed(1.0)); });
                            ui.horizontal(|ui| { ui.label("Row height"); ui.add(DragValue::new(&mut self.metrics.heights[active.row]).range(16.0..=100.0).speed(1.0)); });
                            ui.separator();
                            ui.add_enabled(false, Button::new("Format Cells…")).on_disabled_hover_text(UNAVAILABLE);
                        });
                    });
                });
                self.probes.ribbon_groups.push(rect);
                let rect = group(ui, 210.0, "Editing", |ui| {
                    ui.horizontal_top(|ui| {
                        ui.vertical(|ui| {
                            small(ui,Icon::Sum,"AutoSum",90.0);
                            small(ui,Icon::Paste,"Fill",90.0);
                            let clear = command(ui,Icon::Clear,"Clear",vec2(90.0,23.0),true,false).on_hover_text("Clear selected contents in one undo step");
                            self.probes.commands.insert("clear",clear.rect);
                            if clear.clicked() { let result=self.session.clear(); self.report(result); }
                        });
                        large(ui,Icon::Sort,"Sort &\nFilter",53.0);
                        large(ui,Icon::Find,"Find &\nSelect",53.0);
                    });
                });
                self.probes.ribbon_groups.push(rect);
            });
        });
        self.probes.ribbon_viewport = output.inner_rect;
    }
}

impl Application {
    pub(super) fn view_ribbon(&mut self, ui: &mut Ui, ctx: &Context) {
        let output = ScrollArea::horizontal()
            .id_salt("view-ribbon-scroll")
            .show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = 2.0;
                    let rect = group(ui, 285.0, "Workbook Views", |ui| {
                        ui.horizontal_top(|ui| {
                            let normal =
                                command(ui, Icon::Grid, "Normal", vec2(57.0, 70.0), true, true)
                                    .on_hover_text("Normal worksheet view is active");
                            ui.painter().rect_stroke(
                                normal.rect.shrink(1.0),
                                2,
                                Stroke::new(1.0_f32, TEAL),
                                StrokeKind::Inside,
                            );
                            large(ui, Icon::PageBreak, "Page Break\nPreview", 76.0);
                            large(ui, Icon::PageLayout, "Page\nLayout", 64.0);
                            large(ui, Icon::Grid, "Custom\nViews", 64.0);
                        });
                    });
                    self.probes.ribbon_groups.push(rect);
                    let rect = group(ui, 224.0, "Show", |ui| {
                        ui.horizontal_top(|ui| {
                            ui.vertical(|ui| {
                                ui.add_enabled(false, Checkbox::new(&mut false, "Ruler"))
                                    .on_disabled_hover_text(
                                        "Ruler requires an unavailable page layout view",
                                    );
                                let gridlines = ui.checkbox(&mut self.show_gridlines, "Gridlines");
                                self.probes
                                    .commands
                                    .insert("show-gridlines", gridlines.rect);
                                gridlines.on_hover_text(
                                    "Show worksheet gridlines; cell contents are unchanged",
                                );
                            });
                            ui.add_space(10.0);
                            ui.vertical(|ui| {
                                let formula =
                                    ui.checkbox(&mut self.show_formula_bar, "Formula Bar");
                                self.probes
                                    .commands
                                    .insert("show-formula-bar", formula.rect);
                                if formula.changed() && !self.show_formula_bar {
                                    self.reset_focus(ctx);
                                    // Keep the one shared draft editable when its formula surface is hidden.
                                    if self.session.surface == Surface::Formula {
                                        self.session.surface = Surface::Grid;
                                        self.focus_editor = true;
                                    }
                                }
                                let headings = ui.checkbox(&mut self.show_headings, "Headings");
                                self.probes.commands.insert("show-headings", headings.rect);
                                if headings.changed() {
                                    self.reveal = true;
                                }
                            });
                        });
                    });
                    self.probes.ribbon_groups.push(rect);
                    let rect = group(ui, 184.0, "Zoom", |ui| {
                        ui.horizontal_top(|ui| {
                            let zoom =
                                command(ui, Icon::Find, "Zoom", vec2(51.0, 70.0), true, true);
                            Popup::menu(&zoom).show(|ui| {
                                ui.label("Worksheet zoom");
                                ui.add(
                                    Slider::new(&mut self.metrics.zoom, 0.5..=2.0)
                                        .step_by(0.05)
                                        .show_value(false),
                                );
                                ui.label(format!("{}%", (self.metrics.zoom * 100.0).round()));
                            });
                            let reset =
                                command(ui, Icon::Zoom100, "100%", vec2(51.0, 70.0), true, true)
                                    .on_hover_text("Restore worksheet zoom to 100%");
                            self.probes.commands.insert("view-zoom-reset", reset.rect);
                            if reset.clicked() {
                                self.metrics.zoom = 1.0;
                                self.reveal = true;
                            }
                            large(ui, Icon::Grid, "Zoom to\nSelection", 66.0);
                        });
                    });
                    self.probes.ribbon_groups.push(rect);
                    let rect = group(ui, 293.0, "Window", |ui| {
                        ui.horizontal_top(|ui| {
                            large(ui, Icon::New, "New\nWindow", 57.0);
                            large(ui, Icon::Grid, "Arrange\nAll", 56.0);
                            large(ui, Icon::Freeze, "Freeze\nPanes", 59.0);
                            ui.vertical(|ui| {
                                small(ui, Icon::Grid, "Split", 99.0);
                                small(ui, Icon::Page, "Hide", 99.0);
                                small(ui, Icon::Page, "Unhide", 99.0);
                            });
                        });
                    });
                    self.probes.ribbon_groups.push(rect);
                    let rect = group(ui, 76.0, "Macros", |ui| {
                        large(ui, Icon::Page, "Macros", 65.0);
                    });
                    self.probes.ribbon_groups.push(rect);
                });
            });
        self.probes.ribbon_viewport = output.inner_rect;
    }
}
