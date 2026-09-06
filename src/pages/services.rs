use eframe::egui::{self, Color32, RichText};

use crate::backend::systemd::{self, Service};
use crate::ui::components::*;
use crate::ui::theme::*;

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
        page_header(ui, "⚡", "Управление службами", "Включение и отключение системных сервисов");

        ui.horizontal(|ui| {
            search_field(ui, &mut self.filter, "Поиск служб...");
            ui.add_space(4.0);
            if primary_button(ui, "↻ Обновить") {
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
                    TEXT_DIM
                };

                let status_text = if service.active {
                    "● active"
                } else if service.enabled {
                    "● enabled"
                } else {
                    "● inactive"
                };

                card(ui, |ui| {
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

                            if service.active {
                                if danger_button(ui, "⏹ Стоп") {
                                    action = Some(Action::Stop(service.name.clone()));
                                }
                            } else if service.enabled {
                                if success_button(ui, "▶ Старт") {
                                    action = Some(Action::Start(service.name.clone()));
                                }
                            }

                            let toggle_label = if service.enabled { "Выкл" } else { "Вкл" };
                            if subtle_button(ui, toggle_label) {
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
            status_bar(ui, &self.status_msg, self.status_is_error);
        }
    }
}
