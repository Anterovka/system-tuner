use eframe::egui::{self, RichText};

use crate::backend::sysctl::{self, SysctlParam};
use crate::ui::components::*;
use crate::ui::theme::*;

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

    pub fn show(&mut self, ui: &mut egui::Ui) {
        page_header(ui, "🔧", "Настройки ядра", "Параметры sysctl для оптимизации производительности системы");

        ui.horizontal(|ui| {
            search_field(ui, &mut self.search, "Поиск параметров...");
            ui.add_space(4.0);
            if primary_button(ui, "↻ Обновить") {
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

                card(ui, |ui| {
                    section_label(ui, &param.category);

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
                                .color(TEXT_DIM),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if primary_button(ui, "Применить") {
                                if let Err(e) = sysctl::write_sysctl(&param.key, &param.value) {
                                    self.status_msg = format!("Ошибка: {}", e);
                                    self.status_is_error = true;
                                } else {
                                    self.status_msg =
                                        format!("✓ {} = {}", param.key, param.value);
                                    self.status_is_error = false;
                                }
                            }

                            input_field(ui, &mut param.value, 100.0, "значение");
                        });
                    });
                });

                ui.add_space(6.0);
            }
        });

        if !self.status_msg.is_empty() {
            ui.add_space(8.0);
            status_bar(ui, &self.status_msg, self.status_is_error);
        }
    }
}
