use league_loader::LeagueLoader;
use league_to_lol::load_navigation_grid;
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    #[cfg(unix)]
    let root_dir = r"/mnt/c/Program Files (x86)/WeGameApps/英雄联盟/game";
    #[cfg(windows)]
    let root_dir = r"C:\Program Files (x86)\WeGameApps\英雄联盟\game";

    let loader = LeagueLoader::new(root_dir, "bloom").unwrap();

    let start = Instant::now();

    load_navigation_grid(&loader.map_loader).await.unwrap();

    let end = Instant::now();
    println!("Time taken: {:?}", end.duration_since(start));
}
