use league_to_lol::{
    get_struct_from_file, CONFIG_PATH_GAME, CONFIG_PATH_MAP, CONFIG_PATH_MAP_NAV_GRID,
};
use lol_config::{ConfigGame, ConfigMap, ConfigNavigationGrid};
use lol_loader::{
    LeagueLoaderAnimationClip, LeagueLoaderImage, LeagueLoaderMaterial, LeagueLoaderMesh,
    LeagueLoaderMeshStatic, LeagueLoaderSkinnedMeshInverseBindposes,
};

use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Default)]
pub struct PluginResource;

impl Plugin for PluginResource {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<LeagueLoaderMaterial>();
        app.init_asset_loader::<LeagueLoaderImage>();
        app.init_asset_loader::<LeagueLoaderMesh>();
        app.init_asset_loader::<LeagueLoaderMeshStatic>();
        app.init_asset_loader::<LeagueLoaderAnimationClip>();
        app.init_asset_loader::<LeagueLoaderSkinnedMeshInverseBindposes>();

        let configs: ConfigMap = get_struct_from_file(CONFIG_PATH_MAP).unwrap();
        app.insert_resource(configs);

        let game_configs: ConfigGame = get_struct_from_file(CONFIG_PATH_GAME).unwrap();
        app.insert_resource(game_configs);
        app.init_resource::<ResourceCache>();

        let nav_grid: ConfigNavigationGrid =
            get_struct_from_file(CONFIG_PATH_MAP_NAV_GRID).unwrap();

        app.insert_resource(nav_grid);
    }
}

#[derive(Resource, Default)]
pub struct ResourceCache {
    image: HashMap<String, Handle<Image>>,
    mesh: HashMap<String, Handle<Mesh>>,
}

impl ResourceCache {
    pub fn get_image(&mut self, asset_server: &AssetServer, path: &str) -> Handle<Image> {
        match self.image.get(path) {
            Some(handle) => handle.clone(),
            None => {
                let handle = asset_server.load(path);
                self.image.insert(path.to_string(), handle.clone());
                handle
            }
        }
    }

    pub fn get_mesh(&mut self, asset_server: &AssetServer, path: &str) -> Handle<Mesh> {
        match self.mesh.get(path) {
            Some(handle) => handle.clone(),
            None => {
                let handle = asset_server.load(path);
                self.mesh.insert(path.to_string(), handle.clone());
                handle
            }
        }
    }
}
