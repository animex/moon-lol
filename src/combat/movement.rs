use crate::{
    combat::{navigation::Obstacle, Bounding},
    system_debug,
};
use bevy::prelude::*;
use rvo2::RVOSimulatorWrapper;
use std::{collections::HashMap, time::Instant};
use vleue_navigator::prelude::*;

pub struct PluginMovement;

impl Plugin for PluginMovement {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(FixedUpdate, update);

        app.add_event::<CommandMovementMoveTo>();
        app.add_observer(command_movement_move_to);

        app.add_event::<EventMovementMoveEnd>();
    }
}

#[derive(Component)]
#[require(MovementVelocity)]
pub struct Movement {
    pub speed: f32,
}

#[derive(Component, Default)]
pub struct MovementVelocity(pub Vec2);

#[derive(Component)]
pub struct MovementDestination(pub Vec2);

#[derive(Event, Debug)]
pub struct EventMovementMoveEnd;

#[derive(Event, Debug)]
pub struct CommandMovementMoveTo(pub Vec2);

#[derive(Resource)]
pub struct ObstacleVerticesArray(pub Vec<Vec<[f32; 2]>>);

fn setup(
    mut commands: Commands,
    cachable_obstacles: Query<
        (&GlobalTransform, &PrimitiveObstacle),
        (With<Obstacle>, Without<Movement>),
    >,
) {
    let start = Instant::now();

    let mut obstacle_vertices_array = ObstacleVerticesArray(Vec::new());
    for (global_transform, &primitive_obstacle) in cachable_obstacles.iter() {
        let vertices = primitive_obstacle.get_polygons(
            global_transform,
            &Transform::default(),
            (global_transform.forward(), 0.0),
        );

        let mut vertices_array: Vec<[f32; 2]> = vertices
            .iter()
            .flat_map(|v| v.iter().map(|v| [v.x, v.y]))
            .collect();

        vertices_array.reverse();
        vertices_array.pop();

        obstacle_vertices_array.0.push(vertices_array);
    }
    commands.insert_resource(obstacle_vertices_array);

    debug!("init_obstacle: {:?}", start.elapsed());
}

fn update(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Movement,
        &MovementDestination,
        &mut MovementVelocity,
        &Bounding,
    )>,
    mut q_transform: Query<&mut Transform>,
    timer: Res<Time<Fixed>>,
    obstacle_vertices_array: Res<ObstacleVerticesArray>,
) {
    let mut simulator = RVOSimulatorWrapper::new();
    simulator.set_time_step(timer.timestep().as_secs_f32());

    for vertices_array in obstacle_vertices_array.0.iter() {
        simulator.add_obstacle(&vertices_array);
    }

    simulator.process_obstacles();

    let mut entity_to_index: HashMap<Entity, usize> = HashMap::new();

    for (entity, movement, movement_destination, movement_velocity, bounding) in query.iter_mut() {
        let destination = movement_destination.0;

        let transform = q_transform.get(entity).unwrap();

        let position = transform.translation.xz();

        let (old_velocity, pref_velocity) = {
            let target = destination;
            let direction = target - position;
            let velocity = if direction.length() > 0.0 {
                direction.normalize() * movement.speed
            } else {
                Vec2::ZERO
            };

            (movement_velocity.0, velocity)
        };

        let neighbor_dist = bounding.radius * 2.0;
        let max_neighbors = 25;
        let time_horizon = 10.0;
        let time_horizon_obst = 3.0;
        let radius = bounding.radius;
        let max_speed = movement.speed;
        let index = simulator.add_agent(
            &[position.x, position.y],
            neighbor_dist,
            max_neighbors,
            time_horizon,
            time_horizon_obst,
            radius,
            max_speed,
            &[old_velocity.x, old_velocity.y],
        );

        entity_to_index.insert(entity, index);

        simulator.set_agent_pref_velocity(index, &[pref_velocity.x, pref_velocity.y]);
    }

    simulator.do_step();

    for (entity, _, movement_destination, mut movement_velocity, _) in query.iter_mut() {
        let target = movement_destination.0;

        let mut transform = q_transform.get_mut(entity).unwrap();

        let index = *entity_to_index.get(&entity).unwrap(); // 使用正确的索引

        let current_pos = simulator.get_agent_position(index);
        let current_velocity = simulator.get_agent_velocity(index);

        transform.translation = Vec3::new(current_pos[0], transform.translation.y, current_pos[1]);

        if target.distance(Vec2::new(current_pos[0], current_pos[1])) < 10.0 {
            transform.translation = Vec3::new(target.x, transform.translation.y, target.y);
            commands.entity(entity).remove::<MovementDestination>();
            commands.trigger_targets(EventMovementMoveEnd, entity);
        }

        transform.rotation = Quat::from_rotation_y(current_velocity[0].atan2(current_velocity[1]));

        movement_velocity.0 = Vec2::new(current_velocity[0], current_velocity[1]);
    }
}

fn command_movement_move_to(trigger: Trigger<CommandMovementMoveTo>, mut commands: Commands) {
    let entity = trigger.target();
    let destination = trigger.event().0;

    system_debug!(
        "action_set_move_target",
        "Entity {:?} received move command to {:?}",
        entity,
        destination,
    );

    commands
        .entity(entity)
        .insert(MovementDestination(destination));
}
