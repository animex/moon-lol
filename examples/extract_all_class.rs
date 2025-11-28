use std::collections::HashMap;

use league_loader::{LeagueLoader, LeagueWadLoader};
use league_property::{
    class_map_to_rust_code, extract_all_class, get_hashes, get_hashes_u64, merge_class_maps,
};
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let root_dir = r"D:\Program Files\Riot Games\League of Legends\Game";

    let loader = LeagueLoader::full(root_dir, "bloom").unwrap();

    let start = Instant::now();

    let hash_paths = vec![
        "assets/hashes/hashes.binentries.txt",
        "assets/hashes/hashes.binfields.txt",
        "assets/hashes/hashes.binhashes.txt",
        "assets/hashes/hashes.bintypes.txt",
    ];

    let file_paths = vec![
        "assets/hashes/hashes.game.txt.0",
        "assets/hashes/hashes.game.txt.1",
    ];

    let hashes = get_hashes_u64(&file_paths);

    let file_paths = hashes
        .iter()
        .filter(|(_hash, path)| {
            path.starts_with("clientstates")
                || (path.ends_with(".bin")
                    && (path.contains("data/characters/")
                        || path.contains("map11/bloom.materials.bin")))
        })
        .map(|v| v.1.as_str())
        .collect::<Vec<_>>();

    let rust_code = {
        let mut class_map = HashMap::new();
        for (i, file_path) in file_paths.iter().enumerate() {
            println!("{:?}/{:?}", i, file_paths.len());
            let Ok(bin) = loader.get_prop_bin_by_path(file_path) else {
                println!("{:?} get prop error", file_path);

                continue;
            };
            let bin_class_map = extract_all_class(&bin).await.unwrap();
            merge_class_maps(&mut class_map, bin_class_map);
        }

        let hashes = get_hashes(&hash_paths);

        let rust_code = class_map_to_rust_code(&mut class_map, &hashes)
            .await
            .unwrap();

        rust_code
    };

    std::fs::write("league.rs", rust_code).unwrap();

    let ui_loader =
        LeagueWadLoader::from_relative_path(root_dir, "DATA/FINAL/UI.wad.client").unwrap();

    let file_paths = hashes
        .iter()
        .filter(|(_hash, path)| path.starts_with("clientstates"))
        .map(|v| v.1.as_str())
        .collect::<Vec<_>>();

    let rust_code = {
        let mut class_map = HashMap::new();
        for (i, file_path) in file_paths.iter().enumerate() {
            println!("{:?}/{:?}", i, file_paths.len());
            let Ok(bin) = ui_loader.get_prop_bin_by_path(file_path) else {
                println!("{:?} get prop error", file_path);

                continue;
            };
            let bin_class_map = extract_all_class(&bin).await.unwrap();
            merge_class_maps(&mut class_map, bin_class_map);
        }

        let hashes = get_hashes(&hash_paths);

        let rust_code = class_map_to_rust_code(&mut class_map, &hashes)
            .await
            .unwrap();

        rust_code
    };

    std::fs::write("league_ui.rs", rust_code).unwrap();

    let end = Instant::now();
    println!("Time taken: {:?}", end.duration_since(start));
}
