use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};
use crate::pages::{kernel, services, storage, performance};

const ACCENT: Color32 = Color32::from_rgb(99, 179, 237);
const BG_DARK: Color32 = Color32::from_rgb(22, 27, 34);
const BG_SIDEBAR: Color32 = Color32::from_rgb(30, 37, 48);
const BG_CARD: Color32 = Color32::from_rgb(36, 44, 58);
const TEXT_PRIMARY: Color32 = Color32::from_rgb(201, 209, 217);
const TEXT_SECONDARY: Color32 = Color32::from_rgb(139, 148, 158);

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

fn apply_custom_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.visuals.window_fill = BG_DARK;
    style.visuals.panel_fill = BG_DARK;
    style.visuals.widgets.noninteractive.bg_fill = BG_CARD;
    style.visuals.widgets.noninteractive.weak_bg_fill = BG_CARD;
    style.visuals.widgets.inactive.bg_fill = BG_CARD;
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(45, 55, 72);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(55, 65, 82);
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, TEXT_SECONDARY);
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, TEXT_PRIMARY);
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, ACCENT);
    style.visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
    style.visuals.selection.bg_fill = ACCENT;
    style.visuals.selection.stroke = Stroke::new(1.0_f32, Color32::WHITE);
    style.visuals.extreme_bg_color = BG_DARK;
    style.visuals.faint_bg_color = BG_CARD;

    style.spacing.item_spacing = Vec2::new(8.0, 6.0);
    style.spacing.window_margin = egui::Margin::same(16.0);
    style.visuals.window_rounding = Rounding::same(8.0);
    style.visuals.widgets.noninteractive.rounding = Rounding::same(6.0);
    style.visuals.widgets.inactive.rounding = Rounding::same(6.0);
    style.visuals.widgets.hovered.rounding = Rounding::same(6.0);
    style.visuals.widgets.active.rounding = Rounding::same(6.0);

    ctx.set_style(style);
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        apply_custom_style(&cc.egui_ctx);

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
        let bg = if active {
            Color32::from_rgb(45, 55, 72)
        } else {
            BG_SIDEBAR
        };

        let (rect, response) = ui.allocate_at_least(Vec2::new(ui.available_width(), 40.0), egui::Sense::click());
        if ui.is_rect_visible(rect) {
            ui.painter().rect_filled(rect, Rounding::same(8.0), bg);
            if active {
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect.min, Vec2::new(3.0, rect.height())),
                    Rounding::same(2.0),
                    ACCENT,
                );
            }

            let galley = ui.painter().layout_no_wrap(
                format!("{}  {}", icon, label),
                egui::FontId::proportional(14.0),
                text_color,
            );
            let text_rect = egui::Rect::from_min_size(
                rect.min + Vec2::new(16.0, (rect.height() - galley.size().y) / 2.0),
                galley.size(),
            );
            ui.painter().galley(text_rect.min, galley, text_color);
        }

        response
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .exact_width(220.0)
            .frame(egui::Frame::none().fill(BG_SIDEBAR).inner_margin(12.0))
            .show(ctx, |ui| {
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("⚙")
                            .size(22.0)
                            .color(ACCENT),
                    );
                    ui.label(
                        RichText::new("System Tuner")
                            .size(18.0)
                            .strong()
                            .color(Color32::WHITE),
                    );
                });

                ui.label(
                    RichText::new("Настройка Linux")
                        .size(11.0)
                        .color(TEXT_SECONDARY),
                );

                ui.add_space(16.0);

                if Self::sidebar_button(ui, "🔧", "Ядро (sysctl)", self.current_page == Page::Kernel)
                    .clicked()
                {
                    self.current_page = Page::Kernel;
                }
                if Self::sidebar_button(
                    ui,
                    "⚡",
                    "Службы",
                    self.current_page == Page::Services,
                )
                .clicked()
                {
                    self.current_page = Page::Services;
                }
                if Self::sidebar_button(
                    ui,
                    "💾",
                    "Хранилище",
                    self.current_page == Page::Storage,
                )
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
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new("v0.1.0")
                            .size(10.0)
                            .color(TEXT_SECONDARY),
                    );
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(BG_DARK).inner_margin(20.0))
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
