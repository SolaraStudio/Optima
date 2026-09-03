use crate::css::units::Length;

#[derive(Debug, Clone, Default)]
pub struct BoxModel {
    pub content_x: f32,
    pub content_y: f32,
    pub content_width: f32,
    pub content_height: f32,
    pub margin_top: f32,
    pub margin_right: f32,
    pub margin_bottom: f32,
    pub margin_left: f32,
    pub padding_top: f32,
    pub padding_right: f32,
    pub padding_bottom: f32,
    pub padding_left: f32,
    pub border_top: f32,
    pub border_right: f32,
    pub border_bottom: f32,
    pub border_left: f32,
    pub border_top_color: (f32, f32, f32, f32),
    pub border_right_color: (f32, f32, f32, f32),
    pub border_bottom_color: (f32, f32, f32, f32),
    pub border_left_color: (f32, f32, f32, f32),
}

impl BoxModel {
    pub fn new() -> Self { Self::default() }

    pub fn total_width(&self) -> f32 {
        self.content_width + self.padding_left + self.padding_right
            + self.border_left + self.border_right
            + self.margin_left + self.margin_right
    }

    pub fn total_height(&self) -> f32 {
        self.content_height + self.padding_top + self.padding_bottom
            + self.border_top + self.border_bottom
            + self.margin_top + self.margin_bottom
    }

    pub fn padding_box_width(&self) -> f32 {
        self.content_width + self.padding_left + self.padding_right + self.border_left + self.border_right
    }

    pub fn padding_box_height(&self) -> f32 {
        self.content_height + self.padding_top + self.padding_bottom + self.border_top + self.border_bottom
    }

    pub fn border_box_width(&self) -> f32 {
        self.content_width + self.padding_left + self.padding_right + self.border_left + self.border_right
    }

    pub fn border_box_height(&self) -> f32 {
        self.content_height + self.padding_top + self.padding_bottom + self.border_top + self.border_bottom
    }

    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.content_x && x <= self.content_x + self.total_width()
            && y >= self.content_y && y <= self.content_y + self.total_height()
    }

    pub fn border_rect(&self) -> (f32, f32, f32, f32) {
        (self.content_x - self.border_left, self.content_y - self.border_top,
         self.border_box_width(), self.border_box_height())
    }

    pub fn padding_rect(&self) -> (f32, f32, f32, f32) {
        (self.content_x + self.border_left, self.content_y + self.border_top,
         self.content_width + self.padding_left + self.padding_right,
         self.content_height + self.padding_top + self.padding_bottom)
    }
}
