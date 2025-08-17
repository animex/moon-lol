mod astar;
mod smoother;

pub use astar::*;
pub use smoother::*;

use bevy::prelude::*;
use std::time::Instant;

use crate::core::{CommandMovementFollowPath, ConfigMap, Movement};
use crate::system_debug;

#[derive(Event, Debug)]
pub struct CommandNavigationTo(pub Vec2);

pub struct PluginNavigaton;

impl Plugin for PluginNavigaton {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);

        app.add_observer(command_movement_move_to);
    }
}

fn update(configs: Res<ConfigMap>, mut q_movement: Query<&mut Transform, With<Movement>>) {
    for mut transform in q_movement.iter_mut() {
        transform.translation = configs
            .navigation_grid
            .get_world_position_by_position(transform.translation.xz());
    }
}

fn command_movement_move_to(
    trigger: Trigger<CommandNavigationTo>,
    mut commands: Commands,
    configs: Res<ConfigMap>,
    mut q_transform: Query<&Transform>,
) {
    let entity = trigger.target();
    let destination = trigger.event().0;

    // 获取当前位置
    if let Ok(transform) = q_transform.get_mut(entity) {
        let start_pos = transform.translation;
        let end_pos = Vec3::new(destination.x, start_pos.y, destination.y);

        let start = Instant::now();
        // 使用A*算法规划路径，对于单点移动，创建长度为1的路径
        if let Some(path) = find_path(&configs, start_pos, end_pos) {
            let duration = start.elapsed();

            system_debug!(
                "command_movement_move_to",
                "Path found in {:.6}ms",
                duration.as_millis()
            );
            commands.trigger_targets(CommandMovementFollowPath(path), entity);
        }
    }
}

/// 主要的寻路函数，结合A*和漏斗算法
pub fn find_path(configs: &ConfigMap, start: Vec3, end: Vec3) -> Option<Vec<Vec2>> {
    let grid = &configs.navigation_grid;

    // 首先使用A*找到网格路径
    let grid_path = find_grid_path(grid, start, end)?;

    let simplified_path = simplify_path(&grid_path);

    return Some(
        simplified_path
            .iter()
            .map(|(x, y)| grid.get_position_by_float_xy(Vec2::new(*x, *y)))
            .collect(),
    );
}
