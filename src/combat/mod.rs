mod attack;
mod base;
mod damage;
mod life;
mod movement;
mod navigation;

pub use attack::*;
pub use base::*;
pub use damage::*;
pub use life::*;
pub use movement::*;
pub use navigation::*;

pub struct PluginCombat;

impl bevy::app::Plugin for PluginCombat {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            PluginAttack,
            PluginDamage,
            PluginLife,
            PluginTarget,
            PluginMovement,
            // PluginNavigaton,
        ));
    }
}
