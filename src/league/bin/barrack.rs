use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BarracksConfig {
    pub exp_radius: f32,
    pub gold_radius: f32,
    pub initial_spawn_time_secs: f32,
    pub minion_spawn_interval_secs: f32,
    pub move_speed_increase_increment: i32,
    pub move_speed_increase_initial_delay_secs: f32,
    pub move_speed_increase_interval_secs: f32,
    pub move_speed_increase_max_times: i32,
    pub units: Vec<BarracksMinionConfig>,
    pub upgrade_interval_secs: f32,
    pub upgrades_before_late_game_scaling: i32,
    pub wave_spawn_interval_secs: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BarracksMinionConfig {
    pub minion_type: u8,
    pub minion_upgrade_stats: MinionUpgradeConfig,
    pub unk_8a3fc6eb: u32,
    pub wave_behavior: WaveBehavior,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MinionUpgradeConfig {
    pub armor_max: Option<f32>,
    pub armor_upgrade_growth: Option<f32>,
    pub damage_max: f32,
    pub damage_upgrade_late: Option<f32>,
    pub damage_upgrade: Option<f32>,
    pub hp_max_bonus: f32,
    pub hp_upgrade_late: Option<f32>,
    pub hp_upgrade: f32,
    pub unk_726ae049: Option<f32>,
    pub unk_db0e9d5b: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WaveBehavior {
    #[serde(rename_all = "camelCase")]
    InhibitorWaveBehavior {
        spawn_count_per_inhibitor_down: Vec<i32>,
    },
    #[serde(rename_all = "camelCase")]
    ConstantWaveBehavior { spawn_count: i32 },
    #[serde(rename_all = "camelCase")]
    TimedVariableWaveBehavior {
        behaviors: Vec<TimedWaveBehaviorInfo>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimedWaveBehaviorInfo {
    pub start_time_secs: i32,
    pub behavior: TimedBehavior,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimedBehavior {
    #[serde(rename_all = "camelCase")]
    RotatingWaveBehavior { spawn_counts_by_wave: Vec<i32> },
    #[serde(rename_all = "camelCase")]
    ConstantWaveBehavior { spawn_count: i32 },
}
