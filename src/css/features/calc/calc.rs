#[derive(Debug, Clone, PartialEq)]
pub enum CalcOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CalcTerm {
    Number(f32),
    Percentage(f32),
    Length(f32, String),
    Expression(Box<CalcExpression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalcExpression {
    pub terms: Vec<CalcTerm>,
    pub operators: Vec<CalcOp>,
}

impl CalcExpression {
    pub fn new() -> Self {
        CalcExpression {
            terms: Vec::new(),
            operators: Vec::new(),
        }
    }

    pub fn from_number(value: f32) -> Self {
        let mut expr = CalcExpression::new();
        expr.terms.push(CalcTerm::Number(value));
        expr
    }

    pub fn push_term(&mut self, term: CalcTerm) {
        self.terms.push(term);
    }

    pub fn push_op(&mut self, op: CalcOp) {
        self.operators.push(op);
    }

    pub fn evaluate(&self, context: &CalcContext) -> f32 {
        if self.terms.is_empty() {
            return 0.0;
        }
        let mut values: Vec<f32> = self.terms.iter().map(|t| t.resolve(context)).collect();
        let mut ops = self.operators.clone();

        let mut i = 0;
        while i < ops.len() {
            match ops[i] {
                CalcOp::Mul => {
                    values[i] = values[i] * values[i + 1];
                    values.remove(i + 1);
                    ops.remove(i);
                }
                CalcOp::Div => {
                    if values[i + 1] != 0.0 {
                        values[i] = values[i] / values[i + 1];
                    }
                    values.remove(i + 1);
                    ops.remove(i);
                }
                _ => i += 1,
            }
        }

        i = 0;
        while i < ops.len() {
            match ops[i] {
                CalcOp::Add => {
                    values[i] = values[i] + values[i + 1];
                    values.remove(i + 1);
                    ops.remove(i);
                }
                CalcOp::Sub => {
                    values[i] = values[i] - values[i + 1];
                    values.remove(i + 1);
                    ops.remove(i);
                }
                _ => i += 1,
            }
        }

        values.first().copied().unwrap_or(0.0)
    }

    pub fn simplify(&self) -> CalcExpression {
        let context = CalcContext::new();
        let result = self.evaluate(&context);
        CalcExpression::from_number(result)
    }
}

#[derive(Debug, Clone)]
pub struct CalcContext {
    pub base_font_size: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
}

impl CalcContext {
    pub fn new() -> Self {
        CalcContext {
            base_font_size: 16.0,
            viewport_width: 1920.0,
            viewport_height: 1080.0,
        }
    }

    pub fn with_font_size(base_font_size: f32) -> Self {
        CalcContext {
            base_font_size,
            viewport_width: 1920.0,
            viewport_height: 1080.0,
        }
    }
}

impl CalcTerm {
    pub fn resolve(&self, context: &CalcContext) -> f32 {
        match self {
            CalcTerm::Number(n) => *n,
            CalcTerm::Percentage(p) => p / 100.0,
            CalcTerm::Length(value, unit) => match unit.as_str() {
                "px" => *value,
                "em" | "rem" => value * context.base_font_size,
                "vw" => value / 100.0 * context.viewport_width,
                "vh" => value / 100.0 * context.viewport_height,
                "pt" => value * 1.3333,
                _ => *value,
            },
            CalcTerm::Expression(expr) => expr.evaluate(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        let mut expr = CalcExpression::new();
        expr.push_term(CalcTerm::Number(2.0));
        expr.push_op(CalcOp::Add);
        expr.push_term(CalcTerm::Number(3.0));
        let ctx = CalcContext::new();
        assert_eq!(expr.evaluate(&ctx), 5.0);
    }

    #[test]
    fn test_precedence_mul_before_add() {
        let mut expr = CalcExpression::new();
        expr.push_term(CalcTerm::Number(2.0));
        expr.push_op(CalcOp::Add);
        expr.push_term(CalcTerm::Number(3.0));
        expr.push_op(CalcOp::Mul);
        expr.push_term(CalcTerm::Number(4.0));
        let ctx = CalcContext::new();
        assert_eq!(expr.evaluate(&ctx), 14.0);
    }

    #[test]
    fn test_division() {
        let mut expr = CalcExpression::new();
        expr.push_term(CalcTerm::Number(10.0));
        expr.push_op(CalcOp::Div);
        expr.push_term(CalcTerm::Number(2.0));
        let ctx = CalcContext::new();
        assert_eq!(expr.evaluate(&ctx), 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        let mut expr = CalcExpression::new();
        expr.push_term(CalcTerm::Number(10.0));
        expr.push_op(CalcOp::Div);
        expr.push_term(CalcTerm::Number(0.0));
        let ctx = CalcContext::new();
        assert_eq!(expr.evaluate(&ctx), 10.0);
    }

    #[test]
    fn test_px_length() {
        let mut expr = CalcExpression::new();
        expr.push_term(CalcTerm::Length(10.0, "px".to_string()));
        expr.push_op(CalcOp::Add);
        expr.push_term(CalcTerm::Length(5.0, "px".to_string()));
        let ctx = CalcContext::new();
        assert_eq!(expr.evaluate(&ctx), 15.0);
    }

    #[test]
    fn test_em_length() {
        let expr = CalcExpression::from_number(2.0);
        let ctx = CalcContext::with_font_size(20.0);
        let term = CalcTerm::Length(2.0, "em".to_string());
        assert_eq!(term.resolve(&ctx), 40.0);
        assert_eq!(expr.evaluate(&ctx), 2.0);
    }

    #[test]
    fn test_percentage() {
        let term = CalcTerm::Percentage(50.0);
        let ctx = CalcContext::new();
        assert_eq!(term.resolve(&ctx), 0.5);
    }

    #[test]
    fn test_nested_expression() {
        let inner = {
            let mut e = CalcExpression::new();
            e.push_term(CalcTerm::Number(3.0));
            e.push_op(CalcOp::Mul);
            e.push_term(CalcTerm::Number(4.0));
            e
        };
        let mut outer = CalcExpression::new();
        outer.push_term(CalcTerm::Number(1.0));
        outer.push_op(CalcOp::Add);
        outer.push_term(CalcTerm::Expression(Box::new(inner)));
        let ctx = CalcContext::new();
        assert_eq!(outer.evaluate(&ctx), 13.0);
    }

    #[test]
    fn test_subtraction() {
        let mut expr = CalcExpression::new();
        expr.push_term(CalcTerm::Number(10.0));
        expr.push_op(CalcOp::Sub);
        expr.push_term(CalcTerm::Number(3.0));
        let ctx = CalcContext::new();
        assert_eq!(expr.evaluate(&ctx), 7.0);
    }
}
