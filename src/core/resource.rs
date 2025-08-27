use league_to_lol::{
    get_struct_from_file, CONFIG_PATH_GAME, CONFIG_PATH_MAP, CONFIG_PATH_MAP_NAV_GRID,
};
use lol_config::{ConfigGame, ConfigMap, ConfigNavigationGrid};
use lol_loader::{
    LeagueLoaderAnimationClip, LeagueLoaderImage, LeagueLoaderMaterial, LeagueLoaderMesh,
    LeagueLoaderSkinnedMeshInverseBindposes,
};

use bevy::prelude::*;

#[derive(Default)]
pub struct PluginResource;

impl Plugin for PluginResource {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<LeagueLoaderMaterial>();
        app.init_asset_loader::<LeagueLoaderImage>();
        app.init_asset_loader::<LeagueLoaderMesh>();
        app.init_asset_loader::<LeagueLoaderAnimationClip>();
        app.init_asset_loader::<LeagueLoaderSkinnedMeshInverseBindposes>();

        let configs: ConfigMap = get_struct_from_file(CONFIG_PATH_MAP).unwrap();
        app.insert_resource(configs);

        let game_configs: ConfigGame = get_struct_from_file(CONFIG_PATH_GAME).unwrap();
        app.insert_resource(game_configs);

        let nav_grid: ConfigNavigationGrid =
            get_struct_from_file(CONFIG_PATH_MAP_NAV_GRID).unwrap();
        app.insert_resource(nav_grid);
    }
}
