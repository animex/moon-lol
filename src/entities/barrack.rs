use std::collections::VecDeque;
use std::time::Duration;

use bevy::prelude::*;
use league_core::{
    BarracksConfig, CharacterRecord, ConstantWaveBehavior, EnumMap, EnumWaveBehavior,
    InhibitorWaveBehavior, MapContainer, MapPlaceableContainer, RotatingWaveBehavior,
    TimedVariableWaveBehavior, Unk0xad65d8c4,
};
use lol_config::{HashKey, LoadHashKeyTrait};
use lol_core::{Lane, Team};

use crate::core::{Armor, CommandCharacterSpawn, Damage, Health, Movement};
use crate::entities::Minion;
use crate::{CommandCharacterLoad, MapName, MapState, MinionPath};

#[derive(Default)]
pub struct PluginBarrack;

impl Plugin for PluginBarrack {
    fn build(&self, app: &mut App) {
        app.init_state::<BarrackState>();
        app.init_resource::<InhibitorState>();
        app.add_systems(
            Update,
            (
                update_spawn_barrack
                    .run_if(in_state(MapState::Loaded).and(in_state(BarrackState::Loading))),
                is_character_loaded.run_if(in_state(BarrackState::Loading)),
            ),
        );
        app.add_systems(
            FixedUpdate,
            barracks_spawning_system.run_if(in_state(BarrackState::Loaded)),
        );
    }
}

/// Barrack's dynamic state, used to track timers and spawn queue
#[derive(Component)]
pub struct Barrack {
    pub key_barracks_config: HashKey<BarracksConfig>,
    /// Timer for the next wave spawn
    pub wave_timer: Timer,
    /// Stats upgrade timer
    pub upgrade_timer: Timer,
    /// Movement speed upgrade timer
    pub move_speed_upgrade_timer: Timer,
    /// Timer for spawn interval between each minion within the same wave
    pub intra_spawn_timer: Timer,
    /// Current spawn queue
    pub spawn_queue: VecDeque<(usize, i32)>,
    /// Number of stats upgrades applied
    pub upgrade_count: i32,
    /// Number of movement speed upgrades applied
    pub move_speed_upgrade_count: i32,
    /// Number of waves spawned
    pub wave_count: u32,
}

#[derive(Resource, Default)]
pub struct InhibitorState {
    pub inhibitors_down: usize,
}

#[derive(States, Default, Debug, Hash, Eq, Clone, PartialEq)]
pub enum BarrackState {
    #[default]
    Loading,
    Loaded,
}

fn update_spawn_barrack(
    mut commands: Commands,
    mut res_minion_path: ResMut<MinionPath>,
    res_map_name: Res<MapName>,
    res_assets_map_container: Res<Assets<MapContainer>>,
    res_assets_map_placeable_container: Res<Assets<MapPlaceableContainer>>,
    res_assets_barracks_config: Res<Assets<BarracksConfig>>,
    res_assets_unk_ad65d8c4: Res<Assets<Unk0xad65d8c4>>,
) {
    let map_container = res_assets_map_container
        .load_hash(&res_map_name.get_materials_path())
        .unwrap();

    for (_, &link) in &map_container.chunks {
        let Some(map_placeable_container) = res_assets_map_placeable_container.load_hash(link)
        else {
            continue;
        };

        let Some(items) = map_placeable_container.items.as_ref() else {
            continue;
        };

        for (_, value) in items {
            match value {
                EnumMap::Unk0xba138ae3(unk0xba138ae3) => {
                    if res_assets_barracks_config
                        .load_hash(unk0xba138ae3.definition.barracks_config)
                        .is_none()
                    {
                        return;
                    }
                }
                _ => {}
            }
        }

        for (_, value) in items {
            match value {
                EnumMap::Unk0x3c995caf(unk0x3c995caf) => {
                    let lane = match unk0x3c995caf.name.as_str() {
                        "MinionPath_Top" => Lane::Top,
                        "MinionPath_Mid" => Lane::Mid,
                        "MinionPath_Bot" => Lane::Bot,
                        _ => panic!("Unknown minion path: {}", unk0x3c995caf.name),
                    };

                    let translation = unk0x3c995caf.transform.to_scale_rotation_translation().2;

                    let path = unk0x3c995caf
                        .segments
                        .iter()
                        .map(|v| (v + translation).xz())
                        .collect();

                    res_minion_path.0.entry(lane).or_insert(path);
                }
                EnumMap::Unk0xba138ae3(unk0xba138ae3) => {
                    let key_barracks_config = unk0xba138ae3.definition.barracks_config.into();

                    let barracks_config = res_assets_barracks_config
                        .load_hash(key_barracks_config)
                        .unwrap();

                    for unit in &barracks_config.units {
                        let character = res_assets_unk_ad65d8c4
                            .load_hash(unit.unk_0xfee040bc)
                            .unwrap();

                        commands.trigger(CommandCharacterLoad {
                            character_record: character.definition.character_record.clone(),
                        });
                    }

                    // let initial_delay = barracks_config.initial_spawn_time_secs;
                    let initial_delay = 0.0;

                    commands.spawn((
                        Transform::from_matrix(unk0xba138ae3.transform),
                        Team::from(unk0xba138ae3.definition.team),
                        Lane::from(unk0xba138ae3.definition.unk_0xdbde2288[0].lane),
                        Barrack {
                            key_barracks_config,
                            // First wave has initial delay
                            wave_timer: Timer::from_seconds(initial_delay, TimerMode::Repeating),
                            // Stats upgrade starts counting after first wave spawns
                            upgrade_timer: Timer::new(
                                Duration::from_secs_f32(barracks_config.upgrade_interval_secs),
                                TimerMode::Repeating,
                            ),
                            // Movement speed upgrade has its own independent delay
                            move_speed_upgrade_timer: Timer::new(
                                Duration::from_secs_f32(
                                    barracks_config.move_speed_increase_initial_delay_secs,
                                ),
                                TimerMode::Repeating,
                            ),
                            // Timer for spawn interval between minions
                            intra_spawn_timer: Timer::from_seconds(
                                barracks_config.minion_spawn_interval_secs,
                                TimerMode::Repeating,
                            ),
                            spawn_queue: VecDeque::new(),
                            upgrade_count: 0,
                            move_speed_upgrade_count: 0,
                            wave_count: 0,
                        },
                    ));
                }
                _ => {}
            }
        }
    }
}

