use std::{fs::File, io::Read, path::Path};

use crate::league::LeagueLoaderError;

use bevy::scene::ron;
use serde::{de::DeserializeOwned, Serialize};
use tokio::{fs::File as AsyncFile, io::AsyncWriteExt};

fn ensure_dir_exists(path: &str) -> Result<(), LeagueLoaderError> {
    let dir = Path::new(path).parent().unwrap();
    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}

pub async fn save_struct_to_file<T: Serialize>(
    path: &str,
    data: &T,
) -> Result<(), LeagueLoaderError> {
    // let serialized = bincode::serialize(data)?;
    // let mut file = get_asset_writer(path).await?;
    // file.write_all(&serialized).await?;
    // file.flush().await?;
    // Ok(())
    let mut file = get_asset_writer(path).await?;
    let data_str = ron::ser::to_string_pretty(data, ron::ser::PrettyConfig::default()).unwrap();
    file.write_all(data_str.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}

pub async fn get_asset_writer(path: &str) -> Result<AsyncFile, LeagueLoaderError> {
    let path = format!("assets/{}", path);
    // println!("√ {}", path);
    ensure_dir_exists(&path)?;
    let file = AsyncFile::create(path).await?;
    Ok(file)
}

pub fn get_bin_path(path: &str) -> String {
    format!("{}.bin", path)
}

pub fn get_struct_from_file<T: DeserializeOwned>(path: &str) -> Result<T, LeagueLoaderError> {
    // let mut file = File::open(format!("assets/{}", &get_bin_path(path)))?;
    // let mut data = Vec::new();
    // file.read_to_end(&mut data)?;
    // let data = bincode::deserialize(&data)?;
    // Ok(data)

    let mut file = File::open(format!("assets/{}", &get_bin_path(path)))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    let data = ron::de::from_bytes(&data).unwrap();
    Ok(data)
}
