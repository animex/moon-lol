use std::collections::HashMap;

use bevy::prelude::*;
use lol_core::Team;

use crate::{DamageType, EventDamageCreate, EventDead};

#[derive(Default)]
pub struct PluginAggro;

impl Plugin for PluginAggro {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPostUpdate, aggro_scan);
        app.add_observer(on_team_get_damage);
        app.add_observer(on_target_dead);
    }
}

#[derive(Component)]
#[require(AggroState)]
pub struct Aggro {
    pub range: f32,
}

#[derive(Component, Default)]
pub struct AggroState {
    pub aggros: HashMap<Entity, f32>,
}

#[derive(EntityEvent, Debug)]
pub struct EventAggroTargetFound {
    pub entity: Entity,
    pub target: Entity,
}

pub fn aggro_scan(
    mut commands: Commands,
    q_aggro: Query<(Entity, &Team, &Transform, &Aggro, &AggroState)>,
    q_attackable: Query<(Entity, &Team, &Transform)>,
) {
    for (entity, team, transform, aggro, aggro_state) in q_aggro.iter() {
        let mut best_aggro = 0.0;
        let mut closest_distance = f32::MAX;
        let mut target_entity = Entity::PLACEHOLDER;

        // Iterate through all attackable units to find targets
        for (attackable_entity, attackable_team, attackable_transform) in q_attackable.iter() {
            // Ignore friendly units
            if attackable_team == team || *attackable_team == Team::Neutral {
                continue;
            }

            // Calculate distance and check if within aggro range
            let distance = transform
                .translation
                .distance(attackable_transform.translation);

            if distance >= aggro.range {
                continue;
            }

            // Get aggro value (default is 0)
            let aggro = aggro_state
                .aggros
                .get(&attackable_entity)
                .copied()
                .unwrap_or(0.0);

            // Prioritize targets with higher aggro, choose closer target when aggro is equal
            if aggro > best_aggro || (aggro == best_aggro && distance < closest_distance) {
                best_aggro = aggro;
                closest_distance = distance;
                target_entity = attackable_entity;
            }
        }

        // Trigger event if valid target found
        if target_entity != Entity::PLACEHOLDER {
            debug!("{} found aggro target {}", entity, target_entity);
            commands.trigger(EventAggroTargetFound {
                entity,
                target: target_entity,
            });
        }
    }
}

pub fn on_team_get_damage(
    trigger: On<EventDamageCreate>,
    mut q_aggro: Query<(&Team, &Transform, &Aggro, &mut AggroState)>,
    q_transform: Query<&Transform>,
    q_team: Query<&Team>,
) {
    let source = trigger.source;
    let target = trigger.event_target();

    if trigger.damage_type != DamageType::Physical {
        return;
    }

    let Ok(source_transform) = q_transform.get(source) else {
        return;
    };

    let Ok(target_team) = q_team.get(target) else {
        return;
    };

    for (team, transform, aggro, mut aggro_state) in q_aggro.iter_mut() {
        if target_team != team {
            continue;
        }

        let distance = transform.translation.distance(source_transform.translation);

        if distance >= aggro.range {
            continue;
        }

        let aggro = aggro_state.aggros.get(&source).copied().unwrap_or(0.0);

        aggro_state.aggros.insert(source, aggro + 10.0);
    }
}

