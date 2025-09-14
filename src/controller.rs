use std::fs::File;
use std::fs::create_dir_all;
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::commit_data::{Commit, Utc};

pub fn init() -> anyhow::Result<()> {
    let brdv_dir = PathBuf::from(".brdv");

    // check if it is already exist or not.
    if brdv_dir.exists() {
        anyhow::bail!("Repo already initialized");
    }

    create_dir_all(brdv_dir.join("objects"))?;
    create_dir_all(brdv_dir.join("ref/heads"))?;
    create_dir_all(brdv_dir.join("ref/tags"))?;
    create_dir_all(brdv_dir.join("info"))?;
    create_dir_all(brdv_dir.join("hooks"))?;

    let mut head = File::create(brdv_dir.join("HEAD"))?;
    head.write_all(b"ref: refs/heads/master")?;

    let mut exclude = File::create(brdv_dir.join("info/exclude"))?;
    // exclude.write_all(b"brdv")?;

    let mut config = File::create(brdv_dir.join("config"))?;
    // config.write_all(b"brdv")?;

    let mut description = File::create(brdv_dir.join("description"))?;
    // description.write_all(b"brdv")?;

    println!("Initializes empty brdv repository");
    Ok(())
}

pub fn commit(message: &str) -> anyhow::Result<()> {
    let brdv_dir = PathBuf::from(".brdv");

    if !brdv_dir.exists() {
        anyhow::bail!("brdv repository is not initialized");
    }

    let mut tree_data = Vec::new();

    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.starts_with(&brdv_dir) || path.is_dir() {
            continue;
        }
        let content = std::fs::read(path)?;
        let hash = hash_bytes(&content);
        tree_data.push((path.to_string_lossy().to_string(), hash));
    }

    let tree_bytes = bincode::serialize(&tree_data)?;
    let tree_hash = hash_bytes(&tree_bytes);

    let head_path = brdv_dir.join("HEAD");
    let parent = if head_path.exists() {
        Some(std::fs::read_to_string(&head_path)?)
    } else {
        None
    };

    let commit_data = Commit {
        tree: tree_hash,
        parent,
        author: "Gaurang".to_string(),
        message: message.to_string(),
        timestamp: Utc::now(),
    };
    let commit_data = bincode::serialize(&commit_data)?;
    let commit_hash = hash_bytes(&commit_data);

    let obj_path = brdv_dir.join(format!("objects/{}", commit_hash));
    std::fs::write(obj_path, commit_data)?;

    std::fs::write(head_path, commit_hash.as_bytes())?;
    // let commit_data;
    println!("All the code pushed to the repository, {}", commit_hash);
    Ok(())
}

pub fn log() -> anyhow::Result<()> {
    println!("Printing logs");
    Ok(())
}

fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(data);
    hasher.finalize().to_hex().to_string()
}
