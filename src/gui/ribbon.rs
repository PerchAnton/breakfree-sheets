use super::*;
fn disabled(ui: &mut Ui, label: &str) {
    ui.add_enabled(false, Button::new(label))
        .on_disabled_hover_text("Not implemented in this iteration");
}
fn group(ui: &mut Ui, width: f32, title: &str, content: impl FnOnce(&mut Ui)) {
    ui.allocate_ui_with_layout(vec2(width, 98.0), Layout::top_down(Align::Min), |ui| {
        ui.allocate_ui_with_layout(vec2(width, 76.0), Layout::top_down(Align::Min), content);
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(title)
                    .size(11.0)
                    .color(Color32::from_gray(80)),
            );
        });
    });
    ui.separator();
}
impl Application {
    pub(super) fn home(&mut self, ui: &mut Ui, ctx: &Context) {
        ScrollArea::horizontal().id_salt("ribbon-scroll").show(ui,|ui| { ui.horizontal_top(|ui| {
            group(ui,120.0,"Clipboard",|ui| {
                ui.horizontal(|ui| { if ui.add_enabled(self.clipboard.is_some() && self.session.selection.is_single(),Button::new("Paste")).on_disabled_hover_text("Copy one cell in this session first; range paste is unavailable").clicked() { self.paste(); }
                    ui.vertical(|ui| { if ui.add_enabled(self.session.selection.is_single(), Button::new("Cut")).on_disabled_hover_text("Range cut is not available").clicked() { self.copy(ctx,true); }
                    if ui.add_enabled(self.session.selection.is_single(), Button::new("Copy")).on_disabled_hover_text("Range copy is not available").clicked() { self.copy(ctx,false); } }); });
                disabled(ui,"Format Painter");
            });
            group(ui,164.0,"Font",|ui| {
                ui.horizontal(|ui| { disabled(ui,"Default font  v"); disabled(ui,"11  v"); });
                ui.horizontal(|ui| { for label in ["B","I","U","Borders"] { disabled(ui,label); } });
                ui.horizontal(|ui| { disabled(ui,"Fill v"); disabled(ui,"Font color v"); });
            });
            group(ui,174.0,"Alignment",|ui| {
                ui.horizontal(|ui| { for label in ["Top","Middle","Bottom"] { disabled(ui,label); } });
                ui.horizontal(|ui| { for label in ["Left","Center","Right"] { disabled(ui,label); } });
                ui.horizontal(|ui| { disabled(ui,"Wrap Text"); disabled(ui,"Merge v"); });
            });
            group(ui,142.0,"Number",|ui| {
                ComboBox::from_id_salt("source-kind").selected_text(self.session.draft.kind.label()).width(126.0).show_ui(ui,|ui| {
                    for kind in [InputKind::Auto,InputKind::Text,InputKind::Number,InputKind::Formula] {
                        if ui.selectable_value(&mut self.session.draft.kind,kind,kind.label()).changed() { self.session.surface=Surface::Formula; }
                    }
                });
                ui.horizontal(|ui| { for label in ["$ v","%",",",".0"] { disabled(ui,label); } });
                ui.label(RichText::new("Decimal · 28 digits").small().weak());
            });
            group(ui,154.0,"Styles",|ui| {
                disabled(ui,"Conditional Format v"); disabled(ui,"Format as Table v"); disabled(ui,"Cell Styles v");
            });
            group(ui,140.0,"Cells",|ui| {
                ui.horizontal(|ui| { disabled(ui,"Insert v"); disabled(ui,"Delete v"); });
                let active=self.session.selection.active;
                ui.horizontal(|ui| { ui.label("Width"); ui.add(DragValue::new(&mut self.metrics.widths[active.col]).range(32.0..=400.0).speed(1.0)); });
                ui.horizontal(|ui| { ui.label("Height"); ui.add(DragValue::new(&mut self.metrics.heights[active.row]).range(16.0..=100.0).speed(1.0)); });
            });
            group(ui,138.0,"Editing",|ui| {
                ui.horizontal(|ui| { disabled(ui,"Σ AutoSum"); if ui.button("Clear").clicked() { let result=self.session.clear(); self.report(result); } });
                ui.horizontal(|ui| { disabled(ui,"Fill v"); disabled(ui,"Sort & Filter v"); });
                disabled(ui,"Find & Select v");
            });
        }); });
    }
}
