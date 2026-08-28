use cssparser::{Parser, ParserInput, RuleListParser, QualifiedRuleParser, DeclarationParser};
use cssparser::stylesheet::{Stylesheet, CssRule, CssRuleType};

pub struct CSSParser;

impl CSSParser {
    pub fn parse(css: &str) -> Stylesheet {
        let mut input = ParserInput::new(css);
        let mut parser = Parser::new(&mut input);
        let mut rules = Vec::new();
        while let Ok(rule) = parser.parse_qualified_rule(0, RuleParser {}) {
            rules.push(rule);
        }
        Stylesheet { rules }
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
