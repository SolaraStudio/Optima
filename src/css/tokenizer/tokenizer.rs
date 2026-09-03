use crate::css::colors::Color;
use crate::css::units::Length;
use crate::css::units::LengthUnit;
use crate::css::value::Value;
use cssparser::{Parser, Token};

pub struct CSSParser;

impl CSSParser {
    pub fn parse_identifier(input: &mut Parser) -> Option<String> {
        if let Ok(ident) = input.expect_ident() {
            Some(ident.to_string())
        } else {
            None
        }
    }

    pub fn parse_string(input: &mut Parser) -> Option<String> {
        if let Ok(string) = input.expect_string() {
            Some(string.to_string())
        } else {
            None
        }
    }

    pub fn parse_number(input: &mut Parser) -> Option<f32> {
        if let Ok(num) = input.expect_number() {
            Some(num)
        } else {
            None
        }
    }

    pub fn parse_length(input: &mut Parser) -> Option<Length> {
        let state = input.state();
        if let Ok(num) = input.expect_number() {
            if let Ok(ident) = input.expect_ident() {
                let unit = match ident.as_ref() {
                    "px" => LengthUnit::Px,
                    "em" => LengthUnit::Em,
                    "rem" => LengthUnit::Rem,
                    "%" => LengthUnit::Percent,
                    "vw" => LengthUnit::Vw,
                    "vh" => LengthUnit::Vh,
                    "vmin" => LengthUnit::Vmin,
                    "vmax" => LengthUnit::Vmax,
                    "pt" => LengthUnit::Pt,
                    "pc" => LengthUnit::Pc,
                    "in" => LengthUnit::In,
                    "mm" => LengthUnit::Mm,
                    "cm" => LengthUnit::Cm,
                    _ => return None,
                };
                return Some(Length::new(num, unit));
            }
        }
        input.reset(&state);
        None
    }

    pub fn parse_color(input: &mut Parser) -> Option<Color> {
        let state = input.state();
        if let Ok(ident) = input.expect_ident() {
            let color = match ident.as_ref() {
                "black" => Color::new(0.0, 0.0, 0.0, 1.0),
                "white" => Color::new(1.0, 1.0, 1.0, 1.0),
                "red" => Color::new(1.0, 0.0, 0.0, 1.0),
                "green" => Color::new(0.0, 1.0, 0.0, 1.0),
                "blue" => Color::new(0.0, 0.0, 1.0, 1.0),
                "yellow" => Color::new(1.0, 1.0, 0.0, 1.0),
                "cyan" => Color::new(0.0, 1.0, 1.0, 1.0),
                "magenta" => Color::new(1.0, 0.0, 1.0, 1.0),
                "gray" | "grey" => Color::new(0.5, 0.5, 0.5, 1.0),
                _ => return None,
            };
            return Some(color);
        }
        input.reset(&state);

        match input.next() {
            Ok(Token::Hash(hex)) => {
                let s = hex.as_ref();
                if s.len() == 3 || s.len() == 4 || s.len() == 6 || s.len() == 8 {
                    return Color::from_hex(s);
                }
            }
            _ => {}
        }
        None
    }

    pub fn parse_value(input: &mut Parser) -> Option<Value> {
        let state = input.state();
        if let Some(color) = Self::parse_color(input) {
            return Some(Value::Color(color));
        }
        input.reset(&state);

        let state = input.state();
        if let Some(length) = Self::parse_length(input) {
            return Some(Value::Length(length));
        }
        input.reset(&state);

        let state = input.state();
        if let Some(num) = Self::parse_number(input) {
            return Some(Value::Number(num));
        }
        input.reset(&state);

        let state = input.state();
        if let Some(ident) = Self::parse_identifier(input) {
            return Some(Value::Keyword(ident));
        }
        input.reset(&state);

        let state = input.state();
        if let Some(string) = Self::parse_string(input) {
            return Some(Value::String(string));
        }
        input.reset(&state);

        None
    }
}
