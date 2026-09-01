use super::font_face::FontFaceRule;
use super::stylesheet::Stylesheet;
use cssparser::{Parser, ParserInput, QualifiedRuleParser, DeclarationParser};
use std::collections::HashMap;

pub struct CSSParser;

impl CSSParser {
    pub fn parse(css: &str) -> Stylesheet {
        let mut input = ParserInput::new(css);
        let mut parser = Parser::new(&mut input);
        let mut rules = Vec::new();
        while let Ok(rule) = parser.parse_qualified_rule(0, &mut RuleParser) {
            rules.push(rule);
        }
        Stylesheet { rules }
    }

    pub fn parse_declaration_block(block: &str) -> HashMap<String, String> {
        let mut declarations = HashMap::new();
        let mut input = ParserInput::new(block);
        let mut parser = Parser::new(&mut input);
        while let Ok(decl) = parser.parse_entirely(|i| DeclarationParser::parse_declaration(i)) {
            declarations.insert(decl.name.to_string(), decl.value.to_string());
        }
        declarations
    }

    pub fn parse_selector(selector: &str) -> Option<String> {
        let mut input = ParserInput::new(selector);
        let mut parser = Parser::new(&mut input);
        if let Ok(ident) = parser.expect_ident() {
            Some(ident.to_string())
        } else {
            None
        }
    }

    pub fn parse_font_faces(css: &str) -> Vec<FontFaceRule> {
        FontFaceRule::parse_from_css(css)
    }

    pub fn parse_color(color: &str) -> Option<(u8, u8, u8, u8)> {
        // ... (existing implementation)
        None
    }
}

struct RuleParser;

impl QualifiedRuleParser for RuleParser {
    type Prelude = String;
    type QualifiedRule = cssparser::stylesheet::CssRule;

    fn parse_prelude(&mut self, input: &mut Parser) -> Result<Self::Prelude, cssparser::ParseError<()>> {
        let selector = input.expect_ident()?;
        Ok(selector.to_string())
    }

    fn parse_block(&mut self, prelude: Self::Prelude, input: &mut Parser) -> Result<Self::QualifiedRule, cssparser::ParseError<()>> {
        let mut declarations = Vec::new();
        while let Ok(decl) = input.parse_entirely(|i| DeclarationParser::parse_declaration(i)) {
            declarations.push(decl);
        }
        Ok(cssparser::stylesheet::CssRule::Style {
            prelude,
            declarations,
        })
    }
}
