use league_to_lol::{save_config_map, save_legends, save_navigation_grid};
use tokio::time::Instant;

use league_loader::LeagueLoader;

#[tokio::main]
async fn main() {
    #[cfg(unix)]
    let root_dir = r"/mnt/c/Program Files (x86)/WeGameApps/英雄联盟/game";
    #[cfg(windows)]
    let root_dir = r"C:\Program Files (x86)\WeGameApps\英雄联盟\game";

    let loader = LeagueLoader::new(root_dir, "bloom").unwrap();

    let start = Instant::now();

    save_config_map(&loader.map_loader).await.unwrap();

    save_navigation_grid(&loader.map_loader).await.unwrap();

    match save_legends(root_dir, "Fiora", "Skin44").await {
        Ok(_) => println!("Legends saved"),
        Err(e) => println!("Error saving legends: {:?}", e),
    }

    let end = Instant::now();
    println!("Time taken: {:?}", end.duration_since(start));
}
