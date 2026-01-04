use crate::sprite_data::SpriteData;

// Messages sent to the networking thread
#[derive(Debug)]
pub enum ToNetworking {
    FetchSprite,
    Shutdown,
}

// Messages sent to the main thread
#[derive(Debug)]
pub enum ToMain {
    SpriteFetched(SpriteData),
    Error(String),
    ShutdownAck,
}
