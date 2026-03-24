use super::audio_player_backend::AudioPlayerBackend;
use super::types::AudioPlayer;

#[cfg(not(target_arch = "wasm32"))]
#[path = "rodio_audio_player_actor.rs"]
mod rodio_audio_player_actor;

#[cfg(not(target_arch = "wasm32"))]
#[path = "awedio_audio_player_actor.rs"]
mod awedio_audio_player_actor;

#[path = "silent_audio_player_actor.rs"]
mod silent_audio_player_actor;

#[cfg(target_arch = "wasm32")]
#[path = "browser_audio_player_actor.rs"]
mod browser_audio_player_actor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlatformAudioPlayerKind {
    Silent,
    #[cfg(not(target_arch = "wasm32"))]
    Rodio,
    #[cfg(not(target_arch = "wasm32"))]
    Awedio,
    #[cfg(target_arch = "wasm32")]
    Browser,
}

#[must_use]
pub fn create_platform_audio_player(
    file_path: String,
    backend: AudioPlayerBackend,
) -> Box<dyn AudioPlayer> {
    #[cfg(target_arch = "wasm32")]
    {
        match platform_audio_player_kind(backend) {
            PlatformAudioPlayerKind::Browser => Box::new(
                browser_audio_player_actor::BrowserAudioPlayerActor::new(file_path),
            ),
            PlatformAudioPlayerKind::Silent => {
                Box::new(silent_audio_player_actor::SilentAudioPlayerActor::new())
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        match platform_audio_player_kind(backend) {
            PlatformAudioPlayerKind::Rodio => Box::new(
                rodio_audio_player_actor::RodioAudioPlayerActor::new(file_path),
            ),
            PlatformAudioPlayerKind::Awedio => Box::new(
                awedio_audio_player_actor::AwedioAudioPlayerActor::new(file_path),
            ),
            PlatformAudioPlayerKind::Silent => {
                Box::new(silent_audio_player_actor::SilentAudioPlayerActor::new())
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn platform_audio_player_kind(backend: AudioPlayerBackend) -> PlatformAudioPlayerKind {
    match backend {
        AudioPlayerBackend::Browser => PlatformAudioPlayerKind::Browser,
        AudioPlayerBackend::Rodio | AudioPlayerBackend::Awedio => PlatformAudioPlayerKind::Silent,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn platform_audio_player_kind(backend: AudioPlayerBackend) -> PlatformAudioPlayerKind {
    match backend {
        AudioPlayerBackend::Rodio => PlatformAudioPlayerKind::Rodio,
        AudioPlayerBackend::Awedio => PlatformAudioPlayerKind::Awedio,
        AudioPlayerBackend::Browser => PlatformAudioPlayerKind::Silent,
    }
}

#[cfg(test)]
mod tests {
    use super::PlatformAudioPlayerKind;
    use super::platform_audio_player_kind;
    use crate::audio_player::AudioPlayerBackend;

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn native_backends_map_to_distinct_player_kinds() {
        assert_eq!(
            platform_audio_player_kind(AudioPlayerBackend::Rodio),
            PlatformAudioPlayerKind::Rodio
        );
        assert_eq!(
            platform_audio_player_kind(AudioPlayerBackend::Awedio),
            PlatformAudioPlayerKind::Awedio
        );
        assert_eq!(
            platform_audio_player_kind(AudioPlayerBackend::Browser),
            PlatformAudioPlayerKind::Silent
        );
    }

    #[test]
    #[cfg(target_arch = "wasm32")]
    fn wasm_backend_always_uses_browser_player_kind() {
        assert_eq!(
            platform_audio_player_kind(AudioPlayerBackend::Browser),
            PlatformAudioPlayerKind::Browser
        );
    }
}
