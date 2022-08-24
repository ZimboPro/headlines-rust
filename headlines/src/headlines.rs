use std::{thread, sync::mpsc::{channel, Receiver}};

use eframe::{egui::{FontDefinitions, Context, FontData, TextStyle, RichText, Layout, Hyperlink, Separator, TopBottomPanel, menu, Window, Key}, epaint::{FontId, FontFamily, Color32}, CreationContext, emath::Align};
use newsapi::{NewsAPI};
use serde::{Serialize, Deserialize};

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const BLACK: Color32 = Color32::from_rgb(3, 3, 3);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const RED: Color32 = Color32::from_rgb(255, 0, 0);

pub struct Headlines {
    articles: Vec<NewsCardData>,
    pub config: HeadlinesConfig,
    pub api_key_init: bool,
    pub news_rx: Option<Receiver<NewsCardData>>
}

#[derive(Serialize, Deserialize, Default)]
pub struct HeadlinesConfig {
  pub dark_mode: bool,
  pub api_key: String
}

impl Headlines {
    pub fn new(cc: &CreationContext) -> Self {
        configure_fonts(&cc.egui_ctx);

        let config: HeadlinesConfig = confy::load("headlines").unwrap_or_default();
        let mut temp = Self {
          api_key_init: !config.api_key.is_empty(),
          articles: Vec::new(),
          config,
          news_rx: None
        };
        if temp.api_key_init {
          fetch_news(&mut temp);
        }
        temp
    }

    pub fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);
            let title = format!("▶ {}", a.title);
            if self.config.dark_mode {
              ui.colored_label(WHITE, title);
            } else {
              ui.colored_label(BLACK, title);
            }

            ui.add_space(PADDING);
            let label = RichText::new(&a.desc).text_style(TextStyle::Button);
            ui.label(label);
            if self.config.dark_mode {
              ui.style_mut().visuals.hyperlink_color = CYAN;
            } else {
              ui.style_mut().visuals.hyperlink_color = RED;
            }
            ui.add_space(PADDING);
            ui.with_layout(
                Layout::right_to_left(Align::TOP), 
                |ui| {
                ui.add(Hyperlink::from_label_and_url("Read more",
                &a.url));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());

        }
    }

    pub(crate) fn render_top_panel(& mut self, ctx: &Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("header").show(ctx, |ui| {
          ui.add_space(10.);
          menu::bar(ui, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
              ui.label(RichText::new("📓").text_style(TextStyle::Heading));
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
              let close_btn = ui.button(RichText::new("❌").text_style(TextStyle::Body));
              if close_btn.clicked() {
                frame.close();
              }
              let refresh_btn = ui.button(RichText::new("🔄").text_style(TextStyle::Body));
              if refresh_btn.clicked() {
                self.articles.clear();
                fetch_news(self);
              }
              let theme_btn = ui.button(RichText::new({
                if self.config.dark_mode {
                  "☀️"
                } else {
                  "🌙"
                }
              }).text_style(TextStyle::Body));
              if theme_btn.clicked() {
                self.config.dark_mode = !self.config.dark_mode;
              }
            });
          });
          ui.add_space(10.);
        });
    }

    pub fn render_config(&mut self, ctx: &Context) {
      Window::new("Configuration").show(ctx, |ui| {
        ui.label("Enter your API key for newsapi.org");
        let text_input = ui.text_edit_singleline(&mut self.config.api_key);
        if text_input.lost_focus() && ui.input().key_pressed(Key::Enter) {
          if let Err(e) = confy::store("headlines", HeadlinesConfig {
            dark_mode: self.config.dark_mode,
            api_key: self.config.api_key.to_string()
          }) {
            tracing::error!("Failed saving the app state: {}", e);
          }
          self.api_key_init = true;
          fetch_news(self);
        }
        ui.label("If you haven't registered for the API_KEY, header over to");
        ui.hyperlink("https://newsapi.org");
      });
    }

    pub fn pre_load_articles(&mut self) {
       if let Some(rx) = &self.news_rx {
        match rx.try_recv() {
            Ok(news_data) => {
              self.articles.push(news_data);
            },
            Err(e) => {
              tracing::warn!("Error recieving msg: {}", e);
            }
        }
       }
    }

}

pub struct NewsCardData {
  title: String,
  desc: String,
  url: String
}

fn fetch_news(app: &mut Headlines) {
  let api_key = app.config.api_key.to_string();
  let (news_tx, news_rx) = channel::<>();
  app.news_rx = Some(news_rx);
  thread::spawn(move || {
    match NewsAPI::new(&api_key).fetch() {
      Ok(response) => {
          let articles_resp = response.articles();
          for a in articles_resp.iter() {
              let news = NewsCardData {
                  title: a.title().to_string(),
                  url: a.url().to_string(),
                  desc: a.desc().map(|s| s.to_string()).unwrap_or_else(|| "...".to_string())
              };
              if let Err(e) = news_tx.send(news) {
                tracing::error!("Error sending news data: {}", e);
              }
          }
        },
        Err(e) => {
          tracing::error!("News API Error {:?}", e)
        }
    }
  });
}

fn configure_fonts(ctx: &Context) {
  let mut font_def = FontDefinitions::default();
  font_def.font_data.insert(
      "MesloLGS".to_string(),
      FontData::from_static(include_bytes!("../../MesloLGS_NF_Regular.ttf")),
  );
  font_def.families
      .entry(FontFamily::Proportional)
      .or_default()
      .insert(0, "MesloLGS".to_string());
  font_def.families
      .entry(FontFamily::Monospace)
      .or_default()
      .insert(0, "MesloLGS".to_string());
  
  ctx.set_fonts(font_def);

  let mut style = (*ctx.style()).clone();
  style.text_styles = [
      (TextStyle::Heading, FontId::new(30., FontFamily::Proportional)),
      (TextStyle::Body, FontId::new(20., FontFamily::Proportional)),
      (TextStyle::Monospace, FontId::new(22.0, FontFamily::Proportional)),
      (TextStyle::Button, FontId::new(14.0, FontFamily::Proportional)),
      (TextStyle::Small, FontId::new(10.0, FontFamily::Proportional)),
  ].into();
  ctx.set_style(style);
}
