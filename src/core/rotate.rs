use bevy::prelude::*;

use crate::{
    direction_to_angle, lerp_angle_with_velocity, ArbitrationPipelinePlugin, FinalDecision,
    LastDecision, MovementPipeline, PipelineStages, RequestBuffer,
};

// Default angular velocity: 20 radians per second
const DEFAULT_ANGULAR_VELOCITY: f32 = 20.0;

#[derive(Default)]
pub struct PluginRotate;

impl Plugin for PluginRotate {
    fn build(&self, app: &mut App) {
        app.add_plugins(ArbitrationPipelinePlugin::<CommandRotate, RotatePipeline>::default());

        app.add_systems(
            FixedPostUpdate,
            (
                reduce_rotate_by_priority.in_set(RotatePipeline::Reduce),
                (apply_final_rotate_decision, fixed_update)
                    .chain()
                    .in_set(RotatePipeline::Apply),
                remove_rotate_component.after(RotatePipeline::Cleanup),
            )
                .after(MovementPipeline::Apply),
        );
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Rotate {
    pub angular_velocity: f32, // Angular velocity, unit: radians/second
    pub current_angle: f32,    // Current angle
    pub target_angle: f32,     // Target angle
}

#[derive(EntityEvent, Debug, Clone)]
pub struct CommandRotate {
    pub entity: Entity,
    pub priority: i32,
    pub direction: Vec2,
    pub angular_velocity: Option<f32>,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum RotatePipeline {
    Modify,
    Reduce,
    Apply,
    Cleanup,
}

impl PipelineStages for RotatePipeline {
    fn modify() -> Self {
        RotatePipeline::Modify
    }
    fn reduce() -> Self {
        RotatePipeline::Reduce
    }
    fn apply() -> Self {
        RotatePipeline::Apply
    }
    fn cleanup() -> Self {
        RotatePipeline::Cleanup
    }
}

fn fixed_update(mut query: Query<(&mut Rotate, &mut Transform)>, time: Res<Time<Fixed>>) {
    for (mut rotate, mut transform) in query.iter_mut() {
        let delta_time = time.delta_secs();

        // Use angular velocity for smooth rotation
        let new_angle = lerp_angle_with_velocity(
            rotate.current_angle,
            rotate.target_angle,
            rotate.angular_velocity,
            delta_time,
        );

        // Update current angle
        rotate.current_angle = new_angle;

        // Apply rotation to Transform
        transform.rotation = Quat::from_rotation_y(new_angle);
    }
}

fn remove_rotate_component(mut commands: Commands, query: Query<(Entity, &Rotate), With<Rotate>>) {
    for (entity, rotate) in query.iter() {
        if (rotate.current_angle - rotate.target_angle).abs() < 0.01 {
            commands
                .entity(entity)
                .remove::<(Rotate, LastDecision<CommandRotate>)>();
        }
    }
}

fn reduce_rotate_by_priority(
    mut commands: Commands,
    query: Query<(
        Entity,
        &RequestBuffer<CommandRotate>,
        Option<&LastDecision<CommandRotate>>,
    )>,
) {
    for (entity, buffer, last_decision) in query.iter() {
        if buffer.0.is_empty() {
            continue;
        }

        let mut final_decision = last_decision.map(|v| &v.0);
        let mut found = false;

        for command in buffer.0.iter() {
            match final_decision {
                None => {
                    final_decision = Some(command);
                    found = true;
                }
                Some(current) => {
                    if command.priority >= current.priority {
                        final_decision = Some(command);
                        found = true;
                    }
                }
            }
        }

        if let Some(decision) = final_decision {
            if found {
                commands
                    .entity(entity)
                    .insert(FinalDecision(decision.clone()));
            }
        }
    }
}

fn apply_final_rotate_decision(
    mut commands: Commands,
    query: Query<(Entity, &FinalDecision<CommandRotate>)>,
    transform_query: Query<&Transform>,
) {
    for (entity, final_decision) in query.iter() {
        let command = &final_decision.0;

        // Calculate target angle
        let target_angle = direction_to_angle(command.direction);
        let angular_velocity = command.angular_velocity.unwrap_or(DEFAULT_ANGULAR_VELOCITY);

        // Get current angle (if entity already has Transform component)
        let current_angle = if let Ok(transform) = transform_query.get(entity) {
            transform.rotation.to_euler(EulerRot::YZX).0
        } else {
            0.0
        };

        commands.entity(entity).insert(Rotate {
            angular_velocity,
            current_angle,
            target_angle,
        });
    }
}
