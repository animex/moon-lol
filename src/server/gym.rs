use bevy::prelude::*;

#[derive(Default)]
pub struct PluginGymEnv;

impl Plugin for PluginGymEnv {
    fn build(&self, _app: &mut App) {
        // app.add_systems(FixedUpdate, spawn_target);
    }
}

#[derive(Component)]
pub struct AttackTarget;

// fn spawn_target(
//     mut commands: Commands,
//     q_t: Query<&AttackTarget>,
//     mut res_animation_graph: ResMut<Assets<AnimationGraph>>,
//     asset_server: Res<AssetServer>,
//     res_navigation_grid: Res<ConfigNavigationGrid>,
//     config_game: Res<ConfigGame>,
// ) {
//     if q_t.single().is_ok() {
//         return;
//     }

//     for (_, _, skin) in config_game.legends.iter() {
//         commands.entity(target).insert((
//             Team::Chaos,
//             Health {
//                 value: 6000.0,
//                 max: 6000.0,
//             },
//             AttackTarget,
//         ));
//     }
// }
