#[derive(Debug, Clone, PartialEq)]
pub enum FilterFunction {
    Blur(f32),
    Brightness(f32),
    Contrast(f32),
    Grayscale(f32),
    Opacity(f32),
    Sepia(f32),
    HueRotate(f32),
    Saturate(f32),
    Invert(f32),
    DropShadow(f32, f32, f32, f32),
}

impl FilterFunction {
    pub fn is_identity(&self) -> bool {
        match self {
            FilterFunction::Blur(r) => *r == 0.0,
            FilterFunction::Brightness(v) => *v == 1.0,
            FilterFunction::Contrast(v) => *v == 1.0,
            FilterFunction::Grayscale(v) => *v == 0.0,
            FilterFunction::Opacity(v) => *v == 1.0,
            FilterFunction::Sepia(v) => *v == 0.0,
            FilterFunction::HueRotate(_) => false,
            FilterFunction::Saturate(v) => *v == 1.0,
            FilterFunction::Invert(v) => *v == 0.0,
            FilterFunction::DropShadow(_, _, _, a) => *a == 0.0,
        }
    }

    pub fn to_css(&self) -> String {
        match self {
            FilterFunction::Blur(r) => format!("blur({}px)", r),
            FilterFunction::Brightness(v) => format!("brightness({})", v),
            FilterFunction::Contrast(v) => format!("contrast({})", v),
            FilterFunction::Grayscale(v) => format!("grayscale({})", v),
            FilterFunction::Opacity(v) => format!("opacity({})", v),
            FilterFunction::Sepia(v) => format!("sepia({})", v),
            FilterFunction::HueRotate(deg) => format!("hue-rotate({}deg)", deg),
            FilterFunction::Saturate(v) => format!("saturate({})", v),
            FilterFunction::Invert(v) => format!("invert({})", v),
            FilterFunction::DropShadow(x, y, blur, a) => {
                format!("drop-shadow({}px {}px {}px {})", x, y, blur, a)
            }
        }
    }

    pub fn apply_opacity(&self, value: f32) -> f32 {
        match self {
            FilterFunction::Opacity(o) => value * o,
            _ => value,
        }
    }

    pub fn apply_brightness(&self, value: f32) -> f32 {
        match self {
            FilterFunction::Brightness(b) => value * b,
            _ => value,
        }
    }

