use std::collections::HashMap;
use std::fs::{create_dir_all, write};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use league_loader::{LeagueLoader, LeagueWadLoaderTrait};
use league_property::PropFile;
use lol_config::ASSET_LOADER_REGISTRY;
use rayon::prelude::*;

fn get_hashes_u64(path: &str) -> HashMap<u64, String> {
    let mut hashes = HashMap::new();
    if let Ok(content) = std::fs::read_to_string(path) {
        for line in content.lines() {
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() == 2 {
                if let Ok(hash) = u64::from_str_radix(parts[0], 16) {
                    hashes.insert(hash, parts[1].to_string());
                }
            }
        }
    }
    hashes
}

fn main() {
    let root_dir = r"D:\WeGameApps\League of Legends\Game";
    let output_base_dir = "assets/extract_prop_bins";

    if !Path::new(root_dir).exists() {
        println!("Game directory not found: {}", root_dir);
        return;
    }

    create_dir_all(output_base_dir).unwrap();

    let start = Instant::now();
    let loader = LeagueLoader::full(root_dir).unwrap();
    println!("Loading wad took: {:?}", start.elapsed());

    let hashes = get_hashes_u64("assets/hashes/hashes.game.txt");
    println!("Loading hash mapping took: {:?}", start.elapsed());

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
    let extracted_count = AtomicUsize::new(0);

    println!("Starting to process {} entries...", total_tasks);

    tasks.par_iter().for_each(|&(wad_index, hash)| {
        let wad = &loader.wads[wad_index];

        let current = processed_count.fetch_add(1, Ordering::Relaxed) + 1;
        if current % 10000 == 0 || current == total_tasks {
            println!("Processed {} / {} entries", current, total_tasks);
        }

        let Ok(buffer) = wad.get_wad_entry_buffer_by_hash(hash) else {
            return;
        };

        // Check magic number "PROP"
        if buffer.len() < 4 || &buffer[0..4] != b"PROP" {
            return;
        }

        // Try to parse as PropFile
        let Ok((_, prop_file)) = PropFile::parse(&buffer) else {
            return;
        };

        let mut entries_by_class: HashMap<u32, HashMap<u32, String>> = HashMap::new();

        for (class_hash, entry) in prop_file.iter_class_hash_and_entry() {
            if let Some((_, loader)) = ASSET_LOADER_REGISTRY.loaders.get(&class_hash) {
                if let Ok(ron_str) = loader.to_ron(entry) {
                    entries_by_class
                        .entry(class_hash)
                        .or_default()
                        .insert(entry.hash, ron_str);
                }
            }
        }

        if entries_by_class.is_empty() {
            return;
        }

        // Determine file path and original path comment
        let hex_name = format!("{:016x}", hash);
        let (rel_path, original_path) = if let Some(path) = hashes.get(&hash) {
            let path_obj = Path::new(path);
            let parent = path_obj.parent().unwrap_or(Path::new(""));
            (parent.to_path_buf(), Some(path.clone()))
        } else {
            (Path::new("unknown").to_path_buf(), None)
        };

        let target_dir = Path::new(output_base_dir).join(rel_path);
        let file_path = target_dir.join(format!("{}.ron", hex_name));

        // Create nested directories
        if let Err(e) = create_dir_all(&target_dir) {
            eprintln!("Unable to create directory {:?}: {}", target_dir, e);
            return;
        }

        // Build final RON string
        let mut ron_output = String::new();
        if let Some(path) = original_path {
            ron_output.push_str(&format!("// {}\n", path));
        }
        ron_output.push_str("{\n");
        for (class_hash, entries) in entries_by_class {
            let class_name = ASSET_LOADER_REGISTRY
                .loaders
                .get(&class_hash)
                .map(|v| v.0.as_str())
                .unwrap_or("Unknown");
            ron_output.push_str(&format!("    \"{}\": {{\n", class_name));
            for (entry_hash, entry_ron) in entries {
                let mut lines = entry_ron.lines();
                if let Some(first_line) = lines.next() {
                    ron_output.push_str(&format!("        0x{:08x}: {}", entry_hash, first_line));
                    for line in lines {
                        ron_output.push_str(&format!("\n            {}", line));
                    }
                    ron_output.push_str(",\n");
                }
            }
            ron_output.push_str("    },\n");
        }
        ron_output.push_str("}");

        if let Err(e) = write(&file_path, ron_output) {
            eprintln!("Unable to write file {:?}: {}", file_path, e);
        } else {
            extracted_count.fetch_add(1, Ordering::Relaxed);
        }
    });

    println!(
        "Extraction complete! Extracted {} prop bin files and saved as .ron",
        extracted_count.load(Ordering::Relaxed)
    );
    println!("Total time: {:?}", start.elapsed());
}
