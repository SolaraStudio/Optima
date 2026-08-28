#[derive(Debug, Clone, Copy)]
pub struct BoxModel {
    pub margin: f32,
    pub padding: f32,
    pub border: f32,
    pub margin_top: f32,
    pub margin_bottom: f32,
    pub margin_left: f32,
    pub margin_right: f32,
    pub padding_top: f32,
    pub padding_bottom: f32,
    pub padding_left: f32,
    pub padding_right: f32,
    pub border_top: f32,
    pub border_bottom: f32,
    pub border_left: f32,
    pub border_right: f32,
}

impl BoxModel {
    pub fn new() -> Self {
        Self {
            margin: 0.0,
            padding: 0.0,
            border: 0.0,
            margin_top: 0.0,
            margin_bottom: 0.0,
            margin_left: 0.0,
            margin_right: 0.0,
            padding_top: 0.0,
            padding_bottom: 0.0,
            padding_left: 0.0,
            padding_right: 0.0,
            border_top: 0.0,
            border_bottom: 0.0,
            border_left: 0.0,
            border_right: 0.0,
        }
    }

    pub fn with_margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self.margin_top = margin;
        self.margin_bottom = margin;
        self.margin_left = margin;
        self.margin_right = margin;
        self
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self.padding_top = padding;
        self.padding_bottom = padding;
        self.padding_left = padding;
        self.padding_right = padding;
        self
    }

    pub fn with_border(mut self, border: f32) -> Self {
        self.border = border;
        self.border_top = border;
        self.border_bottom = border;
        self.border_left = border;
        self.border_right = border;
        self
    }

    pub fn with_margin_top(mut self, margin_top: f32) -> Self {
        self.margin_top = margin_top;
        self
    }

    pub fn with_padding_left(mut self, padding_left: f32) -> Self {
        self.padding_left = padding_left;
        self
    }

    pub fn get_total_width(&self, content_width: f32) -> f32 {
        content_width + self.padding_left + self.padding_right + self.border_left + self.border_right
    }

    pub fn get_total_height(&self, content_height: f32) -> f32 {
        content_height + self.padding_top + self.padding_bottom + self.border_top + self.border_bottom
    }

    pub fn get_margin_width(&self) -> f32 {
        self.margin_left + self.margin_right
    }

    pub fn get_margin_height(&self) -> f32 {
        self.margin_top + self.margin_bottom
    }

    pub fn get_border_width(&self) -> f32 {
        self.border_left + self.border_right
    }

    pub fn get_border_height(&self) -> f32 {
        self.border_top + self.border_bottom
    }

    pub fn get_padding_width(&self) -> f32 {
        self.padding_left + self.padding_right
    }

    pub fn get_padding_height(&self) -> f32 {
        self.padding_top + self.padding_bottom
    }
}

impl Default for BoxModel {
    fn default() -> Self {
        Self::new()
    }
}
