use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, process::Command};

fn collect_files(dir: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        } else {
            collect_files(&path, files);
        }
    }
}

fn clean_assets_folder(assets_folder: &PathBuf) {
    let mut files = vec![];
    collect_files(assets_folder, &mut files);

    for file in files {
        let file_name = file.file_name().unwrap().to_str().unwrap();
        if file_name == ".keep" {
            continue;
        }

        fs::remove_file(file).unwrap();
    }

    for entry in fs::read_dir(assets_folder).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            fs::remove_dir_all(path).unwrap();
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // iterate through frontend/src files to generate a list of files to watch
    let mut watch_files = vec![];
    collect_files(Path::new("frontend/src"), &mut watch_files);

    // Config file watch rerun
    println!("cargo:rerun-if-changed=frontend/vite.config.ts");
    println!("cargo:rerun-if-changed=frontend/package.json");
    println!("cargo:rerun-if-changed=frontend/package-lock.json");
    println!("cargo:rerun-if-changed=frontend/tailwind.config.js");
    // Watch frontend/src files
    for file in watch_files {
        println!(
            "cargo:rerun-if-changed={}",
            file.display().to_string().replace("\\", "/").as_str()
        );
    }

    let dist_dir = Path::new("frontend/dist");
    let assets_dir = Path::new(&manifest_dir).join("assets");

    // remove old assets except .keep
    clean_assets_folder(&assets_dir);

    // Build frontend
    let status = Command::new("npm")
        .args(&["run", "build:frontend"])
        .status()
        .unwrap();

    if !status.success() {
        panic!("Failed to build frontend");
    }

    // Copy frontend/dist to root assets/
    // Do not include index.html since we will be serving it directly from the backend
    let mut generated_files = vec![];
    collect_files(dist_dir, &mut generated_files);

    for file in generated_files {
        let file_path = file.as_path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        if file_name != "index.html" {
            let dest = assets_dir.join(file_path.strip_prefix("frontend/dist/").unwrap());
            // make folder if it doesn't exist
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::copy(file, dest).unwrap();
        }
    }

    // Read index.html and create a rust file in out_dir called index_html.rs
    let index_html = fs::read_to_string(dist_dir.join("index.html")).unwrap();

    let mut writer = File::create(Path::new(&out_dir).join("index_html.rs")).unwrap();
    writer
        .write_all(format!("pub const INDEX_HTML: &str = r#\"{}\"#;", index_html).as_bytes())
        .unwrap();
}
