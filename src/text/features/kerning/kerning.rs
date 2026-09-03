use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KerningPair {
    pub left: char,
    pub right: char,
}

#[derive(Debug, Clone)]
pub struct LigatureEntry {
    pub chars: Vec<char>,
    pub glyph_id: u32,
    pub advance: f32,
}

pub struct KerningEngine {
    pairs: HashMap<KerningPair, f32>,
    ligatures: Vec<LigatureEntry>,
    enabled: bool,
    scale: f32,
}

impl KerningEngine {
    pub fn new() -> Self {
        let mut engine = KerningEngine {
            pairs: HashMap::new(),
            ligatures: Vec::new(),
            enabled: true,
            scale: 1.0,
        };
        engine.load_defaults();
        engine
    }

    pub fn load_defaults(&mut self) {
        self.pairs.insert(KerningPair { left: 'A', right: 'V' }, -0.05);
        self.pairs.insert(KerningPair { left: 'A', right: 'W' }, -0.04);
        self.pairs.insert(KerningPair { left: 'A', right: 'T' }, -0.06);
        self.pairs.insert(KerningPair { left: 'T', right: 'o' }, -0.03);
        self.pairs.insert(KerningPair { left: 'T', right: 'a' }, -0.04);
        self.pairs.insert(KerningPair { left: 'V', right: 'a' }, -0.04);
        self.pairs.insert(KerningPair { left: 'L', right: 'T' }, -0.03);
        self.pairs.insert(KerningPair { left: 'f', right: 'i' }, 0.0);
        self.pairs.insert(KerningPair { left: 'f', right: 'l' }, 0.0);

        self.ligatures.push(LigatureEntry {
            chars: vec!['f', 'f', 'i'],
            glyph_id: 0xFB01,
            advance: 0.0,
        });
        self.ligatures.push(LigatureEntry {
            chars: vec!['f', 'f', 'l'],
            glyph_id: 0xFB02,
            advance: 0.0,
        });
        self.ligatures.push(LigatureEntry {
            chars: vec!['f', 'i'],
            glyph_id: 0xFB00,
            advance: 0.0,
        });
        self.ligatures.push(LigatureEntry {
            chars: vec!['f', 'l'],
            glyph_id: 0xFB00 + 1,
            advance: 0.0,
        });
    }

    pub fn set_pair(&mut self, left: char, right: char, adjustment: f32) {
        self.pairs.insert(KerningPair { left, right }, adjustment);
    }

    pub fn get_kerning(&self, left: char, right: char) -> f32 {
        if !self.enabled {
            return 0.0;
        }
        self.pairs
            .get(&KerningPair { left, right })
            .copied()
            .unwrap_or(0.0)
            * self.scale
    }

    pub fn add_ligature(&mut self, chars: Vec<char>, glyph_id: u32, advance: f32) {
        self.ligatures.push(LigatureEntry {
            chars,
            glyph_id,
            advance,
        });
    }

    pub fn find_ligature(&self, text: &[char], pos: usize) -> Option<&LigatureEntry> {
        self.ligatures.iter().find(|lig| {
            let len = lig.chars.len();
            pos + len <= text.len() && text[pos..pos + len] == lig.chars[..]
        })
    }

    pub fn shape_with_kerning(&self, text: &str, base_advance: f32) -> Vec<ShapedGlyph> {
        let chars: Vec<char> = text.chars().collect();
        let mut glyphs = Vec::new();
        let mut pos = 0;

        while pos < chars.len() {
            if let Some(lig) = self.find_ligature(&chars, pos) {
                let lig_len = lig.chars.len();
                let x = glyphs
                    .iter()
                    .map(|g: &ShapedGlyph| g.advance)
                    .sum::<f32>();
                glyphs.push(ShapedGlyph {
                    char: None,
                    glyph_id: lig.glyph_id,
                    x,
                    advance: base_advance * self.scale,
                });
                pos += lig_len;
            } else {
                let ch = chars[pos];
                let x: f32 = glyphs
                    .iter()
                    .map(|g: &ShapedGlyph| g.advance)
                    .sum();
                let kern = if pos + 1 < chars.len() {
                    self.get_kerning(ch, chars[pos + 1])
                } else {
                    0.0
                };
                let advance = base_advance + kern;
                glyphs.push(ShapedGlyph {
                    char: Some(ch),
                    glyph_id: ch as u32,
                    x,
                    advance,
                });
                pos += 1;
            }
        }
        glyphs
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn pair_count(&self) -> usize {
        self.pairs.len()
    }

    pub fn ligature_count(&self) -> usize {
        self.ligatures.len()
    }

    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }

