use std::hash::Hasher;

use bevy::transform::components::Transform;
use twox_hash::XxHash64;

use crate::{
    core::{ConfigGame, Team, CONFIG_PATH_GAME},
    league::{
        get_bin_path, save_struct_to_file, LeagueLoaderError, LeagueWadLoader, LeagueWadMapLoader,
    },
};

pub struct LeagueLoader {
    pub root_dir: String,
}

impl LeagueLoader {
    pub fn new(root_dir: &str) -> Result<LeagueLoader, LeagueLoaderError> {
        Ok(LeagueLoader {
            root_dir: root_dir.to_string(),
        })
    }

    pub fn get_map_loader(&self, map: &str) -> Result<LeagueWadMapLoader, LeagueLoaderError> {
        let loader = LeagueWadLoader::from_relative_path(
            &self.root_dir,
            "DATA/FINAL/Maps/Shipping/Map11.wad.client",
        )?;

        let map_loader = LeagueWadMapLoader::from_loader(loader, map)?;

        Ok(map_loader)
    }

    pub async fn save_legends(&self, champion: &str, skin: &str) -> Result<(), LeagueLoaderError> {
        let wad_relative_path = format!("DATA/FINAL/Champions/{}.wad.client", champion);

        let loader = LeagueWadLoader::from_relative_path(&self.root_dir, &wad_relative_path)?;

        // let character_record_path = format!("Characters/{}/CharacterRecords/Root", character);

        let skin_path: String = format!("Characters/{}/Skins/{}", champion, skin);

        // let character_record = wad_loader.load_character_record(&character_record_path);

        let skin = loader.save_environment_object(&skin_path).await?;

        let config_path = get_bin_path(CONFIG_PATH_GAME);

        let config_game = ConfigGame {
            legends: vec![(Transform::default(), Team::Order, skin)],
        };

        save_struct_to_file(&config_path, &config_game).await?;

        Ok(())
    }

    pub fn hash_wad(s: &str) -> u64 {
        let mut h = XxHash64::with_seed(0);
        h.write(s.to_ascii_lowercase().as_bytes());
        h.finish()
    }

    pub fn hash_bin(s: &str) -> u32 {
        s.to_ascii_lowercase().bytes().fold(0x811c9dc5_u32, |h, b| {
            (h ^ b as u32).wrapping_mul(0x01000193)
        })
    }

    pub fn hash_joint(s: &str) -> u32 {
        let mut hash = 0u32;
        for b in s.to_ascii_lowercase().bytes() {
            hash = (hash << 4) + (b as u32);
            let high = hash & 0xf0000000;
            if high != 0 {
                hash ^= high >> 24;
            }
            hash &= !high;
        }
        hash
    }
}
