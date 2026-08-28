use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ComputedStyle {
    pub color: Option<String>,
    pub background: Option<String>,
    pub font_size: Option<f32>,
    pub font_family: Option<String>,
    pub font_weight: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub display: Option<String>,
    pub position: Option<String>,
    pub margin: Option<f32>,
    pub padding: Option<f32>,
    pub border: Option<f32>,
    pub margin_top: Option<f32>,
    pub margin_bottom: Option<f32>,
    pub margin_left: Option<f32>,
    pub margin_right: Option<f32>,
    pub padding_top: Option<f32>,
    pub padding_bottom: Option<f32>,
    pub padding_left: Option<f32>,
    pub padding_right: Option<f32>,
    pub border_top: Option<f32>,
    pub border_bottom: Option<f32>,
    pub border_left: Option<f32>,
    pub border_right: Option<f32>,
    pub border_color: Option<String>,
    pub border_style: Option<String>,
    pub border_radius: Option<f32>,
    pub opacity: Option<f32>,
    pub visibility: Option<String>,
    pub z_index: Option<i32>,
    pub flex_direction: Option<String>,
    pub justify_content: Option<String>,
    pub align_items: Option<String>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub flex_basis: Option<f32>,
    pub grid_template_columns: Option<String>,
    pub grid_template_rows: Option<String>,
    pub gap: Option<f32>,
    pub overflow: Option<String>,
    pub text_align: Option<String>,
    pub text_decoration: Option<String>,
    pub text_transform: Option<String>,
    pub line_height: Option<f32>,
    pub letter_spacing: Option<f32>,
    pub word_spacing: Option<f32>,
}