    pub fn clear_ligatures(&mut self) {
        self.ligatures.clear();
    }
}

#[derive(Debug, Clone)]
pub struct ShapedGlyph {
    pub char: Option<char>,
    pub glyph_id: u32,
    pub x: f32,
    pub advance: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let engine = KerningEngine::new();
        assert!(engine.is_enabled());
        assert!(engine.pair_count() > 0);
        assert!(engine.ligature_count() > 0);
    }

    #[test]
    fn test_get_kerning() {
        let engine = KerningEngine::new();
        let k = engine.get_kerning('A', 'V');
        assert!(k < 0.0);
    }

    #[test]
    fn test_get_kerning_missing() {
        let engine = KerningEngine::new();
        assert_eq!(engine.get_kerning('z', 'z'), 0.0);
    }

    #[test]
    fn test_get_kerning_disabled() {
        let mut engine = KerningEngine::new();
        engine.set_enabled(false);
        assert_eq!(engine.get_kerning('A', 'V'), 0.0);
    }

    #[test]
    fn test_set_pair() {
        let mut engine = KerningEngine::new();
        engine.set_pair('X', 'Y', -0.1);
        assert_eq!(engine.get_kerning('X', 'Y'), -0.1);
    }

    #[test]
    fn test_scale() {
        let mut engine = KerningEngine::new();
        engine.set_pair('A', 'B', -0.1);
        engine.set_scale(2.0);
        assert_eq!(engine.get_kerning('A', 'B'), -0.2);
    }

    #[test]
    fn test_find_ligature() {
        let engine = KerningEngine::new();
        let chars: Vec<char> = "ffi".chars().collect();
        assert!(engine.find_ligature(&chars, 0).is_some());
    }

    #[test]
    fn test_find_ligature_not_found() {
        let engine = KerningEngine::new();
        let chars: Vec<char> = "abc".chars().collect();
        assert!(engine.find_ligature(&chars, 0).is_none());
    }

    #[test]
    fn test_add_ligature() {
        let mut engine = KerningEngine::new();
        let before = engine.ligature_count();
        engine.add_ligature(vec!['T', 'h'], 0x0054, 0.0);
        assert_eq!(engine.ligature_count(), before + 1);
    }

    #[test]
    fn test_shape_with_kerning_simple() {
        let engine = KerningEngine::new();
        let glyphs = engine.shape_with_kerning("AT", 10.0);
        assert_eq!(glyphs.len(), 2);
        assert_eq!(glyphs[0].char, Some('A'));
        assert_eq!(glyphs[1].char, Some('T'));
        assert!(glyphs[0].advance < 10.0);
    }

    #[test]
    fn test_shape_with_kerning_ligature() {
        let mut engine = KerningEngine::new();
        engine.clear_ligatures();
        engine.add_ligature(vec!['f', 'i'], 0xFB00, 10.0);
        let glyphs = engine.shape_with_kerning("fi", 10.0);
        assert_eq!(glyphs.len(), 1);
        assert!(glyphs[0].char.is_none());
        assert_eq!(glyphs[0].glyph_id, 0xFB00);
    }

    #[test]
    fn test_shape_empty() {
        let engine = KerningEngine::new();
        let glyphs = engine.shape_with_kerning("", 10.0);
        assert!(glyphs.is_empty());
    }

    #[test]
    fn test_clear_pairs() {
        let mut engine = KerningEngine::new();
        engine.clear_pairs();
        assert_eq!(engine.pair_count(), 0);
        assert_eq!(engine.get_kerning('A', 'V'), 0.0);
    }

    #[test]
    fn test_clear_ligatures() {
        let mut engine = KerningEngine::new();
        engine.clear_ligatures();
        assert_eq!(engine.ligature_count(), 0);
    }

    #[test]
    fn test_set_enabled_disabled() {
        let mut engine = KerningEngine::new();
        engine.set_enabled(false);
        assert!(!engine.is_enabled());
        assert_eq!(engine.get_kerning('A', 'V'), 0.0);
    }
}
