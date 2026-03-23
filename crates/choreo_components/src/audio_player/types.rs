use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioPlayerSample {
    pub is_playing: bool,
    pub can_seek: bool,
    pub can_set_speed: bool,
    pub duration: f64,
    pub position: f64,
    pub speed: f64,
    pub volume: f64,
    pub balance: f64,
    pub loop_enabled: bool,
}

pub trait AudioPlayer {
    fn sample(&self) -> AudioPlayerSample;
    fn play(&mut self);
    fn pause(&mut self);
    fn stop(&mut self);
    fn seek(&mut self, position: f64);
    fn seek_and_play(&mut self, position: f64) {
        self.seek(position);
        self.play();
    }
    fn set_speed(&mut self, speed: f64);
    fn set_volume(&mut self, volume: f64);
    fn set_balance(&mut self, balance: f64);
    fn set_loop(&mut self, loop_enabled: bool);
}

pub type StreamFactory = Box<dyn Fn() -> io::Result<Box<dyn io::Read + Send>> + Send + Sync>;