    pub fn apply_contrast(&self, value: f32) -> f32 {
        match self {
            FilterFunction::Contrast(c) => {
                let midpoint = 0.5;
                midpoint + (value - midpoint) * c
            }
            _ => value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterChain {
    pub functions: Vec<FilterFunction>,
}

impl FilterChain {
    pub fn new() -> Self {
        FilterChain {
            functions: Vec::new(),
        }
    }

    pub fn push(&mut self, filter: FilterFunction) {
        self.functions.push(filter);
    }

    pub fn is_empty(&self) -> bool {
        self.functions.is_empty()
    }

    pub fn is_identity(&self) -> bool {
        self.functions.iter().all(|f| f.is_identity())
    }

    pub fn to_css(&self) -> String {
        self.functions
            .iter()
            .map(|f| f.to_css())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn apply_opacity(&self, value: f32) -> f32 {
        self.functions.iter().fold(value, |v, f| f.apply_opacity(v))
    }

    pub fn apply_brightness(&self, value: f32) -> f32 {
        self.functions
            .iter()
            .fold(value, |v, f| f.apply_brightness(v))
    }

    pub fn apply_contrast(&self, value: f32) -> f32 {
        self.functions
            .iter()
            .fold(value, |v, f| f.apply_contrast(v))
    }

    pub fn blur_radius(&self) -> f32 {
        self.functions
            .iter()
            .filter_map(|f| match f {
                FilterFunction::Blur(r) => Some(*r),
                _ => None,
            })
            .sum()
    }

    pub fn hue_rotation_degrees(&self) -> f32 {
        self.functions
            .iter()
            .filter_map(|f| match f {
                FilterFunction::HueRotate(deg) => Some(*deg),
                _ => None,
            })
            .sum()
    }

    pub fn total_grayscale(&self) -> f32 {
        self.functions
            .iter()
            .filter_map(|f| match f {
                FilterFunction::Grayscale(v) => Some(*v),
                _ => None,
            })
            .sum::<f32>()
            .clamp(0.0, 1.0)
    }

    pub fn total_sepia(&self) -> f32 {
        self.functions
            .iter()
            .filter_map(|f| match f {
                FilterFunction::Sepia(v) => Some(*v),
                _ => None,
            })
            .sum::<f32>()
            .clamp(0.0, 1.0)
    }

    pub fn combine(&mut self, other: &FilterChain) {
        for f in &other.functions {
            self.functions.push(f.clone());
        }
    }

    pub fn filter_count(&self) -> usize {
        self.functions.len()
    }
}

pub fn parse_filter_value(input: &str) -> Option<FilterFunction> {
    let input = input.trim();
    if let Some(rest) = input.strip_prefix("blur(") {
        let rest = rest.strip_suffix(')')?;
        let value: f32 = rest.trim().trim_end_matches("px").parse().ok()?;
        Some(FilterFunction::Blur(value))
    } else if let Some(rest) = input.strip_prefix("brightness(") {
        let rest = rest.strip_suffix(')')?;
        let value: f32 = rest.trim().parse().ok()?;
        Some(FilterFunction::Brightness(value))
    } else if let Some(rest) = input.strip_prefix("contrast(") {
        let rest = rest.strip_suffix(')')?;
        let value: f32 = rest.trim().parse().ok()?;
        Some(FilterFunction::Contrast(value))
    } else if let Some(rest) = input.strip_prefix("grayscale(") {
        let rest = rest.strip_suffix(')')?;
        let value: f32 = rest.trim().parse().ok()?;
        Some(FilterFunction::Grayscale(value))
    } else if let Some(rest) = input.strip_prefix("opacity(") {
        let rest = rest.strip_suffix(')')?;
        let value: f32 = rest.trim().parse().ok()?;
        Some(FilterFunction::Opacity(value))
    } else if let Some(rest) = input.strip_prefix("sepia(") {
        let rest = rest.strip_suffix(')')?;
        let value: f32 = rest.trim().parse().ok()?;
        Some(FilterFunction::Sepia(value))
    } else if let Some(rest) = input.strip_prefix("hue-rotate(") {
        let rest = rest.strip_suffix(')')?;
        let value: f32 = rest.trim().trim_end_matches("deg").parse().ok()?;
        Some(FilterFunction::HueRotate(value))
    } else if let Some(rest) = input.strip_prefix("saturate(") {
        let rest = rest.strip_suffix(')')?;
        let value: f32 = rest.trim().parse().ok()?;
        Some(FilterFunction::Saturate(value))
    } else if let Some(rest) = input.strip_prefix("invert(") {
        let rest = rest.strip_suffix(')')?;
        let value: f32 = rest.trim().parse().ok()?;
        Some(FilterFunction::Invert(value))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_identity() {
        assert!(FilterFunction::Blur(0.0).is_identity());
        assert!(FilterFunction::Brightness(1.0).is_identity());
        assert!(FilterFunction::Contrast(1.0).is_identity());
        assert!(FilterFunction::Grayscale(0.0).is_identity());
        assert!(FilterFunction::Opacity(1.0).is_identity());
        assert!(FilterFunction::Sepia(0.0).is_identity());
        assert!(FilterFunction::Saturate(1.0).is_identity());
        assert!(FilterFunction::Invert(0.0).is_identity());
        assert!(!FilterFunction::Blur(5.0).is_identity());
        assert!(!FilterFunction::HueRotate(0.0).is_identity());
    }

    #[test]
    fn test_filter_to_css() {
        assert_eq!(FilterFunction::Blur(5.0).to_css(), "blur(5px)");
        assert_eq!(FilterFunction::Brightness(1.5).to_css(), "brightness(1.5)");
        assert_eq!(FilterFunction::Contrast(2.0).to_css(), "contrast(2)");
        assert_eq!(FilterFunction::Grayscale(1.0).to_css(), "grayscale(1)");
        assert_eq!(
            FilterFunction::HueRotate(90.0).to_css(),
            "hue-rotate(90deg)"
        );
        assert_eq!(FilterFunction::Sepia(0.5).to_css(), "sepia(0.5)");
    }

    #[test]
    fn test_filter_chain_to_css() {
        let mut chain = FilterChain::new();
        chain.push(FilterFunction::Blur(2.0));
        chain.push(FilterFunction::Brightness(1.2));
        assert_eq!(chain.to_css(), "blur(2px) brightness(1.2)");
    }

    #[test]
    fn test_filter_chain_is_identity() {
        let mut chain = FilterChain::new();
        assert!(chain.is_identity());
        chain.push(FilterFunction::Brightness(1.0));
        assert!(chain.is_identity());
        chain.push(FilterFunction::Blur(5.0));
        assert!(!chain.is_identity());
    }

    #[test]
    fn test_apply_opacity_chain() {
        let mut chain = FilterChain::new();
        chain.push(FilterFunction::Opacity(0.5));
        chain.push(FilterFunction::Opacity(0.8));
        let result = chain.apply_opacity(1.0);
        assert!((result - 0.4).abs() < 0.01);
    }

    #[test]
    fn test_apply_brightness_chain() {
        let mut chain = FilterChain::new();
        chain.push(FilterFunction::Brightness(2.0));
        let result = chain.apply_brightness(0.5);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_apply_contrast() {
        let filter = FilterFunction::Contrast(2.0);
        let result = filter.apply_contrast(0.5);
        assert_eq!(result, 0.5);
        let result = filter.apply_contrast(0.3);
        assert!((result - 0.1).abs() < 0.01);
    }

    #[test]
    fn test_blur_radius() {
        let mut chain = FilterChain::new();
        chain.push(FilterFunction::Blur(3.0));
        chain.push(FilterFunction::Blur(2.0));
        assert_eq!(chain.blur_radius(), 5.0);
    }

    #[test]
    fn test_hue_rotation() {
        let mut chain = FilterChain::new();
        chain.push(FilterFunction::HueRotate(45.0));
        chain.push(FilterFunction::HueRotate(30.0));
        assert_eq!(chain.hue_rotation_degrees(), 75.0);
    }

    #[test]
    fn test_total_grayscale() {
        let mut chain = FilterChain::new();
        chain.push(FilterFunction::Grayscale(0.3));
        chain.push(FilterFunction::Grayscale(0.4));
        assert!((chain.total_grayscale() - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_total_grayscale_clamped() {
        let mut chain = FilterChain::new();
        chain.push(FilterFunction::Grayscale(0.8));
        chain.push(FilterFunction::Grayscale(0.8));
        assert_eq!(chain.total_grayscale(), 1.0);
    }

    #[test]
    fn test_combine() {
        let mut a = FilterChain::new();
        a.push(FilterFunction::Blur(1.0));
        let mut b = FilterChain::new();
        b.push(FilterFunction::Brightness(2.0));
        a.combine(&b);
        assert_eq!(a.filter_count(), 2);
    }

    #[test]
    fn test_parse_blur() {
        let f = parse_filter_value("blur(5px)");
        assert_eq!(f, Some(FilterFunction::Blur(5.0)));
    }

    #[test]
    fn test_parse_brightness() {
        let f = parse_filter_value("brightness(1.5)");
        assert_eq!(f, Some(FilterFunction::Brightness(1.5)));
    }

    #[test]
    fn test_parse_contrast() {
        let f = parse_filter_value("contrast(2)");
        assert_eq!(f, Some(FilterFunction::Contrast(2.0)));
    }

    #[test]
    fn test_parse_grayscale() {
        let f = parse_filter_value("grayscale(0.5)");
        assert_eq!(f, Some(FilterFunction::Grayscale(0.5)));
    }

    #[test]
    fn test_parse_hue_rotate() {
        let f = parse_filter_value("hue-rotate(90deg)");
        assert_eq!(f, Some(FilterFunction::HueRotate(90.0)));
    }

    #[test]
    fn test_parse_saturate() {
        let f = parse_filter_value("saturate(2.0)");
        assert_eq!(f, Some(FilterFunction::Saturate(2.0)));
    }

    #[test]
    fn test_parse_invalid() {
        assert!(parse_filter_value("invalid()").is_none());
    }

    #[test]
    fn test_drop_shadow_to_css() {
        assert_eq!(
            FilterFunction::DropShadow(2.0, 4.0, 6.0, 0.5).to_css(),
            "drop-shadow(2px 4px 6px 0.5)"
        );
    }

    #[test]
    fn test_filter_chain_empty() {
        let chain = FilterChain::new();
        assert!(chain.is_empty());
        assert_eq!(chain.to_css(), "");
    }
}
