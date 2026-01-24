use std::io;

pub trait HapticFeedback {
    fn is_supported(&self) -> bool;
    fn perform_click(&self);
}

pub trait AudioPlayer {
    fn is_playing(&self) -> bool;
    fn can_seek(&self) -> bool;
    fn current_position(&self) -> f64;
    fn play(&mut self);
    fn pause(&mut self);
    fn stop(&mut self);
    fn seek(&mut self, position: f64);
}

pub type StreamFactory =
    Box<dyn Fn() -> io::Result<Box<dyn io::Read + Send>> + Send + Sync>;

pub struct AudioPlayerViewModel {
    pub speed: f64,
    pub minimum_speed: f64,
    pub maximum_speed: f64,
    pub volume: f64,
    pub balance: f64,
    pub duration: f64,
    pub position: f64,
    pub tick_values: String,
    pub can_link_scene_to_position: bool,
    pub is_playing: bool,
    pub loop_enabled: bool,
    pub can_seek: bool,
    pub can_set_speed: bool,
    pub stream_factory: Option<StreamFactory>,
    pub preparation_seconds: f64,
    pub pause_seconds: f64,
    pub title: String,
    pub player: Option<Box<dyn AudioPlayer>>,
    pub haptic_feedback: Option<Box<dyn HapticFeedback>>,
}

impl AudioPlayerViewModel {
    pub fn new(haptic_feedback: Option<Box<dyn HapticFeedback>>) -> Self {
        Self {
            speed: 1.0,
            minimum_speed: 0.8,
            maximum_speed: 1.1,
            volume: 1.0,
            balance: 0.0,
            duration: 0.0,
            position: 0.0,
            tick_values: String::new(),
            can_link_scene_to_position: false,
            is_playing: false,
            loop_enabled: false,
            can_seek: false,
            can_set_speed: false,
            stream_factory: None,
            preparation_seconds: 4.0,
            pause_seconds: 0.0,
            title: "Audio".to_string(),
            player: None,
            haptic_feedback,
        }
    }

    pub fn toggle_play_pause(&mut self) {
        if let Some(haptic) = &self.haptic_feedback
            && haptic.is_supported()
        {
            haptic.perform_click();
        }

        let Some(player) = self.player.as_mut() else {
            return;
        };

        if player.is_playing() {
            player.pause();
            self.is_playing = false;
            return;
        }

        player.play();
        self.is_playing = true;
    }

    pub fn stop(&mut self) {
        let Some(player) = self.player.as_mut() else {
            return;
        };

        player.stop();
        self.is_playing = false;
        self.position = 0.0;
    }

    pub fn seek(&mut self, position: f64) {
        let Some(player) = self.player.as_mut() else {
            return;
        };

        if !player.can_seek() {
            return;
        }

        player.seek(position);
        self.position = player.current_position();
    }

    pub fn reload(&mut self) {
        // handled by the stream factory integration
    }

    pub fn link_scene_to_position(&mut self) {
        if let Some(haptic) = &self.haptic_feedback
            && haptic.is_supported()
        {
            haptic.perform_click();
        }
        // handled by behavior integration
    }
}