fn is_character_loaded(
    mut commands: Commands,
    q_barrack: Query<&Barrack>,
    res_assets_barracks_config: Res<Assets<BarracksConfig>>,
    res_assets_character_record: Res<Assets<CharacterRecord>>,
    res_assets_unk_ad65d8c4: Res<Assets<Unk0xad65d8c4>>,
) {
    if q_barrack.is_empty() {
        return;
    }

    for barrack in q_barrack.iter() {
        let barracks_config = res_assets_barracks_config
            .load_hash(barrack.key_barracks_config)
            .unwrap();

        for unit in &barracks_config.units {
            let character = res_assets_unk_ad65d8c4
                .load_hash(unit.unk_0xfee040bc)
                .unwrap();

            if res_assets_character_record
                .load_hash(&character.definition.character_record)
                .is_none()
            {
                return;
            }
        }
    }

    commands.set_state(BarrackState::Loaded);
}

/// Core system: handles barrack timing, upgrades, and spawning logic
fn barracks_spawning_system(
    game_time: Res<Time<Virtual>>,
    inhibitor_state: Res<InhibitorState>,
    mut commands: Commands,
    mut query: Query<(&GlobalTransform, &mut Barrack, &Team, &Lane)>,
    res_assets_character_record: Res<Assets<CharacterRecord>>,
    time: Res<Time>,
    res_assets_barracks_config: Res<Assets<BarracksConfig>>,
    res_assets_unk_ad65d8c4: Res<Assets<Unk0xad65d8c4>>,
) {
    for (transform, mut barrack_state, team, lane) in query.iter_mut() {
        let barracks_config = res_assets_barracks_config
            .load_hash(barrack_state.key_barracks_config)
            .unwrap();

        // --- 1. Update all timers ---
        barrack_state.wave_timer.tick(time.delta());

        // Only start counting upgrades after the first wave
        if barrack_state.wave_count > 0 {
            barrack_state.upgrade_timer.tick(time.delta());
            barrack_state.move_speed_upgrade_timer.tick(time.delta());
        }

        // --- 2. Handle stats and movement speed upgrades ---
        if barrack_state.upgrade_timer.just_finished() {
            barrack_state.upgrade_count += 1;
            debug!(
                "Barrack upgraded! New count: {}",
                barrack_state.upgrade_count
            );
        }

        if barrack_state.move_speed_upgrade_timer.just_finished() {
            if barrack_state.move_speed_upgrade_count
                < barracks_config.move_speed_increase_max_times
            {
                barrack_state.move_speed_upgrade_count += 1;
                debug!(
                    "Minion move speed upgraded! New count: {}",
                    barrack_state.move_speed_upgrade_count
                );
            }
        }

        // --- 3. Check if a new wave of minions needs to spawn ---
        // Only start preparing a new wave when the previous one is fully spawned (queue is empty)
        if barrack_state.wave_timer.just_finished() && barrack_state.spawn_queue.is_empty() {
            barrack_state.wave_count += 1;
            barrack_state
                .wave_timer
                .set_duration(Duration::from_secs_f32(
                    barracks_config.wave_spawn_interval_secs,
                ));

            // Iterate through all minion types in the barrack configuration
            for (index, minion_config) in barracks_config.units.iter().enumerate() {
                let spawn_count = calculate_spawn_count(
                    &minion_config.wave_behavior,
                    game_time.elapsed_secs(),
                    barrack_state.wave_count,
                    &inhibitor_state,
                );

                if spawn_count > 0 {
                    barrack_state.spawn_queue.push_back((index, spawn_count));
                }
            }
        }

        // --- 4. Process spawn queue, spawn minions one by one ---
        if barrack_state.spawn_queue.is_empty() {
            continue;
        }

        barrack_state.intra_spawn_timer.tick(time.delta());

        if !barrack_state.intra_spawn_timer.just_finished() {
            continue;
        }

        let upgrade_count = barrack_state.upgrade_count;
        let move_speed_upgrade_count = barrack_state.move_speed_upgrade_count;

        // Get the minion info at the head of the spawn queue
        let Some(current_spawn) = barrack_state.spawn_queue.front_mut() else {
            continue;
        };

        let config_index = current_spawn.0;
        let minion_config = &barracks_config.units[config_index];
        let upgrade_config = &minion_config.minion_upgrade_stats;

        // --- Calculate minion final stats ---
        let is_late_game = upgrade_count >= barracks_config.upgrades_before_late_game_scaling;

        let character = res_assets_unk_ad65d8c4
            .load_hash(minion_config.unk_0xfee040bc)
            .unwrap();

        let character_record = res_assets_character_record
            .load_hash(&character.definition.character_record)
            .unwrap();

        let mut health = Health::new(character_record.base_hp.unwrap_or(0.0));
        let hp_upgrade = if is_late_game {
            upgrade_config.hp_upgrade_late.unwrap_or(0.0)
        } else {
            upgrade_config.hp_upgrade
        };
        health.max += (hp_upgrade * upgrade_count as f32).min(upgrade_config.hp_max_bonus);

        let mut damage = Damage(character_record.base_damage.unwrap_or(0.0));
        let damage_upgrade = if is_late_game {
            upgrade_config.damage_upgrade_late
        } else {
            upgrade_config.damage_upgrade
        };
        damage.0 += damage_upgrade.unwrap_or(0.0) * upgrade_count as f32;
        damage.0 = damage.0.min(upgrade_config.damage_max);

        let mut armor = Armor(character_record.base_armor.unwrap_or(0.0));
        armor.0 += upgrade_config.armor_upgrade_growth.unwrap_or(0.0) * upgrade_count as f32;
        if let Some(max) = upgrade_config.armor_max {
            armor.0 = armor.0.min(max);
        }

        let mut movement = Movement {
            speed: character_record.base_move_speed.unwrap_or(0.0),
        };
        movement.speed +=
            (barracks_config.move_speed_increase_increment * move_speed_upgrade_count) as f32;

        let entity = commands
            .spawn((
                Transform::from_matrix(transform.to_matrix()),
                Minion::from(minion_config.minion_type),
                lane.clone(),
                team.clone(),
            ))
            .id();

        // Trigger character spawn command (create base components and load skin)
        commands.trigger(CommandCharacterSpawn {
            entity,
            character_record: (&character.definition.character_record).into(),
            skin: (&character.definition.skin).into(),
        });

        commands
            .entity(entity)
            .insert((health, damage, armor, movement));

        // Update queue
        current_spawn.1 -= 1;
        if current_spawn.1 <= 0 {
            barrack_state.spawn_queue.pop_front();
        }
    }
}

