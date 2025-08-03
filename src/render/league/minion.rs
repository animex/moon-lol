use crate::render::LeagueLoader;
use bevy::math::{Mat4, Vec3};
use cdragon_prop::{
    BinEmbed, BinEntry, BinFloat, BinList, BinMatrix, BinS32, BinString, BinStruct, BinU8, BinVec3,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct LeagueMinionPath {
    pub transform: Mat4,
    pub name: String,
    pub segments: Vec<Vec3>,
}

impl From<&BinStruct> for LeagueMinionPath {
    fn from(value: &BinStruct) -> Self {
        let transform = value
            .getv::<BinMatrix>(LeagueLoader::hash_bin("transform").into())
            .map(|v| Mat4::from_cols_array_2d(&v.0))
            .unwrap();

        let name = value
            .getv::<BinString>(LeagueLoader::hash_bin("name").into())
            .map(|v| v.0.clone())
            .unwrap();

        let segments = value
            .getv::<BinList>(LeagueLoader::hash_bin("Segments").into())
            .iter()
            .filter_map(|v| v.downcast::<BinVec3>())
            .flat_map(|v| v.iter().map(|v| Vec3::new(v.0, v.1, v.2)))
            .collect();

        Self {
            transform,
            name,
            segments,
        }
    }
}

#[derive(Debug)]
pub struct LeagueBarracksMinionConfig {
    pub initial_spawn_time_secs: Option<f32>,
    pub wave_spawn_interval_secs: Option<f32>,
    pub minion_spawn_interval_secs: Option<f32>,
    pub upgrade_interval_secs: Option<f32>,
    pub upgrades_before_late_game_scaling: Option<i32>,
    pub move_speed_increase_initial_delay_secs: Option<f32>,
    pub move_speed_increase_interval_secs: Option<f32>,
    pub move_speed_increase_increment: Option<i32>,
    pub move_speed_increase_max_times: Option<i32>,
    pub exp_radius: Option<f32>,
    pub gold_radius: Option<f32>,
    pub units: Vec<BarracksMinionConfig>,
}

impl From<&BinEntry> for LeagueBarracksMinionConfig {
    fn from(value: &BinEntry) -> Self {
        let initial_spawn_time_secs = value
            .getv::<BinFloat>(LeagueLoader::hash_bin("InitialSpawnTimeSecs").into())
            .map(|f| f.0);

        let wave_spawn_interval_secs = value
            .getv::<BinFloat>(LeagueLoader::hash_bin("WaveSpawnIntervalSecs").into())
            .map(|f| f.0);

        let minion_spawn_interval_secs = value
            .getv::<BinFloat>(LeagueLoader::hash_bin("MinionSpawnIntervalSecs").into())
            .map(|f| f.0);

        let upgrade_interval_secs = value
            .getv::<BinFloat>(LeagueLoader::hash_bin("UpgradeIntervalSecs").into())
            .map(|f| f.0);

        let upgrades_before_late_game_scaling = value
            .getv::<BinS32>(LeagueLoader::hash_bin("UpgradesBeforeLateGameScaling").into())
            .map(|i| i.0);

        let move_speed_increase_initial_delay_secs = value
            .getv::<BinFloat>(LeagueLoader::hash_bin("MoveSpeedIncreaseInitialDelaySecs").into())
            .map(|f| f.0);

        let move_speed_increase_interval_secs = value
            .getv::<BinFloat>(LeagueLoader::hash_bin("MoveSpeedIncreaseIntervalSecs").into())
            .map(|f| f.0);

        let move_speed_increase_increment = value
            .getv::<BinS32>(LeagueLoader::hash_bin("MoveSpeedIncreaseIncrement").into())
            .map(|i| i.0);

        let move_speed_increase_max_times = value
            .getv::<BinS32>(LeagueLoader::hash_bin("MoveSpeedIncreaseMaxTimes").into())
            .map(|i| i.0);

        let exp_radius = value
            .getv::<BinFloat>(LeagueLoader::hash_bin("ExpRadius").into())
            .map(|f| f.0);

        let gold_radius = value
            .getv::<BinFloat>(LeagueLoader::hash_bin("goldRadius").into())
            .map(|f| f.0);

        let units = value
            .getv::<BinList>(LeagueLoader::hash_bin("units").into())
            .map(|list| {
                list.downcast::<BinEmbed>()
                    .unwrap()
                    .iter()
                    .map(|embed| BarracksMinionConfig::from(embed))
                    .collect()
            })
            .unwrap_or_default();

        Self {
            initial_spawn_time_secs,
            wave_spawn_interval_secs,
            minion_spawn_interval_secs,
            upgrade_interval_secs,
            upgrades_before_late_game_scaling,
            move_speed_increase_initial_delay_secs,
            move_speed_increase_interval_secs,
            move_speed_increase_increment,
            move_speed_increase_max_times,
            exp_radius,
            gold_radius,
            units,
        }
    }
}

#[derive(Debug)]
pub struct BarracksMinionConfig {
    pub minion_type: Option<u8>,
    pub wave_behavior: Option<WaveBehavior>,
    pub minion_upgrade_stats: Option<MinionUpgradeConfig>,
}

impl From<&BinEmbed> for BarracksMinionConfig {
    fn from(value: &BinEmbed) -> Self {
        let minion_type = value
            .getv::<BinU8>(LeagueLoader::hash_bin("MinionType").into())
            .map(|u| u.0);

        let wave_behavior = value
            .getv::<BinStruct>(LeagueLoader::hash_bin("WaveBehavior").into())
            .map(|s| WaveBehavior::from(s));

        let minion_upgrade_stats = value
            .getv::<BinEmbed>(LeagueLoader::hash_bin("MinionUpgradeStats").into())
            .map(|e| MinionUpgradeConfig::from(e));

        Self {
            minion_type,
            wave_behavior,
            minion_upgrade_stats,
        }
    }
}

#[derive(Debug)]
pub enum WaveBehavior {
    InhibitorWaveBehavior {
        spawn_count_per_inhibitor_down: Vec<i32>,
    },
    ConstantWaveBehavior {
        spawn_count: i32,
    },
    TimedVariableWaveBehavior {
        behaviors: Vec<TimedWaveBehaviorInfo>,
    },
    RotatingWaveBehavior {
        spawn_counts_by_wave: Vec<i32>,
    },
    Unknown,
}

impl From<&BinStruct> for WaveBehavior {
    fn from(value: &BinStruct) -> Self {
        let inhibitor_hash = LeagueLoader::hash_bin("InhibitorWaveBehavior");
        let constant_hash = LeagueLoader::hash_bin("ConstantWaveBehavior");
        let timed_variable_hash = LeagueLoader::hash_bin("TimedVariableWaveBehavior");
        let rotating_hash = LeagueLoader::hash_bin("RotatingWaveBehavior");

        match value.ctype.hash {
            hash if hash == inhibitor_hash => {
                let spawn_count_per_inhibitor_down = value
                    .getv::<BinList>(LeagueLoader::hash_bin("SpawnCountPerInhibitorDown").into())
                    .map(|list| {
                        list.downcast::<BinS32>()
                            .unwrap()
                            .iter()
                            .map(|i| i.0)
                            .collect()
                    })
                    .unwrap_or_default();

                WaveBehavior::InhibitorWaveBehavior {
                    spawn_count_per_inhibitor_down,
                }
            }
            hash if hash == constant_hash => {
                let spawn_count = value
                    .getv::<BinS32>(LeagueLoader::hash_bin("SpawnCount").into())
                    .map(|i| i.0)
                    .unwrap_or(0);

                WaveBehavior::ConstantWaveBehavior { spawn_count }
            }
            hash if hash == timed_variable_hash => {
                let behaviors = value
                    .getv::<BinList>(LeagueLoader::hash_bin("behaviors").into())
                    .map(|list| {
                        list.downcast::<BinEmbed>()
                            .unwrap()
                            .iter()
                            .map(|embed| TimedWaveBehaviorInfo::from(embed))
                            .collect()
                    })
                    .unwrap_or_default();

                WaveBehavior::TimedVariableWaveBehavior { behaviors }
            }
            hash if hash == rotating_hash => {
                let spawn_counts_by_wave = value
                    .getv::<BinList>(LeagueLoader::hash_bin("SpawnCountsByWave").into())
                    .map(|list| {
                        list.downcast::<BinS32>()
                            .unwrap()
                            .iter()
                            .map(|i| i.0)
                            .collect()
                    })
                    .unwrap_or_default();

                WaveBehavior::RotatingWaveBehavior {
                    spawn_counts_by_wave,
                }
            }
            _ => WaveBehavior::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct TimedWaveBehaviorInfo {
    pub start_time_secs: Option<i32>,
    pub behavior: Option<WaveBehavior>,
}

impl From<&BinEmbed> for TimedWaveBehaviorInfo {
    fn from(value: &BinEmbed) -> Self {
        let start_time_secs = value
            .getv::<BinS32>(LeagueLoader::hash_bin("StartTimeSecs").into())
            .map(|i| i.0);

        let behavior = value
            .getv::<BinStruct>(LeagueLoader::hash_bin("Behavior").into())
            .map(|s| WaveBehavior::from(s));

        Self {
            start_time_secs,
            behavior,
        }
    }
}

#[derive(Debug)]
pub struct MinionUpgradeConfig {
    pub armor_max: Option<f32>,
    pub armor_upgrade_growth: Option<f32>,
    pub hp_max_bonus: Option<f32>,
    pub hp_upgrade: Option<f32>,
    pub hp_upgrade_late: Option<f32>,
    pub damage_max: Option<f32>,
    pub damage_upgrade: Option<f32>,
    pub damage_upgrade_late: Option<f32>,
}

impl From<&BinEmbed> for MinionUpgradeConfig {
    fn from(value: &BinEmbed) -> Self {
        let armor_max = value
            .getv::<BinFloat>(LeagueLoader::hash_bin("ArmorMax").into())
            .map(|f| f.0);

        let armor_upgrade_growth = value
            .getv::<BinFloat>(LeagueLoader::hash_bin("ArmorUpgradeGrowth").into())
            .map(|f| f.0);

        let hp_max_bonus = value
            .getv::<BinFloat>(LeagueLoader::hash_bin("HpMaxBonus").into())
            .map(|f| f.0);

        let hp_upgrade = value
            .getv::<BinFloat>(LeagueLoader::hash_bin("HPUpgrade").into())
            .map(|f| f.0);

        let hp_upgrade_late = value
            .getv::<BinFloat>(LeagueLoader::hash_bin("HPUpgradeLate").into())
            .map(|f| f.0);

        let damage_max = value
            .getv::<BinFloat>(LeagueLoader::hash_bin("DamageMax").into())
            .map(|f| f.0);

        let damage_upgrade = value
            .getv::<BinFloat>(LeagueLoader::hash_bin("DamageUpgrade").into())
            .map(|f| f.0);

        let damage_upgrade_late = value
            .getv::<BinFloat>(LeagueLoader::hash_bin("DamageUpgradeLate").into())
            .map(|f| f.0);

        Self {
            armor_max,
            armor_upgrade_growth,
            hp_max_bonus,
            hp_upgrade,
            hp_upgrade_late,
            damage_max,
            damage_upgrade,
            damage_upgrade_late,
        }
    }
}