impl ComputedStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            background: None,
            font_size: None,
            font_family: None,
            font_weight: None,
            width: None,
            height: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            display: None,
            position: None,
            margin: None,
            padding: None,
            border: None,
            margin_top: None,
            margin_bottom: None,
            margin_left: None,
            margin_right: None,
            padding_top: None,
            padding_bottom: None,
            padding_left: None,
            padding_right: None,
            border_top: None,
            border_bottom: None,
            border_left: None,
            border_right: None,
            border_color: None,
            border_style: None,
            border_radius: None,
            opacity: None,
            visibility: None,
            z_index: None,
            flex_direction: None,
            justify_content: None,
            align_items: None,
            flex_grow: None,
            flex_shrink: None,
            flex_basis: None,
            grid_template_columns: None,
            grid_template_rows: None,
            gap: None,
            overflow: None,
            text_align: None,
            text_decoration: None,
            text_transform: None,
            line_height: None,
            letter_spacing: None,
            word_spacing: None,
        }
    }

    pub fn from_declarations(decls: &HashMap<String, String>) -> Self {
        let mut style = Self::new();
        for (key, value) in decls {
            match key.as_str() {
                "color" => style.color = Some(value.clone()),
                "background" => style.background = Some(value.clone()),
                "background-color" => style.background = Some(value.clone()),
                "font-size" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.font_size = Some(v);
                    }
                }
                "font-family" => style.font_family = Some(value.clone()),
                "font-weight" => {
                    if let Ok(v) = value.parse::<f32>() {
                        style.font_weight = Some(v);
                    }
                }
                "width" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.width = Some(v);
                    }
                }
                "height" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.height = Some(v);
                    }
                }
                "min-width" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.min_width = Some(v);
                    }
                }
                "max-width" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.max_width = Some(v);
                    }
                }
                "min-height" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.min_height = Some(v);
                    }
                }
                "max-height" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.max_height = Some(v);
                    }
                }
                "display" => style.display = Some(value.clone()),
                "position" => style.position = Some(value.clone()),
                "margin" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.margin = Some(v);
                    }
                }
                "padding" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.padding = Some(v);
                    }
                }
                "border" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.border = Some(v);
                    }
                }
                "margin-top" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.margin_top = Some(v);
                    }
                }
                "margin-bottom" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.margin_bottom = Some(v);
                    }
                }
                "margin-left" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.margin_left = Some(v);
                    }
                }
                "margin-right" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.margin_right = Some(v);
                    }
                }
                "padding-top" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.padding_top = Some(v);
                    }
                }
                "padding-bottom" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.padding_bottom = Some(v);
                    }
                }
                "padding-left" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.padding_left = Some(v);
                    }
                }
                "padding-right" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.padding_right = Some(v);
                    }
                }
                "border-top" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.border_top = Some(v);
                    }
                }
                "border-bottom" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.border_bottom = Some(v);
                    }
                }
                "border-left" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.border_left = Some(v);
                    }
                }
                "border-right" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.border_right = Some(v);
                    }
                }
                "border-color" => style.border_color = Some(value.clone()),
                "border-style" => style.border_style = Some(value.clone()),
                "border-radius" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.border_radius = Some(v);
                    }
                }
                "opacity" => {
                    if let Ok(v) = value.parse::<f32>() {
                        style.opacity = Some(v);
                    }
                }
                "visibility" => style.visibility = Some(value.clone()),
                "z-index" => {
                    if let Ok(v) = value.parse::<i32>() {
                        style.z_index = Some(v);
                    }
                }
                "flex-direction" => style.flex_direction = Some(value.clone()),
                "justify-content" => style.justify_content = Some(value.clone()),
                "align-items" => style.align_items = Some(value.clone()),
                "flex-grow" => {
                    if let Ok(v) = value.parse::<f32>() {
                        style.flex_grow = Some(v);
                    }
                }
                "flex-shrink" => {
                    if let Ok(v) = value.parse::<f32>() {
                        style.flex_shrink = Some(v);
                    }
                }
                "flex-basis" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.flex_basis = Some(v);
                    }
                }
                "grid-template-columns" => style.grid_template_columns = Some(value.clone()),
                "grid-template-rows" => style.grid_template_rows = Some(value.clone()),
                "gap" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.gap = Some(v);
                    }
                }
                "overflow" => style.overflow = Some(value.clone()),
                "text-align" => style.text_align = Some(value.clone()),
                "text-decoration" => style.text_decoration = Some(value.clone()),
                "text-transform" => style.text_transform = Some(value.clone()),
                "line-height" => {
                    if let Ok(v) = value.parse::<f32>() {
                        style.line_height = Some(v);
                    }
                }
                "letter-spacing" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.letter_spacing = Some(v);
                    }
                }
                "word-spacing" => {
                    if let Ok(v) = value.trim_end_matches("px").parse::<f32>() {
                        style.word_spacing = Some(v);
                    }
                }
                _ => {}
            }
        }
        style
    }

    pub fn inherit_from(&mut self, parent: &ComputedStyle) {
        if self.color.is_none() {
            self.color = parent.color.clone();
        }
        if self.font_size.is_none() {
            self.font_size = parent.font_size;
        }
        if self.font_family.is_none() {
            self.font_family = parent.font_family.clone();
        }
        if self.font_weight.is_none() {
            self.font_weight = parent.font_weight;
        }
        if self.display.is_none() {
            self.display = parent.display.clone();
        }
        if self.visibility.is_none() {
            self.visibility = parent.visibility.clone();
        }
        if self.text_align.is_none() {
            self.text_align = parent.text_align.clone();
        }
        if self.text_transform.is_none() {
            self.text_transform = parent.text_transform.clone();
        }
        if self.line_height.is_none() {
            self.line_height = parent.line_height;
        }
        if self.letter_spacing.is_none() {
            self.letter_spacing = parent.letter_spacing;
        }
        if self.word_spacing.is_none() {
            self.word_spacing = parent.word_spacing;
        }
    }

    pub fn merge(&mut self, other: ComputedStyle) {
        if let Some(v) = other.color { self.color = Some(v); }
        if let Some(v) = other.background { self.background = Some(v); }
        if let Some(v) = other.font_size { self.font_size = Some(v); }
        if let Some(v) = other.font_family { self.font_family = Some(v); }
        if let Some(v) = other.font_weight { self.font_weight = Some(v); }
        if let Some(v) = other.width { self.width = Some(v); }
        if let Some(v) = other.height { self.height = Some(v); }
        if let Some(v) = other.min_width { self.min_width = Some(v); }
        if let Some(v) = other.max_width { self.max_width = Some(v); }
        if let Some(v) = other.min_height { self.min_height = Some(v); }
        if let Some(v) = other.max_height { self.max_height = Some(v); }
        if let Some(v) = other.display { self.display = Some(v); }
        if let Some(v) = other.position { self.position = Some(v); }
        if let Some(v) = other.margin { self.margin = Some(v); }
        if let Some(v) = other.padding { self.padding = Some(v); }
        if let Some(v) = other.border { self.border = Some(v); }
        if let Some(v) = other.margin_top { self.margin_top = Some(v); }
        if let Some(v) = other.margin_bottom { self.margin_bottom = Some(v); }
        if let Some(v) = other.margin_left { self.margin_left = Some(v); }
        if let Some(v) = other.margin_right { self.margin_right = Some(v); }
        if let Some(v) = other.padding_top { self.padding_top = Some(v); }
        if let Some(v) = other.padding_bottom { self.padding_bottom = Some(v); }
        if let Some(v) = other.padding_left { self.padding_left = Some(v); }
        if let Some(v) = other.padding_right { self.padding_right = Some(v); }
        if let Some(v) = other.border_top { self.border_top = Some(v); }
        if let Some(v) = other.border_bottom { self.border_bottom = Some(v); }
        if let Some(v) = other.border_left { self.border_left = Some(v); }
        if let Some(v) = other.border_right { self.border_right = Some(v); }
        if let Some(v) = other.border_color { self.border_color = Some(v); }
        if let Some(v) = other.border_style { self.border_style = Some(v); }
        if let Some(v) = other.border_radius { self.border_radius = Some(v); }
        if let Some(v) = other.opacity { self.opacity = Some(v); }
        if let Some(v) = other.visibility { self.visibility = Some(v); }
        if let Some(v) = other.z_index { self.z_index = Some(v); }
        if let Some(v) = other.flex_direction { self.flex_direction = Some(v); }
        if let Some(v) = other.justify_content { self.justify_content = Some(v); }
        if let Some(v) = other.align_items { self.align_items = Some(v); }
        if let Some(v) = other.flex_grow { self.flex_grow = Some(v); }
        if let Some(v) = other.flex_shrink { self.flex_shrink = Some(v); }
        if let Some(v) = other.flex_basis { self.flex_basis = Some(v); }
        if let Some(v) = other.grid_template_columns { self.grid_template_columns = Some(v); }
        if let Some(v) = other.grid_template_rows { self.grid_template_rows = Some(v); }
        if let Some(v) = other.gap { self.gap = Some(v); }
        if let Some(v) = other.overflow { self.overflow = Some(v); }
        if let Some(v) = other.text_align { self.text_align = Some(v); }
        if let Some(v) = other.text_decoration { self.text_decoration = Some(v); }
        if let Some(v) = other.text_transform { self.text_transform = Some(v); }
        if let Some(v) = other.line_height { self.line_height = Some(v); }
        if let Some(v) = other.letter_spacing { self.letter_spacing = Some(v); }
        if let Some(v) = other.word_spacing { self.word_spacing = Some(v); }
    }

    pub fn get_font_size(&self) -> f32 {
        self.font_size.unwrap_or(16.0)
    }

    pub fn get_width(&self) -> Option<f32> {
        self.width
    }

    pub fn get_height(&self) -> Option<f32> {
        self.height
    }

    pub fn get_display(&self) -> &str {
        self.display.as_deref().unwrap_or("block")
    }

    pub fn is_display_none(&self) -> bool {
        self.display.as_deref() == Some("none")
    }

    pub fn is_visible(&self) -> bool {
        self.visibility.as_deref() != Some("hidden")
    }

    pub fn get_opacity(&self) -> f32 {
        self.opacity.unwrap_or(1.0)
    }

    pub fn get_z_index(&self) -> i32 {
        self.z_index.unwrap_or(0)
    }
}

impl Default for ComputedStyle {
    fn default() -> Self {
        Self::new()
    }
}
