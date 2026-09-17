use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub struct Document {
    pub source_path: String,
    pub content: String,
    pub mode: DocumentMode,
}

pub enum DocumentMode {
    Markdown,
    Converted(String),
}

pub fn load(file_path: &str) -> Result<Document, Box<dyn Error>> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(format!("File '{}' does not exist.", file_path).into());
    }

    if !path.is_file() {
        return Err(format!("'{}' is not a regular file.", file_path).into());
    }

    let extension = path
        .extension()
        .and_then(|x| x.to_str())
        .unwrap_or("")
        .to_lowercase();

    if extension == "md" || extension == "markdown" {
        let content = fs::read_to_string(path)?;

        return Ok(Document {
            source_path: file_path.to_string(),
            content,
            mode: DocumentMode::Markdown,
        });
    }

    println!("markd: '{}' detected. Preparing MarkItDown...", extension);

    let executable = ensure_markitdown()?;

    println!("markd: converting {}...", file_path);

    let output = Command::new(&executable).arg(file_path).output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        return Err(format!("MarkItDown failed:\n{}", stderr.trim()).into());
    }

    let content = String::from_utf8(output.stdout)?;

    if content.trim().is_empty() {
        return Err("MarkItDown returned empty content.".into());
    }

    Ok(Document {
        source_path: file_path.to_string(),
        content,
        mode: DocumentMode::Converted(extension),
    })
}

pub fn print_document(document: &Document) {
    print!("{}", document.content);

    let _ = std::io::stdout().flush();
}

fn find_python() -> Option<String> {
    for candidate in ["python3", "python"] {
        let result = Command::new(candidate)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        if matches!(result, Ok(status) if status.success()) {
            return Some(candidate.to_string());
        }
    }

    None
}

fn markd_cache_dir() -> Result<PathBuf, Box<dyn Error>> {
    let home = env::var_os("HOME").ok_or("HOME environment variable not found.")?;

    Ok(PathBuf::from(home).join(".cache").join("markd"))
}

fn venv_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(markd_cache_dir()?.join("markitdown-venv"))
}

fn markitdown_executable(venv: &Path) -> PathBuf {
    if cfg!(windows) {
        venv.join("Scripts").join("markitdown.exe")
    } else {
        venv.join("bin").join("markitdown")
    }
}

fn venv_python(venv: &Path) -> PathBuf {
    if cfg!(windows) {
        venv.join("Scripts").join("python.exe")
    } else {
        venv.join("bin").join("python")
    }
}

fn existing_markitdown() -> Option<PathBuf> {
    let global = Command::new("markitdown")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if matches!(global, Ok(status) if status.success()) {
        return Some(PathBuf::from("markitdown"));
    }

    let venv = venv_path().ok()?;
    let executable = markitdown_executable(&venv);

    if executable.is_file() {
        let result = Command::new(&executable)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        if matches!(result, Ok(status) if status.success()) {
            return Some(executable);
        }
    }

    None
}

fn ensure_markitdown() -> Result<PathBuf, Box<dyn Error>> {
    if let Some(executable) = existing_markitdown() {
        return Ok(executable);
    }

    println!();
    println!("markd: MarkItDown is not installed.");
    println!("markd: creating a private environment...");
    println!();

    let python = find_python().ok_or(
        "Python 3.10+ is required for automatic \
         MarkItDown installation.",
    )?;

    let cache_dir = markd_cache_dir()?;

    fs::create_dir_all(&cache_dir)?;

    let venv = venv_path()?;

    if !venv.exists() {
        println!("markd: creating virtual environment...");

        let status = Command::new(&python)
            .args(["-m", "venv"])
            .arg(&venv)
            .status()?;

        if !status.success() {
            return Err("Unable to create Python virtual environment.\n\
                 On Debian/Ubuntu install python3-venv:\n\
                 sudo apt install python3-venv"
                .into());
        }
    }

    let python_bin = venv_python(&venv);

    println!("markd: upgrading pip...");

    let status = Command::new(&python_bin)
        .args(["-m", "pip", "install", "--upgrade", "pip"])
        .status()?;

    if !status.success() {
        return Err("Failed to upgrade pip.".into());
    }

    println!("markd: installing MarkItDown...");

    let status = Command::new(&python_bin)
        .args(["-m", "pip", "install", "markitdown[all]"])
        .status()?;

    if !status.success() {
        return Err("Failed to install MarkItDown.".into());
    }

    let executable = markitdown_executable(&venv);

    if !executable.is_file() {
        return Err("MarkItDown installed, but the executable \
             could not be found."
            .into());
    }

    println!("markd: MarkItDown ready.");

    Ok(executable)
}
