use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use lol_core::Team;

use crate::ConfigCharacterSkin;

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct ConfigGame {
    pub legends: Vec<ConfigLegend>,
}

type ConfigLegend = (Transform, Team, ConfigCharacterSkin);
