use std::{collections::HashMap, sync::Arc};

use pancurses::{Attribute, Attributes, ColorPair};
use serde::{Deserialize, Serialize};

use crate::{
    geometry::{triangle::Triangle2D, vector::Vector2D},
    graphics::{
        image_buffer::ImageBuffer, pixel_data::PixelData, textured_triangle::TexturedTriangle2D,
    },
};

fn empty_vec<T>() -> Vec<T> {
    Vec::new()
}

#[derive(Deserialize, Serialize)]
enum AttributeRepr {
    AlternativeCharSet,
    Bold,
    Blink,
    CharText,
    Dim,
    Leftline,
    Invisible,
    Italic,
    Normal,
    Overline,
    Reverse,
    Rightline,
    Strikeout,
    Underline,
}
impl Into<Attribute> for AttributeRepr {
    fn into(self) -> Attribute {
        match self {
            AttributeRepr::AlternativeCharSet => Attribute::AlternativeCharSet,
            AttributeRepr::Bold => Attribute::Bold,
            AttributeRepr::Blink => Attribute::Blink,
            AttributeRepr::CharText => Attribute::CharText,
            AttributeRepr::Dim => Attribute::Dim,
            AttributeRepr::Leftline => Attribute::Leftline,
            AttributeRepr::Invisible => Attribute::Invisible,
            AttributeRepr::Italic => Attribute::Italic,
            AttributeRepr::Normal => Attribute::Normal,
            AttributeRepr::Overline => Attribute::Overline,
            AttributeRepr::Reverse => Attribute::Reverse,
            AttributeRepr::Rightline => Attribute::Rightline,
            AttributeRepr::Strikeout => Attribute::Strikeout,
            AttributeRepr::Underline => Attribute::Underline,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct AttributesRepr {
    color: u8,
    #[serde(default = "empty_vec")]
    flags: Vec<AttributeRepr>,
}
impl Into<Attributes> for AttributesRepr {
    fn into(self) -> Attributes {
        let mut attrs = Attributes::new() | ColorPair(self.color);
        for attr in self.flags {
            attrs = attrs | Into::<Attribute>::into(attr);
        }
        attrs
    }
}

#[derive(Deserialize, Serialize)]
struct PixelRepr(char, AttributesRepr);
impl Into<PixelData> for PixelRepr {
    fn into(self) -> PixelData {
        PixelData::new(self.0, self.1.into())
    }
}

#[derive(Deserialize, Serialize)]
struct TextureRepr {
    background: PixelRepr,
    width: usize,
    height: usize,
    pixels: Vec<PixelRepr>,
}
impl Into<ImageBuffer> for TextureRepr {
    fn into(self) -> ImageBuffer {
        let mut buffer = ImageBuffer::new(self.width, self.height, self.background.into());
        for (n, pixel) in self.pixels.into_iter().enumerate() {
            buffer.set_pixel_at(n % self.width, n / self.height, pixel.into());
        }
        buffer
    }
}

#[derive(Deserialize, Serialize)]
struct PointRepr(f64, f64);
impl Into<Vector2D> for PointRepr {
    fn into(self) -> Vector2D {
        Vector2D::new(self.0, self.1)
    }
}

#[derive(Deserialize, Serialize)]
struct TriangleRepr([PointRepr; 3]);
impl Into<Triangle2D> for TriangleRepr {
    fn into(self) -> Triangle2D {
        Triangle2D::new(self.0.map(Into::<Vector2D>::into))
    }
}

#[derive(Deserialize, Serialize)]
struct TexturedTriangleRepr {
    texture: String,
    geometry: TriangleRepr,
    uv_geometry: TriangleRepr,
}

#[derive(Deserialize, Serialize)]
struct DuckFile {
    textures: HashMap<String, TextureRepr>,
    triangles: Vec<TexturedTriangleRepr>,
}

pub fn read_duckfile(duckfile: &str) -> Vec<TexturedTriangle2D> {
    let duckfile: DuckFile = serde_json::from_str(duckfile).unwrap();
    let textures: HashMap<String, Arc<ImageBuffer>> = duckfile
        .textures
        .into_iter()
        .map(|(name, texture)| (name, Arc::new(texture.into())))
        .collect();
    let triangles: Vec<TexturedTriangle2D> = duckfile
        .triangles
        .into_iter()
        .filter_map(|triangle| {
            Some(TexturedTriangle2D::new(
                triangle.geometry.into(),
                triangle.uv_geometry.into(),
                textures.get(&triangle.texture)?.clone(),
            ))
        })
        .collect();
    triangles
}
