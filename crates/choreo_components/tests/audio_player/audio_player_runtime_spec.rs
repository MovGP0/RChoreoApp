use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use choreo_components::audio_player::AudioPlayerBackend;
use choreo_components::audio_player::runtime::AudioPlayerRuntime;
use choreo_components::audio_player::runtime::apply_player_sample;
use choreo_components::audio_player::state::AudioPlayerState;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn runtime_creates_player_for_platform_backend() {
    let mut runtime = AudioPlayerRuntime::new(AudioPlayerBackend::Rodio);
    let path = unique_temp_file("wav");
    write_test_wav(&path);
    runtime.open_file(path.to_string_lossy().into_owned());
    assert!(runtime.has_player());
    let _ = fs::remove_file(path);
}

#[test]
fn runtime_sample_syncs_audio_state_from_player() {
    let mut runtime = AudioPlayerRuntime::new(AudioPlayerBackend::Rodio);
    let path = unique_temp_file("wav");
    write_test_wav(&path);
    runtime.open_file(path.to_string_lossy().into_owned());
    runtime.seek_and_play(0.5);
    let is_playing = wait_until(
        Duration::from_millis(400),
        Duration::from_millis(20),
        || runtime.sample().is_some_and(|sample| sample.is_playing),
    );
    runtime.set_speed(1.05);
    let speed_applied = wait_until(
        Duration::from_millis(400),
        Duration::from_millis(20),
        || {
            runtime
                .sample()
                .is_some_and(|sample| (sample.speed - 1.05).abs() < 0.0001)
        },
    );
    runtime.set_volume(0.8);
    let volume_applied = wait_until(
        Duration::from_millis(400),
        Duration::from_millis(20),
        || {
            runtime
                .sample()
                .is_some_and(|sample| (sample.volume - 0.8).abs() < 0.0001)
        },
    );
    runtime.set_balance(-0.2);
    let balance_applied = wait_until(
        Duration::from_millis(400),
        Duration::from_millis(20),
        || {
            runtime
                .sample()
                .is_some_and(|sample| (sample.balance - (-0.2)).abs() < 0.0001)
        },
    );
    runtime.set_loop(true);
    let loop_applied = wait_until(
        Duration::from_millis(400),
        Duration::from_millis(20),
        || runtime.sample().is_some_and(|sample| sample.loop_enabled),
    );

    let sample = runtime.sample().expect("runtime should have a sample");
    let mut state = AudioPlayerState::default();
    apply_player_sample(&mut state, sample);

    let mut errors = Vec::new();

    check!(errors, state.has_player);
    check!(errors, is_playing);
    check!(errors, speed_applied);
    check!(errors, volume_applied);
    check!(errors, balance_applied);
    check!(errors, loop_applied);
    check!(errors, state.is_playing);
    check!(errors, state.can_set_speed);
    check_eq!(errors, state.speed, 1.05);
    check_eq!(errors, state.volume, 0.8);
    check_eq!(errors, state.balance, -0.2);
    check!(errors, state.loop_enabled);

    assert_no_errors(errors);

    let _ = fs::remove_file(path);
}

fn unique_temp_file(extension: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    let mut path = std::env::temp_dir();
    path.push(format!("rchoreo_audio_runtime_{nanos}.{extension}"));
    path
}

fn write_test_wav(path: &Path) {
    let sample_rate = 8_000_u32;
    let sample_count = 8_000_usize;
    let data_size = (sample_count * std::mem::size_of::<i16>()) as u32;
    let mut bytes = Vec::with_capacity(44 + data_size as usize);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + data_size).to_le_bytes());
    bytes.extend_from_slice(b"WAVE");
    bytes.extend_from_slice(b"fmt ");
    bytes.extend_from_slice(&16_u32.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&sample_rate.to_le_bytes());
    bytes.extend_from_slice(&(sample_rate * 2).to_le_bytes());
    bytes.extend_from_slice(&2_u16.to_le_bytes());
    bytes.extend_from_slice(&16_u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_size.to_le_bytes());
    for index in 0..sample_count {
        let sample = if index % 32 < 16 {
            i16::MAX / 6
        } else {
            -(i16::MAX / 6)
        };
        bytes.extend_from_slice(&sample.to_le_bytes());
    }
    fs::write(path, bytes).expect("test wav file should be written");
}

fn wait_until(timeout: Duration, interval: Duration, mut predicate: impl FnMut() -> bool) -> bool {
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if predicate() {
            return true;
        }
        thread::sleep(interval);
    }
    predicate()
}
