use crate::text::font::{FontFace, FontMetrics, FontStyle, FontWeight};
use std::collections::HashMap;

pub struct FontFaceParser {
    pub source_url: Option<String>,
    pub font_family: String,
    pub font_weight: FontWeight,
    pub font_style: FontStyle,
    pub descriptors: HashMap<String, String>,
}

impl FontFaceParser {
    pub fn new() -> Self {
        FontFaceParser {
            source_url: None,
            font_family: String::new(),
            font_weight: FontWeight::Regular,
            font_style: FontStyle::Normal,
            descriptors: HashMap::new(),
        }
    }

    pub fn parse_css(css: &str) -> Option<Self> {
        let trimmed = css.trim();
        let block_start = trimmed.find('{')?;
        let block_end = trimmed.rfind('}')?;
        let before = &trimmed[..block_start].trim();
        let inside = &trimmed[block_start + 1..block_end].trim();

        if !before.starts_with("@font-face") {
            return None;
        }

        let mut parser = FontFaceParser::new();
        let declarations = Self::parse_declarations(inside);
        for (prop, val) in declarations {
            match prop.as_str() {
                "font-family" => {
                    parser.font_family = val.trim_matches(|c| c == '\'' || c == '"').to_string();
                }
                "font-weight" => {
                    parser.font_weight = Self::parse_weight(&val);
                }
                "font-style" => {
                    parser.font_style = Self::parse_style(&val);
                }
                "src" => {
                    if let Some(url) = Self::extract_url(&val) {
                        parser.source_url = Some(url);
                    }
                }
                _ => {
                    parser.descriptors.insert(prop, val);
                }
            }
        }

        if parser.font_family.is_empty() {
            return None;
        }

        Some(parser)
    }

    pub fn to_font_face(&self, data: Vec<u8>) -> FontFace {
        FontFace {
            family: self.font_family.clone(),
            weight: self.font_weight,
            style: self.font_style,
            data,
            metrics: FontMetrics::default(),
        }
    }

    pub fn download_font(url: &str) -> Result<Vec<u8>, String> {
        reqwest::blocking::get(url)
            .map_err(|e| e.to_string())?
            .bytes()
            .map(|b| b.to_vec())
            .map_err(|e| e.to_string())
    }

    pub fn download_and_parse(url: &str) -> Result<FontFace, String> {
        let data = Self::download_font(url)?;
        let face = FontFace {
            family: "Downloaded".to_string(),
            weight: FontWeight::Regular,
            style: FontStyle::Normal,
            data: data.clone(),
            metrics: FontMetrics::default(),
        };
        Ok(face)
    }

