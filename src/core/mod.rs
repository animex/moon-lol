mod animation;
mod attack;
mod attack_auto;
mod base;
mod behavior;
mod camera;
mod config;
mod controller;
mod damage;
mod effect;
mod game;
mod life;
mod map;
mod movement;
mod navigation;
mod particle;
mod resource;
mod skill;
mod ui;

pub use animation::*;
pub use attack::*;
pub use attack_auto::*;
pub use base::*;
pub use behavior::*;
pub use camera::*;
pub use config::*;
pub use controller::*;
pub use damage::*;
pub use effect::*;
pub use game::*;
pub use life::*;
pub use map::*;
pub use movement::*;
pub use navigation::*;
pub use particle::*;
pub use resource::*;
pub use skill::*;
pub use ui::*;

use bevy::app::plugin_group;

plugin_group! {
    pub struct PluginCore {
        :PluginAnimation,
        :PluginAttack,
        :PluginAttackAuto,
        :PluginBehavior,
        :PluginCamera,
        :PluginController,
        :PluginDamage,
        :PluginGame,
        :PluginLife,
        :PluginMap,
        :PluginMovement,
        :PluginNavigaton,
        :PluginResource,
        :PluginState,
        :PluginUI,
        :PluginParticle,
        :PluginSkill,
    }
}
