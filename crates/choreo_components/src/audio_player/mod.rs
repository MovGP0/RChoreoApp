mod audio_player_behavior;
mod audio_player_link_scene_behavior;
mod audio_player_position_changed_behavior;
mod audio_player_ticks_behavior;
mod audio_player_view_model;
mod close_audio_file_behavior;
mod messages;
mod open_audio_file_behavior;
mod types;

pub use audio_player_behavior::AudioPlayerBehavior;
pub use audio_player_link_scene_behavior::AudioPlayerLinkSceneBehavior;
pub use audio_player_position_changed_behavior::AudioPlayerPositionChangedBehavior;
pub use audio_player_ticks_behavior::AudioPlayerTicksBehavior;
pub use audio_player_view_model::{
    duration_to_time_text, play_pause_glyph, AudioPlayerViewModel, AudioPlayerViewState,
    PlayPauseGlyph,
};
pub use close_audio_file_behavior::CloseAudioFileBehavior;
pub use messages::{AudioPlayerPositionChangedEvent, CloseAudioFileCommand, OpenAudioFileCommand};
pub use open_audio_file_behavior::OpenAudioFileBehavior;
pub use types::{AudioPlayer, HapticFeedback, StreamFactory};

use crossbeam_channel::{Receiver, Sender};
use crate::behavior::Behavior;

pub struct AudioPlayerDependencies<P: crate::preferences::Preferences> {
    pub open_audio_receiver: Receiver<OpenAudioFileCommand>,
    pub close_audio_receiver: Receiver<CloseAudioFileCommand>,
    pub position_changed_publisher: Sender<AudioPlayerPositionChangedEvent>,
    pub preferences: P,
}

pub fn build_audio_player_behaviors<P: crate::preferences::Preferences + 'static>(
    deps: AudioPlayerDependencies<P>,
) -> Vec<Box<dyn Behavior<AudioPlayerViewModel>>> {
    vec![
        Box::new(AudioPlayerBehavior),
        Box::new(OpenAudioFileBehavior::new(
            deps.open_audio_receiver,
            deps.preferences,
        )),
        Box::new(CloseAudioFileBehavior::new(deps.close_audio_receiver)),
        Box::new(AudioPlayerTicksBehavior),
        Box::new(AudioPlayerLinkSceneBehavior),
        Box::new(AudioPlayerPositionChangedBehavior::new(
            deps.position_changed_publisher,
        )),
    ]
}
