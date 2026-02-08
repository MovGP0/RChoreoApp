mod audio_player_actor;
mod audio_player_behavior;
mod audio_player_link_scene_behavior;
mod audio_player_linking;
mod audio_player_position_changed_behavior;
mod audio_player_ticks_behavior;
mod audio_player_view_model;
mod close_audio_file_behavior;
mod messages;
mod open_audio_file_behavior;
mod types;

pub use crate::haptics::{HapticFeedback, NoopHapticFeedback, PlatformHapticFeedback};
pub use audio_player_actor::create_platform_audio_player;
pub use audio_player_behavior::AudioPlayerBehavior;
pub use audio_player_link_scene_behavior::AudioPlayerLinkSceneBehavior;
pub use audio_player_position_changed_behavior::AudioPlayerPositionChangedBehavior;
pub use audio_player_ticks_behavior::AudioPlayerTicksBehavior;
pub use audio_player_view_model::{
    AudioPlayerViewModel, AudioPlayerViewState, PlayPauseGlyph, duration_to_time_text,
    play_pause_glyph, speed_to_percent_text,
};
pub use close_audio_file_behavior::CloseAudioFileBehavior;
pub use messages::{
    AudioPlayerPositionChangedEvent, CloseAudioFileCommand, LinkSceneToPositionCommand,
    OpenAudioFileCommand,
};
pub use open_audio_file_behavior::OpenAudioFileBehavior;
pub use types::{AudioPlayer, StreamFactory};

use std::rc::Rc;

use crossbeam_channel::{Receiver, Sender};

use crate::behavior::Behavior;
use crate::global::GlobalStateActor;
use crate::preferences::Preferences;

pub struct AudioPlayerBehaviorDependencies {
    pub global_state_store: Rc<GlobalStateActor>,
    pub open_audio_receiver: Receiver<OpenAudioFileCommand>,
    pub close_audio_receiver: Receiver<CloseAudioFileCommand>,
    pub position_changed_sender: Sender<AudioPlayerPositionChangedEvent>,
    pub link_scene_receiver: Receiver<LinkSceneToPositionCommand>,
    pub preferences: Rc<dyn Preferences>,
}

pub fn build_audio_player_behaviors(
    deps: AudioPlayerBehaviorDependencies,
) -> Vec<Box<dyn Behavior<AudioPlayerViewModel>>> {
    vec![
        Box::new(AudioPlayerBehavior),
        Box::new(OpenAudioFileBehavior::new(
            deps.open_audio_receiver,
            deps.preferences,
        )),
        Box::new(CloseAudioFileBehavior::new(deps.close_audio_receiver)),
        Box::new(AudioPlayerTicksBehavior::new(
            deps.global_state_store.clone(),
        )),
        Box::new(AudioPlayerLinkSceneBehavior::new(
            deps.global_state_store,
            deps.link_scene_receiver,
        )),
        Box::new(AudioPlayerPositionChangedBehavior::new(
            deps.position_changed_sender,
        )),
    ]
}
