mod barrack;
mod champion;
mod champions;
mod inhibitor;
mod minion;
mod nexus;
mod turret;

pub use barrack::*;
pub use champion::*;
pub use champions::*;
pub use inhibitor::*;
pub use minion::*;
pub use nexus::*;
pub use turret::*;

use bevy::app::{App, Plugin};

pub struct PluginEntities;

impl Plugin for PluginEntities {
    fn build(&self, app: &mut App) {
        app.add_plugins((PluginMinion, PluginBarrack));
    }
}
