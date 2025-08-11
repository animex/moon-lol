use std::fs::File;

use crate::{
    core::Configs,
    league::{
        LeagueLoaderAnimationClip, LeagueLoaderAnimationGraph, LeagueLoaderImage,
        LeagueLoaderMaterial, LeagueLoaderMesh, LeagueLoaderSkinnedMeshInverseBindposes,
    },
};
use bevy::{prelude::*, scene::ron::de::from_reader};

pub struct PluginResource;

impl Plugin for PluginResource {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<LeagueLoaderMaterial>();
        app.init_asset_loader::<LeagueLoaderImage>();
        app.init_asset_loader::<LeagueLoaderMesh>();
        app.init_asset_loader::<LeagueLoaderAnimationClip>();
        app.init_asset_loader::<LeagueLoaderAnimationGraph>();
        app.init_asset_loader::<LeagueLoaderSkinnedMeshInverseBindposes>();

        let configs: Configs = from_reader(File::open("assets/configs.ron").unwrap()).unwrap();
        app.insert_resource(configs);
    }
}
