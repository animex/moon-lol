use bevy::prelude::*;
use league_loader::LeagueWadLoader;
use lol_config::ConfigGame;
use lol_core::Team;

use crate::{get_bin_path, save_environment_object, save_struct_to_file, Error, CONFIG_PATH_GAME};

pub async fn save_legends(root_dir: &str, champion: &str, skin: &str) -> Result<(), Error> {
    let wad_relative_path = format!("DATA/FINAL/Champions/{}.wad.client", champion);

    let loader = LeagueWadLoader::from_relative_path(&root_dir, &wad_relative_path)?;

    // let character_record_path = format!("Characters/{}/CharacterRecords/Root", character);

    let skin_path: String = format!("Characters/{}/Skins/{}", champion, skin);

    // let character_record = wad_loader.load_character_record(&character_record_path);

    let skin = save_environment_object(&loader, &skin_path).await?;

    let config_path = get_bin_path(CONFIG_PATH_GAME);

    let config_game = ConfigGame {
        legends: vec![(Transform::default(), Team::Order, skin)],
    };

    save_struct_to_file(&config_path, &config_game).await?;

    Ok(())
}
