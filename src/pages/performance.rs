use eframe::egui::{self, Color32, RichText, Rounding, Stroke};

const ACCENT: Color32 = Color32::from_rgb(99, 179, 237);
const BG_CARD: Color32 = Color32::from_rgb(36, 44, 58);
const TEXT_PRIMARY: Color32 = Color32::from_rgb(201, 209, 217);
const TEXT_SECONDARY: Color32 = Color32::from_rgb(139, 148, 158);
const GREEN: Color32 = Color32::from_rgb(63, 185, 80);
const YELLOW: Color32 = Color32::from_rgb(210, 153, 34);
const RED: Color32 = Color32::from_rgb(248, 81, 73);
const PURPLE: Color32 = Color32::from_rgb(163, 113, 247);

use crate::backend::performance::{self, CpuInfo, PowerProfile};

pub struct PerformancePage {
    cpu: CpuInfo,
    profiles: Vec<PowerProfile>,
    selected_profile: usize,
    status_msg: String,
    status_is_error: bool,
}

impl PerformancePage {
    pub fn new() -> Self {
        Self {
            cpu: performance::get_cpu_info(),
            profiles: performance::get_power_profiles(),
            selected_profile: 0,
            status_msg: String::new(),
            status_is_error: false,
        }
    }

    pub fn refresh(&mut self) {
        self.cpu = performance::get_cpu_info();
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("🚀").size(24.0).color(ACCENT));
            ui.label(
                RichText::new("Производительность")
                    .size(22.0)
                    .strong()
                    .color(Color32::WHITE),
            );
        });
        ui.label(
            RichText::new("CPU governor, энергопрофили и оптимизация")
                .color(TEXT_SECONDARY),
        );

        ui.add_space(12.0);

        ui.horizontal(|ui| {
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
            // CPU Info
            let cpu_card = egui::Frame::none()
                .fill(BG_CARD)
                .rounding(Rounding::same(10.0))
                .inner_margin(egui::Margin::same(16.0))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(48, 56, 70)));

            cpu_card.show(ui, |ui| {
                ui.label(RichText::new("CPU").size(10.0).color(TEXT_SECONDARY));
                ui.label(
                    RichText::new("Центральный процессор")
                        .size(16.0)
                        .strong()
                        .color(Color32::WHITE),
                );
                ui.add_space(4.0);

                egui::Grid::new("cpu_info_grid")
                    .num_columns(2)
                    .spacing([20.0, 6.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new("Модель:").color(TEXT_SECONDARY));
                        ui.label(
                            RichText::new(&self.cpu.model)
                                .color(Color32::WHITE)
                                .size(12.0),
                        );
                        ui.end_row();

                        ui.label(RichText::new("Ядра:").color(TEXT_SECONDARY));
                        ui.label(
                            RichText::new(format!("{}", self.cpu.cores))
                                .color(ACCENT)
                                .strong(),
                        );
                        ui.end_row();

                        ui.label(RichText::new("Частота:").color(TEXT_SECONDARY));
                        ui.label(
                            RichText::new(&self.cpu.frequency)
                                .color(GREEN)
                                .strong(),
                        );
                        ui.end_row();

                        ui.label(RichText::new("Governor:").color(TEXT_SECONDARY));
                        ui.label(
                            RichText::new(&self.cpu.governor)
                                .color(PURPLE)
                                .strong(),
                        );
                        ui.end_row();
                    });

                ui.add_space(8.0);
                ui.label(
                    RichText::new("Доступные governors:")
                        .color(TEXT_SECONDARY)
                        .size(11.0),
                );
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    for governor in &self.cpu.available_governors {
                        let is_active = governor == &self.cpu.governor;
                        let btn_color = if is_active { ACCENT } else {
                            Color32::from_rgb(45, 55, 72)
                        };
                        let text_color = if is_active { Color32::WHITE } else { TEXT_SECONDARY };

                        let btn_frame = egui::Frame::none()
                            .fill(btn_color)
                            .rounding(Rounding::same(6.0))
                            .inner_margin(egui::Margin::symmetric(12.0, 4.0));
                        let btn = btn_frame.show(ui, |ui| {
                            ui.label(
                                RichText::new(governor)
                                    .size(11.0)
                                    .color(text_color)
                                    .strong(),
                            );
                        });
                        if btn.response.interact(egui::Sense::click()).clicked() {
                            if let Err(e) = performance::set_governor(governor) {
                                self.status_msg = format!("Ошибка: {}", e);
                                self.status_is_error = true;
                            } else {
                                self.cpu.governor = governor.clone();
                                self.status_msg = format!("✓ Governor = {}", governor);
                                self.status_is_error = false;
                            }
                        }
                    }
                });
            });

            ui.add_space(8.0);

            // Power Profiles
            let profiles_card = egui::Frame::none()
                .fill(BG_CARD)
                .rounding(Rounding::same(10.0))
                .inner_margin(egui::Margin::same(16.0))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(48, 56, 70)));

            profiles_card.show(ui, |ui| {
                ui.label(
                    RichText::new("ЭНЕРГОПРОФИЛИ")
                        .size(10.0)
                        .color(TEXT_SECONDARY),
                );
                ui.label(
                    RichText::new("Профили энергопотребления")
                        .size(16.0)
                        .strong()
                        .color(Color32::WHITE),
                );
                ui.add_space(8.0);

                let profile_colors = [RED, YELLOW, GREEN];
                let profile_icons = ["⚡", "⚖", "🔋"];

                for (i, profile) in self.profiles.iter().enumerate() {
                    let is_selected = self.selected_profile == i;
                    let card_color = if is_selected {
                        Color32::from_rgb(45, 55, 72)
                    } else {
                        Color32::from_rgb(30, 37, 48)
                    };
                    let border_color = if is_selected {
                        profile_colors[i]
                    } else {
                        Color32::from_rgb(48, 56, 70)
                    };

                    let profile_card = egui::Frame::none()
                        .fill(card_color)
                        .rounding(Rounding::same(8.0))
                        .inner_margin(egui::Margin::same(12.0))
                        .stroke(Stroke::new(
                            if is_selected { 2.0_f32 } else { 1.0_f32 },
                            border_color,
                        ));

                    let resp = profile_card
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(profile_icons[i])
                                        .size(18.0),
                                );
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new(&profile.name)
                                            .size(14.0)
                                            .strong()
                                            .color(Color32::WHITE),
                                    );
                                    ui.label(
                                        RichText::new(&profile.description)
                                            .size(11.0)
                                            .color(TEXT_SECONDARY),
                                    );
                                });
                            });
                        })
                        .response;

                    if resp.interact(egui::Sense::click()).clicked() {
                        self.selected_profile = i;
                    }

                    ui.add_space(4.0);
                }

                ui.add_space(8.0);

                let apply_frame = egui::Frame::none()
                    .fill(ACCENT)
                    .rounding(Rounding::same(8.0))
                    .inner_margin(egui::Margin::symmetric(24.0, 10.0));
                let apply_btn = apply_frame.show(ui, |ui| {
                    ui.label(
                        RichText::new("Применить профиль")
                            .size(13.0)
                            .color(Color32::WHITE)
                            .strong(),
                    );
                });
                if apply_btn.response.interact(egui::Sense::click()).clicked() {
                    let profile = &self.profiles[self.selected_profile];
                    if let Err(e) = performance::apply_power_profile(profile) {
                        self.status_msg = format!("Ошибка: {}", e);
                        self.status_is_error = true;
                    } else {
                        self.status_msg = format!("✓ Профиль '{}' применён", profile.name);
                        self.status_is_error = false;
                    }
                }
            });

            ui.add_space(8.0);

            // Profile Details
            let profile = &self.profiles[self.selected_profile];
            let details_card = egui::Frame::none()
                .fill(BG_CARD)
                .rounding(Rounding::same(10.0))
                .inner_margin(egui::Margin::same(16.0))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(48, 56, 70)));

            details_card.show(ui, |ui| {
                ui.label(
                    RichText::new("ДЕТАЛИ ПРОФИЛЯ")
                        .size(10.0)
                        .color(TEXT_SECONDARY),
                );
                ui.label(
                    RichText::new(&profile.name)
                        .size(16.0)
                        .strong()
                        .color(Color32::WHITE),
                );
                ui.add_space(4.0);

                egui::Grid::new("profile_details")
                    .num_columns(3)
                    .spacing([16.0, 6.0])
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new("Параметр")
                                .strong()
                                .color(TEXT_SECONDARY)
                                .size(11.0),
                        );
                        ui.label(
                            RichText::new("Значение")
                                .strong()
                                .color(TEXT_SECONDARY)
                                .size(11.0),
                        );
                        ui.label(
                            RichText::new("Описание")
                                .strong()
                                .color(TEXT_SECONDARY)
                                .size(11.0),
                        );
                        ui.end_row();

                        for setting in &profile.settings {
                            ui.label(
                                RichText::new(&setting.key)
                                    .color(TEXT_PRIMARY)
                                    .size(11.0),
                            );
                            ui.label(
                                RichText::new(&setting.value)
                                    .color(ACCENT)
                                    .family(egui::FontFamily::Monospace)
                                    .size(11.0),
                            );
                            ui.label(
                                RichText::new(&setting.description)
                                    .color(TEXT_SECONDARY)
                                    .size(11.0),
                            );
                            ui.end_row();
                        }
                    });
            });
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
