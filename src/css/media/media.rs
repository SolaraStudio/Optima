#[derive(Debug, Clone)]
pub struct MediaQuery {
    pub condition: MediaCondition,
}

#[derive(Debug, Clone)]
pub struct MediaCondition {
    pub features: Vec<MediaFeature>,
}

#[derive(Debug, Clone)]
pub struct MediaFeature {
    pub name: String,
    pub value: Option<String>,
}

impl MediaQuery {
    pub fn new(condition: MediaCondition) -> Self {
        MediaQuery { condition }
    }

    pub fn matches(&self, viewport: &Viewport) -> bool {
        for feature in &self.condition.features {
            if !Self::matches_feature(feature, viewport) {
                return false;
            }
        }
        true
    }

    fn matches_feature(feature: &MediaFeature, viewport: &Viewport) -> bool {
        match feature.name.as_str() {
            "width" => {
                if let Some(val) = &feature.value {
                    if let Some(width) = parse_media_value(val) {
                        return viewport.width >= width;
                    }
                }
                true
            }
            "height" => {
                if let Some(val) = &feature.value {
                    if let Some(height) = parse_media_value(val) {
                        return viewport.height >= height;
                    }
                }
                true
            }
            "min-width" => {
                if let Some(val) = &feature.value {
                    if let Some(width) = parse_media_value(val) {
                        return viewport.width >= width;
                    }
                }
                true
            }
            "max-width" => {
                if let Some(val) = &feature.value {
                    if let Some(width) = parse_media_value(val) {
                        return viewport.width <= width;
                    }
                }
                true
            }
            "orientation" => {
                if let Some(val) = &feature.value {
                    if val == "portrait" {
                        return viewport.width <= viewport.height;
                    } else if val == "landscape" {
                        return viewport.width > viewport.height;
                    }
                }
                true
            }
            _ => true,
        }
    }
}

fn parse_media_value(val: &str) -> Option<u32> {
    let val = val.trim();
    if val.ends_with("px") {
        val.trim_end_matches("px").parse::<u32>().ok()
    } else {
        val.parse::<u32>().ok()
    }
}

pub struct Viewport {
    pub width: u32,
    pub height: u32,
}
