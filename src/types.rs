use std::collections::HashMap;

pub struct Bitmap {
    pub data: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub struct FontBitmap {
    pub data: HashMap<char, CharInfo>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub struct CharInfo {
    pub data: Vec<f32>,
    pub min: usize,
}