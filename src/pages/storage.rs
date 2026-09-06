use eframe::egui::{self, Color32, RichText, Rounding, Stroke};

const ACCENT: Color32 = Color32::from_rgb(99, 179, 237);
const BG_CARD: Color32 = Color32::from_rgb(36, 44, 58);
const TEXT_PRIMARY: Color32 = Color32::from_rgb(201, 209, 217);
const TEXT_SECONDARY: Color32 = Color32::from_rgb(139, 148, 158);
const GREEN: Color32 = Color32::from_rgb(63, 185, 80);
const RED: Color32 = Color32::from_rgb(248, 81, 73);
const ORANGE: Color32 = Color32::from_rgb(219, 109, 40);

use crate::backend::storage::{self, SwapInfo, ZramInfo, DiskInfo};

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
        ui.horizontal(|ui| {
            ui.label(RichText::new("💾").size(24.0).color(ACCENT));
            ui.label(
                RichText::new("Настройки хранилища")
                    .size(22.0)
                    .strong()
                    .color(Color32::WHITE),
            );
        });
        ui.label(
            RichText::new("Swap, ZRAM, TRIM и информация о дисках")
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
            // Swap Section
            let card = egui::Frame::none()
                .fill(BG_CARD)
                .rounding(Rounding::same(10.0))
                .inner_margin(egui::Margin::same(16.0))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(48, 56, 70)));

            card.show(ui, |ui| {
                ui.label(
                    RichText::new("ОЗУ / SWAP")
                        .size(10.0)
                        .color(TEXT_SECONDARY),
                );
                ui.label(
                    RichText::new("Swap")
                        .size(16.0)
                        .strong()
                        .color(Color32::WHITE),
                );
                ui.add_space(4.0);

                egui::Grid::new("swap_grid")
                    .num_columns(3)
                    .spacing([20.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new("Всего:").color(TEXT_SECONDARY));
                        ui.label(
                            RichText::new(&self.swap.total)
                                .color(Color32::WHITE)
                                .strong(),
                        );
                        ui.end_row();

                        ui.label(RichText::new("Использовано:").color(TEXT_SECONDARY));
                        ui.label(
                            RichText::new(&self.swap.used)
                                .color(ORANGE)
                                .strong(),
                        );
                        ui.end_row();

                        ui.label(RichText::new("Свободно:").color(TEXT_SECONDARY));
                        ui.label(
                            RichText::new(&self.swap.free)
                                .color(GREEN)
                                .strong(),
                        );
                        ui.end_row();
                    });

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Swappiness:")
                            .color(TEXT_SECONDARY),
                    );

                    let mut swappiness: u32 = self.swap.swappiness.parse().unwrap_or(60);

                    let slider_frame = egui::Frame::none()
                        .fill(Color32::from_rgb(28, 34, 44))
                        .rounding(Rounding::same(6.0))
                        .inner_margin(egui::Margin::symmetric(8.0, 4.0));
                    slider_frame.show(ui, |ui| {
                        ui.add(
                            egui::Slider::new(&mut swappiness, 0..=200),
                        );
                    });

                    ui.label(
                        RichText::new(format!("{}", swappiness))
                            .family(egui::FontFamily::Monospace)
                            .color(ACCENT)
                            .strong(),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let btn_frame = egui::Frame::none()
                            .fill(ACCENT)
                            .rounding(Rounding::same(6.0))
                            .inner_margin(egui::Margin::symmetric(12.0, 4.0));
                        let btn = btn_frame.show(ui, |ui| {
                            ui.label(
                                RichText::new("Применить")
                                    .size(11.0)
                                    .color(Color32::WHITE)
                                    .strong(),
                            );
                        });
                        if btn.response.interact(egui::Sense::click()).clicked() {
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

            // ZRAM + TRIM row
            ui.columns(2, |cols| {
                // ZRAM
                let zram_card = egui::Frame::none()
                    .fill(BG_CARD)
                    .rounding(Rounding::same(10.0))
                    .inner_margin(egui::Margin::same(16.0))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(48, 56, 70)));

                zram_card.show(&mut cols[0], |ui| {
                    ui.label(
                        RichText::new("ZRAM")
                            .size(10.0)
                            .color(TEXT_SECONDARY),
                    );
                    ui.label(
                        RichText::new("ZRAM")
                            .size(16.0)
                            .strong()
                            .color(Color32::WHITE),
                    );
                    ui.add_space(4.0);

                    if self.zram.available {
                        ui.label(
                            RichText::new(format!("Размер: {}", self.zram.size))
                                .color(TEXT_PRIMARY),
                        );
                        ui.label(
                            RichText::new(format!("Алгоритм: {}", self.zram.algorithm))
                                .color(TEXT_PRIMARY),
                        );
                    } else {
                        ui.label(
                            RichText::new("ZRAM не настроен")
                                .color(TEXT_SECONDARY),
                        );
                    }
                });

                // TRIM
                let trim_card = egui::Frame::none()
                    .fill(BG_CARD)
                    .rounding(Rounding::same(10.0))
                    .inner_margin(egui::Margin::same(16.0))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(48, 56, 70)));

                trim_card.show(&mut cols[1], |ui| {
                    ui.label(
                        RichText::new("SSD TRIM")
                            .size(10.0)
                            .color(TEXT_SECONDARY),
                    );
                    ui.label(
                        RichText::new("TRIM")
                            .size(16.0)
                            .strong()
                            .color(Color32::WHITE),
                    );
                    ui.add_space(4.0);

                    let trim_status = if self.trim_enabled {
                        "● Активен"
                    } else {
                        "● Неактивен"
                    };
                    ui.label(
                        RichText::new(trim_status)
                            .color(if self.trim_enabled { GREEN } else { TEXT_SECONDARY }),
                    );

                    ui.add_space(8.0);

                    if !self.trim_enabled {
                        let btn_frame = egui::Frame::none()
                            .fill(GREEN)
                            .rounding(Rounding::same(6.0))
                            .inner_margin(egui::Margin::symmetric(12.0, 4.0));
                        let btn = btn_frame.show(ui, |ui| {
                            ui.label(
                                RichText::new("Включить TRIM")
                                    .size(11.0)
                                    .color(Color32::WHITE)
                                    .strong(),
                            );
                        });
                        if btn.response.interact(egui::Sense::click()).clicked() {
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

            // Disks
            let disks_card = egui::Frame::none()
                .fill(BG_CARD)
                .rounding(Rounding::same(10.0))
                .inner_margin(egui::Margin::same(16.0))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(48, 56, 70)));

            disks_card.show(ui, |ui| {
                ui.label(
                    RichText::new("ДИСКИ")
                        .size(10.0)
                        .color(TEXT_SECONDARY),
                );
                ui.label(
                    RichText::new("Дисковые накопители")
                        .size(16.0)
                        .strong()
                        .color(Color32::WHITE),
                );
                ui.add_space(8.0);

                egui::Grid::new("disks_grid")
                    .num_columns(5)
                    .spacing([16.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new("Устройство")
                                .strong()
                                .color(TEXT_SECONDARY)
                                .size(11.0),
                        );
                        ui.label(
                            RichText::new("Точка монтирования")
                                .strong()
                                .color(TEXT_SECONDARY)
                                .size(11.0),
                        );
                        ui.label(
                            RichText::new("ФС")
                                .strong()
                                .color(TEXT_SECONDARY)
                                .size(11.0),
                        );
                        ui.label(
                            RichText::new("Размер")
                                .strong()
                                .color(TEXT_SECONDARY)
                                .size(11.0),
                        );
                        ui.label(
                            RichText::new("Тип")
                                .strong()
                                .color(TEXT_SECONDARY)
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
                                    .color(TEXT_PRIMARY)
                                    .size(11.0),
                            );
                            ui.label(
                                RichText::new(&disk.fs_type)
                                    .color(TEXT_PRIMARY)
                                    .size(11.0),
                            );
                            ui.label(
                                RichText::new(&disk.size)
                                    .color(TEXT_PRIMARY)
                                    .size(11.0),
                            );
                            let disk_type = if disk.is_ssd { "SSD" } else { "HDD" };
                            let disk_color = if disk.is_ssd { GREEN } else { TEXT_PRIMARY };
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