fn on_target_dead(trigger: On<EventDead>, mut q_aggro: Query<&mut AggroState>) {
    let dead_entity = trigger.event_target();

    for mut aggro_state in q_aggro.iter_mut() {
        aggro_state.aggros.remove(&dead_entity);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::prelude::*;
    use bevy::time::TimeUpdateStrategy;

    use super::*;
    use crate::{DamageResult, DamageType, EventDamageCreate, EventDead};

    // Used in tests to capture the target selected by the system
    #[derive(Resource, Default)]
    struct LastTarget(Option<Entity>);

    // Test helper function: build App and inject necessary Plugins and Resources
    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(PluginAggro);
        // Manually control time updates so app.update() can run one FixedUpdate
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_micros(
            15625,
        )));
        app.init_resource::<LastTarget>();

        // Register observer: when Aggro system emits "target found" event, record to Resource
        app.add_observer(
            |event: On<EventAggroTargetFound>, mut res: ResMut<LastTarget>| {
                res.0 = Some(event.event().target);
            },
        );

        // Run once to let TimePlugin record initial values
        app.update();

        app
    }

    // Helper to build DamageResult
    fn mock_damage_result() -> DamageResult {
        DamageResult {
            final_damage: 10.0,
            white_shield_absorbed: 0.0,
            magic_shield_absorbed: 0.0,
            reduced_damage: 0.0,
            armor_reduced_damage: 0.0,
            original_damage: 10.0,
        }
    }

    #[test]
    fn test_scan_closest_target_when_no_aggro() {
        let mut app = setup_app();
        let world = app.world_mut();

        // 1. Create unit with aggro system (Order)
        let _me = world
            .spawn((Team::Order, Transform::default(), Aggro { range: 100.0 }))
            .id();

        // 2. Create two enemies, one close (10m), one far (20m)
        let enemy_near = world
            .spawn((Team::Chaos, Transform::from_xyz(10.0, 0.0, 0.0)))
            .id();

        let _enemy_far = world
            .spawn((Team::Chaos, Transform::from_xyz(20.0, 0.0, 0.0)))
            .id();

        // Run the system
        app.update();

        // Assert: when no aggro values exist, should select the closest enemy
        let target = app.world().resource::<LastTarget>().0;
        assert_eq!(target, Some(enemy_near), "should prioritize the closest target");
    }

    #[test]
    fn test_scan_high_aggro_priority() {
        let mut app = setup_app();
        let world = app.world_mut();

        let me = world
            .spawn((Team::Order, Transform::default(), Aggro { range: 100.0 }))
            .id();

        let enemy_near = world
            .spawn((Team::Chaos, Transform::from_xyz(10.0, 0.0, 0.0)))
            .id();

        let enemy_far = world
            .spawn((Team::Chaos, Transform::from_xyz(50.0, 0.0, 0.0)))
            .id();

        // Manually inject aggro values: far enemy has very high aggro
        let mut aggro_state = world.get_mut::<AggroState>(me).unwrap();
        aggro_state.aggros.insert(enemy_near, 0.0);
        aggro_state.aggros.insert(enemy_far, 100.0);

        app.update();

        // Assert: should ignore distance and select the target with highest aggro
        let target = app.world().resource::<LastTarget>().0;
        assert_eq!(target, Some(enemy_far), "should prioritize target with highest aggro");
    }

    #[test]
    fn test_scan_ignore_out_of_range() {
        let mut app = setup_app();
        let world = app.world_mut();

        // Own range is only 10
        world.spawn((Team::Order, Transform::default(), Aggro { range: 10.0 }));

        // Enemy is at distance 20
        world.spawn((Team::Chaos, Transform::from_xyz(20.0, 0.0, 0.0)));

        app.update();

        // Assert: no target should be selected
        let target = app.world().resource::<LastTarget>().0;
        assert_eq!(target, None, "out of range targets should be ignored");
    }

    #[test]
    fn test_damage_increases_aggro() {
        let mut app = setup_app();
        let world = app.world_mut();

        // Simulate scenario: ally is attacked, nearby guard should generate aggro against attacker
        let attacker = world
            .spawn((Team::Order, Transform::from_xyz(5.0, 0.0, 0.0)))
            .id();
        let ally = world.spawn((Team::Chaos, Transform::default())).id();

        let guard = world
            .spawn((Team::Chaos, Transform::default(), Aggro { range: 50.0 }))
            .id();

        // Trigger damage event
        world.trigger(EventDamageCreate {
            entity: ally,
            source: attacker,
            damage_type: DamageType::Physical,
            damage_result: mock_damage_result(),
        });

        app.update();

        // Verify guard's aggro list
        let state = app.world().get::<AggroState>(guard).unwrap();
        let aggro_val = state.aggros.get(&attacker).copied().unwrap_or(0.0);

        assert_eq!(aggro_val, 10.0, "ally being hit should increase attacker's aggro value");
    }

    #[test]
    fn test_remove_aggro_on_dead() {
        let mut app = setup_app();
        let world = app.world_mut();

        let enemy = world.spawn(Team::Chaos).id();
        let me = world.spawn((Team::Order, Aggro { range: 100.0 })).id();

        // 1. Initialize aggro
        if let Some(mut state) = world.get_mut::<AggroState>(me) {
            state.aggros.insert(enemy, 50.0);
        }

        // 2. Trigger target death event (triggered on target entity)
        // Note: According to on_target_dead implementation, EventDead needs to be triggered on the dead entity
        world.trigger(EventDead { entity: enemy });

        app.update();

        // 3. Verify cleanup logic
        let state = app.world().get::<AggroState>(me).unwrap();
        assert!(
            !state.aggros.contains_key(&enemy),
            "target should be removed from aggro list after death"
        );
    }
}
