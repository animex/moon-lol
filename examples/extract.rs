use std::collections::{HashMap, HashSet};

use league_core::{UiElementEffectAnimationDataTextureData, UiElementIconData};
use league_loader::{LeagueWadGroupLoader, PropBinLoader};
use league_property::from_entry;
use league_to_lol::{
    get_bin_path, save_config_map, save_struct_to_file, save_wad_entry_to_file, CONFIG_UI,
};
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let root_dir = r"D:\Program Files\Riot Games\League of Legends\Game";

    let start = Instant::now();

    let mut ui_elements = HashMap::new();

    let mut textures = HashSet::new();

    let ui_loader = LeagueWadGroupLoader::from_relative_path(
        root_dir,
        vec!["DATA/FINAL/UI.wad.client", "DATA/FINAL/UI.zh_MY.wad.client"],
    );

    for wad_entry in ui_loader.iter_wad_entries() {
        let Ok(prop_bin) = ui_loader.get_prop_bin_by_hash(*wad_entry.0) else {
            continue;
        };

        for entry in prop_bin.entries.iter() {
            let Ok(ui_element_icon_data) = from_entry::<UiElementIconData>(entry) else {
                continue;
            };

            if let Some(texture_data) = ui_element_icon_data.texture_data.as_ref() {
                match texture_data {
                    UiElementEffectAnimationDataTextureData::AtlasData(atlas_data) => {
                        if !textures.contains(&atlas_data.m_texture_name) {
                            textures.insert(atlas_data.m_texture_name.clone());
                        }
                    }
                    _ => {}
                }
            }

            ui_elements.insert(ui_element_icon_data.name.clone(), ui_element_icon_data);
        }
    }

    for texture in textures {
        match save_wad_entry_to_file(&ui_loader, &texture).await {
            Ok(_) => {}
            Err(e) => println!("{}: {:?}", texture, e),
        };
    }

    save_struct_to_file(&get_bin_path(CONFIG_UI), &ui_elements)
        .await
        .unwrap();

    save_config_map(
        root_dir,
        "base_srx",
        vec![("Fiora", "Skin22"), ("Hwei", "Skin0")],
    )
    .await
    .unwrap();

    let end = Instant::now();

    println!("Time taken: {:?}", end.duration_since(start));
}
