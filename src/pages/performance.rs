use eframe::egui::{self, Color32, RichText, Rounding, Stroke};

use crate::backend::performance::{self, CpuInfo, PowerProfile};
use crate::ui::components::*;
use crate::ui::theme::*;

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
        page_header(ui, "🚀", "Производительность", "CPU governor, энергопрофили и оптимизация");

        ui.horizontal(|ui| {
            if primary_button(ui, "↻ Обновить") {
                self.refresh();
            }
        });

        ui.add_space(8.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            // ── CPU Info ──
            card(ui, |ui| {
                card_header(ui, "CPU", "Центральный процессор");

                egui::Grid::new("cpu_info_grid")
                    .num_columns(2)
                    .spacing([20.0, 6.0])
                    .show(ui, |ui| {
                        stat_row(ui, "Модель:", &self.cpu.model, TEXT);
                        stat_row(ui, "Ядра:", &self.cpu.cores.to_string(), ACCENT);
                        stat_row(ui, "Частота:", &self.cpu.frequency, GREEN);
                        stat_row(ui, "Governor:", &self.cpu.governor, PURPLE);
                    });

                ui.add_space(8.0);
                ui.label(
                    RichText::new("Доступные governors:")
                        .color(TEXT_DIM)
                        .size(11.0),
                );
                ui.add_space(4.0);

                ui.horizontal_wrapped(|ui| {
                    for governor in &self.cpu.available_governors {
                        let is_active = governor == &self.cpu.governor;
                        if pill_button(ui, governor, is_active) {
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

            // ── Power Profiles ──
            card(ui, |ui| {
                card_header(ui, "ЭНЕРГОПРОФИЛИ", "Профили энергопотребления");

                let profile_colors = [RED, YELLOW, GREEN];
                let profile_icons = ["⚡", "⚖", "🔋"];

                for (i, profile) in self.profiles.iter().enumerate() {
                    let is_selected = self.selected_profile == i;
                    let card_bg = if is_selected {
                        BG_ELEVATED
                    } else {
                        BG_CARD
                    };
                    let border = if is_selected {
                        profile_colors[i]
                    } else {
                        BORDER_SUBTLE
                    };

                    let inner = egui::Frame::none()
                        .fill(card_bg)
                        .rounding(Rounding::same(8.0))
                        .inner_margin(egui::Margin::same(12.0))
                        .stroke(Stroke::new(
                            if is_selected { 1.5_f32 } else { 1.0_f32 },
                            border,
                        ));

                    let resp = inner
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(profile_icons[i]).size(18.0));
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

                ui.add_space(4.0);

                if primary_button(ui, "Применить профиль") {
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

            // ── Profile Details ──
            let profile = &self.profiles[self.selected_profile];
            card(ui, |ui| {
                card_header(ui, "ДЕТАЛИ ПРОФИЛЯ", &profile.name);

                egui::Grid::new("profile_details")
                    .num_columns(3)
                    .spacing([16.0, 6.0])
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new("Параметр")
                                .strong()
                                .color(TEXT_DIM)
                                .size(11.0),
                        );
                        ui.label(
                            RichText::new("Значение")
                                .strong()
                                .color(TEXT_DIM)
                                .size(11.0),
                        );
                        ui.label(
                            RichText::new("Описание")
                                .strong()
                                .color(TEXT_DIM)
                                .size(11.0),
                        );
                        ui.end_row();

                        for setting in &profile.settings {
                            ui.label(
                                RichText::new(&setting.key)
                                    .color(TEXT)
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
            status_bar(ui, &self.status_msg, self.status_is_error);
        }
    }
}
