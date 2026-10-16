//! Original geometric command marks, drawn in logical points by the native painter.
//! No external icon font, product artwork or downloaded ribbon assets are used.
use super::*;

#[derive(Clone, Copy)]
pub(super) enum Icon {
    Save,
    Undo,
    Redo,
    Paste,
    Cut,
    Copy,
    Brush,
    Bold,
    Italic,
    Underline,
    Borders,
    Fill,
    FontColor,
    Align(u8),
    Wrap,
    Merge,
    Conditional,
    Table,
    Styles,
    Insert,
    Delete,
    Format,
    Sum,
    Clear,
    Sort,
    Find,
    Check,
    Cancel,
    Function,
    Open,
    New,
    Grid,
    Page,
    PageLayout,
    PageBreak,
    Zoom100,
    Freeze,
    Minus,
    Plus,
}

pub(super) fn paint_icon(p: &Painter, rect: Rect, icon: Icon, color: Color32) {
    let point = |x: f32, y: f32| rect.min + vec2(x, y) * rect.size() / 24.0;
    let stroke = Stroke::new((rect.width() / 18.0).max(1.0), color);
    let line = |a: (f32, f32), b: (f32, f32)| {
        p.line_segment([point(a.0, a.1), point(b.0, b.1)], stroke);
    };
    let box_at = |x: f32, y: f32, w: f32, h: f32| {
        p.rect_stroke(
            Rect::from_min_max(point(x, y), point(x + w, y + h)),
            1,
            stroke,
            StrokeKind::Inside,
        );
    };
    let letter = |s: &str, size: f32| {
        p.text(
            rect.center(),
            Align2::CENTER_CENTER,
            s,
            FontId::proportional(size * rect.height() / 24.0),
            color,
        );
    };
    match icon {
        Icon::Save => {
            box_at(3.0, 3.0, 18.0, 18.0);
            box_at(7.0, 3.0, 10.0, 6.0);
            box_at(7.0, 13.0, 10.0, 8.0);
        }
        Icon::Undo | Icon::Redo => {
            let mirror = matches!(icon, Icon::Redo);
            let pts = [
                (5.0, 9.0),
                (11.0, 6.0),
                (17.0, 8.0),
                (19.0, 13.0),
                (17.0, 18.0),
            ];
            p.add(Shape::line(
                pts.iter()
                    .map(|&(x, y)| point(if mirror { 24.0 - x } else { x }, y))
                    .collect(),
                stroke,
            ));
            if mirror {
                line((19.0, 3.0), (19.0, 9.0));
                line((19.0, 9.0), (13.0, 9.0));
            } else {
                line((5.0, 3.0), (5.0, 9.0));
                line((5.0, 9.0), (11.0, 9.0));
            }
        }
        Icon::Paste => {
            box_at(5.0, 5.0, 15.0, 17.0);
            box_at(9.0, 2.0, 7.0, 5.0);
            line((9.0, 12.0), (16.0, 12.0));
            line((9.0, 16.0), (16.0, 16.0));
        }
        Icon::Copy => {
            box_at(3.0, 3.0, 12.0, 15.0);
            box_at(9.0, 8.0, 12.0, 14.0);
        }
        Icon::Cut => {
            for x in [6.0, 17.0] {
                p.circle_stroke(point(x, 18.0), rect.width() / 7.0, stroke);
            }
            line((7.0, 15.0), (18.0, 3.0));
            line((16.0, 15.0), (5.0, 3.0));
        }
        Icon::Brush => {
            box_at(3.0, 3.0, 18.0, 7.0);
            line((12.0, 10.0), (12.0, 21.0));
            line((8.0, 5.0), (8.0, 8.0));
        }
        Icon::Bold => letter("B", 23.0),
        Icon::Italic => {
            line((12.0, 3.0), (19.0, 3.0));
            line((5.0, 21.0), (12.0, 21.0));
            line((16.0, 3.0), (8.0, 21.0));
        }
        Icon::Underline => {
            letter("U", 20.0);
            line((5.0, 22.0), (19.0, 22.0));
        }
        Icon::FontColor => {
            letter("A", 20.0);
            line((3.0, 23.0), (21.0, 23.0));
        }
        Icon::Fill => {
            p.add(Shape::closed_line(
                vec![
                    point(5.0, 4.0),
                    point(18.0, 13.0),
                    point(11.0, 20.0),
                    point(2.0, 11.0),
                ],
                stroke,
            ));
            line((5.0, 4.0), (9.0, 1.0));
            line((2.0, 23.0), (22.0, 23.0));
        }
        Icon::Align(kind) => {
            let vertical = kind < 3;
            for i in 0..3 {
                let width = if i == 1 { 10.0 } else { 16.0 };
                let x = if vertical || kind == 3 {
                    4.0
                } else if kind == 4 {
                    (24.0 - width) / 2.0
                } else {
                    20.0 - width
                };
                let y = if vertical {
                    4.0 + f32::from(kind) * 4.0 + i as f32 * 3.0
                } else {
                    5.0 + i as f32 * 6.0
                };
                line((x, y), (x + width, y));
            }
        }
        Icon::Wrap => {
            line((3.0, 5.0), (20.0, 5.0));
            line((3.0, 10.0), (14.0, 10.0));
            line((20.0, 10.0), (20.0, 17.0));
            line((20.0, 17.0), (9.0, 17.0));
            line((9.0, 17.0), (13.0, 13.0));
            line((9.0, 17.0), (13.0, 21.0));
        }
        Icon::Merge => {
            box_at(2.0, 5.0, 20.0, 14.0);
            line((3.0, 12.0), (10.0, 12.0));
            line((21.0, 12.0), (14.0, 12.0));
            line((10.0, 12.0), (7.0, 9.0));
            line((14.0, 12.0), (17.0, 15.0));
        }
        Icon::Borders
        | Icon::Table
        | Icon::Grid
        | Icon::Conditional
        | Icon::Styles
        | Icon::Insert
        | Icon::Delete
        | Icon::Format => {
            box_at(3.0, 4.0, 18.0, 16.0);
            for i in [9.0, 15.0] {
                line((i, 4.0), (i, 20.0));
            }
            for y in [9.0, 15.0] {
                line((3.0, y), (21.0, y));
            }
            if !matches!(icon, Icon::Borders | Icon::Grid) {
                p.rect_filled(
                    Rect::from_min_max(point(3.5, 4.5), point(20.5, 8.5)),
                    0,
                    color.gamma_multiply(0.4),
                );
            }
            if matches!(icon, Icon::Insert | Icon::Delete) {
                p.rect_filled(
                    Rect::from_min_max(point(13.0, 12.0), point(24.0, 24.0)),
                    1,
                    Color32::WHITE,
                );
                line((14.0, 18.0), (23.0, 18.0));
                if matches!(icon, Icon::Insert) {
                    line((18.5, 13.5), (18.5, 22.5));
                }
            }
        }
        Icon::Sum => {
            line((20.0, 3.0), (5.0, 3.0));
            line((5.0, 3.0), (13.0, 12.0));
            line((13.0, 12.0), (5.0, 21.0));
            line((5.0, 21.0), (20.0, 21.0));
        }
        Icon::Clear => {
            p.add(Shape::closed_line(
                vec![
                    point(3.0, 15.0),
                    point(14.0, 3.0),
                    point(22.0, 10.0),
                    point(11.0, 22.0),
                ],
                stroke,
            ));
            line((7.0, 11.0), (15.0, 18.0));
            line((11.0, 22.0), (23.0, 22.0));
        }
        Icon::Sort => {
            line((5.0, 3.0), (5.0, 21.0));
            line((5.0, 21.0), (2.0, 17.0));
            line((5.0, 21.0), (8.0, 17.0));
            for i in 0..3 {
                line(
                    (12.0, 5.0 + i as f32 * 6.0),
                    (16.0 + i as f32 * 3.0, 5.0 + i as f32 * 6.0),
                );
            }
        }
        Icon::Find => {
            p.circle_stroke(point(10.0, 10.0), rect.width() / 3.1, stroke);
            line((16.0, 16.0), (23.0, 23.0));
        }
        Icon::Check => {
            line((4.0, 12.0), (10.0, 18.0));
            line((10.0, 18.0), (21.0, 5.0));
        }
        Icon::Cancel => {
            line((6.0, 6.0), (18.0, 18.0));
            line((18.0, 6.0), (6.0, 18.0));
        }
        Icon::Function => letter("ƒx", 21.0),
        Icon::Open => {
            box_at(2.0, 8.0, 20.0, 13.0);
            line((3.0, 8.0), (3.0, 4.0));
            line((3.0, 4.0), (10.0, 4.0));
            line((10.0, 4.0), (13.0, 8.0));
        }
        Icon::New | Icon::Page => {
            box_at(5.0, 2.0, 15.0, 20.0);
            if matches!(icon, Icon::New) {
                line((9.0, 12.0), (16.0, 12.0));
                line((12.5, 8.5), (12.5, 15.5));
            }
        }
        Icon::PageLayout | Icon::PageBreak | Icon::Zoom100 => {
            box_at(4.0, 2.0, 16.0, 20.0);
            match icon {
                Icon::Zoom100 => letter("100", 9.0),
                Icon::PageLayout => {
                    box_at(7.0, 5.0, 10.0, 3.0);
                    box_at(7.0, 10.0, 10.0, 8.0);
                }
                _ => {
                    for x in [6.0, 10.0, 14.0] {
                        line((x, 12.0), (x + 2.0, 12.0));
                    }
                    for y in [4.0, 8.0, 14.0, 18.0] {
                        line((12.0, y), (12.0, y + 2.0));
                    }
                }
            }
        }
        Icon::Freeze => {
            box_at(2.0, 3.0, 20.0, 18.0);
            p.rect_filled(
                Rect::from_min_max(point(3.0, 4.0), point(21.0, 9.0)),
                0,
                color.gamma_multiply(0.35),
            );
            for x in [8.0, 15.0] {
                line((x, 3.0), (x, 21.0));
            }
            for y in [9.0, 15.0] {
                line((2.0, y), (22.0, y));
            }
        }
        Icon::Minus | Icon::Plus => {
            line((5.0, 12.0), (19.0, 12.0));
            if matches!(icon, Icon::Plus) {
                line((12.0, 5.0), (12.0, 19.0));
            }
        }
    }
}

