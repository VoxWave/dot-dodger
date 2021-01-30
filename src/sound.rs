use std::{
    collections::HashMap,
    sync::{
        mpsc::Receiver,
        mpsc::Sender,
        Mutex,    
    },
};

use ggez::{Context, audio::{SoundData, SoundSource, Source}};

pub struct SoundPlayer {
    library: HashMap<String, SoundData>,
    music: Option<(String, Source)>,
    playing_sounds: Vec<Source>,
    commands: Receiver<SoundMessage>,
}

pub enum SoundMessage {
    PlayMusic{name: String, looping: bool},
    StopMusic,
    PauseMusic,
    PlaySound{name: String, looping: bool},
}

impl SoundPlayer {
    pub fn new(ctx: &mut Context, recv: Receiver<SoundMessage>) -> Self {
        let mut sounds = HashMap::new();
        let music = SoundData::new(ctx, "/music/stuck.mp3").unwrap();
        sounds.insert("stuck".to_string(), music.clone());
        sounds.insert("desert".to_string(), SoundData::new(ctx, "/music/desert.mp3").unwrap());
        let mut music = Source::from_data(ctx, music).unwrap();
        music.play(ctx).unwrap();
        SoundPlayer {
            library: sounds,
            music: Some(("stuck".to_string(), music)),
            playing_sounds: Vec::new(),
            commands: recv,
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        use SoundMessage::*;
        for cmd in self.commands.try_iter() {
            match cmd {
                PlayMusic{name, looping} => {
                    match self.music.take() {
                        Some((cur_name, mut src)) => {
                            if name != cur_name {
                                let mut music = Source::from_data(ctx, self.library.get(&name).unwrap().clone()).unwrap();
                                src.stop(ctx);
                                music.set_repeat(looping);
                                music.play(ctx).unwrap();
                                self.music = Some((name, music));
                            } else {
                                self.music = Some((cur_name, src));
                            }
                        }
                        None => {
                            let mut music = Source::from_data(ctx, self.library.get(&name).unwrap().clone()).unwrap();
                            music.set_repeat(looping);
                            music.play(ctx).unwrap();
                            self.music = Some((name, music));
                        }
                    };
                },
                _ => {}
            }
        }
    }
}

pub struct SoundChannel(pub Mutex<Sender<SoundMessage>>);