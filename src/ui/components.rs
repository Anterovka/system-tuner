use eframe::egui::{self, Color32, FontId, RichText, Rounding, Stroke, Vec2};

use super::theme::*;

pub fn card_header(ui: &mut egui::Ui, label: &str, title: &str) {
    ui.add_space(2.0);
    ui.label(
        RichText::new(label)
            .size(10.0)
            .color(TEXT_DIM)
            .strong(),
    );
    ui.label(
        RichText::new(title)
            .size(16.0)
            .strong()
            .color(TEXT),
    );
    ui.add_space(4.0);
}

pub fn card(ui: &mut egui::Ui, add_body: impl FnOnce(&mut egui::Ui)) {
    let frame = egui::Frame::none()
        .fill(BG_CARD)
        .rounding(Rounding::same(CARD_ROUNDING))
        .inner_margin(egui::Margin::same(16.0))
        .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE));
    frame.show(ui, |ui| add_body(ui));
}

pub fn primary_button(ui: &mut egui::Ui, label: &str) -> bool {
    button(ui, label, ACCENT, Color32::WHITE)
}

pub fn danger_button(ui: &mut egui::Ui, label: &str) -> bool {
    button(ui, label, RED_DARK, RED)
}

pub fn success_button(ui: &mut egui::Ui, label: &str) -> bool {
    button(ui, label, GREEN_DARK, GREEN)
}

pub fn subtle_button(ui: &mut egui::Ui, label: &str) -> bool {
    button(ui, label, BG_ELEVATED, TEXT_SECONDARY)
}

fn button(ui: &mut egui::Ui, label: &str, bg: Color32, text: Color32) -> bool {
    let galley = ui.painter().layout_no_wrap(
        label.to_string(),
        FontId::proportional(12.0),
        text,
    );
    let padding = Vec2::new(12.0, 6.0);
    let desired_size = galley.size() + padding * 2.0;
    let (rect, response) = ui.allocate_at_least(desired_size, egui::Sense::click());

    let bg = if response.hovered() {
        blend_color(bg, text, 0.15)
    } else {
        bg
    };

    ui.painter()
        .rect_filled(rect, Rounding::same(BTN_ROUNDING), bg);

    let text_rect = egui::Rect::from_min_size(
        rect.min + padding,
        galley.size(),
    );
    ui.painter().galley(text_rect.min, galley, text);

    response.clicked()
}

pub fn pill_button(ui: &mut egui::Ui, label: &str, active: bool) -> bool {
    let bg = if active { ACCENT } else { BG_ELEVATED };
    let text_color = if active { Color32::WHITE } else { TEXT_SECONDARY };

    let galley = ui.painter().layout_no_wrap(
        label.to_string(),
        FontId::proportional(11.0),
        text_color,
    );
    let padding = Vec2::new(10.0, 4.0);
    let desired_size = galley.size() + padding * 2.0;
    let (rect, response) = ui.allocate_at_least(desired_size, egui::Sense::click());

    let bg = if response.hovered() && !active {
        BG_ELEVATED
    } else {
        bg
    };

    ui.painter()
        .rect_filled(rect, Rounding::same(14.0), bg);

    let text_rect = egui::Rect::from_min_size(
        rect.min + padding,
        galley.size(),
    );
    ui.painter().galley(text_rect.min, galley, text_color);

    response.clicked()
}

pub fn page_header(ui: &mut egui::Ui, icon: &str, title: &str, subtitle: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(icon).size(26.0).color(ACCENT));
        ui.add_space(4.0);
        ui.label(
            RichText::new(title)
                .size(22.0)
                .strong()
                .color(Color32::WHITE),
        );
    });
    ui.add_space(2.0);
    ui.label(RichText::new(subtitle).size(12.0).color(TEXT_SECONDARY));
    ui.add_space(12.0);
}

pub fn search_field(ui: &mut egui::Ui, value: &mut String, placeholder: &str) {
    let frame = egui::Frame::none()
        .fill(BG_INPUT)
        .rounding(Rounding::same(INPUT_ROUNDING))
        .inner_margin(egui::Margin::symmetric(10.0, 6.0))
        .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE));
    frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("🔍").size(13.0).color(TEXT_DIM));
            ui.add(
                egui::TextEdit::singleline(value)
                    .hint_text(placeholder)
                    .desired_width(200.0)
                    .margin(Vec2::new(2.0, 2.0)),
            );
        });
    });
}

pub fn input_field(ui: &mut egui::Ui, value: &mut String, width: f32, placeholder: &str) {
    let frame = egui::Frame::none()
        .fill(BG_INPUT)
        .rounding(Rounding::same(INPUT_ROUNDING))
        .inner_margin(egui::Margin::symmetric(8.0, 4.0))
        .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE));
    frame.show(ui, |ui| {
        ui.add(
            egui::TextEdit::singleline(value)
                .desired_width(width)
                .hint_text(placeholder)
                .margin(Vec2::new(2.0, 2.0)),
        );
    });
}

pub fn status_bar(ui: &mut egui::Ui, msg: &str, is_error: bool) {
    if msg.is_empty() {
        return;
    }
    let (bg, fg) = if is_error {
        (RED_DARK, RED)
    } else {
        (GREEN_DARK, GREEN)
    };
    let frame = egui::Frame::none()
        .fill(bg)
        .rounding(Rounding::same(8.0))
        .inner_margin(egui::Margin::same(10.0))
        .stroke(Stroke::new(1.0_f32, fg));
    frame.show(ui, |ui| {
        ui.label(RichText::new(msg).size(12.0).color(fg));
    });
}

pub fn separator(ui: &mut egui::Ui) {
    ui.add_space(8.0);
    let available = ui.available_size();
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(available.x, 1.0),
        egui::Sense::hover(),
    );
    ui.painter()
        .rect_filled(rect, 0.0, BORDER_SUBTLE);
    ui.add_space(8.0);
}

pub fn stat_row(ui: &mut egui::Ui, label: &str, value: &str, color: Color32) {
    ui.label(RichText::new(label).size(12.0).color(TEXT_SECONDARY));
    ui.label(
        RichText::new(value)
            .size(12.0)
            .color(color)
            .strong(),
    );
    ui.end_row();
}

pub fn section_label(ui: &mut egui::Ui, text: &str) {
    ui.label(
        RichText::new(text)
            .size(10.0)
            .color(TEXT_DIM)
            .strong(),
    );
}

fn blend_color(base: Color32, overlay: Color32, factor: f32) -> Color32 {
    let r = (base.r() as f32 * (1.0 - factor) + overlay.r() as f32 * factor) as u8;
    let g = (base.g() as f32 * (1.0 - factor) + overlay.g() as f32 * factor) as u8;
    let b = (base.b() as f32 * (1.0 - factor) + overlay.b() as f32 * factor) as u8;
    Color32::from_rgb(r, g, b)
}
