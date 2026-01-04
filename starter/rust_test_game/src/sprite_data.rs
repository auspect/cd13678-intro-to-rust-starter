use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SpriteData {
    pub x: f32,
    pub y: f32,
    pub width: i32,
    pub height: i32,
    pub r: i32,
    pub g: i32,
    pub b: i32,
}