/// Flat command with native focus/disabled semantics; labels also reach widget metadata.
pub(super) fn command(
    ui: &mut Ui,
    icon: Icon,
    label: &str,
    size: Vec2,
    enabled: bool,
    large: bool,
) -> Response {
    ui.add_enabled_ui(enabled, |ui| {
        let (rect, response) = ui.allocate_exact_size(size, Sense::click());
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), label));
        let enabled = ui.is_enabled();
        if enabled && (response.hovered() || response.has_focus()) {
            ui.painter().rect(
                rect,
                3,
                Color32::from_rgb(226, 239, 236),
                Stroke::new(1.0_f32, Color32::from_rgb(162, 203, 196)),
                StrokeKind::Inside,
            );
        }
        let color = if enabled {
            Color32::from_rgb(50, 59, 59)
        } else {
            Color32::from_gray(158)
        };
        let edge = if large { 28.0 } else { 17.0 };
        let center = if large {
            pos2(rect.center().x, rect.top() + 20.0)
        } else if label.is_empty() {
            rect.center()
        } else {
            pos2(rect.left() + 13.0, rect.center().y)
        };
        paint_icon(
            ui.painter(),
            Rect::from_center_size(center, Vec2::splat(edge)),
            icon,
            color,
        );
        if !label.is_empty() {
            ui.painter().text(
                if large {
                    pos2(rect.center().x, rect.top() + 39.0)
                } else {
                    pos2(rect.left() + 27.0, rect.center().y)
                },
                if large {
                    Align2::CENTER_TOP
                } else {
                    Align2::LEFT_CENTER
                },
                label,
                FontId::proportional(11.5),
                color,
            );
        }
        response
    })
    .inner
}
