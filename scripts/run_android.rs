#!/usr/bin/env rust-script
use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::time::Duration;
use std::time::Instant;

const DEFAULT_AVD_NAME: &str = "Medium_Phone_API_36.1";
const DEFAULT_MANIFEST_PATH: &str = "apps/android/Cargo.toml";
const DEFAULT_PACKAGE_NAME: &str = "rchoreo_android";
const DEFAULT_TARGET_DIR: &str = "target/android";
const EMULATOR_BOOT_TIMEOUT: Duration = Duration::from_secs(300);
const DEVICE_DISCOVERY_TIMEOUT: Duration = Duration::from_secs(120);

fn main() -> Result<(), Box<dyn Error>>
{
    let config = Config::parse(env::args().skip(1).collect())?;
    let repo_root = find_repo_root(env::current_dir()?)?;
    let sdk = AndroidSdk::discover()?;
    validate_android_toolchain(&sdk)?;

    let device_serial = ensure_device(&config, &sdk)?;
    let build_target = match &config.build_target
    {
        Some(target) => target.clone(),
        None => detect_target(&config, &sdk, &device_serial)?,
    };

    ensure_rust_target_installed(&build_target)?;

    let manifest_path = repo_root.join(&config.manifest_path);
    let target_dir = repo_root.join(DEFAULT_TARGET_DIR);

    println!("Using device: {device_serial}");
    println!("Using target: {build_target}");
    println!("Using manifest: {}", manifest_path.display());

    let mut cargo_command = Command::new("cargo");
    cargo_command.current_dir(&repo_root);
    cargo_command.arg("apk");
    cargo_command.arg("run");
    cargo_command.arg("--manifest-path");
    cargo_command.arg(&manifest_path);
    cargo_command.arg("--package");
    cargo_command.arg(DEFAULT_PACKAGE_NAME);
    cargo_command.arg("--lib");
    cargo_command.arg("--target");
    cargo_command.arg(&build_target);
    cargo_command.arg("--target-dir");
    cargo_command.arg(&target_dir);
    cargo_command.arg("--device");
    cargo_command.arg(&device_serial);

    if config.release
    {
        cargo_command.arg("--release");
    }

    if !config.logcat
    {
        cargo_command.arg("--no-logcat");
    }

    apply_android_environment(&mut cargo_command, &sdk);
    run_command(&mut cargo_command)?;

    Ok(())
}

struct Config
{
    avd_name: Option<String>,
    device_serial: Option<String>,
    build_target: Option<String>,
    release: bool,
    logcat: bool,
    skip_emulator: bool,
    manifest_path: PathBuf,
}

impl Config
{
    fn parse(arguments: Vec<String>) -> Result<Self, Box<dyn Error>>
    {
        let arguments = if matches!(arguments.first().map(String::as_str), Some("--"))
        {
            arguments.into_iter().skip(1).collect()
        }
        else
        {
            arguments
        };

        let mut avd_name = None;
        let mut device_serial = None;
        let mut build_target = None;
        let mut release = false;
        let mut logcat = false;
        let mut skip_emulator = false;
        let mut manifest_path = PathBuf::from(DEFAULT_MANIFEST_PATH);

        let mut index = 0;
        while index < arguments.len()
        {
            match arguments[index].as_str()
            {
                "--avd" =>
                {
                    index += 1;
                    avd_name = Some(required_value(&arguments, index, "--avd")?);
                }
                "--device" =>
                {
                    index += 1;
                    device_serial = Some(required_value(&arguments, index, "--device")?);
                }
                "--target" =>
                {
                    index += 1;
                    build_target = Some(required_value(&arguments, index, "--target")?);
                }
                "--manifest-path" =>
                {
                    index += 1;
                    manifest_path = PathBuf::from(required_value(
                        &arguments,
                        index,
                        "--manifest-path",
                    )?);
                }
                "--release" =>
                {
                    release = true;
                }
                "--logcat" =>
                {
                    logcat = true;
                }
                "--skip-emulator" =>
                {
                    skip_emulator = true;
                }
                "--help" | "-h" =>
                {
                    print_usage();
                    std::process::exit(0);
                }
                other =>
                {
                    return Err(format!("Unknown argument: {other}").into());
                }
            }

            index += 1;
        }

        Ok(Self {
            avd_name,
            device_serial,
            build_target,
            release,
            logcat,
            skip_emulator,
            manifest_path,
        })
    }
}

struct AndroidSdk
{
    sdk_root: PathBuf,
    ndk_root: PathBuf,
    adb_path: PathBuf,
    emulator_path: PathBuf,
}

