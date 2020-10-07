#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn r_f32(&self) -> f32 {
        self.r as f32 / 255.0
    }
    pub fn g_f32(&self) -> f32 {
        self.g as f32 / 255.0
    }
    pub fn b_f32(&self) -> f32 {
        self.b as f32 / 255.0
    }
    pub fn a_f32(&self) -> f32 {
        self.a as f32 / 255.0
    }
}
