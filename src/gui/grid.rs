use super::*;
use crate::cells::{Address, COLS, ROWS, Value};
const GUTTER: f32 = 44.0;
const HEADER: f32 = 24.0;
impl Application {
    pub(super) fn grid(&mut self, ui: &mut Ui, ctx: &Context) {
        let output = ScrollArea::both()
            .id_salt("worksheet-scroll")
            .auto_shrink([false, false])
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .scroll_source(egui::scroll_area::ScrollSource {
                drag: false,
                ..Default::default()
            })
            .show_viewport(ui, |ui, viewport| {
                ui.set_min_size(vec2(
                    GUTTER + self.metrics.column(COLS),
                    HEADER + self.metrics.row(ROWS),
                ));
                let origin = ui.min_rect().min;
                let screen = Rect::from_min_size(origin + viewport.min.to_vec2(), viewport.size());
                let body = Rect::from_min_max(screen.min + vec2(GUTTER, HEADER), screen.max)
                    .intersect(ui.clip_rect());
                let data = origin + vec2(GUTTER, HEADER);
                self.probes.grid = body;
                self.probes.cell_origin = data;
                self.probes.painted_cells = 0;
                let rows = self
                    .metrics
                    .visible_rows(viewport.min.y, (viewport.height() - HEADER).max(0.0));
                let cols = self
                    .metrics
                    .visible_columns(viewport.min.x, (viewport.width() - GUTTER).max(0.0));
                let painter = ui.painter().with_clip_rect(body);
                let (first, last) = self.session.selection.bounds();
                let range = Rect::from_min_max(
                    data + vec2(self.metrics.column(first.col), self.metrics.row(first.row)),
                    data + vec2(
                        self.metrics.column(last.col + 1),
                        self.metrics.row(last.row + 1),
                    ),
                );
                painter.rect_filled(range, 0, Color32::from_rgb(225, 241, 238));
                let active = self.session.selection.active;
                let active_rect = Rect::from_min_size(
                    data + vec2(
                        self.metrics.column(active.col),
                        self.metrics.row(active.row),
                    ),
                    vec2(
                        self.metrics.widths[active.col],
                        self.metrics.heights[active.row],
                    ) * self.metrics.zoom,
                );
                self.probes.active = active_rect;
                painter.rect_filled(active_rect, 0, Color32::WHITE);
                for row in rows.clone() {
                    for col in cols.clone() {
                        let cell = Address { col, row };
                        let rect = Rect::from_min_size(
                            data + vec2(self.metrics.column(col), self.metrics.row(row)),
                            vec2(self.metrics.widths[col], self.metrics.heights[row])
                                * self.metrics.zoom,
                        );
                        if !rect.intersects(body) {
                            continue;
                        }
                        self.probes.painted_cells += 1;
                        painter.rect_stroke(
                            rect,
                            0,
                            Stroke::new(0.5_f32, Color32::from_gray(218)),
                            StrokeKind::Inside,
                        );
                        if let Some(value) = self.session.editor.values.get(&cell) {
                            let (position, alignment) = if matches!(value, Value::Number(_)) {
                                (rect.right_center() - vec2(5.0, 0.0), Align2::RIGHT_CENTER)
                            } else {
                                (rect.left_center() + vec2(5.0, 0.0), Align2::LEFT_CENTER)
                            };
                            painter
                                .with_clip_rect(rect.shrink2(vec2(4.0, 1.0)).intersect(body))
                                .text(
                                    position,
                                    alignment,
                                    value.to_string(),
                                    FontId::proportional(13.0 * self.metrics.zoom),
                                    if matches!(value, Value::Error(_)) {
                                        Color32::from_rgb(165, 38, 38)
                                    } else {
                                        Color32::from_gray(35)
                                    },
                                );
                        }
                    }
                }
                painter.rect_stroke(
                    range.shrink(0.5),
                    0,
                    Stroke::new(2.0_f32, TEAL),
                    StrokeKind::Inside,
                );
                if !self.session.selection.is_single() {
                    painter.rect_stroke(
                        active_rect.shrink(0.5),
                        0,
                        Stroke::new(1.5_f32, TEAL),
                        StrokeKind::Inside,
                    );
                }
                // Headers are painted at the viewport edge, not at the scrolled content edge.
                let header_painter = ui
                    .painter()
                    .with_clip_rect(screen.intersect(ui.clip_rect()));
                header_painter.rect_filled(
                    Rect::from_min_size(screen.min, vec2(screen.width(), HEADER)),
                    0,
                    Color32::from_gray(245),
                );
                header_painter.rect_filled(
                    Rect::from_min_size(screen.min, vec2(GUTTER, screen.height())),
                    0,
                    Color32::from_gray(245),
                );
                for col in cols.clone() {
                    let rect = Rect::from_min_size(
                        pos2(data.x + self.metrics.column(col), screen.top()),
                        vec2(self.metrics.widths[col] * self.metrics.zoom, HEADER),
                    );
                    let selected = (first.col..=last.col).contains(&col);
                    header_painter.rect_filled(
                        rect,
                        0,
                        if selected {
                            Color32::from_rgb(211, 235, 230)
                        } else {
                            Color32::from_gray(245)
                        },
                    );
                    header_painter.rect_stroke(
                        rect,
                        0,
                        Stroke::new(0.6_f32, Color32::from_gray(205)),
                        StrokeKind::Inside,
                    );
                    if selected {
                        header_painter.line_segment(
                            [rect.left_bottom(), rect.right_bottom()],
                            Stroke::new(1.5_f32, TEAL),
                        );
                    }
                    header_painter
                        .with_clip_rect(rect.intersect(screen).intersect(Rect::from_min_max(
                            screen.min + vec2(GUTTER, 0.0),
                            screen.max,
                        )))
                        .text(
                            rect.center(),
                            Align2::CENTER_CENTER,
                            ((b'A' + col as u8) as char).to_string(),
                            FontId::proportional(12.0),
                            Color32::from_gray(60),
                        );
                }
                for row in rows.clone() {
                    let rect = Rect::from_min_size(
                        pos2(screen.left(), data.y + self.metrics.row(row)),
                        vec2(GUTTER, self.metrics.heights[row] * self.metrics.zoom),
                    );
                    header_painter.rect_filled(
                        rect,
                        0,
                        if (first.row..=last.row).contains(&row) {
                            Color32::from_rgb(211, 235, 230)
                        } else {
                            Color32::from_gray(245)
                        },
                    );
                    header_painter.rect_stroke(
                        rect,
                        0,
                        Stroke::new(0.6_f32, Color32::from_gray(205)),
                        StrokeKind::Inside,
                    );
                    if (first.row..=last.row).contains(&row) {
                        header_painter.line_segment(
                            [rect.right_top(), rect.right_bottom()],
                            Stroke::new(1.5_f32, TEAL),
                        );
                    }
                    header_painter
                        .with_clip_rect(Rect::from_min_max(
                            screen.min + vec2(0.0, HEADER),
                            screen.max,
                        ))
                        .text(
                            rect.center(),
                            Align2::CENTER_CENTER,
                            (row + 1).to_string(),
                            FontId::proportional(12.0),
                            Color32::from_gray(60),
                        );
                }
                header_painter.rect_filled(
                    Rect::from_min_size(screen.min, vec2(GUTTER, HEADER)),
                    0,
                    Color32::from_gray(238),
                );
                let response = ui.interact(screen, Id::new("worksheet"), Sense::click_and_drag());
                if self.pending.is_none() {
                    if (response.clicked()
                        || (response.is_pointer_button_down_on()
                            && ui.input(|i| i.pointer.button_pressed(PointerButton::Primary))))
                        && let Some(pointer) = response.interact_pointer_pos()
                    {
                        let shift = ui.input(|i| i.modifiers.shift);
                        if body.contains(pointer) {
                            if let Some(target) =
                                self.metrics.hit(pointer.x - data.x, pointer.y - data.y)
                            {
                                let result = self.session.select(target, shift);
                                if self.report(result) {
                                    self.name = self.session.selection.label();
                                    self.reset_focus(ctx);
                                }
                                if response.double_clicked() {
                                    self.session.begin_edit(Surface::Grid, None);
                                    self.focus_editor = true;
                                }
                            }
                        } else if pointer.y < body.top() && pointer.x >= body.left() {
                            if let Some(target) = self.metrics.hit(pointer.x - data.x, 0.0) {
                                let result = self.session.commit();
                                if self.report(result) {
                                    self.session.selection = crate::interaction::Selection {
                                        anchor: Address {
                                            col: target.col,
                                            row: 0,
                                        },
                                        active: Address {
                                            col: target.col,
                                            row: ROWS - 1,
                                        },
                                    };
                                    self.session.reset_draft();
                                    self.name = self.session.selection.label();
                                    self.reset_focus(ctx);
                                }
                            }
                        } else if pointer.x < body.left()
                            && pointer.y >= body.top()
                            && let Some(target) = self.metrics.hit(0.0, pointer.y - data.y)
                        {
                            let result = self.session.commit();
                            if self.report(result) {
                                self.session.selection = crate::interaction::Selection {
                                    anchor: Address {
                                        col: 0,
                                        row: target.row,
                                    },
                                    active: Address {
                                        col: COLS - 1,
                                        row: target.row,
                                    },
                                };
                                self.session.reset_draft();
                                self.name = self.session.selection.label();
                                self.reset_focus(ctx);
                            }
                        }
                    }
                    if response.dragged()
                        && let Some(pointer) = response.interact_pointer_pos()
                        && body.contains(pointer)
                        && self.session.surface == Surface::Selection
                        && let Some(target) =
                            self.metrics.hit(pointer.x - data.x, pointer.y - data.y)
                    {
                        let result = self.session.select(target, true);
                        self.report(result);
                        self.name = self.session.selection.label();
                    }
                    // Only visible header boundaries have hit targets; cells share one grid interaction.
                    for col in cols.clone() {
                        let x = data.x + self.metrics.column(col + 1);
                        if x < body.left() {
                            continue;
                        }
                        let edge = Rect::from_min_max(
                            pos2(x - 3.0, screen.top()),
                            pos2(x + 3.0, screen.top() + HEADER),
                        );
                        let resize = ui
                            .interact(edge, Id::new(("column-width", col)), Sense::drag())
                            .on_hover_cursor(CursorIcon::ResizeHorizontal);
                        if resize.dragged() {
                            self.metrics.resize_column(
                                col,
                                self.metrics.widths[col]
                                    + resize.drag_delta().x / self.metrics.zoom,
                            );
                        }
                    }
                    for row in rows {
                        let y = data.y + self.metrics.row(row + 1);
                        if y < body.top() {
                            continue;
                        }
                        let edge = Rect::from_min_max(
                            pos2(screen.left(), y - 3.0),
                            pos2(screen.left() + GUTTER, y + 3.0),
                        );
                        let resize = ui
                            .interact(edge, Id::new(("row-height", row)), Sense::drag())
                            .on_hover_cursor(CursorIcon::ResizeVertical);
                        if resize.dragged() {
                            self.metrics.resize_row(
                                row,
                                self.metrics.heights[row]
                                    + resize.drag_delta().y / self.metrics.zoom,
                            );
                        }
                    }
                    if self.session.surface == Surface::Grid {
                        let response = ui.put(
                            active_rect.shrink(1.0),
                            TextEdit::singleline(&mut self.session.draft.source)
                                .id(Id::new("grid-source"))
                                .font(FontId::proportional(13.0 * self.metrics.zoom))
                                .margin(vec2(3.0, 1.0))
                                .frame(true),
                        );
                        if self.focus_editor {
                            response.request_focus();
                            self.focus_editor = false;
                        }
                    }
                }
                if self.reveal {
                    ui.scroll_to_rect(active_rect.expand2(vec2(GUTTER, HEADER)), None);
                    self.reveal = false;
                }
            });
        let _ = output;
    }
}