    fn parse_declarations(block: &str) -> Vec<(String, String)> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut depth = 0u32;
        for ch in block.chars() {
            match ch {
                '(' => {
                    depth += 1;
                    current.push(ch);
                }
                ')' => {
                    depth = depth.saturating_sub(1);
                    current.push(ch);
                }
                ';' if depth == 0 => {
                    if let Some((prop, val)) = Self::split_declaration(&current) {
                        result.push((prop, val));
                    }
                    current.clear();
                }
                _ => {
                    current.push(ch);
                }
            }
        }
        if !current.trim().is_empty() {
            if let Some((prop, val)) = Self::split_declaration(&current) {
                result.push((prop, val));
            }
        }
        result
    }

    fn split_declaration(decl: &str) -> Option<(String, String)> {
        let decl = decl.trim();
        let colon_pos = decl.find(':')?;
        let prop = decl[..colon_pos].trim().to_string();
        let val = decl[colon_pos + 1..].trim().to_string();
        if prop.is_empty() {
            return None;
        }
        Some((prop, val))
    }

    fn extract_url(value: &str) -> Option<String> {
        if let Some(start) = value.find("url(") {
            let after = &value[start + 4..];
            if let Some(end) = after.find(')') {
                let url = after[..end]
                    .trim_matches(|c| c == '\'' || c == '"')
                    .to_string();
                if !url.is_empty() {
                    return Some(url);
                }
            }
        }
        None
    }

    fn parse_weight(value: &str) -> FontWeight {
        match value.trim() {
            "100" | "thin" => FontWeight::Thin,
            "200" | "extra-light" | "extralight" => FontWeight::ExtraLight,
            "300" | "light" => FontWeight::Light,
            "400" | "normal" | "regular" => FontWeight::Regular,
            "500" | "medium" => FontWeight::Medium,
            "600" | "semi-bold" | "semibold" => FontWeight::SemiBold,
            "700" | "bold" => FontWeight::Bold,
            "800" | "extra-bold" | "extrabold" => FontWeight::ExtraBold,
            "900" | "black" => FontWeight::Black,
            _ => FontWeight::Regular,
        }
    }

    fn parse_style(value: &str) -> FontStyle {
        match value.trim() {
            "italic" => FontStyle::Italic,
            "oblique" => FontStyle::Oblique,
            _ => FontStyle::Normal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _p = FontFaceParser::new();
    }

    #[test]
    fn test_parse_css_basic() {
        let css = r#"@font-face {
            font-family: "MyFont";
            src: url("myfont.woff2") format("woff2");
            font-weight: 400;
            font-style: normal;
        }"#;
        let parsed = FontFaceParser::parse_css(css);
        assert!(parsed.is_some());
        let p = parsed.unwrap();
        assert_eq!(p.font_family, "MyFont");
        assert_eq!(p.font_weight, FontWeight::Regular);
        assert_eq!(p.font_style, FontStyle::Normal);
        assert_eq!(p.source_url, Some("myfont.woff2".to_string()));
    }

    #[test]
    fn test_parse_css_bold() {
        let css = r#"@font-face {
            font-family: BoldFont;
            src: url("bold.otf");
            font-weight: bold;
        }"#;
        let p = FontFaceParser::parse_css(css).unwrap();
        assert_eq!(p.font_weight, FontWeight::Bold);
    }

    #[test]
    fn test_parse_css_italic() {
        let css = r#"@font-face {
            font-family: ItalicFont;
            src: url("italic.ttf");
            font-style: italic;
        }"#;
        let p = FontFaceParser::parse_css(css).unwrap();
        assert_eq!(p.font_style, FontStyle::Italic);
    }

    #[test]
    fn test_parse_css_invalid() {
        assert!(FontFaceParser::parse_css("body { color: red; }").is_none());
        assert!(FontFaceParser::parse_css("not valid css").is_none());
    }

    #[test]
    fn test_parse_css_no_family() {
        let css = "@font-face { src: url(x); }";
        assert!(FontFaceParser::parse_css(css).is_none());
    }

    #[test]
    fn test_to_font_face() {
        let mut p = FontFaceParser::new();
        p.font_family = "Test".to_string();
        p.font_weight = FontWeight::Bold;
        p.font_style = FontStyle::Italic;
        let face = p.to_font_face(vec![1, 2, 3]);
        assert_eq!(face.family, "Test");
        assert_eq!(face.weight, FontWeight::Bold);
        assert_eq!(face.style, FontStyle::Italic);
        assert_eq!(face.data, vec![1, 2, 3]);
    }

    #[test]
    fn test_extract_url() {
        assert_eq!(
            FontFaceParser::extract_url("url(\"test.woff2\")"),
            Some("test.woff2".to_string())
        );
        assert_eq!(
            FontFaceParser::extract_url("url('test.ttf')"),
            Some("test.ttf".to_string())
        );
        assert_eq!(
            FontFaceParser::extract_url("url(test.otf)"),
            Some("test.otf".to_string())
        );
        assert_eq!(FontFaceParser::extract_url("local(MyFont)"), None);
    }

    #[test]
    fn test_parse_weight() {
        assert_eq!(FontFaceParser::parse_weight("100"), FontWeight::Thin);
        assert_eq!(FontFaceParser::parse_weight("bold"), FontWeight::Bold);
        assert_eq!(FontFaceParser::parse_weight("700"), FontWeight::Bold);
        assert_eq!(FontFaceParser::parse_weight("normal"), FontWeight::Regular);
        assert_eq!(FontFaceParser::parse_weight("black"), FontWeight::Black);
        assert_eq!(FontFaceParser::parse_weight("900"), FontWeight::Black);
    }

    #[test]
    fn test_parse_style() {
        assert_eq!(FontFaceParser::parse_style("normal"), FontStyle::Normal);
        assert_eq!(FontFaceParser::parse_style("italic"), FontStyle::Italic);
        assert_eq!(FontFaceParser::parse_style("oblique"), FontStyle::Oblique);
    }

    #[test]
    fn test_parse_declarations() {
        let block = "font-family: Test; font-weight: bold; src: url(x);";
        let decls = FontFaceParser::parse_declarations(block);
        assert_eq!(decls.len(), 3);
        assert_eq!(decls[0].0, "font-family");
        assert_eq!(decls[0].1, "Test");
        assert_eq!(decls[1].0, "font-weight");
        assert_eq!(decls[1].1, "bold");
    }

    #[test]
    fn test_split_declaration() {
        let (prop, val) = FontFaceParser::split_declaration("color: red").unwrap();
        assert_eq!(prop, "color");
        assert_eq!(val, "red");
        assert!(FontFaceParser::split_declaration("bad").is_none());
    }

    #[test]
    fn test_parse_css_with_extra_descriptors() {
        let css = r#"@font-face {
            font-family: Extra;
            src: url(extra.woff2);
            font-display: swap;
            unicode-range: U+0000-00FF;
        }"#;
        let p = FontFaceParser::parse_css(css).unwrap();
        assert_eq!(p.font_family, "Extra");
        assert_eq!(p.descriptors.get("font-display"), Some(&"swap".to_string()));
        assert_eq!(
            p.descriptors.get("unicode-range"),
            Some(&"U+0000-00FF".to_string())
        );
    }
}
