use crate::text::font_cache::FontCache;
use cssparser::{Parser, ParserInput, DeclarationParser, QualifiedRuleParser};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FontFaceRule {
    pub family: String,
    pub src: String,
    pub weight: String,
    pub style: String,
}

impl FontFaceRule {
    pub fn parse_from_css(css: &str) -> Vec<Self> {
        let mut input = ParserInput::new(css);
        let mut parser = Parser::new(&mut input);
        let mut rules = Vec::new();

        while let Ok(rule) = parser.parse_qualified_rule(0, &mut FontFaceRuleParser) {
            if let Some(font_face) = rule {
                rules.push(font_face);
            }
        }
        rules
    }

    pub fn process(&self, cache: &FontCache) {
        let clean_url = self.src
            .trim()
            .trim_start_matches("url(")
            .trim_end_matches(')')
            .trim_matches('"')
            .trim_matches('\'');
        if let Some(font) = cache.load_from_url(&self.family, clean_url) {
            log::info!("Loaded @font-face font: {} from {}", self.family, clean_url);
        } else {
            log::warn!("Failed to load @font-face font: {} from {}", self.family, clean_url);
        }
    }
}

struct FontFaceRuleParser;

impl QualifiedRuleParser for FontFaceRuleParser {
    type Prelude = String;
    type QualifiedRule = Option<FontFaceRule>;

    fn parse_prelude(&mut self, input: &mut Parser) -> Result<Self::Prelude, cssparser::ParseError<()>> {
        let ident = input.expect_ident()?;
        Ok(ident.to_string())
    }

    fn parse_block(&mut self, prelude: Self::Prelude, input: &mut Parser) -> Result<Self::QualifiedRule, cssparser::ParseError<()>> {
        if prelude != "@font-face" {
            return Ok(None);
        }

        let mut declarations = HashMap::new();
        while let Ok(decl) = input.parse_entirely(|i| DeclarationParser::parse_declaration(i)) {
            declarations.insert(decl.name.to_string(), decl.value.to_string());
        }

        let family = declarations.get("font-family").map(|s| s.trim_matches('"').trim_matches('\'').to_string()).unwrap_or_default();
        let src = declarations.get("src").map(|s| s.to_string()).unwrap_or_default();
        let weight = declarations.get("font-weight").map(|s| s.to_string()).unwrap_or_else(|| "400".to_string());
        let style = declarations.get("font-style").map(|s| s.to_string()).unwrap_or_else(|| "normal".to_string());

        if !family.is_empty() && !src.is_empty() {
            Ok(Some(FontFaceRule {
                family,
                src,
                weight,
                style,
            }))
        } else {
            Ok(None)
        }
    }
}
