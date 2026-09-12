use crate::config::InitArgs;
use include_dir::{Dir, include_dir};
use std::fs;
use std::path::{Path, PathBuf};

use std::process::Command;

pub static TEMPLATE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/example");

pub fn initialize_project(target_dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    fs::create_dir_all(target_dir)?;
    let mut created_files = Vec::new();

    fn extract_dir(dir: &Dir, base_path: &Path, created: &mut Vec<PathBuf>) -> std::io::Result<()> {
        for entry in dir.entries() {
            let path = base_path.join(entry.path());

            match entry {
                include_dir::DirEntry::Dir(d) => {
                    fs::create_dir_all(&path)?;
                    extract_dir(d, base_path, created)?;
                }
                include_dir::DirEntry::File(f) => {
                    if let Some(parent) = path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::write(&path, f.contents())?;
                    created.push(entry.path().to_path_buf());
                }
            }
        }
        Ok(())
    }

    extract_dir(&TEMPLATE_DIR, target_dir, &mut created_files)?;
    created_files.sort();
    created_files.dedup();
    Ok(created_files)
}

fn is_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn init_git_repo(target_dir: &Path) -> std::io::Result<()> {
    if !is_git_installed() {
        cliclack::log::warning("Git is not installed, skipping git repository initialization")?;
        return Ok(());
    }

    let init_status = Command::new("git")
        .args(["init", "-b", "main"])
        .current_dir(target_dir)
        .stdout(std::process::Stdio::null())
        .status()?;

    if !init_status.success() {
        return Err(std::io::Error::other("Failed to initialize git repository"));
    }

    let add_status = Command::new("git")
        .args(["add", "-A"])
        .current_dir(target_dir)
        .stdout(std::process::Stdio::null())
        .status()?;

    if !add_status.success() {
        return Err(std::io::Error::other(
            "Failed to add files to git repository",
        ));
    }

    let commit_status = Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(target_dir)
        .stdout(std::process::Stdio::null())
        .status()?;

    if !commit_status.success() {
        return Err(std::io::Error::other(
            "Failed to make initial commit in git repository",
        ));
    }

    Ok(())
}

pub fn handle_init(args: &InitArgs) -> std::io::Result<()> {
    let target_dir = &args.path;

    cliclack::intro("surge init")?;

    let files = match initialize_project(target_dir) {
        Ok(files) => files,
        Err(err) => {
            cliclack::outro_cancel(format!("Failed to initialize project: {err}"))?;
            return Err(err);
        }
    };

    for file in &files {
        cliclack::log::step(format!("Created {}", file.display()))?;
    }

    if let Err(err) = init_git_repo(target_dir) {
        cliclack::outro_cancel(format!("Failed to initialize git repository: {err}"))?;
        return Err(err);
    }

    let display_path = if target_dir == Path::new(".") {
        "current directory".to_string()
    } else {
        format!("{}", target_dir.display())
    };

    cliclack::outro(format!(
        "Successfully initialized project in {display_path}"
    ))?;

    Ok(())
}
