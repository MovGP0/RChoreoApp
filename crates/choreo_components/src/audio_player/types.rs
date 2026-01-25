use std::io;

pub trait HapticFeedback {
    fn is_supported(&self) -> bool;
    fn perform_click(&self);
}

pub trait AudioPlayer {
    fn is_playing(&self) -> bool;
    fn can_seek(&self) -> bool;
    fn can_set_speed(&self) -> bool;
    fn duration(&self) -> f64;
    fn current_position(&self) -> f64;
    fn play(&mut self);
    fn pause(&mut self);
    fn stop(&mut self);
    fn seek(&mut self, position: f64);
    fn set_speed(&mut self, speed: f64);
    fn set_volume(&mut self, volume: f64);
    fn set_balance(&mut self, balance: f64);
    fn set_loop(&mut self, loop_enabled: bool);
}

pub type StreamFactory =
    Box<dyn Fn() -> io::Result<Box<dyn io::Read + Send>> + Send + Sync>;
