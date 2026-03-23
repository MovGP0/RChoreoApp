#!/usr/bin/env rust-script
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

const DEFAULT_PORT: u16 = 8000;

fn main() -> Result<(), Box<dyn Error>>
{
    let repo_root = find_repo_root(env::current_dir()?)?;
    let wasm_dir = repo_root.join("apps").join("wasm");

    ensure_wasm_build(&repo_root)?;

    let url = format!("http://127.0.0.1:{DEFAULT_PORT}/index.html");
    open_browser(&url)?;

    let listener = TcpListener::bind(("127.0.0.1", DEFAULT_PORT))?;
    println!("Serving {} on {url}", wasm_dir.display());

    for stream in listener.incoming()
    {
        match stream
        {
            Ok(stream) =>
            {
                if let Err(error) = handle_request(stream, &wasm_dir)
                {
                    eprintln!("Request failed: {error}");
                }
            }
            Err(error) =>
            {
                eprintln!("Connection failed: {error}");
            }
        }
    }

    Ok(())
}

fn find_repo_root(start_dir: PathBuf) -> Result<PathBuf, Box<dyn Error>>
{
    let mut current = start_dir.as_path();
    loop
    {
        if current.join("apps").join("wasm").join("Cargo.toml").exists()
        {
            return Ok(current.to_path_buf());
        }

        current = current
            .parent()
            .ok_or("Unable to locate workspace root from current directory.")?;
    }
}

fn ensure_wasm_build(repo_root: &Path) -> Result<(), Box<dyn Error>>
{
    let mut command = Command::new("wasm-pack");
    command.current_dir(repo_root);
    command.arg("build");
    command.arg("--release");
    command.arg("--target");
    command.arg("web");
    command.arg("apps/wasm");

    run_command(&mut command)
}

fn open_browser(url: &str) -> Result<(), Box<dyn Error>>
{
    let mut command = if cfg!(target_os = "windows")
    {
        let mut command = Command::new("cmd");
        command.arg("/C");
        command.arg("start");
        command.arg("");
        command.arg(url);
        command
    }
    else if cfg!(target_os = "macos")
    {
        let mut command = Command::new("open");
        command.arg(url);
        command
    }
    else
    {
        let mut command = Command::new("xdg-open");
        command.arg(url);
        command
    };

    command.spawn()?;
    Ok(())
}

fn handle_request(
    mut stream: TcpStream,
    wasm_dir: &Path,
) -> Result<(), Box<dyn Error>>
{
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    let Some(path) = request_path(&request_line) else
    {
        write_response(
            &mut stream,
            "400 Bad Request",
            "text/plain; charset=utf-8",
            b"Bad Request",
        )?;
        return Ok(());
    };

    let file_path = resolve_requested_file(wasm_dir, path)?;
    match file_path
    {
        Some(file_path) =>
        {
            let bytes = fs::read(&file_path)?;
            let content_type = content_type_for(&file_path);
            write_response(&mut stream, "200 OK", content_type, &bytes)?;
        }
        None =>
        {
            write_response(
                &mut stream,
                "404 Not Found",
                "text/plain; charset=utf-8",
                b"Not Found",
            )?;
        }
    }

    Ok(())
}

fn request_path(request_line: &str) -> Option<&str>
{
    let mut segments = request_line.split_whitespace();
    let method = segments.next()?;
    let path = segments.next()?;

    if method != "GET"
    {
        return None;
    }

    Some(path)
}

fn resolve_requested_file(
    wasm_dir: &Path,
    request_path: &str,
) -> Result<Option<PathBuf>, Box<dyn Error>>
{
    let relative_path = request_path
        .split('?')
        .next()
        .unwrap_or("/")
        .trim_start_matches('/');

    let relative_path = if relative_path.is_empty()
    {
        PathBuf::from("index.html")
    }
    else
    {
        PathBuf::from(relative_path)
    };

    let wasm_dir = wasm_dir.canonicalize()?;
    let file_path = wasm_dir.join(relative_path);
    if !file_path.exists()
    {
        return Ok(None);
    }

    let canonical_file_path = file_path.canonicalize()?;
    if !canonical_file_path.starts_with(&wasm_dir)
    {
        return Ok(None);
    }

    if canonical_file_path.is_file()
    {
        Ok(Some(canonical_file_path))
    }
    else
    {
        Ok(None)
    }
}

fn content_type_for(path: &Path) -> &'static str
{
    match path.extension().and_then(OsStr::to_str)
    {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "application/javascript",
        Some("wasm") => "application/wasm",
        Some("css") => "text/css; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("json") => "application/json",
        Some("map") => "application/json",
        _ => "application/octet-stream",
    }
}

fn write_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
) -> Result<(), Box<dyn Error>>
{
    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    stream.write_all(body)?;
    stream.flush()?;
    Ok(())
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
