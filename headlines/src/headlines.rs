use eframe::{egui::{FontDefinitions, Context, FontData, TextStyle, RichText, Layout, Hyperlink, Separator, TopBottomPanel, menu}, epaint::{FontId, FontFamily, Color32}, CreationContext, emath::Align};

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct Headlines {
    articles: Vec<NewsCardData>
}

impl Headlines {
    pub fn new(cc: &CreationContext) -> Self {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title {}", a),
            desc: format!("desc {}", a),
            url: format!("http://example.com/{}", a),
        });
        configure_fonts(&cc.egui_ctx);
        Self { articles: Vec::from_iter(iter) }
    }

    pub fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);
            let title = format!("▶ {}", a.title);
            ui.colored_label(WHITE, title);
            ui.add_space(PADDING);
            let label = RichText::new(&a.desc).text_style(TextStyle::Button);
            ui.label(label);
            ui.style_mut().visuals.hyperlink_color = CYAN;
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

    pub(crate) fn render_top_panel(&self, ctx: &Context) {
        TopBottomPanel::top("header").show(ctx, |ui| {
          ui.add_space(10.);
          menu::bar(ui, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
              ui.label(RichText::new("📓").text_style(TextStyle::Heading));
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
              let close_btn = ui.button(RichText::new("❌").text_style(TextStyle::Body));
              let refresh_btn = ui.button(RichText::new("🔄").text_style(TextStyle::Body));
              let theme_btn = ui.button(RichText::new("🌙").text_style(TextStyle::Body));
              
            });
          });
          ui.add_space(10.);
        });
    }
}

struct NewsCardData {
  title: String,
  desc: String,
  url: String
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