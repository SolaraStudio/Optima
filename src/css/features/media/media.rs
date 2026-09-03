#[derive(Debug, Clone, PartialEq)]
pub enum MediaFeature {
    MinWidth(f32),
    MaxWidth(f32),
    MinHeight(f32),
    MaxHeight(f32),
    Orientation(Orientation),
    PrefersColorScheme(ColorScheme),
    PrefersReducedMotion(bool),
    AspectRatio(f32, f32),
    Hover(HoverCapability),
    Pointer(PointerCapability),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Orientation {
    Portrait,
    Landscape,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorScheme {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HoverCapability {
    Hover,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointerCapability {
    Fine,
    Coarse,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MediaType {
    Screen,
    Print,
    All,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MediaCombinator {
    And,
    Or,
    Not,
}

#[derive(Debug, Clone)]
pub struct MediaQuery {
    pub media_type: MediaType,
    pub features: Vec<MediaFeature>,
    pub combinators: Vec<MediaCombinator>,
}

impl MediaQuery {
    pub fn new(media_type: MediaType) -> Self {
        MediaQuery {
            media_type,
            features: Vec::new(),
            combinators: Vec::new(),
        }
    }

    pub fn all() -> Self {
        MediaQuery::new(MediaType::All)
    }

    pub fn screen() -> Self {
        MediaQuery::new(MediaType::Screen)
    }

    pub fn print() -> Self {
        MediaQuery::new(MediaType::Print)
    }

    pub fn add_feature(&mut self, feature: MediaFeature) {
        if !self.features.is_empty() {
            self.combinators.push(MediaCombinator::And);
        }
        self.features.push(feature);
    }

    pub fn matches(&self, context: &MediaContext) -> bool {
        let type_matches = match self.media_type {
            MediaType::All => true,
            MediaType::Screen => context.is_screen,
            MediaType::Print => !context.is_screen,
        };
        if !type_matches {
            return false;
        }
        if self.features.is_empty() {
            return true;
        }
        let mut results: Vec<bool> = self.features.iter().map(|f| f.matches(context)).collect();
        for (i, comb) in self.combinators.iter().enumerate() {
            match comb {
                MediaCombinator::Not => {
                    if i < results.len() {
                        results[i] = !results[i];
                    }
                }
                _ => {}
            }
        }
        results.iter().all(|&r| r)
    }
}

impl MediaFeature {
    pub fn matches(&self, context: &MediaContext) -> bool {
        match self {
            MediaFeature::MinWidth(w) => context.width >= *w,
            MediaFeature::MaxWidth(w) => context.width <= *w,
            MediaFeature::MinHeight(h) => context.height >= *h,
            MediaFeature::MaxHeight(h) => context.height <= *h,
            MediaFeature::Orientation(o) => {
                let actual = if context.width >= context.height {
                    Orientation::Landscape
                } else {
                    Orientation::Portrait
                };
                actual == *o
            }
            MediaFeature::PrefersColorScheme(scheme) => context.color_scheme == *scheme,
            MediaFeature::PrefersReducedMotion(prefers) => context.prefers_reduced_motion == *prefers,
            MediaFeature::AspectRatio(w, h) => {
                let ratio = context.width / context.height;
                let expected = w / h;
                (ratio - expected).abs() < 0.01
            }
            MediaFeature::Hover(cap) => context.hover == *cap,
            MediaFeature::Pointer(cap) => context.pointer == *cap,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MediaContext {
    pub width: f32,
    pub height: f32,
    pub is_screen: bool,
    pub color_scheme: ColorScheme,
    pub prefers_reduced_motion: bool,
    pub hover: HoverCapability,
    pub pointer: PointerCapability,
}

impl MediaContext {
    pub fn new() -> Self {
        MediaContext {
            width: 1920.0,
            height: 1080.0,
            is_screen: true,
            color_scheme: ColorScheme::Light,
            prefers_reduced_motion: false,
            hover: HoverCapability::Hover,
            pointer: PointerCapability::Fine,
        }
    }

    pub fn with_dimensions(width: f32, height: f32) -> Self {
        MediaContext {
            width,
            height,
            ..MediaContext::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_width_match() {
        let mq = MediaQuery {
            media_type: MediaType::Screen,
            features: vec![MediaFeature::MinWidth(768.0)],
            combinators: Vec::new(),
        };
        let ctx = MediaContext::with_dimensions(1024.0, 768.0);
        assert!(mq.matches(&ctx));
    }

    #[test]
    fn test_min_width_no_match() {
        let mq = MediaQuery {
            media_type: MediaType::Screen,
            features: vec![MediaFeature::MinWidth(768.0)],
            combinators: Vec::new(),
        };
        let ctx = MediaContext::with_dimensions(500.0, 768.0);
        assert!(!mq.matches(&ctx));
    }

    #[test]
    fn test_max_width_match() {
        let mq = MediaQuery {
            media_type: MediaType::All,
            features: vec![MediaFeature::MaxWidth(600.0)],
            combinators: Vec::new(),
        };
        let ctx = MediaContext::with_dimensions(400.0, 800.0);
        assert!(mq.matches(&ctx));
    }

    #[test]
    fn test_dark_mode() {
        let mut mq = MediaQuery::screen();
        mq.add_feature(MediaFeature::PrefersColorScheme(ColorScheme::Dark));
        let mut ctx = MediaContext::new();
        ctx.color_scheme = ColorScheme::Dark;
        assert!(mq.matches(&ctx));
        ctx.color_scheme = ColorScheme::Light;
        assert!(!mq.matches(&ctx));
    }

    #[test]
    fn test_multiple_features_and() {
        let mut mq = MediaQuery::screen();
        mq.add_feature(MediaFeature::MinWidth(768.0));
        mq.add_feature(MediaFeature::MaxWidth(1024.0));
        let ctx_small = MediaContext::with_dimensions(500.0, 800.0);
        assert!(!mq.matches(&ctx_small));
        let ctx_ok = MediaContext::with_dimensions(900.0, 800.0);
        assert!(mq.matches(&ctx_ok));
    }

    #[test]
    fn test_print_media_type() {
        let mq = MediaQuery::print();
        let ctx_screen = MediaContext::new();
        assert!(!mq.matches(&ctx_screen));
    }

    #[test]
    fn test_all_media_type() {
        let mq = MediaQuery::all();
        let ctx = MediaContext::new();
        assert!(mq.matches(&ctx));
    }

    #[test]
    fn test_orientation() {
        let mut mq = MediaQuery::all();
        mq.add_feature(MediaFeature::Orientation(Orientation::Landscape));
        let ctx = MediaContext::with_dimensions(1920.0, 1080.0);
        assert!(mq.matches(&ctx));
        let ctx_portrait = MediaContext::with_dimensions(500.0, 800.0);
        assert!(!mq.matches(&ctx_portrait));
    }

    #[test]
    fn test_aspect_ratio() {
        let mut mq = MediaQuery::all();
        mq.add_feature(MediaFeature::AspectRatio(16.0, 9.0));
        let ctx = MediaContext::with_dimensions(1920.0, 1080.0);
        assert!(mq.matches(&ctx));
    }

    #[test]
    fn test_reduced_motion() {
        let mut mq = MediaQuery::all();
        mq.add_feature(MediaFeature::PrefersReducedMotion(true));
        let mut ctx = MediaContext::new();
        ctx.prefers_reduced_motion = true;
        assert!(mq.matches(&ctx));
        ctx.prefers_reduced_motion = false;
        assert!(!mq.matches(&ctx));
    }
}
