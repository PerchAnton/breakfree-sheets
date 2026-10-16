use super::{
    icons::{Icon, command, paint_icon},
    *,
};
const CHROME: Color32 = Color32::from_rgb(245, 247, 246);
const EDGE: Color32 = Color32::from_rgb(211, 217, 215);
impl Application {
    pub(super) fn title_strip(&mut self, ctx: &Context) {
        TopBottomPanel::top("title-strip")
            .exact_height(38.0)
            .frame(
                Frame::NONE
                    .fill(Color32::from_rgb(234, 243, 240))
                    .inner_margin(Margin::symmetric(12, 5)),
            )
            .show(ctx, |ui| {
                ui.add_enabled_ui(self.pending.is_none(), |ui| {
                    ui.horizontal(|ui| {
                        if let Some(logo) = &self.logo {
                            ui.image((logo.id(), vec2(24.0, 24.0)));
                        }
                        ui.add_space(4.0);
                        let save = command(ui, Icon::Save, "", vec2(28.0, 26.0), true, false)
                            .on_hover_text("Save (Ctrl+S)");
                        self.probes.commands.insert("save", save.rect);
                        if save.clicked() {
                            self.save(false);
                        }
                        let undo = command(
                            ui,
                            Icon::Undo,
                            "",
                            vec2(28.0, 26.0),
                            self.session.editor.can_undo() || self.session.draft_dirty(),
                            false,
                        )
                        .on_hover_text("Undo (Ctrl+Z)")
                        .on_disabled_hover_text("Nothing to undo");
                        self.probes.commands.insert("undo", undo.rect);
                        if undo.clicked() {
                            self.session.undo();
                            self.selected();
                            self.reset_focus(ctx);
                        }
                        let redo = command(
                            ui,
                            Icon::Redo,
                            "",
                            vec2(28.0, 26.0),
                            self.session.editor.can_redo(),
                            false,
                        )
                        .on_hover_text("Redo (Ctrl+Y)")
                        .on_disabled_hover_text("Nothing to redo");
                        self.probes.commands.insert("redo", redo.rect);
                        if redo.clicked() {
                            self.session.redo();
                            self.selected();
                            self.reset_focus(ctx);
                        }
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new(self.session.title().replace(" — BreakFree Sheets", ""))
                                .color(INK)
                                .size(13.0),
                        );
                        ui.label(
                            RichText::new("—  BreakFree Sheets")
                                .color(Color32::from_gray(90))
                                .size(12.0),
                        );
                    });
                });
            });
    }
    pub(super) fn ribbon_chrome(&mut self, ctx: &Context) {
        TopBottomPanel::top("ribbon-tabs")
            .exact_height(31.0)
            .frame(
                Frame::NONE
                    .fill(CHROME)
                    .inner_margin(Margin::symmetric(12, 2)),
            )
            .show(ctx, |ui| {
                ui.add_enabled_ui(self.pending.is_none(), |ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;
                    ui.horizontal(|ui| {
                        let file = ui.add(
                            Button::new(RichText::new("File").color(TEAL))
                                .frame(false)
                                .min_size(vec2(42.0, 26.0)),
                        );
                        self.probes.commands.insert("file", file.rect);
                        if file.clicked() {
                            self.backstage = true;
                            self.reset_focus(ctx);
                        }
                        for tab in [
                            "Home",
                            "Insert",
                            "Page Layout",
                            "Formulas",
                            "Data",
                            "Review",
                            "View",
                            "Help",
                        ] {
                            let selected = self.tab == tab;
                            let response = ui.add(
                                Button::new(RichText::new(tab).color(if selected {
                                    TEAL
                                } else {
                                    Color32::from_gray(50)
                                }))
                                .frame(false)
                                .min_size(vec2(40.0, 26.0)),
                            );
                            self.probes.ribbon_tabs.insert(tab, response.rect);
                            if response.double_clicked()
                                && let Some((previous, collapsed)) = self.ribbon_before_tab_click
                                && previous == tab
                            {
                                self.ribbon_collapsed = !collapsed;
                            } else if response.clicked() {
                                self.ribbon_before_tab_click = Some((tab, self.ribbon_collapsed));
                                self.ribbon_collapsed = false;
                                self.tab = tab.into();
                            }
                            if selected {
                                ui.painter().line_segment(
                                    [
                                        response.rect.left_bottom() + vec2(5.0, -1.0),
                                        response.rect.right_bottom() - vec2(5.0, 1.0),
                                    ],
                                    Stroke::new(2.5_f32, TEAL),
                                );
                            }
                        }
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let label = if self.ribbon_collapsed {
                                "Expand ribbon"
                            } else {
                                "Collapse ribbon"
                            };
                            let (rect, response) =
                                ui.allocate_exact_size(vec2(28.0, 26.0), Sense::click());
                            response.widget_info(|| {
                                WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), label)
                            });
                            let response = response.on_hover_text(format!("{label} (Ctrl+F1)"));
                            if response.hovered() {
                                ui.painter()
                                    .rect_filled(rect, 2, Color32::from_rgb(220, 239, 235));
                            }
                            let direction = if self.ribbon_collapsed { 1.0 } else { -1.0 };
                            let c = rect.center();
                            ui.painter().add(Shape::line(
                                vec![
                                    c + vec2(-4.0, -2.0 * direction),
                                    c + vec2(0.0, 2.0 * direction),
                                    c + vec2(4.0, -2.0 * direction),
                                ],
                                Stroke::new(1.4_f32, INK),
                            ));
                            self.probes.commands.insert("toggle-ribbon", rect);
                            if response.clicked() {
                                self.ribbon_collapsed = !self.ribbon_collapsed;
                            }
                        });
                    });
                });
            });
        if self.ribbon_collapsed {
            return;
        }
        // A solid horizontal scrollbar needs its own height at narrow logical widths.
        // Reserving it keeps group captions inside the ribbon's clip rectangle.
        let ribbon_height = if ctx.available_rect().width() < 1280.0 {
            119.0
        } else {
            105.0
        };
        TopBottomPanel::top("ribbon")
            .exact_height(ribbon_height)
            .frame(
                Frame::NONE
                    .fill(CHROME)
                    .inner_margin(Margin::symmetric(6, 3)),
            )
            .show(ctx, |ui| {
                Frame::NONE
                    .fill(Color32::WHITE)
                    .corner_radius(5)
                    .inner_margin(Margin::symmetric(4, 1))
                    .show(ui, |ui| {
                        ui.set_min_size(vec2(ui.available_width(), 95.0));
                        ui.add_enabled_ui(self.pending.is_none(), |ui| {
                            if self.tab == "Home" {
                                self.home(ui, ctx);
                            } else if self.tab == "View" {
                                self.view_ribbon(ui, ctx);
                            } else {
                                ui.add_space(12.0);
                                ui.label(RichText::new(&self.tab).size(14.0));
                                ui.label(
                                    RichText::new("Commands on this tab are not available yet.")
                                        .color(Color32::from_gray(110)),
                                );
                            }
                        });
                    });
            });
    }
    pub(super) fn formula_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("formula-bar").exact_height(34.0).frame(Frame::NONE.fill(CHROME).inner_margin(Margin::symmetric(6,4))).show(ctx,|ui| {
            ui.add_enabled_ui(self.pending.is_none(),|ui| {
                ui.spacing_mut().item_spacing.x=4.0;
                ui.horizontal(|ui| {
                    let name=ui.add_sized([100.0,25.0],TextEdit::singleline(&mut self.name).id(Id::new("name-box")).margin(vec2(7.0,5.0)));
                    self.probes.name=name.rect;
                    if name.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) { let result=self.session.go_to(&self.name); if self.report(result) {self.selected();} }
                    ui.add_space(5.0);
                    let editing=self.session.surface!=Surface::Selection || self.session.draft_dirty();
                    let cancel=command(ui,Icon::Cancel,"",vec2(26.0,25.0),editing,false).on_hover_text("Cancel entry (Escape)").on_disabled_hover_text("No cell entry to cancel");
                    self.probes.commands.insert("cancel-entry",cancel.rect);
                    if cancel.clicked() {self.session.reset_draft();self.reset_focus(ctx);}
                    let apply=command(ui,Icon::Check,"",vec2(26.0,25.0),editing,false).on_hover_text("Apply cell entry").on_disabled_hover_text("No cell entry to apply");
                    self.probes.commands.insert("apply-entry",apply.rect);
                    if apply.clicked() {let result=self.session.commit();self.report(result);self.reset_focus(ctx);}
                    command(ui,Icon::Function,"",vec2(29.0,25.0),false,false).on_disabled_hover_text("Functions are unavailable; arithmetic and cell references are supported");
                    let source=ui.add_sized([ui.available_width(),25.0],TextEdit::singleline(&mut self.session.draft.source).id(Id::new("formula-source")).font(FontId::proportional(13.0)).margin(vec2(8.0,5.0)));
                    self.probes.formula=source.rect;
                    if source.has_focus() || source.changed() {self.session.surface=Surface::Formula;}
                });
            });
        });
    }
    pub(super) fn message_bar(&mut self, ctx: &Context) {
        if let Some(error) = self.error.clone() {
            TopBottomPanel::top("message")
                .frame(
                    Frame::NONE
                        .fill(Color32::from_rgb(255, 241, 230))
                        .inner_margin(4),
                )
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(error).color(Color32::from_rgb(140, 45, 24)));
                        if ui.small_button("Dismiss").clicked() {
                            self.error = None;
                        }
                    });
                });
        }
    }
    pub(super) fn bottom_chrome(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("status")
            .exact_height(25.0)
            .frame(
                Frame::NONE
                    .fill(CHROME)
                    .inner_margin(Margin::symmetric(8, 1)),
            )
            .show(ctx, |ui| {
                ui.add_enabled_ui(self.pending.is_none(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(match self.session.surface {
                                Surface::Selection => "Ready",
                                Surface::Grid => "Edit",
                                Surface::Formula => "Enter",
                            })
                            .size(11.0),
                        );
                        ui.separator();
                        ui.label(
                            RichText::new(self.session.selection.label())
                                .size(11.0)
                                .color(Color32::from_gray(90)),
                        );
                        if let Some(crate::cells::Value::Error(error)) = self
                            .session
                            .editor
                            .values
                            .get(&self.session.selection.active)
                        {
                            ui.label(
                                RichText::new(error.code.to_string()).color(Color32::DARK_RED),
                            )
                            .on_hover_text(&error.message);
                        }
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let reset = ui
                                .add(
                                    Button::new(
                                        RichText::new(format!(
                                            "{}%",
                                            (self.metrics.zoom * 100.0).round()
                                        ))
                                        .size(11.0),
                                    )
                                    .frame(false)
                                    .min_size(vec2(40.0, 22.0)),
                                )
                                .on_hover_text("Reset worksheet zoom to 100%");
                            if reset.clicked() {
                                self.metrics.zoom = 1.0;
                            }
                            let plus = command(
                                ui,
                                Icon::Plus,
                                "",
                                vec2(20.0, 22.0),
                                self.metrics.zoom < 2.0,
                                false,
                            )
                            .on_hover_text("Zoom in");
                            self.probes.commands.insert("zoom-in", plus.rect);
                            if plus.clicked() {
                                self.metrics.zoom = (self.metrics.zoom + 0.1).min(2.0);
                            }
                            ui.spacing_mut().slider_width = 104.0;
                            ui.add(
                                Slider::new(&mut self.metrics.zoom, 0.5..=2.0)
                                    .show_value(false)
                                    .step_by(0.05),
                            );
                            let minus = command(
                                ui,
                                Icon::Minus,
                                "",
                                vec2(20.0, 22.0),
                                self.metrics.zoom > 0.5,
                                false,
                            )
                            .on_hover_text("Zoom out");
                            self.probes.commands.insert("zoom-out", minus.rect);
                            if minus.clicked() {
                                self.metrics.zoom = (self.metrics.zoom - 0.1).max(0.5);
                            }
                            ui.separator();
                            command(ui, Icon::PageLayout, "", vec2(28.0, 22.0), false, false)
                                .on_disabled_hover_text("Page layout view is unavailable");
                            let normal = command(ui, Icon::Grid, "", vec2(28.0, 22.0), true, false)
                                .on_hover_text("Normal worksheet view");
                            ui.painter().line_segment(
                                [normal.rect.left_bottom(), normal.rect.right_bottom()],
                                Stroke::new(2.0_f32, TEAL),
                            );
                        });
                    });
                });
            });
        TopBottomPanel::bottom("sheet-tabs")
            .exact_height(30.0)
            .frame(
                Frame::NONE
                    .fill(CHROME)
                    .stroke(Stroke::new(0.5_f32, EDGE))
                    .inner_margin(Margin::symmetric(8, 1)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    for label in ["‹", "›"] {
                        ui.add_enabled(
                            false,
                            Button::new(label).frame(false).min_size(vec2(25.0, 26.0)),
                        )
                        .on_disabled_hover_text("Only one worksheet is supported");
                    }
                    ui.add_space(10.0);
                    let tab = ui.add(
                        Button::new(RichText::new("Sheet1").color(TEAL).strong())
                            .fill(Color32::WHITE)
                            .stroke(Stroke::NONE)
                            .corner_radius(0)
                            .min_size(vec2(79.0, 27.0)),
                    );
                    ui.painter().line_segment(
                        [tab.rect.left_bottom(), tab.rect.right_bottom()],
                        Stroke::new(2.5_f32, TEAL),
                    );
                    ui.add_space(7.0);
                    command(ui, Icon::Plus, "", vec2(27.0, 26.0), false, false)
                        .on_disabled_hover_text("Adding worksheets is unavailable");
                });
            });
    }
    pub(super) fn file_page(&mut self, ctx: &Context) {
        self.message_bar(ctx);
        SidePanel::left("file-navigation")
            .exact_width(174.0)
            .resizable(false)
            .frame(
                Frame::NONE
                    .fill(TEAL)
                    .inner_margin(Margin::symmetric(12, 16)),
            )
            .show(ctx, |ui| {
                ui.add_enabled_ui(self.pending.is_none(), |ui| {
                    ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                    let back = ui.add(
                        Button::new("Back to workbook")
                            .frame(false)
                            .min_size(vec2(150.0, 34.0)),
                    );
                    self.probes.commands.insert("file-back", back.rect);
                    if back.clicked() {
                        self.backstage = false;
                    }
                    ui.add_space(18.0);
                    for (id, label, icon) in [
                        ("file-new", "New", Icon::New),
                        ("file-open", "Open", Icon::Open),
                        ("file-save", "Save", Icon::Save),
                        ("file-save-as", "Save As", Icon::Save),
                        ("file-close", "Close", Icon::Cancel),
                    ] {
                        let (rect, response) =
                            ui.allocate_exact_size(vec2(150.0, 38.0), Sense::click());
                        response.widget_info(|| {
                            WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), label)
                        });
                        if response.hovered() || response.has_focus() {
                            ui.painter()
                                .rect_filled(rect, 2, Color32::from_white_alpha(28));
                        }
                        paint_icon(
                            ui.painter(),
                            Rect::from_center_size(
                                rect.left_center() + vec2(18.0, 0.0),
                                vec2(18.0, 18.0),
                            ),
                            icon,
                            Color32::WHITE,
                        );
                        ui.painter().text(
                            rect.left_center() + vec2(40.0, 0.0),
                            Align2::LEFT_CENTER,
                            label,
                            FontId::proportional(13.0),
                            Color32::WHITE,
                        );
                        self.probes.commands.insert(id, response.rect);
                        if response.clicked() {
                            match id {
                                "file-new" => self.request(Action::New, ctx),
                                "file-open" => self.choose_open(ctx),
                                "file-save" => {
                                    if self.save(false) {
                                        self.backstage = false;
                                    }
                                }
                                "file-save-as" => {
                                    if self.save(true) {
                                        self.backstage = false;
                                    }
                                }
                                "file-close" => self.request(Action::Close, ctx),
                                _ => {}
                            }
                        }
                    }
                });
            });
        CentralPanel::default()
            .frame(
                Frame::NONE
                    .fill(Color32::WHITE)
                    .inner_margin(Margin::symmetric(38, 28)),
            )
            .show(ctx, |ui| {
                ui.add_enabled_ui(self.pending.is_none(), |ui| {
                    ui.heading(RichText::new("Workbook").size(26.0).color(INK));
                    ui.add_space(14.0);
                    ui.label(
                        RichText::new(self.session.title().replace(" — BreakFree Sheets", ""))
                            .size(18.0),
                    );
                    ui.add_space(8.0);
                    ui.label(if self.session.dirty() {
                        "Unsaved changes"
                    } else if self.session.path.is_some() {
                        "Saved locally"
                    } else {
                        "New workbook"
                    });
                    if let Some(path) = &self.session.path {
                        ui.label(
                            RichText::new(path.display().to_string())
                                .color(Color32::from_gray(100)),
                        );
                    }
                    ui.add_space(24.0);
                    ui.separator();
                    ui.add_space(16.0);
                    ui.label("Create a workbook or open a local BreakFree Sheets file.");
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        let new = command(
                            ui,
                            Icon::New,
                            "Blank workbook",
                            vec2(170.0, 34.0),
                            true,
                            false,
                        );
                        if new.clicked() {
                            self.request(Action::New, ctx);
                        }
                        let open =
                            command(ui, Icon::Open, "Browse…", vec2(140.0, 34.0), true, false);
                        if open.clicked() {
                            self.choose_open(ctx);
                        }
                    });
                    ui.add_space(24.0);
                    ui.label(RichText::new("BreakFree Sheets").strong());
                    ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
                    ui.label(
                        RichText::new("Local workbook • 1 worksheet • .bfsheet")
                            .color(Color32::from_gray(100)),
                    );
                });
            });
    }
}
