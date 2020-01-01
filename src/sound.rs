use std::collections::HashMap;

use ggez::{Context, audio::{SoundData, SoundSource, Source}};

pub struct SoundPlayer {
    library: HashMap<String, SoundData>,
    music: Option<(String, Source)>,
    playing_sounds: Vec<Source>
}

pub enum SoundMessage {
    PlayMusic{name: String, looping: bool},
    StopMusic,
    PauseMusic,
    PlaySound()
}

impl SoundPlayer {
    pub fn new(ctx: &mut Context, ) -> Self {
        let sounds = HashMap::new();
        let music = SoundData::new(ctx, "/music/Stuck.mp3").unwrap();
        let mut music = Source::from_data(ctx, music).unwrap();
        music.play().unwrap();
        SoundPlayer {
            library: sounds,
            music: Some(("Stuck", music)),
        }
    }

    pub fn update(ctx: &mut Context)
}