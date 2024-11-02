use std::collections::HashMap;

pub struct Bitmap {
    pub data: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

pub struct FontBitmap {
    pub data: HashMap<char, CharInfo>,
    pub width: usize,
    pub height: usize,
}

pub struct CharInfo {
    pub data: Vec<f32>,
    // pub min: f32,
}