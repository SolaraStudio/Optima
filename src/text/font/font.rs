use font_kit::font::Font;
use font_kit::source::SystemSource;

pub struct FontLoader;

impl FontLoader {
    pub fn load(name: &str) -> Option<Font> {
        let source = SystemSource::new();
        let font = source
            .select_best_match(&[name], &font_kit::properties::Properties::default())
            .ok()?;
        font.load().ok()
    }

    pub fn load_default() -> Option<Font> {
        Self::load("sans-serif")
    }

    pub fn load_system_fonts() -> Vec<String> {
        let source = SystemSource::new();
        let fonts = source.all_fonts().unwrap_or_default();
        fonts
            .iter()
            .filter_map(|f| f.load().ok())
            .filter_map(|f| f.full_name().ok())
            .collect()
    }
}
