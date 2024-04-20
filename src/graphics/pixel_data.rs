use pancurses::Attributes;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PixelData {
    character: char,
    attributes: Attributes,
}

impl PixelData {
    pub fn new(character: char, attributes: Attributes) -> Self {
        Self {
            character,
            attributes,
        }
    }

    pub fn character(&self) -> char {
        self.character
    }

    pub fn attributes(&self) -> Attributes {
        self.attributes
    }
}
