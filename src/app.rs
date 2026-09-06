use eframe::egui::{self, Color32, FontId, RichText, Rounding, Stroke, Vec2};
use crate::pages::{kernel, services, storage, performance};
use crate::ui::theme::*;

#[derive(PartialEq)]
enum Page {
    Kernel,
    Services,
    Storage,
    Performance,
}

pub struct App {
    current_page: Page,
    kernel_page: kernel::KernelPage,
    services_page: services::ServicesPage,
    storage_page: storage::StoragePage,
    performance_page: performance::PerformancePage,
}

fn apply_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.visuals.window_fill = BG_BASE;
    style.visuals.panel_fill = BG_BASE;
    style.visuals.extreme_bg_color = BG_BASE;
    style.visuals.faint_bg_color = BG_CARD;

    style.visuals.widgets.noninteractive.bg_fill = BG_CARD;
    style.visuals.widgets.noninteractive.weak_bg_fill = BG_CARD;
    style.visuals.widgets.inactive.bg_fill = BG_INPUT;
    style.visuals.widgets.hovered.bg_fill = BG_ELEVATED;
    style.visuals.widgets.active.bg_fill = BG_ELEVATED;

    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, TEXT_SECONDARY);
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, TEXT);
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, ACCENT);
    style.visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);

    style.visuals.selection.bg_fill = ACCENT;
    style.visuals.selection.stroke = Stroke::new(1.0_f32, Color32::WHITE);

    style.visuals.window_rounding = Rounding::same(10.0);
    style.visuals.widgets.noninteractive.rounding = Rounding::same(6.0);
    style.visuals.widgets.inactive.rounding = Rounding::same(6.0);
    style.visuals.widgets.hovered.rounding = Rounding::same(6.0);
    style.visuals.widgets.active.rounding = Rounding::same(6.0);

    style.spacing.item_spacing = Vec2::new(8.0, 6.0);
    style.spacing.window_margin = egui::Margin::same(16.0);

    ctx.set_style(style);
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        apply_style(&cc.egui_ctx);

        Self {
            current_page: Page::Kernel,
            kernel_page: kernel::KernelPage::new(),
            services_page: services::ServicesPage::new(),
            storage_page: storage::StoragePage::new(),
            performance_page: performance::PerformancePage::new(),
        }
    }

    fn sidebar_button(
        ui: &mut egui::Ui,
        icon: &str,
        label: &str,
        active: bool,
    ) -> egui::Response {
        let text_color = if active { Color32::WHITE } else { TEXT_SECONDARY };
        let text_color_hover = if active { Color32::WHITE } else { TEXT };

        let desired = Vec2::new(ui.available_width(), 38.0);
        let (rect, response) = ui.allocate_at_least(desired, egui::Sense::click());

        let bg = if response.hovered() || active {
            BG_ELEVATED
        } else {
            BG_SIDEBAR
        };

        if ui.is_rect_visible(rect) {
            ui.painter()
                .rect_filled(rect, Rounding::same(8.0), bg);

            if active {
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(
                        rect.min + Vec2::new(0.0, 6.0),
                        Vec2::new(3.0, rect.height() - 12.0),
                    ),
                    Rounding::same(2.0),
                    ACCENT,
                );
            }

            let final_color = if response.hovered() && !active {
                text_color_hover
            } else {
                text_color
            };

            let galley = ui.painter().layout_no_wrap(
                format!("{}  {}", icon, label),
                FontId::proportional(13.0),
                final_color,
            );
            let text_rect = egui::Rect::from_min_size(
                rect.min + Vec2::new(14.0, (rect.height() - galley.size().y) / 2.0),
                galley.size(),
            );
            ui.painter().galley(text_rect.min, galley, final_color);
        }

        response
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .exact_width(220.0)
            .frame(egui::Frame::none().fill(BG_SIDEBAR).inner_margin(10.0))
            .show(ctx, |ui| {
                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    ui.add_space(4.0);
                    ui.label(RichText::new("⚙").size(20.0).color(ACCENT));
                    ui.add_space(2.0);
                    ui.label(
                        RichText::new("System Tuner")
                            .size(16.0)
                            .strong()
                            .color(Color32::WHITE),
                    );
                });

                ui.add_space(2.0);
                ui.add_space(4.0);

                ui.label(
                    RichText::new("Настройка Linux")
                        .size(11.0)
                        .color(TEXT_DIM),
                );

                ui.add_space(12.0);

                ui.label(
                    RichText::new("НАВИГАЦИЯ")
                        .size(10.0)
                        .color(TEXT_DIM)
                        .strong(),
                );
                ui.add_space(6.0);

                if Self::sidebar_button(ui, "🔧", "Ядро (sysctl)", self.current_page == Page::Kernel)
                    .clicked()
                {
                    self.current_page = Page::Kernel;
                }
                if Self::sidebar_button(ui, "⚡", "Службы", self.current_page == Page::Services)
                    .clicked()
                {
                    self.current_page = Page::Services;
                }
                if Self::sidebar_button(ui, "💾", "Хранилище", self.current_page == Page::Storage)
                    .clicked()
                {
                    self.current_page = Page::Storage;
                }
                if Self::sidebar_button(
                    ui,
                    "🚀",
                    "Производительность",
                    self.current_page == Page::Performance,
                )
                .clicked()
                {
                    self.current_page = Page::Performance;
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new("v0.1.0")
                            .size(10.0)
                            .color(TEXT_DIM),
                    );
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(BG_BASE).inner_margin(24.0))
            .show(ctx, |ui| {
                match self.current_page {
                    Page::Kernel => self.kernel_page.show(ui),
                    Page::Services => self.services_page.show(ui),
                    Page::Storage => self.storage_page.show(ui),
                    Page::Performance => self.performance_page.show(ui),
                }
            });
    }
}
