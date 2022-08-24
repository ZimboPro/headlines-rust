#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui::{CentralPanel, ScrollArea, Ui, Separator, TopBottomPanel, Context, RichText, Hyperlink, Visuals}, run_native, epaint::Vec2, App};
use headlines::{Headlines, PADDING};
mod headlines;


impl App for Headlines {
    
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }
        if !self.api_key_init {
            self.render_config(ctx);
        } else {
            self.pre_load_articles();
            self.render_top_panel(ctx, frame);
            render_footer(ctx);
            CentralPanel::default().show(ctx, |ui| {
                render_header(ui);
                ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                    self.render_news_cards(ui);
                });
            });
        }
    }
}

fn render_footer(ctx: &Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.label(RichText::new("API Source:: newsapi.org").monospace());
            ui.add(Hyperlink::from_label_and_url(
                "Made with egui", 
                "https://github.com/emilk/egui")
            );
            ui.add(Hyperlink::from_label_and_url(
                "Made with egui", 
                "https://github.com/emilk/egui")
            );
            ui.add_space(10.);
        });
    });
}

fn render_header(ui: & mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("headlines");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}

fn main() {
    tracing_subscriber::fmt::init();
    let options = eframe::NativeOptions { initial_window_size: Some(Vec2::new(540., 960.)), ..Default::default() };
    run_native(
        "Headlines",
        options,
        Box::new(|cc| Box::new(Headlines::new(cc)))
    );
}
