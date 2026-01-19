use std::fs::write;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use bevy::prelude::*;
use league_loader::{LeagueLoader, LeagueWadLoaderTrait};
use league_utils::get_extension_by_bytes;
use rayon::prelude::*;

fn main() {
    let root_dir = r"D:\WeGameApps\League of Legends\Game";

    let start = Instant::now();

    let loader = LeagueLoader::from_relative_path(
        root_dir,
        vec![
            "DATA/FINAL/UI.wad.client",
            "DATA/FINAL/UI.zh_CN.wad.client",
            "DATA/FINAL/Maps/Shipping/Map11.wad.client",
            "DATA/FINAL/Champions/Riven.wad.client",
            "DATA/FINAL/Champions/Fiora.wad.client",
            "DATA/FINAL/Bootstrap.windows.wad.client",
        ],
    );

    println!("Loading wad took: {:?}", start.elapsed());

    let dir = Path::new("assets/data");
    if !dir.exists() {
        std::fs::create_dir_all(dir).unwrap();
    }

    let start = Instant::now();

    // Collect all (wad_index, entry_hash) tasks
    let tasks: Vec<_> = loader
        .wads
        .iter()
        .enumerate()
        .flat_map(|(wad_index, wad)| {
            wad.wad
                .entries
                .keys()
                .copied()
                .map(move |hash| (wad_index, hash))
        })
        .collect();

    let total_tasks = tasks.len();
    let processed_count = AtomicUsize::new(0);

    tasks.par_iter().for_each(|(wad_index, hash)| {
        let wad = &loader.wads[*wad_index];

        // Increment count regardless of success or failure
        let current = processed_count.fetch_add(1, Ordering::Relaxed) + 1;
        if current % 10000 == 0 || current == total_tasks {
            println!("Processed {} / {} entries", current, total_tasks);
        }

        let Ok(buffer) = wad.get_wad_entry_buffer_by_hash(*hash) else {
            return;
        };

        let extension = get_extension_by_bytes(&buffer);

        if extension == "tex" || extension == "dds" {
            write(format!("assets/data/{:x}.{}", hash, extension), buffer).unwrap();
        } else {
            write(format!("assets/data/{:x}.lol", hash), buffer).unwrap();
        }
    });

    println!("Time elapsed: {:?}", start.elapsed());
}
