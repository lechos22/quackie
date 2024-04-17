use pancurses::ColorPair;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelData {
    character: char,
    color_pair: ColorPair,
}

impl PixelData {
    pub fn new(character: char, color_pair: ColorPair) -> Self {
        Self {
            character,
            color_pair,
        }
    }

    pub fn character(&self) -> char {
        self.character
    }

    pub fn color_pair(&self) -> ColorPair {
        self.color_pair
    }
}