impl AndroidSdk
{
    fn discover() -> Result<Self, Box<dyn Error>>
    {
        let sdk_root = env::var_os("ANDROID_HOME")
            .or_else(|| env::var_os("ANDROID_SDK_ROOT"))
            .map(PathBuf::from)
            .or_else(default_sdk_root)
            .ok_or("Unable to resolve Android SDK root. Set ANDROID_HOME.")?;

        let ndk_root = env::var_os("ANDROID_NDK_ROOT")
            .or_else(|| env::var_os("ANDROID_NDK_HOME"))
            .map(PathBuf::from)
            .or_else(|| discover_latest_ndk(&sdk_root))
            .ok_or("Unable to resolve Android NDK root. Set ANDROID_NDK_ROOT.")?;

        let adb_path = resolve_tool_path("adb", sdk_root.join("platform-tools").join(exe("adb")))?;
        let emulator_path = resolve_tool_path(
            "emulator",
            sdk_root.join("emulator").join(exe("emulator")),
        )?;

        Ok(Self {
            sdk_root,
            ndk_root,
            adb_path,
            emulator_path,
        })
    }
}

fn required_value(
    arguments: &[String],
    index: usize,
    flag: &str,
) -> Result<String, Box<dyn Error>>
{
    arguments
        .get(index)
        .cloned()
        .ok_or_else(|| format!("Missing value for {flag}").into())
}

fn print_usage()
{
    println!("Usage: rust-script scripts/run_android.rs -- [options]");
    println!("Options:");
    println!("  --avd <name>            Emulator AVD name to start if no device is running");
    println!("  --device <serial>       Existing adb device serial to use");
    println!("  --target <triple>       Rust Android target override");
    println!("  --release               Build a release APK");
    println!("  --logcat                Keep cargo-apk logcat attached");
    println!("  --skip-emulator         Do not start an emulator automatically");
    println!("  --manifest-path <path>  Override the Android Cargo manifest path");
}

fn find_repo_root(start_dir: PathBuf) -> Result<PathBuf, Box<dyn Error>>
{
    let mut current = start_dir.as_path();
    loop
    {
        if current.join("apps").join("android").join("Cargo.toml").exists()
        {
            return Ok(current.to_path_buf());
        }

        current = current
            .parent()
            .ok_or("Unable to locate workspace root from current directory.")?;
    }
}

fn default_sdk_root() -> Option<PathBuf>
{
    env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .map(|path| path.join("Android").join("Sdk"))
        .filter(|path| path.exists())
}

fn discover_latest_ndk(sdk_root: &Path) -> Option<PathBuf>
{
    let ndk_root = sdk_root.join("ndk");
    let mut ndk_versions = fs::read_dir(ndk_root).ok()?;
    let mut entries = Vec::new();

    while let Some(Ok(entry)) = ndk_versions.next()
    {
        let path = entry.path();
        if path.is_dir()
        {
            entries.push(path);
        }
    }

    entries.sort();
    entries.pop()
}

fn resolve_tool_path(tool_name: &str, fallback: PathBuf) -> Result<PathBuf, Box<dyn Error>>
{
    if let Some(path) = find_on_path(tool_name)
    {
        return Ok(path);
    }

    if fallback.exists()
    {
        return Ok(fallback);
    }

    Err(format!("Unable to resolve `{tool_name}`.").into())
}

fn find_on_path(tool_name: &str) -> Option<PathBuf>
{
    let path_variable = env::var_os("PATH")?;

    for entry in env::split_paths(&path_variable)
    {
        let candidate = entry.join(exe(tool_name));
        if candidate.exists()
        {
            return Some(candidate);
        }
    }

    None
}

fn exe(tool_name: &str) -> String
{
    if cfg!(windows)
    {
        format!("{tool_name}.exe")
    }
    else
    {
        tool_name.to_string()
    }
}

fn ensure_device(config: &Config, sdk: &AndroidSdk) -> Result<String, Box<dyn Error>>
{
    if let Some(device_serial) = &config.device_serial
    {
        return Ok(device_serial.clone());
    }

    let existing_devices = adb_devices(&sdk.adb_path)?;
    if let Some(device_serial) = existing_devices.iter().find(|serial| serial.starts_with("emulator-"))
    {
        return Ok(device_serial.clone());
    }

    if let Some(device_serial) = existing_devices.first()
    {
        return Ok(device_serial.clone());
    }

    if config.skip_emulator
    {
        return Err("No adb device is available and --skip-emulator was used.".into());
    }

    let avd_name = config
        .avd_name
        .clone()
        .unwrap_or_else(|| DEFAULT_AVD_NAME.to_string());

    start_emulator(&sdk.emulator_path, &avd_name)?;
    wait_for_new_emulator(&sdk.adb_path, &existing_devices)
}

