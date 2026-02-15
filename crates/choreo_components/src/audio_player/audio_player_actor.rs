use super::audio_player_backend::AudioPlayerBackend;
use super::types::AudioPlayer;

#[cfg(not(target_arch = "wasm32"))]
#[path = "rodio_audio_player_actor.rs"]
mod rodio_audio_player_actor;

#[cfg(not(target_arch = "wasm32"))]
#[path = "awedio_audio_player_actor.rs"]
mod awedio_audio_player_actor;

#[cfg(target_arch = "wasm32")]
#[path = "browser_audio_player_actor.rs"]
mod browser_audio_player_actor;

pub fn create_platform_audio_player(
    file_path: String,
    backend: AudioPlayerBackend,
) -> Box<dyn AudioPlayer> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = backend;
        Box::new(browser_audio_player_actor::BrowserAudioPlayerActor::new(file_path))
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        match backend {
            AudioPlayerBackend::Awedio => {
                Box::new(awedio_audio_player_actor::AwedioAudioPlayerActor::new(file_path))
            }
            AudioPlayerBackend::Rodio | AudioPlayerBackend::Browser => {
                Box::new(rodio_audio_player_actor::RodioAudioPlayerActor::new(file_path))
            }
        }
    }
}
