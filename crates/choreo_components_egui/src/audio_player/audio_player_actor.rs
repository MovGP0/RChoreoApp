use super::audio_player_backend::AudioPlayerBackend;
use super::types::AudioPlayer;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native_audio_player_actor.rs"]
mod native_audio_player_actor;

#[cfg(target_arch = "wasm32")]
#[path = "browser_audio_player_actor.rs"]
mod browser_audio_player_actor;

#[must_use]
pub fn create_platform_audio_player(
    file_path: String,
    backend: AudioPlayerBackend,
) -> Box<dyn AudioPlayer> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = backend;
        Box::new(browser_audio_player_actor::BrowserAudioPlayerActor::new(
            file_path,
        ))
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        match backend {
            AudioPlayerBackend::Awedio
            | AudioPlayerBackend::Rodio
            | AudioPlayerBackend::Browser => Box::new(
                native_audio_player_actor::NativeAudioPlayerActor::new(file_path),
            ),
        }
    }
}
