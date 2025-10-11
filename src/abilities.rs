mod duelists_dance;
mod lunge;

pub use duelists_dance::*;
pub use lunge::*;

use bevy::app::plugin_group;

plugin_group! {
    pub struct PluginAbilities {
        :PluginDuelistsDance,
        :PluginLunge,
    }
}