/// Helper function: calculate spawn count based on different WaveBehavior types
fn calculate_spawn_count(
    behavior: &EnumWaveBehavior,
    game_time_secs: f32,
    wave_count: u32,
    inhibitor_state: &InhibitorState,
) -> i32 {
    match behavior {
        EnumWaveBehavior::ConstantWaveBehavior(ConstantWaveBehavior { spawn_count }) => {
            *spawn_count
        }
        EnumWaveBehavior::InhibitorWaveBehavior(InhibitorWaveBehavior {
            spawn_count_per_inhibitor_down,
        }) => {
            if inhibitor_state.inhibitors_down == 0 {
                return 0;
            }

            spawn_count_per_inhibitor_down
                .get(inhibitor_state.inhibitors_down - 1)
                .copied()
                .unwrap_or(0)
        }
        EnumWaveBehavior::TimedVariableWaveBehavior(TimedVariableWaveBehavior {
            behaviors,
            ..
        }) => {
            // Find the most appropriate behavior for the current time point
            let mut active_behavior = None;
            for timed_behavior in behaviors.iter().rev() {
                if game_time_secs >= timed_behavior.start_time_secs as f32 {
                    active_behavior = Some(&timed_behavior.behavior);
                    break;
                }
            }

            if let Some(active_behavior) = active_behavior {
                // Recursive call
                calculate_spawn_count(active_behavior, game_time_secs, wave_count, inhibitor_state)
            } else {
                0
            }
        }
        EnumWaveBehavior::RotatingWaveBehavior(RotatingWaveBehavior {
            spawn_counts_by_wave,
        }) => {
            if spawn_counts_by_wave.is_empty() {
                0
            } else {
                spawn_counts_by_wave
                    [((wave_count - 1) % spawn_counts_by_wave.len() as u32) as usize]
            }
        }
    }
}
