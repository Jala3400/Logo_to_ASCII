pub struct Bitmap {
    pub data: Vec<f32>,
    pub width: usize,
    pub height: usize,
    pub max_brightness: f32,
}

pub struct FontBitmap {
    pub data: Vec<CharInfo>, // It is ordered by the min value, from highest to lowest
}

pub struct CharInfo {
    pub char: char,
    pub data: [f32; 8 * 16],
    pub min: usize,
}

impl FontBitmap {
    pub fn insert_ord(&mut self, char_info: CharInfo) {
        let mut i = 0;
        while i < self.data.len() && self.data[i].min < char_info.min {
            i += 1;
        }
        self.data.insert(i, char_info);
    }
}
