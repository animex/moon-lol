use core::f32;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::core::rotate_to_direction;

#[derive(Default)]
pub struct PluginMovement;

impl Plugin for PluginMovement {
    fn build(&self, app: &mut App) {
        app.register_type::<Movement>();

        app.add_event::<CommandMovementStart>();
        app.add_event::<EventMovementStart>();
        app.add_observer(command_movement_start);

        app.add_event::<CommandMovementStop>();
        app.add_observer(command_movement_stop);

        app.add_event::<EventMovementEnd>();
        app.add_observer(on_movement_stop);

        app.add_systems(
            FixedPostUpdate,
            (finalize_decision_system, update_path_movement).chain(),
        );
    }
}

#[derive(Component, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
#[require(MovementState)]
pub struct Movement {
    pub speed: f32,
}

#[derive(Component, Default)]
pub struct MovementState {
    pub path: Vec<Vec2>,
    pub speed: Option<f32>,
    pub direction: Vec2,
    pub velocity: Vec2,
    pub current_target_index: usize,
    pub completed: bool,
}

#[derive(Event, Debug, Clone)]
pub struct CommandMovementStart {
    pub priority: i32,
    pub path: Vec<Vec2>,
    pub speed: Option<f32>,
}

#[derive(Event, Debug)]
pub struct CommandMovementStop;

#[derive(Event, Debug)]
pub struct EventMovementStart;

#[derive(Event, Debug)]
pub struct EventMovementEnd;

impl MovementState {
    pub fn reset_path(&mut self, path: &Vec<Vec2>) {
        self.path = path.clone();
        self.speed = None;
        self.current_target_index = 0;
        self.completed = false;
        self.velocity = Vec2::ZERO;
        self.direction = Vec2::ZERO;
    }

    pub fn clear_path(&mut self) {
        self.path.clear();
        self.current_target_index = 0;
        self.completed = false;
        self.velocity = Vec2::ZERO;
        self.direction = Vec2::ZERO;
    }

    pub fn is_moving(&self) -> bool {
        self.current_target_index < self.path.len() - 1
    }

    pub fn with_speed(&mut self, speed: f32) -> &mut Self {
        self.speed = Some(speed);
        self
    }
}

fn update_path_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &Movement, &mut MovementState)>,
    mut q_transform: Query<&mut Transform>,
    time: Res<Time<Fixed>>,
) {
    let dt = time.delta_secs();

    for (entity, movement, mut movement_state) in query.iter_mut() {
        if movement_state.completed || movement_state.path.is_empty() {
            continue;
        }

        let mut transform = q_transform.get_mut(entity).unwrap();

        let speed = movement_state.speed.unwrap_or(movement.speed);

        // 本帧可移动的总距离
        let mut remaining_distance_this_frame = speed * dt;
        // 记录本帧最后的移动方向，用于更新旋转
        let mut last_direction = Vec2::ZERO;

        // 只要本帧还有可移动的距离，就继续处理
        while remaining_distance_this_frame > 0.0 {
            // 首先，检查当前的目标点是否有效
            let target = match movement_state.path.get(movement_state.current_target_index) {
                Some(p) => *p,
                None => {
                    if !movement_state.completed {
                        movement_state.completed = true;
                    }
                    break;
                }
            };

            let current_pos = transform.translation.xz();
            let vector_to_target = target - current_pos;
            let distance_to_target = vector_to_target.length();

            if distance_to_target.abs() < f32::EPSILON {
                let new_index = movement_state.current_target_index + 1;
                if new_index >= movement_state.path.len() {
                    movement_state.completed = true;
                    break;
                } else {
                    movement_state.current_target_index = new_index;
                    continue;
                }
            }

            last_direction = vector_to_target.normalize();

            if distance_to_target > remaining_distance_this_frame {
                let new_pos = current_pos + last_direction * remaining_distance_this_frame;
                transform.translation.x = new_pos.x;
                transform.translation.z = new_pos.y;
                remaining_distance_this_frame = 0.0;
            } else {
                transform.translation.x = target.x;
                transform.translation.z = target.y;
                remaining_distance_this_frame -= distance_to_target;

                let new_index = movement_state.current_target_index + 1;
                if new_index >= movement_state.path.len() {
                    movement_state.completed = true;
                    break;
                } else {
                    movement_state.current_target_index = new_index;
                }
            }
        }

        // 在循环结束后，根据最终状态统一更新
        if movement_state.completed {
            movement_state.velocity = Vec2::ZERO;
            movement_state.direction = Vec2::ZERO;
            movement_state.clear_path();
            // 恢复您原来的事件触发命令
            commands.trigger_targets(EventMovementEnd, entity);
        } else {
            movement_state.direction = last_direction;
            movement_state.velocity = last_direction * speed;
        }

        // 更新旋转
        if last_direction.length_squared() > 0.0 {
            rotate_to_direction(&mut transform, last_direction);
        }
    }
}

#[derive(Component, Clone)]
struct MovementCurrentMovement(CommandMovementStart);

fn command_movement_start(
    trigger: Trigger<CommandMovementStart>,
    mut commands: Commands,
    query: Query<&MovementCurrentMovement>,
) {
    let target_entity = trigger.target();

    let event = trigger.event();

    if event.path.is_empty() {
        return;
    }

    if let Ok(current_best) = query.get(target_entity) {
        // 如果新请求的优先级更高，则替换
        if event.priority >= current_best.0.priority {
            commands
                .entity(target_entity)
                .insert(MovementCurrentMovement(event.clone()));
        }
    } else {
        // 这是本帧第一个请求，直接插入
        commands
            .entity(target_entity)
            .insert(MovementCurrentMovement(event.clone()));
    }
}

fn finalize_decision_system(
    mut commands: Commands,
    // 查询所有在本帧被修改过“最佳请求”的实体
    query: Query<(Entity, &MovementCurrentMovement), Changed<MovementCurrentMovement>>,
    mut q_transform: Query<&mut MovementState>,
) {
    for (entity, best_request) in query.iter() {
        // 1. 添加“最终决策”

        let Ok(mut movement_state) = q_transform.get_mut(entity) else {
            return;
        };

        movement_state.reset_path(&best_request.0.path);

        if let Some(speed) = best_request.0.speed {
            movement_state.with_speed(speed);
        }

        commands.trigger_targets(EventMovementStart, entity);

        // 2. 移除临时组件，为下一帧做准备
    }
}

fn on_movement_stop(trigger: Trigger<EventMovementEnd>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .remove::<MovementCurrentMovement>();
}

fn command_movement_stop(
    trigger: Trigger<CommandMovementStop>,
    mut commands: Commands,
    mut q_movement: Query<&mut MovementState>,
) {
    let entity = trigger.target();

    let Ok(mut movement_state) = q_movement.get_mut(entity) else {
        return;
    };

    movement_state.clear_path();

    commands.trigger_targets(EventMovementEnd, entity);
}
