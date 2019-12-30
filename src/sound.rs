use std::collections::HashMap;

use ggez::{Context, audio::{SoundData, SoundSource, Source}};

pub struct SoundPlayer {
    //sounds: HashMap<String, SoundData>,
    music: Source
}

impl SoundPlayer {
    pub fn new(ctx: &mut Context) -> Self {
        let mut music = Source::new(ctx, "/music/Stuck.mp3").unwrap();
        music.play().unwrap();
        SoundPlayer {
            music,
        }
    }
}