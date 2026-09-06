use eframe::egui::{self, RichText, Rounding};

use crate::backend::storage::{self, DiskInfo, SwapInfo, ZramInfo};
use crate::ui::components::*;
use crate::ui::theme::*;

pub struct StoragePage {
    swap: SwapInfo,
    zram: ZramInfo,
    disks: Vec<DiskInfo>,
    trim_enabled: bool,
    status_msg: String,
    status_is_error: bool,
}

impl StoragePage {
    pub fn new() -> Self {
        Self {
            swap: storage::get_swap_info(),
            zram: storage::get_zram_info(),
            disks: storage::get_disks(),
            trim_enabled: storage::get_trim_status(),
            status_msg: String::new(),
            status_is_error: false,
        }
    }

    pub fn refresh(&mut self) {
        self.swap = storage::get_swap_info();
        self.zram = storage::get_zram_info();
        self.disks = storage::get_disks();
        self.trim_enabled = storage::get_trim_status();
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        page_header(ui, "💾", "Настройки хранилища", "Swap, ZRAM, TRIM и информация о дисках");

        ui.horizontal(|ui| {
            if primary_button(ui, "↻ Обновить") {
                self.refresh();
            }
        });

        ui.add_space(8.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            // ── Swap ──
            card(ui, |ui| {
                card_header(ui, "ОЗУ / SWAP", "Swap");

                egui::Grid::new("swap_grid")
                    .num_columns(3)
                    .spacing([20.0, 8.0])
                    .show(ui, |ui| {
                        stat_row(ui, "Всего:", &self.swap.total, TEXT);
                        stat_row(ui, "Использовано:", &self.swap.used, ORANGE);
                        stat_row(ui, "Свободно:", &self.swap.free, GREEN);
                    });

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Swappiness:").color(TEXT_SECONDARY));

                    let mut swappiness: u32 = self.swap.swappiness.parse().unwrap_or(60);

                    let slider_frame = egui::Frame::none()
                        .fill(BG_INPUT)
                        .rounding(Rounding::same(INPUT_ROUNDING))
                        .inner_margin(egui::Margin::symmetric(8.0, 4.0));
                    slider_frame.show(ui, |ui| {
                        ui.add(egui::Slider::new(&mut swappiness, 0..=200));
                    });

                    ui.label(
                        RichText::new(format!("{}", swappiness))
                            .family(egui::FontFamily::Monospace)
                            .color(ACCENT)
                            .strong(),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if primary_button(ui, "Применить") {
                            if let Err(e) = storage::set_swappiness(swappiness) {
                                self.status_msg = format!("Ошибка: {}", e);
                                self.status_is_error = true;
                            } else {
                                self.swap.swappiness = swappiness.to_string();
                                self.status_msg = format!("✓ Swappiness = {}", swappiness);
                                self.status_is_error = false;
                            }
                        }
                    });
                });
            });

            ui.add_space(8.0);

            // ── ZRAM + TRIM ──
            ui.columns(2, |cols| {
                card(&mut cols[0], |ui| {
                    card_header(ui, "ZRAM", "ZRAM");

                    if self.zram.available {
                        ui.label(
                            RichText::new(format!("Размер: {}", self.zram.size)).color(TEXT),
                        );
                        ui.label(
                            RichText::new(format!("Алгоритм: {}", self.zram.algorithm))
                                .color(TEXT),
                        );
                    } else {
                        ui.label(
                            RichText::new("ZRAM не настроен").color(TEXT_DIM),
                        );
                    }
                });

                card(&mut cols[1], |ui| {
                    card_header(ui, "SSD TRIM", "TRIM");

                    let trim_status = if self.trim_enabled {
                        "● Активен"
                    } else {
                        "● Неактивен"
                    };
                    ui.label(
                        RichText::new(trim_status)
                            .color(if self.trim_enabled { GREEN } else { TEXT_DIM }),
                    );

                    if !self.trim_enabled {
                        ui.add_space(8.0);
                        if success_button(ui, "Включить TRIM") {
                            if let Err(e) = storage::enable_trim() {
                                self.status_msg = format!("Ошибка: {}", e);
                                self.status_is_error = true;
                            } else {
                                self.trim_enabled = true;
                                self.status_msg = "✓ TRIM включен".to_string();
                                self.status_is_error = false;
                            }
                        }
                    }
                });
            });

            ui.add_space(8.0);

            // ── Disks ──
            card(ui, |ui| {
                card_header(ui, "ДИСКИ", "Дисковые накопители");

                egui::Grid::new("disks_grid")
                    .num_columns(5)
                    .spacing([16.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new("Устройство")
                                .strong()
                                .color(TEXT_DIM)
                                .size(11.0),
                        );
                        ui.label(
                            RichText::new("Точка монтирования")
                                .strong()
                                .color(TEXT_DIM)
                                .size(11.0),
                        );
                        ui.label(
                            RichText::new("ФС")
                                .strong()
                                .color(TEXT_DIM)
                                .size(11.0),
                        );
                        ui.label(
                            RichText::new("Размер")
                                .strong()
                                .color(TEXT_DIM)
                                .size(11.0),
                        );
                        ui.label(
                            RichText::new("Тип")
                                .strong()
                                .color(TEXT_DIM)
                                .size(11.0),
                        );
                        ui.end_row();

                        for disk in &self.disks {
                            ui.label(
                                RichText::new(&disk.device)
                                    .family(egui::FontFamily::Monospace)
                                    .color(ACCENT)
                                    .size(11.0),
                            );
                            ui.label(
                                RichText::new(&disk.mount)
                                    .color(TEXT)
                                    .size(11.0),
                            );
                            ui.label(
                                RichText::new(&disk.fs_type)
                                    .color(TEXT)
                                    .size(11.0),
                            );
                            ui.label(
                                RichText::new(&disk.size)
                                    .color(TEXT)
                                    .size(11.0),
                            );
                            let disk_type = if disk.is_ssd { "SSD" } else { "HDD" };
                            let disk_color = if disk.is_ssd { GREEN } else { TEXT };
                            ui.label(
                                RichText::new(disk_type)
                                    .color(disk_color)
                                    .size(11.0)
                                    .strong(),
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
