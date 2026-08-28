use cssparser::{Parser, ParserInput, QualifiedRuleParser, DeclarationParser};
use cssparser::stylesheet::{Stylesheet, CssRule};
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

    pub fn parse_color(color: &str) -> Option<(u8, u8, u8, u8)> {
        if color.starts_with('#') {
            let hex = &color[1..];
            match hex.len() {
                3 => {
                    let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                    let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                    let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                    Some((r, g, b, 255))
                }
                6 => {
                    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                    Some((r, g, b, 255))
                }
                8 => {
                    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                    let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                    Some((r, g, b, a))
                }
                _ => None,
            }
        } else {
            match color {
                "black" => Some((0, 0, 0, 255)),
                "white" => Some((255, 255, 255, 255)),
                "red" => Some((255, 0, 0, 255)),
                "green" => Some((0, 255, 0, 255)),
                "blue" => Some((0, 0, 255, 255)),
                "yellow" => Some((255, 255, 0, 255)),
                "cyan" => Some((0, 255, 255, 255)),
                "magenta" => Some((255, 0, 255, 255)),
                "gray" | "grey" => Some((128, 128, 128, 255)),
                _ => None,
            }
        }
    }
}

struct RuleParser;

impl QualifiedRuleParser for RuleParser {
    type Prelude = String;
    type QualifiedRule = CssRule;

    fn parse_prelude(&mut self, input: &mut Parser) -> Result<Self::Prelude, cssparser::ParseError<()>> {
        let selector = input.expect_ident()?;
        Ok(selector.to_string())
    }

    fn parse_block(&mut self, prelude: Self::Prelude, input: &mut Parser) -> Result<Self::QualifiedRule, cssparser::ParseError<()>> {
        let mut declarations = Vec::new();
        while let Ok(decl) = input.parse_entirely(|i| DeclarationParser::parse_declaration(i)) {
            declarations.push(decl);
        }
        Ok(CssRule::Style {
            prelude,
            declarations,
        })
    }
}
