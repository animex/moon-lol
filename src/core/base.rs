mod ability_resource;
mod bounding;
mod buff;
mod direction;
mod level;
mod pipeline;
mod position;
mod state;

pub use ability_resource::*;
pub use bounding::*;
pub use buff::*;
pub use direction::*;
pub use level::*;
pub use pipeline::*;
pub use position::*;
pub use state::*;

use bevy::app::{App, Plugin};

use lol_core::Team;

#[derive(Default)]
pub struct PluginBase;

impl Plugin for PluginBase {
    fn build(&self, app: &mut App) {
        app.register_type::<Team>();
    }
}
