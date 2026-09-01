use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn copy_directory(source: &Path, destination: &Path) -> std::io::Result<()> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if source_path.is_dir() {
            copy_directory(&source_path, &destination_path)?;
        } else {
            fs::copy(source_path, destination_path)?;
        }
    }

    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=materials");
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=webflow-runtime.rc");

    if env::var("TARGET")
        .map(|target| target.contains("-windows-"))
        .unwrap_or(false)
    {
        embed_resource::compile("webflow-runtime.rc", embed_resource::NONE)
            .manifest_optional()
            .unwrap();
    }

    let source = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("materials");
    if !source.exists() {
        return;
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let Some(profile_dir) = out_dir.ancestors().nth(3) else {
        return;
    };

    let destination = profile_dir.join("materials");
    if let Err(error) = copy_directory(&source, &destination) {
        println!("cargo:warning=Failed to copy materials: {error}");
    }

    let templates = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("templates");
    if templates.exists() {
        if let Err(error) = copy_directory(&templates, &profile_dir.join("templates")) {
            println!("cargo:warning=Failed to copy templates: {error}");
        }
    }
}
