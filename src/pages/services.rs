use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};

const ACCENT: Color32 = Color32::from_rgb(99, 179, 237);
const BG_CARD: Color32 = Color32::from_rgb(36, 44, 58);
const TEXT_SECONDARY: Color32 = Color32::from_rgb(139, 148, 158);
const GREEN: Color32 = Color32::from_rgb(63, 185, 80);
const YELLOW: Color32 = Color32::from_rgb(210, 153, 34);
const RED: Color32 = Color32::from_rgb(248, 81, 73);

use crate::backend::systemd::{self, Service};

pub struct ServicesPage {
    services: Vec<Service>,
    filter: String,
    status_msg: String,
    status_is_error: bool,
}

enum Action {
    ToggleEnable(String, bool),
    Stop(String),
    Start(String),
}

impl ServicesPage {
    pub fn new() -> Self {
        let services = systemd::list_services("");
        Self {
            services,
            filter: String::new(),
            status_msg: String::new(),
            status_is_error: false,
        }
    }

    pub fn refresh(&mut self) {
        self.services = systemd::list_services(&self.filter);
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("⚡").size(24.0).color(ACCENT));
            ui.label(
                RichText::new("Управление службами")
                    .size(22.0)
                    .strong()
                    .color(Color32::WHITE),
            );
        });
        ui.label(
            RichText::new("Включение и отключение системных сервисов")
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
                    egui::TextEdit::singleline(&mut self.filter)
                        .hint_text("Поиск служб...")
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

        let mut action: Option<Action> = None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            for service in &self.services {
                if !self.filter.is_empty()
                    && !service
                        .name
                        .to_lowercase()
                        .contains(&self.filter.to_lowercase())
                {
                    continue;
                }

                let status_color = if service.active {
                    GREEN
                } else if service.enabled {
                    YELLOW
                } else {
                    TEXT_SECONDARY
                };

                let status_text = if service.active {
                    "● active"
                } else if service.enabled {
                    "● enabled"
                } else {
                    "● inactive"
                };

                let card = egui::Frame::none()
                    .fill(BG_CARD)
                    .rounding(Rounding::same(10.0))
                    .inner_margin(egui::Margin::same(14.0))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(48, 56, 70)));

                card.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(&service.name)
                                    .family(egui::FontFamily::Monospace)
                                    .size(14.0)
                                    .color(Color32::WHITE)
                                    .strong(),
                            );
                            ui.label(
                                RichText::new(&service.description)
                                    .size(12.0)
                                    .color(TEXT_SECONDARY),
                            );
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(status_text)
                                    .size(11.0)
                                    .color(status_color),
                            );

                            let stop_frame = egui::Frame::none()
                                .fill(Color32::from_rgb(60, 25, 25))
                                .rounding(Rounding::same(6.0))
                                .inner_margin(egui::Margin::symmetric(10.0, 4.0));
                            let start_frame = egui::Frame::none()
                                .fill(Color32::from_rgb(25, 50, 35))
                                .rounding(Rounding::same(6.0))
                                .inner_margin(egui::Margin::symmetric(10.0, 4.0));
                            let toggle_frame = egui::Frame::none()
                                .fill(Color32::from_rgb(45, 55, 72))
                                .rounding(Rounding::same(6.0))
                                .inner_margin(egui::Margin::symmetric(10.0, 4.0));

                            if service.active {
                                let btn = stop_frame.show(ui, |ui| {
                                    ui.label(
                                        RichText::new("⏹ Стоп")
                                            .size(11.0)
                                            .color(RED)
                                            .strong(),
                                    );
                                });
                                if btn.response.interact(egui::Sense::click()).clicked() {
                                    action = Some(Action::Stop(service.name.clone()));
                                }
                            } else if service.enabled {
                                let btn = start_frame.show(ui, |ui| {
                                    ui.label(
                                        RichText::new("▶ Старт")
                                            .size(11.0)
                                            .color(GREEN)
                                            .strong(),
                                    );
                                });
                                if btn.response.interact(egui::Sense::click()).clicked() {
                                    action = Some(Action::Start(service.name.clone()));
                                }
                            }

                            let toggle_text = if service.enabled {
                                "Выкл"
                            } else {
                                "Вкл"
                            };
                            let btn = toggle_frame.show(ui, |ui| {
                                ui.label(
                                    RichText::new(toggle_text)
                                        .size(11.0)
                                        .color(ACCENT)
                                        .strong(),
                                );
                            });
                            if btn.response.interact(egui::Sense::click()).clicked() {
                                action = Some(Action::ToggleEnable(
                                    service.name.clone(),
                                    service.enabled,
                                ));
                            }
                        });
                    });
                });

                ui.add_space(4.0);
            }
        });

        if let Some(action) = action {
            match action {
                Action::ToggleEnable(name, was_enabled) => {
                    let result = if was_enabled {
                        systemd::disable_service(&name)
                    } else {
                        systemd::enable_service(&name)
                    };
                    match result {
                        Ok(()) => {
                            self.status_msg = format!(
                                "✓ {} {}",
                                if was_enabled { "Отключена" } else { "Включена" },
                                name
                            );
                            self.status_is_error = false;
                        }
                        Err(e) => {
                            self.status_msg = format!("Ошибка: {}", e);
                            self.status_is_error = true;
                        }
                    }
                }
                Action::Stop(name) => match systemd::stop_service(&name) {
                    Ok(()) => {
                        self.status_msg = format!("✓ Остановлена {}", name);
                        self.status_is_error = false;
                    }
                    Err(e) => {
                        self.status_msg = format!("Ошибка: {}", e);
                        self.status_is_error = true;
                    }
                },
                Action::Start(name) => match systemd::start_service(&name) {
                    Ok(()) => {
                        self.status_msg = format!("✓ Запущена {}", name);
                        self.status_is_error = false;
                    }
                    Err(e) => {
                        self.status_msg = format!("Ошибка: {}", e);
                        self.status_is_error = true;
                    }
                },
            }
            self.refresh();
        }

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
