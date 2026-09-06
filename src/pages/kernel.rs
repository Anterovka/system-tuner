use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};

const ACCENT: Color32 = Color32::from_rgb(99, 179, 237);
const BG_CARD: Color32 = Color32::from_rgb(36, 44, 58);
const TEXT_SECONDARY: Color32 = Color32::from_rgb(139, 148, 158);
const GREEN: Color32 = Color32::from_rgb(63, 185, 80);
const RED: Color32 = Color32::from_rgb(248, 81, 73);

use crate::backend::sysctl::{self, SysctlParam};

pub struct KernelPage {
    params: Vec<SysctlParam>,
    search: String,
    status_msg: String,
    status_is_error: bool,
}

impl KernelPage {
    pub fn new() -> Self {
        let params = sysctl::get_common_params();
        Self {
            params,
            search: String::new(),
            status_msg: String::new(),
            status_is_error: false,
        }
    }

    pub fn refresh(&mut self) {
        self.params = sysctl::get_common_params();
    }

    fn card_header(ui: &mut egui::Ui, title: &str) {
        ui.add_space(4.0);
        ui.label(RichText::new(title).size(10.0).color(TEXT_SECONDARY));
        ui.add_space(2.0);
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("🔧")
                    .size(24.0)
                    .color(ACCENT),
            );
            ui.label(
                RichText::new("Настройки ядра")
                    .size(22.0)
                    .strong()
                    .color(Color32::WHITE),
            );
        });
        ui.label(
            RichText::new("Параметры sysctl для оптимизации производительности системы")
                .color(TEXT_SECONDARY),
        );

        ui.add_space(12.0);

        ui.horizontal(|ui| {
            let search_bg = BG_CARD;
            let frame = egui::Frame::none()
                .fill(search_bg)
                .rounding(Rounding::same(8.0))
                .inner_margin(egui::Margin::symmetric(12.0, 8.0));
            frame.show(ui, |ui| {
                ui.label(RichText::new("🔍").color(TEXT_SECONDARY));
                ui.add(
                    egui::TextEdit::singleline(&mut self.search)
                        .hint_text("Поиск параметров...")
                        .desired_width(200.0)
                        .margin(Vec2::new(4.0, 2.0)),
                );
            });

            let btn_frame = egui::Frame::none()
                .fill(ACCENT)
                .rounding(Rounding::same(8.0))
                .inner_margin(egui::Margin::symmetric(16.0, 8.0));
            let btn = btn_frame.show(ui, |ui| {
                ui.label(
                    RichText::new("↻ Обновить")
                        .color(Color32::WHITE)
                        .strong(),
                );
            });
            if btn.response.interact(egui::Sense::click()).clicked() {
                self.refresh();
            }
        });

        ui.add_space(8.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            for param in &mut self.params {
                if !self.search.is_empty()
                    && !param.key.to_lowercase().contains(&self.search.to_lowercase())
                    && !param
                        .description
                        .to_lowercase()
                        .contains(&self.search.to_lowercase())
                {
                    continue;
                }

                let card = egui::Frame::none()
                    .fill(BG_CARD)
                    .rounding(Rounding::same(10.0))
                    .inner_margin(egui::Margin::same(14.0))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(48, 56, 70)));

                card.show(ui, |ui| {
                    Self::card_header(ui, &param.category);

                    ui.label(
                        RichText::new(&param.key)
                            .family(egui::FontFamily::Monospace)
                            .size(13.0)
                            .color(ACCENT),
                    );
                    ui.label(
                        RichText::new(&param.description)
                            .size(12.0)
                            .color(TEXT_SECONDARY),
                    );

                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("По умолчанию: {}", param.default))
                                .size(11.0)
                                .color(TEXT_SECONDARY),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let apply_frame = egui::Frame::none()
                                .fill(ACCENT)
                                .rounding(Rounding::same(6.0))
                                .inner_margin(egui::Margin::symmetric(12.0, 4.0));
                            let apply_btn = apply_frame.show(ui, |ui| {
                                ui.label(
                                    RichText::new("Применить")
                                        .size(11.0)
                                        .color(Color32::WHITE)
                                        .strong(),
                                );
                            });
                            if apply_btn.response.interact(egui::Sense::click()).clicked() {
                                if let Err(e) = sysctl::write_sysctl(&param.key, &param.value) {
                                    self.status_msg = format!("Ошибка: {}", e);
                                    self.status_is_error = true;
                                } else {
                                    self.status_msg =
                                        format!("✓ {} = {}", param.key, param.value);
                                    self.status_is_error = false;
                                }
                            }

                            let input_frame = egui::Frame::none()
                                .fill(Color32::from_rgb(28, 34, 44))
                                .rounding(Rounding::same(6.0))
                                .inner_margin(egui::Margin::symmetric(8.0, 4.0));
                            input_frame.show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::singleline(&mut param.value)
                                        .desired_width(100.0)
                                        .hint_text("значение")
                                        .margin(Vec2::new(2.0, 2.0)),
                                );
                            });
                        });
                    });
                });

                ui.add_space(6.0);
            }
        });

        if !self.status_msg.is_empty() {
            ui.add_space(8.0);
            let status_frame = egui::Frame::none()
                .fill(if self.status_is_error {
                    Color32::from_rgb(60, 20, 20)
                } else {
                    Color32::from_rgb(20, 50, 30)
                })
                .rounding(Rounding::same(8.0))
                .inner_margin(egui::Margin::same(12.0))
                .stroke(Stroke::new(
                    1.0_f32,
                    if self.status_is_error { RED } else { GREEN },
                ));
            status_frame.show(ui, |ui| {
                ui.label(
                    RichText::new(&self.status_msg)
                        .color(if self.status_is_error { RED } else { GREEN })
                        .size(12.0),
                );
            });
        }
    }
}
