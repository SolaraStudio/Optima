#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LengthUnit {
    Px,
    Em,
    Rem,
    Percent,
    Vw,
    Vh,
    Vmin,
    Vmax,
    Pt,
    Pc,
    In,
    Mm,
    Cm,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length {
    pub value: f32,
    pub unit: LengthUnit,
}

impl Length {
    pub fn new(value: f32, unit: LengthUnit) -> Self {
        Length { value, unit }
    }

    pub fn to_px(&self, base_font_size: f32) -> f32 {
        match self.unit {
            LengthUnit::Px => self.value,
            LengthUnit::Em => self.value * base_font_size,
            LengthUnit::Rem => self.value * base_font_size,
            LengthUnit::Percent => self.value / 100.0,
            LengthUnit::Vw => self.value / 100.0,
            LengthUnit::Vh => self.value / 100.0,
            LengthUnit::Vmin => self.value / 100.0,
            LengthUnit::Vmax => self.value / 100.0,
            LengthUnit::Pt => self.value * 1.3333,
            LengthUnit::Pc => self.value * 16.0,
            LengthUnit::In => self.value * 96.0,
            LengthUnit::Mm => self.value * 3.7795,
            LengthUnit::Cm => self.value * 37.7953,
        }
    }
}