fn adb_devices(adb_path: &Path) -> Result<Vec<String>, Box<dyn Error>>
{
    let output = Command::new(adb_path).arg("devices").output()?;
    if !output.status.success()
    {
        return Err("`adb devices` failed.".into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    let mut devices = Vec::new();

    for line in stdout.lines().skip(1)
    {
        let mut segments = line.split_whitespace();
        let Some(serial) = segments.next() else
        {
            continue;
        };
        let Some(state) = segments.next() else
        {
            continue;
        };

        if state == "device"
        {
            devices.push(serial.to_string());
        }
    }

    Ok(devices)
}

fn start_emulator(emulator_path: &Path, avd_name: &str) -> Result<(), Box<dyn Error>>
{
    println!("Starting emulator `{avd_name}`...");

    Command::new(emulator_path)
        .arg("-avd")
        .arg(avd_name)
        .arg("-no-snapshot-save")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    Ok(())
}

fn wait_for_new_emulator(
    adb_path: &Path,
    existing_devices: &[String],
) -> Result<String, Box<dyn Error>>
{
    let existing: BTreeSet<_> = existing_devices.iter().cloned().collect();
    let started_at = Instant::now();

    loop
    {
        if started_at.elapsed() > DEVICE_DISCOVERY_TIMEOUT
        {
            return Err("Timed out waiting for the emulator to appear in adb.".into());
        }

        let current_devices = adb_devices(adb_path)?;
        if let Some(serial) = current_devices
            .into_iter()
            .find(|serial| serial.starts_with("emulator-") && !existing.contains(serial))
        {
            wait_for_boot_completed(adb_path, &serial)?;
            return Ok(serial);
        }

        thread::sleep(Duration::from_secs(2));
    }
}

fn wait_for_boot_completed(adb_path: &Path, device_serial: &str) -> Result<(), Box<dyn Error>>
{
    println!("Waiting for `{device_serial}` to finish booting...");

    let status = Command::new(adb_path).arg("-s").arg(device_serial).arg("wait-for-device").status()?;
    if !status.success()
    {
        return Err("`adb wait-for-device` failed.".into());
    }

    let started_at = Instant::now();
    loop
    {
        if started_at.elapsed() > EMULATOR_BOOT_TIMEOUT
        {
            return Err("Timed out waiting for the emulator to finish booting.".into());
        }

        let output = Command::new(adb_path)
            .arg("-s")
            .arg(device_serial)
            .arg("shell")
            .arg("getprop")
            .arg("sys.boot_completed")
            .output()?;

        if output.status.success() && String::from_utf8(output.stdout)?.trim() == "1"
        {
            return Ok(());
        }

        thread::sleep(Duration::from_secs(2));
    }
}

fn detect_target(
    config: &Config,
    sdk: &AndroidSdk,
    device_serial: &str,
) -> Result<String, Box<dyn Error>>
{
    if let Some(abi) = read_device_abi(&sdk.adb_path, device_serial)?
    {
        return abi_to_rust_target(&abi)
            .map(str::to_owned)
            .ok_or_else(|| format!("Unsupported device ABI: {abi}").into());
    }

    let avd_name = config
        .avd_name
        .clone()
        .unwrap_or_else(|| DEFAULT_AVD_NAME.to_string());

    let abi = read_avd_abi(&avd_name)?;
    abi_to_rust_target(&abi)
        .map(str::to_owned)
        .ok_or_else(|| format!("Unsupported AVD ABI: {abi}").into())
}

fn read_device_abi(
    adb_path: &Path,
    device_serial: &str,
) -> Result<Option<String>, Box<dyn Error>>
{
    let output = Command::new(adb_path)
        .arg("-s")
        .arg(device_serial)
        .arg("shell")
        .arg("getprop")
        .arg("ro.product.cpu.abi")
        .output()?;

    if !output.status.success()
    {
        return Ok(None);
    }

    let abi = String::from_utf8(output.stdout)?.trim().to_string();
    if abi.is_empty()
    {
        Ok(None)
    }
    else
    {
        Ok(Some(abi))
    }
}

fn read_avd_abi(avd_name: &str) -> Result<String, Box<dyn Error>>
{
    let avd_root = env::var_os("USERPROFILE")
        .map(PathBuf::from)
        .ok_or("Unable to resolve USERPROFILE.")?
        .join(".android")
        .join("avd");

    let avd_ini_path = avd_root.join(format!("{avd_name}.ini"));
    let avd_ini = fs::read_to_string(&avd_ini_path)?;
    let avd_directory = parse_avd_path(&avd_ini)
        .ok_or_else(|| format!("Unable to resolve AVD directory from {}", avd_ini_path.display()))?;

    let config = fs::read_to_string(avd_directory.join("config.ini"))?;
    parse_key_value(&config, "abi.type")
        .ok_or_else(|| format!("Unable to resolve `abi.type` for AVD `{avd_name}`").into())
}

fn parse_avd_path(contents: &str) -> Option<PathBuf>
{
    parse_key_value(contents, "path").map(PathBuf::from)
}

fn parse_key_value(contents: &str, key: &str) -> Option<String>
{
    contents
        .lines()
        .find_map(|line| line.strip_prefix(&format!("{key}=")))
        .map(|value| value.trim().to_string())
}

fn abi_to_rust_target(abi: &str) -> Option<&'static str>
{
    match abi
    {
        "arm64-v8a" => Some("aarch64-linux-android"),
        "x86_64" => Some("x86_64-linux-android"),
        "x86" => Some("i686-linux-android"),
        "armeabi-v7a" => Some("armv7-linux-androideabi"),
        _ => None,
    }
}

fn ensure_rust_target_installed(target: &str) -> Result<(), Box<dyn Error>>
{
    let output = Command::new("rustup")
        .arg("target")
        .arg("list")
        .arg("--installed")
        .output()?;

    if !output.status.success()
    {
        return Err("Unable to query installed Rust targets.".into());
    }

    let installed = String::from_utf8(output.stdout)?;
    if installed.lines().any(|line| line.trim() == target)
    {
        return Ok(());
    }

    println!("Installing Rust target `{target}`...");
    let mut command = Command::new("rustup");
    command.arg("target");
    command.arg("add");
    command.arg(target);
    run_command(&mut command)
}

fn apply_android_environment(command: &mut Command, sdk: &AndroidSdk)
{
    command.env("ANDROID_HOME", &sdk.sdk_root);
    command.env_remove("ANDROID_SDK_ROOT");
    command.env("ANDROID_NDK_ROOT", &sdk.ndk_root);
    command.env("ANDROID_NDK_HOME", &sdk.ndk_root);
}

fn validate_android_toolchain(sdk: &AndroidSdk) -> Result<(), Box<dyn Error>>
{
    let ndk_max_api = read_ndk_max_api(&sdk.ndk_root)?;
    let platform_versions = read_sdk_platform_versions(&sdk.sdk_root)?;

    if platform_versions.iter().any(|version| *version <= ndk_max_api)
    {
        return Ok(());
    }

    let versions = if platform_versions.is_empty()
    {
        "none".to_string()
    }
    else
    {
        platform_versions
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    };

    Err(format!(
        "No Android SDK platform is compatible with the installed NDK. \
Installed SDK platforms: [{versions}]. \
Installed NDK maximum API level: {ndk_max_api}. \
Install Android SDK Platform {ndk_max_api} or newer NDK support for the installed SDK platform."
    )
    .into())
}

fn read_ndk_max_api(ndk_root: &Path) -> Result<u32, Box<dyn Error>>
{
    let platforms_json = fs::read_to_string(ndk_root.join("meta").join("platforms.json"))?;
    let max_line = platforms_json
        .lines()
        .find(|line| line.trim_start().starts_with("\"max\""))
        .ok_or("Unable to resolve the NDK max API level.")?;

    let max_value = max_line
        .split(':')
        .nth(1)
        .ok_or("Unable to parse the NDK max API level.")?
        .trim()
        .trim_end_matches(',');

    Ok(max_value.parse()?)
}

fn read_sdk_platform_versions(sdk_root: &Path) -> Result<Vec<u32>, Box<dyn Error>>
{
    let platforms_root = sdk_root.join("platforms");
    if !platforms_root.exists()
    {
        return Ok(Vec::new());
    }

    let mut versions = Vec::new();
    for entry in fs::read_dir(platforms_root)?
    {
        let entry = entry?;
        let Some(name) = entry.file_name().to_str().map(str::to_owned) else
        {
            continue;
        };

        let Some(version) = name.strip_prefix("android-") else
        {
            continue;
        };

        if let Ok(parsed) = version.parse::<u32>()
        {
            versions.push(parsed);
        }
    }

    versions.sort_unstable();
    Ok(versions)
}

fn run_command(command: &mut Command) -> Result<(), Box<dyn Error>>
{
    let printable = format_command(command);
    println!("> {printable}");

    let status = command.status()?;
    if status.success()
    {
        Ok(())
    }
    else
    {
        Err(format!("Command failed: {printable}").into())
    }
}

fn format_command(command: &Command) -> String
{
    let program = command.get_program().to_string_lossy().to_string();
    let arguments = command
        .get_args()
        .map(shell_escape)
        .collect::<Vec<_>>()
        .join(" ");

    if arguments.is_empty()
    {
        program
    }
    else
    {
        format!("{program} {arguments}")
    }
}

fn shell_escape(value: &OsStr) -> String
{
    let value = value.to_string_lossy();
    if value.contains(' ')
    {
        format!("\"{value}\"")
    }
    else
    {
        value.to_string()
    }
}
