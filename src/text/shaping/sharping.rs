use rustybuzz::{UnicodeBuffer, GlyphInfo, GlyphBuffer};
use font_kit::font::Font;
use font_kit::source::SystemSource;

pub struct TextShaper;

impl TextShaper {
    pub fn shape(text: &str, font_size: f32, font_name: &str) -> Vec<GlyphInfo> {
        let source = SystemSource::new();
        let font = source
            .select_best_match(&[font_name], &font_kit::properties::Properties::default())
            .ok()
            .and_then(|f| f.load().ok());
        if let Some(font) = font {
            let face = font.face();
            let font_ref = rustybuzz::Font::from_face(face, font_size, rustybuzz::face::Face::default());
            let mut buffer = UnicodeBuffer::new().push_str(text);
            let buffer = rustybuzz::shape(&font_ref, buffer);
            buffer.glyph_infos().to_vec()
        } else {
            Vec::new()
        }
    }

    pub fn shape_with_fallback(text: &str, font_size: f32) -> Vec<GlyphInfo> {
        Self::shape(text, font_size, "sans-serif")
    }

    pub fn shape_with_font(text: &str, font: &Font, font_size: f32) -> Vec<GlyphInfo> {
        let face = font.face();
        let font_ref = rustybuzz::Font::from_face(face, font_size, rustybuzz::face::Face::default());
        let mut buffer = UnicodeBuffer::new().push_str(text);
        let buffer = rustybuzz::shape(&font_ref, buffer);
        buffer.glyph_infos().to_vec()
    }

    pub fn shape_with_features(
        text: &str,
        font: &Font,
        font_size: f32,
        features: &[rustybuzz::Feature],
    ) -> Vec<GlyphInfo> {
        let face = font.face();
        let mut font_ref = rustybuzz::Font::from_face(face, font_size, rustybuzz::face::Face::default());
        font_ref.set_features(features);
        let mut buffer = UnicodeBuffer::new().push_str(text);
        let buffer = rustybuzz::shape(&font_ref, buffer);
        buffer.glyph_infos().to_vec()
    }
}
