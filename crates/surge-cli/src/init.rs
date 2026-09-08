use crate::config::InitArgs;
use include_dir::{Dir, include_dir};
use std::fs;
use std::path::{Path, PathBuf};

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
