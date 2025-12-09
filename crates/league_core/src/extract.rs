use std::collections::HashMap;

use bevy::{
    asset::Asset,
    math::{Mat4, Vec2, Vec3, Vec4},
    reflect::TypePath,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbilitiesUiData {
    pub champion_spells: Vec<SpellSlotDetailedUiDefinition>,
    pub custom_abilities: Option<CustomAbilitiesUiData>,
    pub passive: SpellSlotDetailedUiDefinition,
    pub spell_rank_pips: Option<SpellRankPipsUiData>,
    pub spell_rank_text: Option<SpellRankTextUiData>,
    pub summoner_spells: Vec<SpellSlotDetailedUiDefinition>,
    pub unk_0x40aa9d58: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct AbilityObject {
    pub ability_traits: Option<u32>,
    pub m_child_spells: Option<Vec<u32>>,
    pub m_lifetime_manually_managed: Option<bool>,
    pub m_name: String,
    pub m_root_spell: u32,
    pub m_type: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbilityPrompt {
    pub anim_link: u32,
    pub text_link: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbilityResourceBarData {
    pub ability_resource_bars: EnumResourceMeter,
    pub backdrop: Option<u32>,
    pub standard_tick: Option<u32>,
    pub use_animated_skins: Option<bool>,
    pub value_text: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbilityResourceByCoefficientCalculationPart {
    pub m_ability_resource: Option<u8>,
    pub m_coefficient: f32,
    pub m_stat_formula: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbilityResourceDynamicMaterialFloatDriver {
    pub slot: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbilityResourcePipSpacerTypeMap {
    pub additional_pip_spacer_types: HashMap<u32, u32>,
    pub default_pip_spacer: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbilityResourcePipTypeMap {
    pub additional_pip_types: HashMap<u32, u32>,
    pub default_empty_pip: u32,
    pub default_large_pip: u32,
    pub default_medium_pip: u32,
    pub default_small_pip: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbilityResourcePipsData {
    pub backdrop: u32,
    pub pip_spacer: AbilityResourcePipSpacerTypeMap,
    pub pips: AbilityResourcePipTypeMap,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbilityResourceSlotInfo {
    pub ar_allow_max_value_to_be_overridden: Option<bool>,
    pub ar_base: Option<f32>,
    pub ar_base_factor_regen: Option<f32>,
    pub ar_base_static_regen: Option<f32>,
    pub ar_display_as_pips: Option<bool>,
    pub ar_has_regen_text: Option<bool>,
    pub ar_increments: Option<f32>,
    pub ar_is_shown: Option<bool>,
    pub ar_max_segments: Option<i32>,
    pub ar_negative_spacer: Option<bool>,
    pub ar_override_empty_pip_name: Option<String>,
    pub ar_override_large_pip_name: Option<String>,
    pub ar_override_medium_pip_name: Option<String>,
    pub ar_override_small_pip_name: Option<String>,
    pub ar_override_spacer_name: Option<String>,
    pub ar_per_level: Option<f32>,
    pub ar_regen_per_level: Option<f32>,
    pub ar_type: Option<u8>,
    pub hide_empty_pips: Option<bool>,
    pub unk_0x4eb6a404: Option<u8>,
    pub visibility_flags: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbilityResourceStateColorOptions {
    pub color: Option<[u8; 4]>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbilityResourceStateData {
    pub animation_suffix: Option<String>,
    pub colorblind_palette: Option<AbilityResourceStateColorOptions>,
    pub default_palette: Option<AbilityResourceStateColorOptions>,
    pub texture_suffix: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbilityResourceThresholdIndicatorRange {
    pub range_end: f32,
    pub range_start: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct AbilityResourceTypeConfig {
    pub ammo: AbilityResourceTypeData,
    pub battle_fury: AbilityResourceTypeData,
    pub bloodwell: AbilityResourceTypeData,
    pub dragon_fury: AbilityResourceTypeData,
    pub energy: AbilityResourceTypeData,
    pub ferocity: AbilityResourceTypeData,
    pub heat: AbilityResourceTypeData,
    pub mana: AbilityResourceTypeData,
    pub moon_light: AbilityResourceTypeData,
    pub none: AbilityResourceTypeData,
    pub other: AbilityResourceTypeData,
    pub primal_fury: AbilityResourceTypeData,
    pub rage: AbilityResourceTypeData,
    pub shield: AbilityResourceTypeData,
    pub wind: AbilityResourceTypeData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AbilityResourceTypeData {
    pub show_ability_resource: Option<bool>,
    pub show_regen: Option<bool>,
    pub states: Vec<AbilityResourceStateData>,
    pub threshold_indicator_ranges: Option<Vec<AbilityResourceThresholdIndicatorRange>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AcceleratingMovement {
    pub m_acceleration: Option<f32>,
    pub m_initial_speed: Option<f32>,
    pub m_max_speed: f32,
    pub m_min_speed: Option<f32>,
    pub m_offset_initial_target_height: Option<f32>,
    pub m_project_target_to_cast_range: Option<bool>,
    pub m_start_bone_name: Option<String>,
    pub m_start_bone_skin_overrides: Option<HashMap<u32, String>>,
    pub m_target_bone_name: Option<String>,
    pub m_target_height_augment: Option<f32>,
    pub m_tracks_target: Option<bool>,
    pub m_use_ground_height_at_target: Option<bool>,
    pub m_use_height_offset_at_end: Option<bool>,
    pub m_visuals_track_hidden_targets: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ActiveItemTelemetryData {
    pub active_item_telemetry_event_options: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddLevelTimer {
    pub callback: LevelScriptFunctionLink,
    pub initial_delay_secs: FloatGet,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AiSpellData {
    pub m_block_level: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AllTrueMaterialDriver {
    pub m_drivers: Option<Vec<Box<EnumDriver>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnchorDouble {
    pub anchor_left: Option<Vec2>,
    pub anchor_right: Option<Vec2>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnchorSingle {
    pub anchor: Vec2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AndInputSourceBool {
    pub sources: Vec<Box<EnumInputSource>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnimationFractionDynamicMaterialFloatDriver {
    pub m_animation_name: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct AnimationGraphData {
    pub m_blend_data_table: Option<HashMap<u64, EnumBlendData>>,
    pub m_cascade_blend_value: Option<f32>,
    pub m_clip_data_map: Option<HashMap<u32, EnumClipData>>,
    pub m_mask_data_map: Option<HashMap<u32, MaskData>>,
    pub m_sync_group_data_map: Option<HashMap<u32, SyncGroupData>>,
    pub m_track_data_map: HashMap<u32, TrackData>,
    pub m_use_cascade_blend: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnimationResourceData {
    pub m_animation_file_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementDefinition {
    pub default_data: AnnouncementDefinitionData,
    pub mutator_overrides: Option<HashMap<String, AnnouncementDefinitionData>>,
    pub unk_0x61fca04c: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementDefinitionData {
    pub allied_elements: Option<u32>,
    pub can_be_made_obsolete: Option<bool>,
    pub chat_message_key: Option<String>,
    pub common_elements: Option<u32>,
    pub enemy_elements: Option<u32>,
    pub make_lower_priority_events_obsolete: Option<bool>,
    pub priority: Option<u16>,
    pub sound_key: Option<String>,
    pub spectator_sound_key: Option<String>,
    pub style: u32,
    pub text_key: Option<String>,
    pub unk_0xab1323ac: Option<u32>,
    pub unk_0xb0665909: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementIcon {
    pub allied_element_group: u32,
    pub border_ring_element: u32,
    pub enemy_element_group: u32,
    pub main_content_element: u32,
    pub main_icon_group: u32,
    pub mastery_border_level4: u32,
    pub mastery_border_level5: u32,
    pub mastery_border_level6: u32,
    pub mastery_border_level7: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementMap {
    pub announcements: HashMap<String, u32>,
    pub death_recap_offset_region: u32,
    pub parent_list: Option<u32>,
    pub spectator_offset_region: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementStyleBasic {
    pub min_announcement_duration: Option<f32>,
    pub on_hide_transition_data: HudMenuTransitionData,
    pub on_show_transition_data: Option<HudMenuTransitionData>,
    pub text_field: u32,
    pub unk_0x3bfb7f66: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementStyleOneIcon {
    pub min_announcement_duration: Option<f32>,
    pub on_hide_transition_data: Option<HudMenuTransitionData>,
    pub on_show_transition_data: Option<HudMenuTransitionData>,
    pub source_icon: u32,
    pub text_field: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementStyleTwoIconsFlanking {
    pub left_icon: u32,
    pub min_announcement_duration: Option<f32>,
    pub right_icon: u32,
    pub text_field: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementsViewController {
    pub base_loadable: u32,
    pub main_banner_scene: u32,
    pub path_hash_to_self: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArenaSkinLoadoutGridButtonData {
    pub check: u32,
    pub favorite_icon: u32,
    pub game_pass: u32,
    pub group: u32,
    pub icon: u32,
    pub lock: u32,
    pub unk_0xd02a6781: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AtlasData {
    pub m_texture_name: String,
    pub m_texture_source_resolution_height: Option<u32>,
    pub m_texture_source_resolution_width: Option<u32>,
    pub m_texture_uv: Option<Vec4>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AtlasData3SliceH {
    pub left_right_widths: Vec2,
    pub m_texture_name: String,
    pub m_texture_source_resolution_height: u32,
    pub m_texture_source_resolution_width: u32,
    pub texture_us: Vec4,
    pub texture_vs: Vec2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AtlasData3SliceV {
    pub m_texture_name: String,
    pub m_texture_source_resolution_height: u32,
    pub m_texture_source_resolution_width: u32,
    pub texture_us: Vec2,
    pub texture_vs: Vec4,
    pub top_bottom_heights: Vec2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AtlasData9Slice {
    pub left_right_widths: Vec2,
    pub m_texture_name: String,
    pub m_texture_source_resolution_height: u32,
    pub m_texture_source_resolution_width: u32,
    pub texture_us: Vec4,
    pub texture_vs: Vec4,
    pub top_bottom_heights: Vec2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AtomicClipData {
    pub accessorylist: Option<Vec<KeyFrameFloatMapClipAccessoryData>>,
    pub end_frame: Option<f32>,
    pub m_animation_interruption_group_names: Option<Vec<u32>>,
    pub m_animation_resource_data: AnimationResourceData,
    pub m_event_data_map: Option<HashMap<u32, EnumEventData>>,
    pub m_flags: Option<u32>,
    pub m_mask_data_name: Option<u32>,
    pub m_sync_group_data_name: Option<u32>,
    pub m_tick_duration: Option<f32>,
    pub m_track_data_name: u32,
    pub m_updater_resource_data: Option<UpdaterResourceData>,
    pub start_frame: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AttackEvents {
    pub roll_for_critical_hit_result: bool,
    pub trigger_launch_attack: bool,
    pub trigger_once_per_enemy_of_parent: bool,
    pub trigger_once_per_parent: bool,
    pub trigger_only_once: bool,
    pub trigger_pre_attack: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AttackSlotData {
    pub m_attack_cast_time: Option<f32>,
    pub m_attack_delay_cast_offset_percent: Option<f32>,
    pub m_attack_delay_cast_offset_percent_attack_speed_ratio: Option<f32>,
    pub m_attack_name: Option<String>,
    pub m_attack_probability: Option<f32>,
    pub m_attack_total_time: Option<f32>,
    pub m_override_autoattack_cast_time: Option<OverrideAutoAttackCastTimeData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AugmentDisplayTagData {
    pub augment_display_tag_frame: u32,
    pub augment_display_tag_row: u32,
    pub augment_display_tag_spacer: u32,
    pub augment_display_tag_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AugmentSlot {
    pub augment_grid_item: u32,
    pub backdrop: u32,
    pub hit_area: u32,
    pub icon: u32,
    pub penetration_recently: Option<u32>,
    pub unk_0x7ee47949: Option<String>,
    pub unk_0xbbebf3d: u32,
    pub unk_0xc0ac6e28: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AugmentSlotData {
    pub augment_button: u32,
    pub augment_description: u32,
    pub augment_display_tag_data: AugmentDisplayTagData,
    pub augment_group: u32,
    pub augment_hover_vfx: u32,
    pub augment_icon_large: u32,
    pub augment_icon_medium: u32,
    pub augment_idle_vfx: u32,
    pub augment_name: u32,
    pub augment_not_picked_vfx: u32,
    pub augment_picked_vfx: u32,
    pub augment_refresh_overlay_vfx: u32,
    pub augment_refresh_vfx: u32,
    pub reroll_button_data: RerollButtonData,
    pub unk_0x344729ae: u32,
    pub unk_0x4ddb7d67: u32,
    pub unk_0x76215b4c: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BankUnit {
    pub asynchrone: Option<bool>,
    pub bank_path: Option<Vec<String>>,
    pub events: Option<Vec<String>>,
    pub name: String,
    pub voice_over: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BarTypeMap {
    pub additional_bar_types: Option<HashMap<u32, u32>>,
    pub default_bar: u32,
    pub min_display_percent_override: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BarracksMinionConfig {
    pub minion_type: u8,
    pub minion_upgrade_stats: MinionUpgradeConfig,
    pub unk_0xfee040bc: u32,
    pub wave_behavior: EnumWaveBehavior,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct BasicSkinAugment {
    pub catalog_entry: CatalogEntry,
    pub m_description_tra_key: Option<String>,
    pub m_name_tra_key: Option<String>,
    pub modifiers: Vec<EnumUnk0x51ada002>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct BattleBunnyMissFortuneViewController {
    pub bunny_element_off_states: Vec<u32>,
    pub bunny_element_on_states: Vec<u32>,
    pub eye_candy: bool,
    pub loadable: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub transition_off_state: u32,
    pub transition_on_state: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct BehaviorLevelController {
    pub guid: String,
    pub level_behaviors: HashMap<u32, EndOfGameCeremonyBehavior>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct BinFileContainer {
    pub filepath_hash: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlendingSwitchMaterialDriver {
    pub m_blend_time: Option<f32>,
    pub m_default_value: Box<EnumDriver>,
    pub m_elements: Vec<Box<SwitchMaterialDriverElement>>,
    pub m_override_blend_times: Option<Vec<f32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BoolGet {
    pub value: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BoolTableGet {
    pub table: Option<EnumVars>,
    pub var: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BoolTableSet {
    pub table: Option<EnumVars>,
    pub var: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BorderPropertyData {
    pub border_path: String,
    pub border_treatment: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct BorderSkinAugment {
    pub border: BorderPropertyData,
    pub catalog_entry: CatalogEntry,
    pub m_name_tra_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BotsSpellData {
    pub damage_tag: Option<u32>,
    pub unk_0x38382c53: Option<Vec<Unk0x150d1b92>>,
    pub unk_0x591f8423: Option<f32>,
    pub unk_0x6d548702: Option<GameCalculation>,
    pub unk_0xec17e271: Option<Vec<Unk0xb09016f6>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BreakBlock {
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Breakpoint {
    pub m_additional_bonus_at_this_level: Option<f32>,
    pub m_bonus_per_level_at_and_after: Option<f32>,
    pub m_level: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastViewController {
    pub base_loadable: u32,
    pub event_league_combo_box_definition: u32,
    pub main_scene: u32,
    pub path_hash_to_self: u64,
    pub select_league_config_combo_box_definition: u32,
    pub select_league_flavor_combo_box_definition: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrowserElementData {
    pub id: String,
    pub instance_data: Vec<BrowserInstanceData>,
    pub preferred_order: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrowserInstanceData {
    pub background_image: TextureOverride,
    pub enabled: bool,
    pub end_date: String,
    pub id: String,
    pub region_data: EnabledRegionData,
    pub title_text_tra: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BuffCounterByCoefficientCalculationPart {
    pub m_buff_name: u32,
    pub m_coefficient: f32,
    pub m_icon_key: Option<String>,
    pub m_scaling_tag_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BuffCounterByNamedDataValueCalculationPart {
    pub m_buff_name: u32,
    pub m_data_value: u32,
    pub m_icon_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BuffCounterDynamicMaterialFloatDriver {
    pub m_script_name: Option<String>,
    pub spell: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct BuffData {
    pub can_timeout_while_casting: Option<bool>,
    pub m_buff_attribute_flag: Option<u8>,
    pub m_description: Option<String>,
    pub m_float_vars_decimals: Option<Vec<i32>>,
    pub m_show_accumulated_duration: Option<bool>,
    pub m_show_duration: Option<bool>,
    pub m_tooltip_data: Option<TooltipInstanceBuff>,
    pub persistent_effect_conditions: Option<Vec<PersistentEffectConditionData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BuffDisplayData {
    pub cooldown_fx: u32,
    pub group: u32,
    pub hit_region: u32,
    pub icon: u32,
    pub icon_border_negative: u32,
    pub icon_border_positive: u32,
    pub stacks_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BuffDisplayList {
    pub buff_display_template: BuffDisplayData,
    pub layout: u32,
    pub max_buff_display_count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct BuffScript {
    pub functions: Option<HashMap<u32, ScriptFunction>>,
    pub global_properties: Option<ScriptGlobalProperties>,
    pub path: u32,
    pub script_name: String,
    pub sequences: Option<HashMap<u32, RootScriptSequence>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ByCharLevelBreakpointsCalculationPart {
    pub m_breakpoints: Option<Vec<Breakpoint>>,
    pub m_initial_bonus_per_level: Option<f32>,
    pub m_level1_value: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ByCharLevelFormulaCalculationPart {
    pub m_values: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ByCharLevelInterpolationCalculationPart {
    pub m_end_value: Option<f32>,
    pub m_scale_by_stat_progression_multiplier: Option<bool>,
    pub m_scale_past_default_max_level: Option<bool>,
    pub m_start_value: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct CandidateListViewController {
    pub backdrop: u32,
    pub base_loadable: u32,
    pub highlight: Vec<u32>,
    pub index: Vec<u32>,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub text: Vec<u32>,
    pub unk_0x2eed7e1b: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cast {
    pub roll_for_critical_hit_result: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CastOnMovementComplete {
    pub roll_for_critical_hit_result: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CatalogEntry {
    pub content_id: String,
    pub item_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CcBehaviorData {
    pub cc_behavior: TargetingPriorityList,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CcScoreMultipliers {
    pub asleep: f32,
    pub blind: f32,
    pub charm: f32,
    pub disarm: f32,
    pub drowsy: f32,
    pub fear: f32,
    pub flee: f32,
    pub grounded: f32,
    pub knock_up: f32,
    pub knockback: f32,
    pub nearsight: f32,
    pub polymorph: f32,
    pub root: f32,
    pub silence: f32,
    pub slow: f32,
    pub stun: f32,
    pub suppression: f32,
    pub taunt: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct CelebrationViewController {
    pub base_loadable: u32,
    pub body_text_handle: u32,
    pub celebration_companion_background_vfx: u32,
    pub celebration_companion_foreground_vfx: u32,
    pub celebration_tier2_star_vfx: u32,
    pub celebration_tier3_star_vfx: u32,
    pub celebration_vignette_asset_vfx: u32,
    pub celebration_vignette_vfx: u32,
    pub claimed_reward_layout: u32,
    pub claimed_reward_template: Unk0x82ece567,
    pub default_rated_tier_tra_key: String,
    pub delayed_icon_wait_time_sec: f32,
    pub egg_tier_vfx: HashMap<u32, u32>,
    pub generic_background_texture: u32,
    pub icon_frame_handle: u32,
    pub icon_handle: u32,
    pub icon_handle_delayed: u32,
    pub path_hash_to_self: u64,
    pub ranked_background_texture: u32,
    pub ranked_tier_vfx: HashMap<u32, u32>,
    pub scene: u32,
    pub subtitle_text_handle: u32,
    pub tablet_override_loadable: u32,
    pub title_text_handle: u32,
    pub unk_0x33b801d7: String,
    pub unk_0xb72f9920: u32,
    pub unk_0xb9524edd: u32,
    pub unk_0xbc4102dc: String,
    pub unk_0xd02a6781: u32,
    pub unk_0xf09752c9: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CensoredImage {
    pub image: String,
    pub uncensored_images: Option<HashMap<u32, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChampionGoldUiData {
    pub champion_gold_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChampionNameUiData {
    pub champion_name_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChampionPerkKeystoneUiData {
    pub keystone_icon: u32,
    pub keystone_substyle_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ChampionRuneRecommendationsContext {}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ChampionSelectionViewController {
    pub base_loadable: u32,
    pub champion_slot_ui_data: Vec<UiChampionSelectionSlotData>,
    pub path_hash_to_self: u64,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ChampionTransformSelectionViewController {
    pub energy_gain_anim: u32,
    pub loadable: u32,
    pub meter_fill: u32,
    pub meter_inner_ring: u32,
    pub meter_outer_ring: u32,
    pub meter_vanish_anim: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub timer_text_assassin: u32,
    pub timer_text_slayer: u32,
    pub transform_button_assassin: u32,
    pub transform_button_slayer: u32,
    pub transform_centerline: u32,
    pub transform_ready_anim: u32,
    pub transform_ready_attention_anim: u32,
    pub transform_ready_idle_assassin_anim: u32,
    pub transform_ready_idle_slayer_anim: u32,
    pub transform_ready_out_anim: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMissileSpeed {
    pub m_speed_change_type: Option<u32>,
    pub m_speed_value: f32,
    pub trigger_only_once: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMissileWidth {
    pub width_change_type: u32,
    pub width_value: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CharacterHealthBarDataRecord {
    pub alpha_out_while_untargetable: Option<bool>,
    pub attach_to_bone: Option<String>,
    pub character_state_indicator_max_count: Option<u32>,
    pub hp_per_tick: Option<f32>,
    pub show_character_state_indicator_to_allies: Option<bool>,
    pub show_character_state_indicator_to_enemies: Option<bool>,
    pub show_while_untargetable: Option<bool>,
    pub unit_health_bar_style: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CharacterInitRequirement {
    pub m_relative_team: u8,
    pub participant_champion: u32,
    pub required_champion: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CharacterLevelRequirement {
    pub m_level: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CharacterPassiveData {
    pub m_allow_on_clones: Option<bool>,
    pub m_child_spells: Option<Vec<u32>>,
    pub m_component_buffs: Option<Vec<u32>>,
    pub m_display_flags: Option<u8>,
    pub m_parent_passive_buff: u32,
    pub skin_filter: Option<SkinFilterData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CharacterQuestDefinitionsData {
    pub is_enabled: bool,
    pub quest_name: String,
    pub unk_0x869c3e32: Vec<Unk0x7b40445f>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct CharacterQuestListConfig {
    pub character_quest_definitions_list: Vec<CharacterQuestDefinitionsData>,
    pub character_quest_list: Vec<Unk0xe2ff8b22>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CharacterQuestObjective {
    pub objective_name: u32,
    pub objective_reward_list: Vec<CharacterQuestReward>,
    pub unk_0x42cd1140: Vec<u32>,
    pub unk_0x856d8176: Vec<Unk0xa495afda>,
    pub unk_0xcbca11f6: Option<Vec<CharacterQuestReward>>,
    pub unk_0xf7c78187: Vec<Unk0xa495afda>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CharacterQuestReward {
    pub reward_recipient: u32,
    pub spell: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct CharacterRecord {
    pub acquisition_range: Option<f32>,
    pub ally_champ_specific_health_suffix: Option<String>,
    pub area_indicator_max_distance: Option<f32>,
    pub area_indicator_min_distance: Option<f32>,
    pub area_indicator_min_radius: Option<f32>,
    pub area_indicator_radius: Option<f32>,
    pub area_indicator_target_distance: Option<f32>,
    pub area_indicator_texture_size: Option<f32>,
    pub armor_per_level: Option<f32>,
    pub attack_auto_interrupt_percent: Option<f32>,
    pub attack_range: Option<f32>,
    pub attack_speed: Option<f32>,
    pub attack_speed_per_level: Option<f32>,
    pub attack_speed_ratio: Option<f32>,
    pub base_armor: Option<f32>,
    pub base_crit_chance: Option<f32>,
    pub base_damage: Option<f32>,
    pub base_factor_hp_regen: Option<f32>,
    pub base_hp: Option<f32>,
    pub base_move_speed: Option<f32>,
    pub base_spell_block: Option<f32>,
    pub base_static_hp_regen: Option<f32>,
    pub basic_attack: Option<AttackSlotData>,
    pub char_audio_name_override: Option<String>,
    pub character_tool_data: Option<CharacterToolData>,
    pub crit_attacks: Option<Vec<AttackSlotData>>,
    pub crit_damage_multiplier: Option<f32>,
    pub crit_per_level: Option<f32>,
    pub critical_attack: Option<String>,
    pub damage_per_level: Option<f32>,
    pub death_event_listening_radius: Option<f32>,
    pub death_time: Option<f32>,
    pub disabled_target_laser_effects: Option<TargetLaserComponentEffects>,
    pub disguise_minimap_icon_override: Option<String>,
    pub enemy_champ_specific_health_suffix: Option<String>,
    pub enemy_tooltip: Option<String>,
    pub evolution_data: Option<EvolutionDescription>,
    pub exp_given_on_death: Option<f32>,
    pub experience_radius: Option<f32>,
    pub extra_attacks: Option<Vec<AttackSlotData>>,
    pub extra_spells: Option<Vec<String>>,
    pub first_acquisition_range: Option<f32>,
    pub flags: Option<u32>,
    pub friendly_tooltip: Option<String>,
    pub friendly_ux_override_exclude_tags_string: Option<String>,
    pub friendly_ux_override_include_tags_string: Option<String>,
    pub friendly_ux_override_team: Option<u32>,
    pub global_exp_given_on_death: Option<f32>,
    pub global_gold_given_on_death: Option<f32>,
    pub gold_given_on_death: Option<f32>,
    pub gold_radius: Option<f32>,
    pub health_bar_full_parallax: Option<bool>,
    pub health_bar_height: Option<f32>,
    pub highlight_healthbar_icons: Option<bool>,
    pub hit_fx_scale: Option<f32>,
    pub hover_indicator_minimap_override: Option<String>,
    pub hover_indicator_radius: Option<f32>,
    pub hover_indicator_radius_minimap: Option<f32>,
    pub hover_indicator_rotate_to_player: Option<bool>,
    pub hover_indicator_texture_name: Option<String>,
    pub hover_line_indicator_base_texture_name: Option<String>,
    pub hover_line_indicator_target_texture_name: Option<String>,
    pub hover_line_indicator_width: Option<f32>,
    pub hover_line_indicator_width_minimap: Option<f32>,
    pub hp_per_level: Option<f32>,
    pub hp_regen_per_level: Option<f32>,
    pub joint_for_anim_adjusted_selection: Option<String>,
    pub launch_area_data: Option<LaunchAreaData>,
    pub local_exp_given_on_death: Option<f32>,
    pub local_gold_given_on_death: Option<f32>,
    pub local_gold_split_with_last_hitter: Option<bool>,
    pub m_abilities: Option<Vec<u32>>,
    pub m_ability_slot_cc: Option<Vec<i32>>,
    pub m_adaptive_force_to_ability_power_weight: Option<f32>,
    pub m_character_calculations: Option<HashMap<u32, GameCalculation>>,
    pub m_character_name: String,
    pub m_character_passive_buffs: Option<Vec<CharacterPassiveData>>,
    pub m_character_passive_spell: Option<u32>,
    pub m_client_side_item_inventory: Option<Vec<u32>>,
    pub m_education_tool_data: Option<ToolEducationData>,
    pub m_fallback_character_name: Option<String>,
    pub m_perk_replacements: Option<PerkReplacementList>,
    pub m_preferred_perk_style: Option<u32>,
    pub m_use_cc_animations: Option<bool>,
    pub minimap_icon_override: Option<String>,
    pub minion_flags: Option<u32>,
    pub minion_score_value: Option<f32>,
    pub name: Option<String>,
    pub occluded_unit_selectable_distance: Option<f32>,
    pub on_kill_event: Option<u32>,
    pub on_kill_event_for_spectator: Option<u32>,
    pub on_kill_event_steal: Option<u32>,
    pub outline_b_box_expansion: Option<f32>,
    pub override_gameplay_collision_radius: Option<f32>,
    pub pack_manager_data: Option<PackManagerData>,
    pub par_name: Option<String>,
    pub passive1_icon_name: Option<String>,
    pub passive_lua_name: Option<String>,
    pub passive_name: Option<String>,
    pub passive_range: Option<f32>,
    pub passive_spell: Option<String>,
    pub passive_tool_tip: Option<String>,
    pub pathfinding_collision_radius: Option<f32>,
    pub perception_bounding_box_size: Option<Vec3>,
    pub perception_bubble_radius: Option<f32>,
    pub platform_enabled: Option<bool>,
    pub primary_ability_resource: Option<AbilityResourceSlotInfo>,
    pub purchase_identities: Option<Vec<u32>>,
    pub rec_spell_rank_up_info_list: Option<RecSpellRankUpInfoList>,
    pub record_as_ward: Option<bool>,
    pub secondary_ability_resource: Option<AbilityResourceSlotInfo>,
    pub selection_height: Option<f32>,
    pub selection_radius: Option<f32>,
    pub self_cb_champ_specific_health_suffix: Option<String>,
    pub self_champ_specific_health_suffix: Option<String>,
    pub significance: Option<f32>,
    pub silhouette_attachment_anim: Option<String>,
    pub spell_block_per_level: Option<f32>,
    pub spell_level_up_info: Option<Vec<SpellLevelUpInfo>>,
    pub spell_names: Option<Vec<String>>,
    pub spells: Option<Vec<u32>>,
    pub target_laser_effects: Option<TargetLaserComponentEffects>,
    pub tower_targeting_priority_boost: Option<f32>,
    pub treat_auto_attacks_as_normal_spells: Option<TreatAutoAttacksAsNormalSpells>,
    pub unit_tags_string: Option<String>,
    pub unk_0x3f975e4a: Option<bool>,
    pub unk_0x43135375: Option<f32>,
    pub unk_0x6854087e: Option<Vec<Unk0x47f13ab0>>,
    pub unk_0x9836cd87: Option<u8>,
    pub unk_0xc1984296: Option<Vec<u32>>,
    pub unk_0xc5c48b41: Option<u8>,
    pub unk_0xdd661aab: Option<Unk0x280745b1>,
    pub untargetable_spawn_time: Option<f32>,
    pub use_riot_relationships: Option<bool>,
    pub useable_data: Option<UseableData>,
    pub wake_up_range: Option<f32>,
    pub weapon_materials: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CharacterToolData {
    pub alternate_forms: Option<Vec<ToolAlternateForm>>,
    pub attack_rank: Option<i32>,
    pub attack_speed: Option<f32>,
    pub base_attack_speed_bonus: Option<f32>,
    pub base_spell_effectiveness: Option<f32>,
    pub bot_default_spell1: Option<String>,
    pub bot_default_spell2: Option<String>,
    pub bot_enabled: Option<bool>,
    pub bot_enabled_mm: Option<bool>,
    pub cast_shadows: Option<bool>,
    pub champion_id: Option<i32>,
    pub chasing_attack_range_percent: Option<f32>,
    pub classification: Option<String>,
    pub defense_rank: Option<i32>,
    pub description: Option<String>,
    pub difficulty_rank: Option<i32>,
    pub inherits: Option<ToolInheritsData>,
    pub level_spell_effectiveness: Option<f32>,
    pub lore2: Option<String>,
    pub magic_rank: Option<i32>,
    pub map_ai_presence: Option<HashMap<u32, ToolAiPresence>>,
    pub par_fade_color: Option<String>,
    pub pass_lev1_desc: Option<Vec<String>>,
    pub passive_data: Option<Vec<ToolPassiveData>>,
    pub post_attack_move_delay: Option<f32>,
    pub rec_items: Option<Vec<String>>,
    pub roles: Option<String>,
    pub search_tags: Option<String>,
    pub search_tags_secondary: Option<String>,
    pub soul_given_on_death: Option<f32>,
    pub sound: Option<ToolSoundData>,
    pub spell_data: Option<Vec<ToolSpellDesc>>,
    pub tips3: Option<String>,
    pub tutorial_rec_items: Option<Vec<String>>,
    pub unk_0xaa75da9d: Option<bool>,
    pub weapon_material_crit: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatChannelSelectionComboBoxDefinition {
    pub all_chat_channel_tra: String,
    pub chat_channel_option_list_backdrop: u32,
    pub chat_channel_option_list_hover: u32,
    pub combo_box_header_button: u32,
    pub combo_box_header_text: u32,
    pub option_reference_region: u32,
    pub option_reference_text: u32,
    pub party_chat_channel_tra: String,
    pub team_chat_channel_tra: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ChatOnlyAnnouncementDefinitionDeprecated {
    pub default_data: AnnouncementDefinitionData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ChatViewController {
    pub base_loadable: u32,
    pub chat_frame_bounds: u32,
    pub chat_hud_restriction_text: u32,
    pub chat_hud_text: u32,
    pub chat_hud_text_background: u32,
    pub chat_input_center: u32,
    pub chat_input_indicator_tooltips: u32,
    pub chat_text_edit: u32,
    pub combo_box_definition: ChatChannelSelectionComboBoxDefinition,
    pub expanded_chat_element: u32,
    pub path_hash_to_self: u64,
    pub scene_chat_channel_select_view: u32,
    pub scene_chat_text_area: u32,
    pub scene_chat_text_edit_bg: u32,
    pub view_pane_link: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct CherryAugmentSelectionViewController {
    pub augment_selection_grid: u32,
    pub augment_slot_data: AugmentSlotData,
    pub base_loadable: u32,
    pub main_scene: u32,
    pub path_hash_to_self: u64,
    pub timeout_prevent_clicks_on_show: f32,
    pub unk_0x2f50e99b: Unk0x67d3ab82,
    pub unk_0x6da1a863: Unk0xfb1989a3,
    pub unk_0xce64b8ab: Unk0xfb1989a3,
    pub unk_0xf8d5ccda: Unk0xfb1989a3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CherryCombatInfoDisplayTeam {
    pub background: u32,
    pub champion_layout: u32,
    pub fill_meter: u32,
    pub team_hit_region: u32,
    pub team_icon: u32,
    pub team_tooltip_anchor: u32,
    pub unk_0x848de224: u32,
    pub unk_0xc7b47c84: u32,
    pub unk_0xe51c9c6c: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct CherryCombatInfoDisplayViewController {
    pub base_loadable: u32,
    pub champion_template: Unk0x10df77cc,
    pub enemy_life_health_text_color: [u8; 4],
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub self_life_health_text_color: [u8; 4],
    pub team_left_display: CherryCombatInfoDisplayTeam,
    pub team_right_display: CherryCombatInfoDisplayTeam,
    pub unk_0x28c91c47: u32,
    pub unk_0x493e057d: String,
    pub unk_0xb620637e: u32,
    pub unk_0xc0f962db: f32,
    pub unk_0xc7463e46: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct CherryEliminationViewController {
    pub background: u32,
    pub base_loadable: u32,
    pub elimination_highlight_vfx: u32,
    pub leave_button: u32,
    pub path_hash_to_self: u64,
    pub rank_text: u32,
    pub scene: u32,
    pub spectate_button: u32,
    pub title_text: u32,
    pub unk_0x156acc5e: f32,
    pub unk_0x94cd2b4b: u32,
    pub unk_0xd83858e8: String,
    pub victory_highlight_vfx: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct CherryRoundsViewController {
    pub base_loadable: u32,
    pub current_phase_icon: Unk0x9784901f,
    pub left_phase_icons: Vec<Unk0x9784901f>,
    pub path_hash_to_self: u64,
    pub right_phase_icons: Vec<Unk0x9784901f>,
    pub round_label: u32,
    pub round_label_tra: String,
    pub scene: u32,
    pub timer_left_bar: Unk0x5b5e6994,
    pub timer_right_bar: Unk0x5b5e6994,
    pub timer_text: u32,
    pub timer_text_default_color: [u8; 4],
    pub tooltip_anchor: u32,
    pub unk_0x3aa6852c: u32,
    pub unk_0xb03d7e4b: [u8; 4],
    pub unk_0xb1f34e3f: Unk0xf2dd2d14,
    pub unk_0xcf3fe190: u32,
    pub unk_0xd24a0877: HashMap<u8, String>,
    pub unk_0xe527f39d: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct CherryScoreboardViewController {
    pub base_loadable: u32,
    pub main_tooltip_anchor: u32,
    pub matchup_history_data: Unk0x8444401a,
    pub path_hash_to_self: u64,
    pub player_slot_height_reference: u32,
    pub report_modal_anchor: u32,
    pub score_line_ui: ScoreLineCherryUiData,
    pub scoreboard_scene: u32,
    pub summoner_social_card: UiSummonerSocialCardData,
    pub team_data: Vec<Unk0x69057401>,
    pub unk_0x31d7f734: HashMap<u8, u32>,
    pub unk_0xb1f34e3f: Unk0xf2dd2d14,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CherrySpectateMatchDialogText {
    pub unk_0x14810744: String,
    pub unk_0x270bd9c2: String,
    pub unk_0x9e7b5cd7: String,
    pub unk_0xd9e1766a: String,
    pub unk_0xdecfb3d8: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct CherrySpectateMatchDialogViewController {
    pub base_loadable: u32,
    pub dialog_text: CherrySpectateMatchDialogText,
    pub match_grid: u32,
    pub match_template: Unk0x77fb37dd,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub title_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CherryTeamFlyoutPanelFightTeamFrame {
    pub fight_team_background: IconStateData,
    pub opponent_icon: u32,
    pub unk_0x100be200: u32,
    pub unk_0xbb99c3fc: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CherryTeamFlyoutPanelTeamFrame {
    pub critical_state: IconStateData,
    pub default_state: IconStateData,
    pub eliminated_state: IconStateData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CherryTeamFlyoutPanelTeamTemplate {
    pub bounds: u32,
    pub champion_layout: u32,
    pub champion_scene: u32,
    pub champion_template: CherryTeamFlyoutPanelTeamTemplateChampion,
    pub default_fill_meter_color: [u8; 4],
    pub default_health_text_color: [u8; 4],
    pub fight_team_frame: CherryTeamFlyoutPanelFightTeamFrame,
    pub fill_meter: u32,
    pub frames: CherryTeamFlyoutPanelTeamFrame,
    pub health_text: u32,
    pub health_text_scene: u32,
    pub hover_glow: Unk0xcc2c0827,
    pub scene: u32,
    pub team_button: u32,
    pub team_champions_button: u32,
    pub team_icon: u32,
    pub unk_0x337254f3: IconStateData,
    pub unk_0x4ea12677: IconStateData,
    pub unk_0x5388a5b: u32,
    pub unk_0xa4577e5a: Unk0x6a68b4f1,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CherryTeamFlyoutPanelTeamTemplateChampion {
    pub portrait_frames: Unk0xc7ca4925,
    pub portrait_grid_item: u32,
    pub portrait_icon: u32,
    pub portrait_reference: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct CherryTeamFlyoutPanelViewController {
    pub background_frame: u32,
    pub base_loadable: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub team_opponent_template: CherryTeamFlyoutPanelTeamTemplate,
    pub team_self_template: CherryTeamFlyoutPanelTeamTemplate,
    pub unk_0x1bb70a95: f32,
    pub unk_0x93a6a62b: Unk0x393df345,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CherryUiPlayerPortraitData {
    pub frame: u32,
    pub portrait_icon: u32,
    pub portrait_uv: Vec4,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ChildMapVisibilityController {
    pub parent_mode: Option<u32>,
    pub parents: Vec<u32>,
    pub path_hash: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct CinematicBarsViewController {
    pub base_loadable: u32,
    pub bottom_black_bar: u32,
    pub bottom_scene: u32,
    pub left_black_bar: u32,
    pub left_scene: u32,
    pub path_hash_to_self: u64,
    pub right_black_bar: u32,
    pub right_scene: u32,
    pub top_black_bar: u32,
    pub top_scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CircleMovement {
    pub m_angular_velocity: Option<f32>,
    pub m_lifetime: f32,
    pub m_linear_velocity: Option<f32>,
    pub m_start_bone_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClampSubPartsCalculationPart {
    pub m_ceiling: Option<f32>,
    pub m_floor: Option<f32>,
    pub m_subparts: Vec<Box<EnumAbilityResourceByCoefficientCalculationPart>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ClashLogo {
    pub m_clash_logo_color_id: Option<u32>,
    pub m_clash_logo_id: Option<u32>,
    pub m_logo_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearTargetAndKeepMoving {
    pub let_server_drive_target_position: Option<bool>,
    pub m_override_height_augment: Option<f32>,
    pub m_override_movement: Option<FixedSpeedMovement>,
    pub m_override_range: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ClientStateAudioDataProperties {
    pub bank_paths: Vec<String>,
    pub theme_music: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ClientStateCommonSettings {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ColorChooserMaterialDriver {
    pub m_bool_driver: Box<EnumDriver>,
    pub m_color_off: Option<Vec4>,
    pub m_color_on: Option<Vec4>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ColorGraphMaterialDriver {
    pub colors: VfxAnimatedColorVariableData,
    pub driver: Box<EnumDriver>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct CommonUiTunables {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompanionLoadoutGridButtonData {
    pub check: u32,
    pub favorite_icon: u32,
    pub game_pass: u32,
    pub group: u32,
    pub icon: u32,
    pub lock: u32,
    pub unk_0xd02a6781: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ComparisonScriptCondition {
    pub operation: Option<u32>,
    pub value1: Box<EnumAddLevelTimer>,
    pub value2: Box<EnumAddLevelTimer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConcatenateStringsBlock {
    pub is_disabled: Option<bool>,
    pub result: StringTableSet,
    pub string1: Box<EnumAddLevelTimer>,
    pub string2: Box<EnumAddLevelTimer>,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConditionBoolClipData {
    pub dont_stomp_transition_clip: Option<bool>,
    pub m_change_animation_mid_play: Option<bool>,
    pub m_child_anim_delay_switch_time: Option<f32>,
    pub m_false_condition_clip_name: u32,
    pub m_flags: Option<u32>,
    pub m_play_anim_change_from_beginning: Option<bool>,
    pub m_true_condition_clip_name: u32,
    pub sync_frame_on_change_anim: Option<bool>,
    pub updater: EnumParametricUpdater,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConditionFloatClipData {
    pub dont_stomp_transition_clip: Option<bool>,
    pub m_change_animation_mid_play: Option<bool>,
    pub m_child_anim_delay_switch_time: Option<f32>,
    pub m_condition_float_pair_data_list: Vec<ConditionFloatPairData>,
    pub m_flags: Option<u32>,
    pub m_play_anim_change_from_beginning: Option<bool>,
    pub sync_frame_on_change_anim: Option<bool>,
    pub updater: EnumParametricUpdater,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConditionFloatPairData {
    pub m_clip_name: u32,
    pub m_hold_animation_to_higher: Option<f32>,
    pub m_hold_animation_to_lower: Option<f32>,
    pub m_value: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConformToPathEventData {
    pub m_blend_in_time: Option<f32>,
    pub m_blend_out_time: Option<f32>,
    pub m_end_frame: Option<f32>,
    pub m_fire_if_animation_ends_early: Option<bool>,
    pub m_is_self_only: Option<bool>,
    pub m_mask_data_name: u32,
    pub m_name: Option<u32>,
    pub m_start_frame: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConformToPathRigPoseModifierData {
    pub activation_angle: Option<f32>,
    pub activation_distance: Option<f32>,
    pub blend_distance: Option<f32>,
    pub m_damping_value: Option<f32>,
    pub m_default_mask_name: Option<u32>,
    pub m_ending_joint_name: u32,
    pub m_frequency: Option<f32>,
    pub m_max_bone_angle: Option<f32>,
    pub m_starting_joint_name: u32,
    pub m_vel_multiplier: Option<f32>,
    pub only_activate_in_turns: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConstantWaveBehavior {
    pub spawn_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualActionCooldownModifications {
    pub dont_reset_timer: Option<bool>,
    pub ignore_timer: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ContextualActionData {
    pub m_cooldown: Option<f32>,
    pub m_health_percent_threshold: Option<f32>,
    pub m_object_path: String,
    pub m_situations: Option<HashMap<u32, ContextualSituation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualActionPlayAnimation {
    pub m_hashed_animation_name: Option<u32>,
    pub m_hashed_situation_trigger: Option<u32>,
    pub m_play_as_emote: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualActionPlayVo {
    pub m_ally_event_name: Option<String>,
    pub m_enemy_event_name: Option<String>,
    pub m_max_occurences: Option<u32>,
    pub m_self_event_name: Option<String>,
    pub m_spectator_event_name: Option<String>,
    pub m_wait_for_announcer_queue: Option<bool>,
    pub m_wait_timeout: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualActionTriggerEvent {
    pub m_hashed_situation_trigger: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionAmountHealed {
    pub amount_healed: f32,
    pub compare_op: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionAnyOtherHero {
    pub m_child_conditions: Vec<Box<EnumContextualCondition>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionBuffCounterChanged {
    pub buff: u32,
    pub unk_0x395e4a90: Option<bool>,
    pub unk_0xfa24eccf: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionBuffCounterReachedLimitFromZero {
    pub compare_op: Option<u8>,
    pub m_buff: u32,
    pub m_counter_highest_reached: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionBuffCounterSetToZeroAfterLimitReached {
    pub m_buff: u32,
    pub m_counter_highest_reached: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionCastTarget {
    pub is_ally: Option<bool>,
    pub is_enemy: Option<bool>,
    pub is_self: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionChanceToPlay {
    pub m_percent_chance_to_play: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionCharacter {
    pub m_character_type: Option<u8>,
    pub m_child_conditions: Option<Vec<Box<EnumContextualCondition>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionCharacterDistance {
    pub m_compare_op: Option<u8>,
    pub m_distance: Option<f32>,
    pub m_distance_target: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionCharacterHasCac {
    pub m_cacs: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionCharacterHasTimeRemainingForAnimation {
    pub m_animation_name: u32,
    pub m_min_remaining_time: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionCharacterHealth {
    pub m_compare_op: u8,
    pub m_target_health: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionCharacterLevel {
    pub m_character_level: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionCharacterMetadata {
    pub m_category: String,
    pub m_data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionCharacterName {
    pub m_characters: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionCharacterPlayingAnimation {
    pub m_animation_name_hash: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionCharacterSkinId {
    pub m_skin_i_ds: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionCharacterUnitTags {
    pub m_unit_tags: Option<ObjectTags>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionCustomTimer {
    pub m_custom_timer: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionGameTimer {
    pub m_compare_op: Option<u8>,
    pub m_game_time_in_minutes: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionGlobalObjectiveBountyForTeam {
    pub team_compare_op: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionGlobalObjectiveBountyState {
    pub target_global_bounty_state: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionHasGold {
    pub m_gold: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionIsAlly {
    pub m_is_ally: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionIsSelf {
    pub is_self: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionItemId {
    pub m_items: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionItemPriceMinimum {
    pub m_item_price_minimum: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionItemVoGroup {
    pub m_item_vo_group_hash: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionKillCount {
    pub m_total_kills: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionMapId {
    pub m_map_i_ds: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionMapRegionName {
    pub m_region_name: String,
    pub m_region_type: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionMarkerName {
    pub m_marker_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionMoveDistance {
    pub m_compare_op: u8,
    pub m_distance: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionMultikillSize {
    pub m_multikill_size: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionNearbyChampionCount {
    pub m_compare_op: u8,
    pub m_count: u32,
    pub m_team_compare_op: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionNegation {
    pub m_child_condition: Box<EnumContextualCondition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionNeutralCampId {
    pub m_camp_id: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionNumberOfCharactersNearTargetPos {
    pub m_team_compare_op: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionObjectiveTakeByMyTeam {
    pub m_taken_objective: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionOwnerTeamNetChampionKills {
    pub m_kill_advantage_compare_op: Option<u8>,
    pub m_owner_team_net_kill_advantage: Option<i32>,
    pub m_time_frame_seconds: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionPreviousMapRegionName {
    pub m_region_name: String,
    pub m_region_type: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionRuleCooldown {
    pub m_rule_cooldown: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionShopOpenCount {
    pub m_shop_open_count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionSpell {
    pub m_child_conditions: Option<Vec<Box<EnumContextualCondition>>>,
    pub m_spell_slot: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionSpellBuffName {
    pub spell_buff: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionSpellIsReady {
    pub m_spell_is_ready: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionSpellLevel {
    pub m_spell_level: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionSpellName {
    pub m_spell: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionSpellSlot {
    pub m_spell_slot: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionTeammateDeathsNearby {
    pub m_teammate_deaths: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionTimeSinceStealthStateChange {
    pub m_time_threshold: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualConditionWinningTeam {
    pub is_same_team: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualRule {
    pub can_stomp_self: Option<bool>,
    pub chance_weight: Option<f32>,
    pub cooldown_modifications: Option<ContextualActionCooldownModifications>,
    pub m_animation_action: Option<ContextualActionPlayAnimation>,
    pub m_audio_action: Option<ContextualActionPlayVo>,
    pub m_condition_relationship: Option<u32>,
    pub m_conditions: Option<Vec<EnumContextualCondition>>,
    pub m_override_cac_cooldown: Option<bool>,
    pub m_priority: Option<i32>,
    pub m_rule_name: Option<String>,
    pub m_trigger_event_action: Option<ContextualActionTriggerEvent>,
    pub stomp_lower_priority: Option<bool>,
    pub unk_0x20749c51: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextualSituation {
    pub m_choose_random_valid_rule: Option<bool>,
    pub m_cool_down_time: Option<f32>,
    pub m_rules: Option<Vec<ContextualRule>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CooldownEffectUiData {
    pub cooldown_complete_effect: Option<u32>,
    pub cooldown_jump_effect: Option<u32>,
    pub cooldown_text: Option<u32>,
    pub radial_effect: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CooldownGemUiData {
    pub ally_gem: u32,
    pub cooldown_effects: CooldownEffectUiData,
    pub enemy_gem: Option<u32>,
    pub gem_background: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CopyCustomTableBlock {
    pub out_table: CustomTableSet,
    pub table: CustomTableGet,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateCustomTableBlock {
    pub dest: CustomTableSet,
    pub is_disabled: Option<bool>,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateNeutralCamp {
    pub camp_level: u16,
    pub facing_location: VectorTableGet,
    pub initial_spawn_time: FloatTableGet,
    pub is_disabled: Option<bool>,
    pub location: VectorTableGet,
    pub minimap_icon: String,
    pub minimap_icon_offset: VectorGet,
    pub respawn_delay_secs: FloatTableGet,
    pub reveal_event: u16,
    pub scoreboard_timer: u16,
    pub spawn_behavior: CustomNeutralCampSpawnBehavior,
    pub spawn_duration_secs: FloatGet,
    pub stop_spawn_time_secs: FloatGet,
    pub unk_0x5a4ef4e7: Unk0x5b4f4df6,
    pub unk_0x7d27af7f: bool,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CssIcon {
    pub texture: String,
    pub y_adjustment: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct CssSheet {
    pub icons: Option<HashMap<String, CssIcon>>,
    pub path_hash_to_self: u64,
    pub styles: HashMap<String, CssStyle>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CssStyle {
    pub bold: Option<bool>,
    pub color: Option<[u8; 4]>,
    pub italics: Option<bool>,
    pub underline: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct CursorConfig {
    pub m_hover_not_useable_cursor: CursorDataCaptureCooldownContext,
    pub m_single_context_cursors: Vec<CursorData>,
    pub m_team_context_cursors: Vec<CursorDataTeamContext>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CursorData {
    pub m_colorblind_target_champions_only_texture_name: Option<String>,
    pub m_colorblind_texture_name: Option<String>,
    pub m_hot_spot: Option<Vec2>,
    pub m_target_champions_only_texture_name: Option<String>,
    pub m_texture_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CursorDataCaptureCooldownContext {
    pub m_data: Vec<CursorData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CursorDataTeamContext {
    pub m_data: Vec<CursorData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurveTheDifferenceHeightSolver {
    pub m_initial_target_height_offset: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomAbilitiesUiData {
    pub extra_abilities: HashMap<u32, SpellSlotDetailedUiDefinition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomNeutralCampSpawnBehavior {
    pub callback: LevelScriptFunctionLink,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomTableContainsValueBlock {
    pub matching_key: ScriptTableSet,
    pub src_table: CustomTableGet,
    pub unk_0xd851ffa3: String,
    pub value: ScriptTableGet,
    pub was_found: BoolTableSet,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomTableGet {
    pub table: Option<EnumVars>,
    pub var: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomTableSet {
    pub table: Option<EnumVars>,
    pub var: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomTargeterDefinitions {
    pub m_targeter_definitions: Vec<EnumTargeterDefinition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DamageSkinLoadoutGridButtonData {
    pub check: u32,
    pub favorite_icon: u32,
    pub game_pass: u32,
    pub group: u32,
    pub icon: u32,
    pub lock: u32,
    pub unk_0xd02a6781: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct DeathTimes {
    pub m_scaling_increment_time: u32,
    pub m_scaling_percent_cap: f32,
    pub m_scaling_percent_increase: Option<f32>,
    pub m_scaling_points: Vec<DeathTimesScalingPoint>,
    pub m_scaling_start_time: u32,
    pub m_time_dead_per_level: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeathTimesScalingPoint {
    pub m_percent_increase: f32,
    pub m_start_time: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DebugPrintToChatBlock {
    pub color: Option<[u8; 4]>,
    pub is_disabled: Option<bool>,
    pub src: Option<Box<EnumAddLevelTimer>>,
    pub to_say: Option<StringGet>,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecelToLocationMovement {
    pub m_acceleration: f32,
    pub m_initial_speed: f32,
    pub m_max_speed: f32,
    pub m_min_speed: f32,
    pub m_project_target_to_cast_range: bool,
    pub m_start_bone_name: Option<String>,
    pub m_target_bone_name: Option<String>,
    pub m_target_height_augment: Option<f32>,
    pub m_tracks_target: bool,
    pub m_use_height_offset_at_end: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Defaultvisibility {
    pub m_perception_bubble_radius: Option<f32>,
    pub m_target_controls_visibility: Option<bool>,
    pub m_visible_to_owner_team_only: Option<bool>,
    pub trail_time_to_consider_for_visibility: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DelayStart {
    pub m_delay_time: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DelayedBoolMaterialDriver {
    pub m_bool_driver: Box<EnumDriver>,
    pub m_delay_off: Option<f32>,
    pub m_delay_on: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct DesignerEvent {
    pub event_parameters: Option<HashMap<String, ScriptTableGet>>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DestroyCustomTableBlock {
    pub table_name: CustomTableGet,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DestroyOnMovementComplete {
    pub m_delay: Option<i32>,
    pub render_particles_after_destroy: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DetailedItemSlots {
    pub item0: ItemSlotDetailedUiData,
    pub item1: ItemSlotDetailedUiData,
    pub item2: ItemSlotDetailedUiData,
    pub item3: ItemSlotDetailedUiData,
    pub item4: ItemSlotDetailedUiData,
    pub item5: ItemSlotDetailedUiData,
    pub item6: ItemSlotDetailedUiData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct DirectedTrackingElementViewController {
    pub animated_rotating_arrow: u32,
    pub arrow_anchor_base: u32,
    pub glowing_rotating_arrow: u32,
    pub loadable: u32,
    pub path_hash_to_self: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisplayBasicStatUiData {
    pub basic_group: u32,
    pub basic_hover_region: u32,
    pub basic_icon: u32,
    pub basic_stat_amount: u32,
    pub basic_stat_name: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisplayStatUiData {
    pub is_detailed_stat: Option<bool>,
    pub new_stat_type: Option<u8>,
    pub stat_name_tra: String,
    pub stat_type: Option<u8>,
    pub texture_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisplayStatsUiData {
    pub basic_layout: u32,
    pub basic_stat_ui: DisplayBasicStatUiData,
    pub detailed_layout: u32,
    pub detailed_stat_ui: Unk0xee28fb8d,
    pub display_stats: Vec<DisplayStatUiData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DistanceToPlayerMaterialFloatDriver {
    pub max_distance: f32,
    pub min_distance: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DoubleSidedTipStyle {
    pub directional_tip_elements: Vec<u32>,
    pub reverse_directional_tip_elements: Vec<u32>,
    pub sliver: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DrawAreaList {
    pub draw_regions: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DrawablePositionLocator {
    pub angle_offset_radian: Option<f32>,
    pub base_position: Option<u32>,
    pub distance_offset: Option<f32>,
    pub orientation_type: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DynamicMaterialDef {
    pub parameters: Option<Vec<DynamicMaterialParameterDef>>,
    pub static_switch: Option<DynamicMaterialStaticSwitch>,
    pub textures: Option<Vec<DynamicMaterialTextureSwapDef>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DynamicMaterialParameterDef {
    pub driver: EnumDriver,
    pub enabled: Option<bool>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DynamicMaterialStaticSwitch {
    pub driver: EnumDriver,
    pub enabled: Option<bool>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DynamicMaterialTextureSwapDef {
    pub enabled: Option<bool>,
    pub name: String,
    pub options: Option<Vec<DynamicMaterialTextureSwapOption>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DynamicMaterialTextureSwapOption {
    pub driver: EnumDriver,
    pub texture_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ESportLeagueEntry {
    pub league_name: String,
    pub texture_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ESportTeamEntry {
    pub league_name: Option<String>,
    pub team_name: String,
    pub texture_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EffectValueCalculationPart {
    pub m_effect_index: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ElementalSelectionViewController {
    pub anim_scene: u32,
    pub close_button: u32,
    pub element_colors: Vec<[u8; 4]>,
    pub element_icons_full_buttons: Vec<u32>,
    pub element_icons_small: Vec<u32>,
    pub element_meters: Vec<u32>,
    pub element_transform_buttons: Vec<u32>,
    pub fading_scene: u32,
    pub final_form_left_fx: u32,
    pub final_form_right_fx: u32,
    pub first_element_selected_animations: Vec<u32>,
    pub first_element_selected_delayed_animations: Vec<u32>,
    pub glowing_ring: u32,
    pub innerring: u32,
    pub loadable: u32,
    pub meter_full_animations: Vec<u32>,
    pub meter_scene: u32,
    pub path_hash_to_self: u64,
    pub second_element_selected_animations: Vec<u32>,
    pub second_element_selected_delayed_animations: Vec<u32>,
    pub thick_outer_ring: u32,
    pub transform_scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmoteConfigData {
    pub fade_transision_time: f32,
    pub floating_distance_per_second: f32,
    pub height_multiplier: f32,
    pub initial_alpha: f32,
    pub respoition_movement_time: f32,
    pub ui_sound: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmoteItemData {
    pub frame: u32,
    pub group: u32,
    pub icon: u32,
    pub intro: u32,
    pub portrait: u32,
    pub portrait_frame: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmoteLoadoutGridButtonData {
    pub check: u32,
    pub game_pass: u32,
    pub group: u32,
    pub icon: u32,
    pub lock: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct EmoteRadialViewController {
    pub active_wheel_scene: u32,
    pub background_cooldown_effect: u32,
    pub base_loadable: u32,
    pub emote_icons: Vec<u32>,
    pub minimap_active_wheel_scene: u32,
    pub minimap_background_cooldown_effect: u32,
    pub minimap_radial_buttons: Vec<RadialMenuButtonDefinitionBase>,
    pub override_center_button_region: Option<u32>,
    pub path_hash_to_self: u64,
    pub radial_buttons: Vec<RadialMenuHoverButtonDefinition>,
    pub selection_rotating_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct EmotesViewController {
    pub base_loadable: u32,
    pub emote_data: EmoteItemData,
    pub emote_settings: EmoteConfigData,
    pub main_scene: u32,
    pub path_hash_to_self: u64,
    pub top_region: u32,
    pub unk_0x40c8d1c2: u32,
    pub unk_0x949bbb90: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnableLookAtEventData {
    pub m_enable_look_at: Option<bool>,
    pub m_end_frame: Option<f32>,
    pub m_lock_current_values: Option<bool>,
    pub m_name: Option<u32>,
    pub m_start_frame: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnabledRegionData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EndGameForLevelBlock {
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameCeremonyBehavior {
    pub fade_minions_delay_secs: f32,
    pub fade_minions_time_secs: f32,
    pub pan_to_hq_time_secs: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameViewController {
    pub base_loadable: u32,
    pub continue_button: u32,
    pub defeat_group_handle: u32,
    pub main_scene: u32,
    pub path_hash_to_self: u64,
    pub tencent_override_loadable: u32,
    pub unk_0x7a765367: Vec<Unk0xffc26938>,
    pub unk_0xae51ebbc: HashMap<u8, String>,
    pub unk_0xbfd85eff: u32,
    pub vanguard_bg_scene: u32,
    pub vanguard_continue_button: u32,
    pub vanguard_scene: u32,
    pub victory_group_handle: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct EnemyRespawnTimersViewController {
    pub base_loadable: u32,
    pub flipped_minimap_override: u32,
    pub path_hash_to_self: u64,
    pub portrait_data: Vec<UiPlayerPortraitData>,
    pub scene: u32,
    pub unk_0x949bbb90: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnterFowVisibility {
    pub m_missile_client_exit_fow_prediction: Option<bool>,
    pub m_missile_client_wait_for_target_update_before_missile_show: Option<bool>,
    pub m_perception_bubble_radius: Option<f32>,
    pub m_target_controls_visibility: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumAbilityResourceByCoefficientCalculationPart {
    AbilityResourceByCoefficientCalculationPart(AbilityResourceByCoefficientCalculationPart),
    BuffCounterByCoefficientCalculationPart(BuffCounterByCoefficientCalculationPart),
    BuffCounterByNamedDataValueCalculationPart(BuffCounterByNamedDataValueCalculationPart),
    ByCharLevelBreakpointsCalculationPart(ByCharLevelBreakpointsCalculationPart),
    ByCharLevelFormulaCalculationPart(ByCharLevelFormulaCalculationPart),
    ByCharLevelInterpolationCalculationPart(ByCharLevelInterpolationCalculationPart),
    ClampSubPartsCalculationPart(ClampSubPartsCalculationPart),
    CooldownMultiplierCalculationPart,
    EffectValueCalculationPart(EffectValueCalculationPart),
    ExponentSubPartsCalculationPart(ExponentSubPartsCalculationPart),
    NamedDataValueCalculationPart(NamedDataValueCalculationPart),
    NumberCalculationPart(NumberCalculationPart),
    PercentageOfBuffNameElapsed(PercentageOfBuffNameElapsed),
    ProductOfSubPartsCalculationPart(ProductOfSubPartsCalculationPart),
    StatByCoefficientCalculationPart(StatByCoefficientCalculationPart),
    StatByNamedDataValueCalculationPart(StatByNamedDataValueCalculationPart),
    StatBySubPartCalculationPart(StatBySubPartCalculationPart),
    StatEfficiencyPerHundred(StatEfficiencyPerHundred),
    SubPartScaledProportionalToStat(SubPartScaledProportionalToStat),
    SumOfSubPartsCalculationPart(SumOfSubPartsCalculationPart),
    Unk0x1d452085(Unk0x1d452085),
    Unk0x382277da(Unk0x382277da),
    Unk0x8a96ea3c(Unk0x8a96ea3c),
    Unk0x9e9e2e5c(Unk0x9e9e2e5c),
    Unk0xb22609db(Unk0xb22609db),
    Unk0xba007871(Unk0xba007871),
    Unk0xee18a47b(Unk0xee18a47b),
    Unk0xf3cbe7b2(Unk0xf3cbe7b2),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumAddLevelTimer {
    AddLevelTimer(AddLevelTimer),
    AnchorDouble(AnchorDouble),
    AnchorSingle(AnchorSingle),
    BoolGet(BoolGet),
    BoolTableGet(BoolTableGet),
    BoolTableSet(BoolTableSet),
    CastOnHit,
    CastOnMovementComplete(CastOnMovementComplete),
    ComparisonScriptCondition(ComparisonScriptCondition),
    ConcatenateStringsBlock(ConcatenateStringsBlock),
    CopyCustomTableBlock(CopyCustomTableBlock),
    CreateCustomTableBlock(CreateCustomTableBlock),
    CreateNeutralCamp(CreateNeutralCamp),
    CustomTableContainsValueBlock(CustomTableContainsValueBlock),
    CustomTableSet(CustomTableSet),
    DebugPrintToChatBlock(DebugPrintToChatBlock),
    DelayStart(DelayStart),
    DestroyCustomTableBlock(DestroyCustomTableBlock),
    DestroyOnExitMap,
    DestroyOnHit,
    DestroyOnMovementComplete(DestroyOnMovementComplete),
    EndGameForLevelBlock(EndGameForLevelBlock),
    FixedDistanceIgnoringTerrain(FixedDistanceIgnoringTerrain),
    FloatGet(FloatGet),
    FloatTableGet(FloatTableGet),
    FloatTableSet(FloatTableSet),
    ForEachInCustomTableBlock(ForEachInCustomTableBlock),
    GdsMapObject(GdsMapObject),
    GetGameStartCountdownTime(GetGameStartCountdownTime),
    GetKeyValueInCustomTableBlock(GetKeyValueInCustomTableBlock),
    GetModePreloadFlags(GetModePreloadFlags),
    GetSizeOfCustomTableBlock(GetSizeOfCustomTableBlock),
    InsertIntoCustomTableBlock(InsertIntoCustomTableBlock),
    IntGet(IntGet),
    IntTableGet(IntTableGet),
    IntTableSet(IntTableSet),
    LoopBlock(LoopBlock),
    MapAnimatedProp(MapAnimatedProp),
    MapAudio(MapAudio),
    MapBakeProperties(MapBakeProperties),
    MapCubemapProbe(MapCubemapProbe),
    MapGroup(MapGroup),
    MapLocator(MapLocator),
    MapNavGrid(MapNavGrid),
    MapParticle(MapParticle),
    MapScriptLocator(MapScriptLocator),
    MapSunProperties(MapSunProperties),
    MapTerrainPaint(MapTerrainPaint),
    PersistentEffectConditionData(PersistentEffectConditionData),
    PreloadCharacter(PreloadCharacter),
    PreloadCharacterWithSkinId(PreloadCharacterWithSkinId),
    RemoveFromCustomTableBlock(RemoveFromCustomTableBlock),
    ResimulateTrailVfxOnEnterVisibility(ResimulateTrailVfxOnEnterVisibility),
    ReturnToCasterOnMovementComplete(ReturnToCasterOnMovementComplete),
    ScriptCommentBlock(ScriptCommentBlock),
    ScriptTableGet(ScriptTableGet),
    ScriptTableSet(ScriptTableSet),
    SetInvulnerableBlock(SetInvulnerableBlock),
    SetKeyValueInCustomTableBlock(SetKeyValueInCustomTableBlock),
    SetModePreloadFlags(SetModePreloadFlags),
    SetTargetableBlock(SetTargetableBlock),
    SetVarInTableBlock(SetVarInTableBlock),
    SortCustomTableBlock(SortCustomTableBlock),
    SpellLevelUpUiData(SpellLevelUpUiData),
    SpellPipsUiData(SpellPipsUiData),
    StringGet(StringGet),
    StringTableGet(StringTableGet),
    StringTableSet(StringTableSet),
    TableValueExistsScriptCondition(TableValueExistsScriptCondition),
    TriggerFromScript(TriggerFromScript),
    TriggerOnDelay(TriggerOnDelay),
    TriggerOnHit(TriggerOnHit),
    TriggerOnMovementComplete(TriggerOnMovementComplete),
    Unk0x0,
    Unk0x132b63da(Unk0x132b63da),
    Unk0x15ebaa9c(Unk0x15ebaa9c),
    Unk0x1a6cd1f8,
    Unk0x1d1499b7(Unk0x1d1499b7),
    Unk0x1f1f50f2(Unk0x1f1f50f2),
    Unk0x1f8480d8(Unk0x1f8480d8),
    Unk0x25e3f5d0(Unk0x25e3f5d0),
    Unk0x28628b50(Unk0x28628b50),
    Unk0x3aa63a24(Unk0x3aa63a24),
    Unk0x3c995caf(Unk0x3c995caf),
    Unk0x4b30a085(Unk0x4b30a085),
    Unk0x4cf74021(Unk0x4cf74021),
    Unk0x5b4f4df6(Unk0x5b4f4df6),
    Unk0x5d85166e(Unk0x5d85166e),
    Unk0x6548f43(Unk0x6548f43),
    Unk0x667eafac(Unk0x667eafac),
    Unk0x6c93a1b4(Unk0x6c93a1b4),
    Unk0x72651449(Unk0x72651449),
    Unk0x72e9216e(Unk0x72e9216e),
    Unk0x72f86c81(Unk0x72f86c81),
    Unk0x7dcd3672(Unk0x7dcd3672),
    Unk0x851b8dc5(Unk0x851b8dc5),
    Unk0x851d2116(Unk0x851d2116),
    Unk0x91fd0920,
    Unk0x92b6b7fa(Unk0x92b6b7fa),
    Unk0x9aa5b4bc(Unk0x9aa5b4bc),
    Unk0x9c33070d(Unk0x9c33070d),
    Unk0xad65d8c4(Unk0xad65d8c4),
    Unk0xba138ae3(Unk0xba138ae3),
    Unk0xcdb1c8f6(Unk0xcdb1c8f6),
    Unk0xcf4a55da(Unk0xcf4a55da),
    Unk0xd178749c(Unk0xd178749c),
    Unk0xe524b2fc(Unk0xe524b2fc),
    Unk0xea69d6f3(Unk0xea69d6f3),
    Unk0xeb997689(Unk0xeb997689),
    Unk0xed070692(Unk0xed070692),
    Unk0xed124985(Unk0xed124985),
    Unk0xf090d2e7(Unk0xf090d2e7),
    VectorGet(VectorGet),
    VectorTableGet(VectorTableGet),
    VectorTableSet(VectorTableSet),
    WidthPerSecond(WidthPerSecond),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumArea {
    Area,
    AreaClamped,
    Cone,
    Direction,
    DragDirection,
    Location,
    LocationClamped,
    SelfAoe,
    Target(Target),
    TargetOrLocation,
    TerrainLocation,
    TerrainType(TerrainType),
    WallDetection(WallDetection),
    MySelf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumAttackEvents {
    AttackEvents(AttackEvents),
    CallOnMissileBounce,
    Cast(Cast),
    ChangeMissileSpeed(ChangeMissileSpeed),
    ChangeMissileWidth(ChangeMissileWidth),
    ClearAlreadyHitTracking,
    ClearTargetAndKeepMoving(ClearTargetAndKeepMoving),
    Destroy,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumBehavior {
    FadeOverTimeBehavior(FadeOverTimeBehavior),
    FadeToExplicitValueBehavior(FadeToExplicitValueBehavior),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumBlendData {
    TimeBlendData(TimeBlendData),
    TransitionClipBlendData(TransitionClipBlendData),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumClipData {
    AtomicClipData(AtomicClipData),
    ConditionBoolClipData(ConditionBoolClipData),
    ConditionFloatClipData(ConditionFloatClipData),
    ParallelClipData(ParallelClipData),
    ParametricClipData(ParametricClipData),
    SelectorClipData(SelectorClipData),
    SequencerClipData(SequencerClipData),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumContextualCondition {
    ContextualConditionAmountHealed(ContextualConditionAmountHealed),
    ContextualConditionAnyOtherHero(ContextualConditionAnyOtherHero),
    ContextualConditionBuffCounterChanged(ContextualConditionBuffCounterChanged),
    ContextualConditionBuffCounterReachedLimitFromZero(
        ContextualConditionBuffCounterReachedLimitFromZero,
    ),
    ContextualConditionBuffCounterSetToZeroAfterLimitReached(
        ContextualConditionBuffCounterSetToZeroAfterLimitReached,
    ),
    ContextualConditionCastTarget(ContextualConditionCastTarget),
    ContextualConditionChanceToPlay(ContextualConditionChanceToPlay),
    ContextualConditionCharacter(ContextualConditionCharacter),
    ContextualConditionCharacterDistance(ContextualConditionCharacterDistance),
    ContextualConditionCharacterHasCac(ContextualConditionCharacterHasCac),
    ContextualConditionCharacterHasTimeRemainingForAnimation(
        ContextualConditionCharacterHasTimeRemainingForAnimation,
    ),
    ContextualConditionCharacterHealth(ContextualConditionCharacterHealth),
    ContextualConditionCharacterInRangeForSyncedAnimation,
    ContextualConditionCharacterIsCastingRecall,
    ContextualConditionCharacterLevel(ContextualConditionCharacterLevel),
    ContextualConditionCharacterMetadata(ContextualConditionCharacterMetadata),
    ContextualConditionCharacterName(ContextualConditionCharacterName),
    ContextualConditionCharacterPlayingAnimation(ContextualConditionCharacterPlayingAnimation),
    ContextualConditionCharacterPlayingEmote,
    ContextualConditionCharacterSkinId(ContextualConditionCharacterSkinId),
    ContextualConditionCharacterUnitTags(ContextualConditionCharacterUnitTags),
    ContextualConditionCustomTimer(ContextualConditionCustomTimer),
    ContextualConditionGameTimer(ContextualConditionGameTimer),
    ContextualConditionGlobalObjectiveBountyFirstActivation,
    ContextualConditionGlobalObjectiveBountyForTeam(
        ContextualConditionGlobalObjectiveBountyForTeam,
    ),
    ContextualConditionGlobalObjectiveBountyState(ContextualConditionGlobalObjectiveBountyState),
    ContextualConditionHasGold(ContextualConditionHasGold),
    ContextualConditionIsAlly(ContextualConditionIsAlly),
    ContextualConditionIsSelf(ContextualConditionIsSelf),
    ContextualConditionItemId(ContextualConditionItemId),
    ContextualConditionItemPriceMinimum(ContextualConditionItemPriceMinimum),
    ContextualConditionItemVoGroup(ContextualConditionItemVoGroup),
    ContextualConditionKillCount(ContextualConditionKillCount),
    ContextualConditionMapId(ContextualConditionMapId),
    ContextualConditionMapRegionName(ContextualConditionMapRegionName),
    ContextualConditionMarkerName(ContextualConditionMarkerName),
    ContextualConditionMoveDistance(ContextualConditionMoveDistance),
    ContextualConditionMultikillSize(ContextualConditionMultikillSize),
    ContextualConditionNearbyChampionCount(ContextualConditionNearbyChampionCount),
    ContextualConditionNegation(ContextualConditionNegation),
    ContextualConditionNeutralCampId(ContextualConditionNeutralCampId),
    ContextualConditionNumberOfCharactersNearTargetPos(
        ContextualConditionNumberOfCharactersNearTargetPos,
    ),
    ContextualConditionObjectiveTakeByMyTeam(ContextualConditionObjectiveTakeByMyTeam),
    ContextualConditionOwnerTeamNetChampionKills(ContextualConditionOwnerTeamNetChampionKills),
    ContextualConditionPreviousMapRegionName(ContextualConditionPreviousMapRegionName),
    ContextualConditionRuleCooldown(ContextualConditionRuleCooldown),
    ContextualConditionShopOpenCount(ContextualConditionShopOpenCount),
    ContextualConditionSpell(ContextualConditionSpell),
    ContextualConditionSpellBuffName(ContextualConditionSpellBuffName),
    ContextualConditionSpellIsReady(ContextualConditionSpellIsReady),
    ContextualConditionSpellLevel(ContextualConditionSpellLevel),
    ContextualConditionSpellName(ContextualConditionSpellName),
    ContextualConditionSpellSlot(ContextualConditionSpellSlot),
    ContextualConditionTeammateDeathsNearby(ContextualConditionTeammateDeathsNearby),
    ContextualConditionTimeSinceStealthStateChange(ContextualConditionTimeSinceStealthStateChange),
    ContextualConditionWinningTeam(ContextualConditionWinningTeam),
    Unk0x1a4d9bd(Unk0x1a4d9bd),
    Unk0x2363fb10(Unk0x2363fb10),
    Unk0x4ab36eb5(Unk0x4ab36eb5),
    Unk0x4af7e9f2(Unk0x4af7e9f2),
    Unk0x61b427f(Unk0x61b427f),
    Unk0x6a50b5d7(Unk0x6a50b5d7),
    Unk0xac1764ca,
    Unk0xb6da23cb(Unk0xb6da23cb),
    Unk0xd55b5c23,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumData {
    AtlasData(AtlasData),
    AtlasData3SliceH(AtlasData3SliceH),
    AtlasData3SliceV(AtlasData3SliceV),
    AtlasData9Slice(AtlasData9Slice),
    LooseUiTextureData(LooseUiTextureData),
    LooseUiTextureData3SliceH(LooseUiTextureData3SliceH),
    LooseUiTextureData3SliceV(LooseUiTextureData3SliceV),
    LooseUiTextureData9Slice(LooseUiTextureData9Slice),
    Unk0x5eaead1a,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumDefaultvisibility {
    Defaultvisibility(Defaultvisibility),
    EnterFowVisibility(EnterFowVisibility),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumDriver {
    AbilityResourceDynamicMaterialFloatDriver(AbilityResourceDynamicMaterialFloatDriver),
    AllTrueMaterialDriver(AllTrueMaterialDriver),
    AnimationFractionDynamicMaterialFloatDriver(AnimationFractionDynamicMaterialFloatDriver),
    BlendingSwitchMaterialDriver(BlendingSwitchMaterialDriver),
    BuffCounterDynamicMaterialFloatDriver(BuffCounterDynamicMaterialFloatDriver),
    ColorChooserMaterialDriver(ColorChooserMaterialDriver),
    ColorGraphMaterialDriver(ColorGraphMaterialDriver),
    DelayedBoolMaterialDriver(DelayedBoolMaterialDriver),
    DistanceToPlayerMaterialFloatDriver(DistanceToPlayerMaterialFloatDriver),
    FixedDurationTriggeredBoolDriver(FixedDurationTriggeredBoolDriver),
    Float4LiteralMaterialDriver(Float4LiteralMaterialDriver),
    FloatComparisonMaterialDriver(FloatComparisonMaterialDriver),
    FloatGraphMaterialDriver(FloatGraphMaterialDriver),
    FloatLiteralMaterialDriver(FloatLiteralMaterialDriver),
    HasBuffDynamicMaterialBoolDriver(HasBuffDynamicMaterialBoolDriver),
    HasBuffOfTypeBoolDriver(HasBuffOfTypeBoolDriver),
    HasBuffWithAttributeBoolDriver,
    HasGearDynamicMaterialBoolDriver(HasGearDynamicMaterialBoolDriver),
    HealthDynamicMaterialFloatDriver,
    IsAnimationPlayingDynamicMaterialBoolDriver(IsAnimationPlayingDynamicMaterialBoolDriver),
    IsAttackingBoolDriver,
    IsCastingBoolDriver(IsCastingBoolDriver),
    IsDeadDynamicMaterialBoolDriver,
    IsEnemyDynamicMaterialBoolDriver,
    IsInGrassDynamicMaterialBoolDriver,
    IsLocalPlayerBoolDriver,
    IsMovingBoolDriver,
    IsOptionEnabledBoolDriver(IsOptionEnabledBoolDriver),
    KeyFrameFloatClipReaderDriver(KeyFrameFloatClipReaderDriver),
    LearnedSpellDynamicMaterialBoolDriver(LearnedSpellDynamicMaterialBoolDriver),
    LerpMaterialDriver(LerpMaterialDriver),
    LerpVec4LogicDriver(LerpVec4LogicDriver),
    MaxMaterialDriver(MaxMaterialDriver),
    MinMaterialDriver(MinMaterialDriver),
    NotMaterialDriver(NotMaterialDriver),
    OneTrueMaterialDriver(OneTrueMaterialDriver),
    PlayerPositionDynamicMaterialDriver,
    RemapFloatMaterialDriver(RemapFloatMaterialDriver),
    RemapVec4MaterialDriver(RemapVec4MaterialDriver),
    SineMaterialDriver(SineMaterialDriver),
    SpecificColorMaterialDriver(SpecificColorMaterialDriver),
    SpellRankIntDriver(SpellRankIntDriver),
    SubmeshVisibilityBoolDriver(SubmeshVisibilityBoolDriver),
    SwitchMaterialDriver(SwitchMaterialDriver),
    TimeMaterialDriver(TimeMaterialDriver),
    Unk0x53dfc5b5(Unk0x53dfc5b5),
    Unk0x5b2fdd66(Unk0x5b2fdd66),
    Unk0x608a3ee7,
    Unk0x635d04b7(Unk0x635d04b7),
    Unk0x77b42f3f,
    Unk0x83a9f4f8,
    Unk0x9bc366ca(Unk0x9bc366ca),
    Unk0xb7b43e1d(Unk0xb7b43e1d),
    Unk0xbafc3e15(Unk0xbafc3e15),
    Unk0xc83eb239,
    Unk0xcc35f742(Unk0xcc35f742),
    Unk0xe622d482,
    Unk0xef339ef9(Unk0xef339ef9),
    Unk0xf5821f8b,
    Unk0xfe70e9c4(Unk0xfe70e9c4),
    UvScaleBiasFromAnimationDynamicMaterialDriver(UvScaleBiasFromAnimationDynamicMaterialDriver),
    VelocityDynamicMaterialFloatDriver,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumElementData {
    BrowserElementData(BrowserElementData),
    StoreElementData(StoreElementData),
    TroveElementData(TroveElementData),
    Unk0x3bd6ea88(Unk0x3bd6ea88),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumEventData {
    ConformToPathEventData(ConformToPathEventData),
    EnableLookAtEventData(EnableLookAtEventData),
    FaceTargetEventData(FaceTargetEventData),
    FadeEventData(FadeEventData),
    IdleParticlesVisibilityEventData(IdleParticlesVisibilityEventData),
    JointOrientationEventData(JointOrientationEventData),
    JointSnapEventData(JointSnapEventData),
    LockRootOrientationEventData(LockRootOrientationEventData),
    ParticleEventData(ParticleEventData),
    SoundEventData(SoundEventData),
    SpringPhysicsEventData(SpringPhysicsEventData),
    StopAnimationEventData(StopAnimationEventData),
    SubmeshVisibilityEventData(SubmeshVisibilityEventData),
    SyncedAnimationEventData(SyncedAnimationEventData),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumFacing {
    VeritcalFacingMatchVelocity,
    VerticalFacingFaceTarget,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumGameCalculation {
    GameCalculation(GameCalculation),
    GameCalculationConditional(GameCalculationConditional),
    GameCalculationModified(GameCalculationModified),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumGameModeConstant {
    GameModeConstantBool(GameModeConstantBool),
    GameModeConstantFloat(GameModeConstantFloat),
    GameModeConstantInteger(GameModeConstantInteger),
    GameModeConstantString(GameModeConstantString),
    GameModeConstantStringVector(GameModeConstantStringVector),
    GameModeConstantTraKey(GameModeConstantTraKey),
    GameModeConstantVector3f(GameModeConstantVector3f),
    Unk0x8beb0550(Unk0x8beb0550),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumHasAllSubRequirementsCastRequirement {
    HasAllSubRequirementsCastRequirement(HasAllSubRequirementsCastRequirement),
    HasAtleastNSubRequirementsCastRequirement(HasAtleastNSubRequirementsCastRequirement),
    HasBuffCastRequirement(HasBuffCastRequirement),
    HasNNearbyUnitsRequirement(HasNNearbyUnitsRequirement),
    HasNNearbyVisibleUnitsRequirement(HasNNearbyVisibleUnitsRequirement),
    HasTypeAndStatusFlags(HasTypeAndStatusFlags),
    HasUnitTagsCastRequirement(HasUnitTagsCastRequirement),
    IsSpecifiedUnitCastRequirement(IsSpecifiedUnitCastRequirement),
    SameTeamCastRequirement(SameTeamCastRequirement),
    Unk0x303c66f4(Unk0x303c66f4),
    Unk0xe2ef74d0(Unk0xe2ef74d0),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumHealthBarTickStyle {
    HealthBarTickStyleHero(HealthBarTickStyleHero),
    HealthBarTickStyleTftCompanion(HealthBarTickStyleTftCompanion),
    HealthBarTickStyleUnit(HealthBarTickStyleUnit),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumHeightSolver {
    BlendedLinearHeightSolver,
    CurveTheDifferenceHeightSolver(CurveTheDifferenceHeightSolver),
    FollowTerrainHeightSolver(FollowTerrainHeightSolver),
    GravityHeightSolver(GravityHeightSolver),
    SinusoidalHeightSolver(SinusoidalHeightSolver),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumIconElement {
    IconElementCircleMaskeExtension,
    IconElementGradientExtension,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumIndicatorType {
    IndicatorTypeGlobal(IndicatorTypeGlobal),
    IndicatorTypeLocal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumInputSource {
    AndInputSourceBool(AndInputSourceBool),
    GamepadAxisInputSourceFloat(GamepadAxisInputSourceFloat),
    GamepadButtonInputSourceBool(GamepadButtonInputSourceBool),
    InputSourceBoolAsFloat(InputSourceBoolAsFloat),
    InputSourceFloatAsBool(InputSourceFloatAsBool),
    KeyInputSourceBool(KeyInputSourceBool),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumLayoutStyle {
    LayoutStyleGrid(LayoutStyleGrid),
    LayoutStyleHorizontalList(LayoutStyleHorizontalList),
    LayoutStyleVerticalList(LayoutStyleVerticalList),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumLoadingScreenPlayerCard {
    LoadingScreenPlayerCardClassicData(LoadingScreenPlayerCardClassicData),
    LoadingScreenPlayerCardTftData(LoadingScreenPlayerCardTftData),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumMinimapIcon {
    MinimapIconChangeOpacity(MinimapIconChangeOpacity),
    MinimapIconRotate(MinimapIconRotate),
    MinimapIconScale(MinimapIconScale),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumMovement {
    AcceleratingMovement(AcceleratingMovement),
    CircleMovement(CircleMovement),
    DecelToLocationMovement(DecelToLocationMovement),
    FixedSpeedMovement(FixedSpeedMovement),
    FixedSpeedSplineMovement(FixedSpeedSplineMovement),
    FixedTimeMovement(FixedTimeMovement),
    FixedTimeSplineMovement(FixedTimeSplineMovement),
    ParametricMovement(ParametricMovement),
    PhysicsMovement(PhysicsMovement),
    SyncCircleMovement(SyncCircleMovement),
    TrackMouseMovement(TrackMouseMovement),
    WallFollowMovement(WallFollowMovement),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumOptionItem {
    OptionItemBorder(OptionItemBorder),
    OptionItemButton(OptionItemButton),
    OptionItemCheckbox(OptionItemCheckbox),
    OptionItemColumns(OptionItemColumns),
    OptionItemDropdown(OptionItemDropdown),
    OptionItemGroup(OptionItemGroup),
    OptionItemHotkeys(OptionItemHotkeys),
    OptionItemLabel(OptionItemLabel),
    OptionItemResolutionDropdown(OptionItemResolutionDropdown),
    OptionItemSliderFloat(OptionItemSliderFloat),
    OptionItemSliderGraphicsQuality(OptionItemSliderGraphicsQuality),
    OptionItemSliderInt(OptionItemSliderInt),
    OptionItemSliderVolume(OptionItemSliderVolume),
    OptionItemVoiceInputDeviceDropdown(OptionItemVoiceInputDeviceDropdown),
    Unk0x165c0117(Unk0x165c0117),
    Unk0x1e26bfa8(Unk0x1e26bfa8),
    Unk0x81580a34(Unk0x81580a34),
    Unk0x9ef1e737(Unk0x9ef1e737),
    Unk0xa9d60c77(Unk0xa9d60c77),
    Unk0xb602063b(Unk0xb602063b),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumOptionItemFilter {
    OptionItemFilterAnd(OptionItemFilterAnd),
    OptionItemFilterClassicMusicAllowed,
    OptionItemFilterFeatureToggle(OptionItemFilterFeatureToggle),
    OptionItemFilterGameStyle(OptionItemFilterGameStyle),
    OptionItemFilterHwRequirement(OptionItemFilterHwRequirement),
    OptionItemFilterIos,
    OptionItemFilterMutator(OptionItemFilterMutator),
    OptionItemFilterNot(OptionItemFilterNot),
    OptionItemFilterOsx,
    OptionItemFilterPromoteAccount,
    OptionItemFilterReplayApi,
    OptionItemFilterVoiceChat,
    OptionItemFilterWindows,
    Unk0x1fb61ff0,
    Unk0x4e771e24,
    Unk0x6b5fc3eb,
    Unk0x70749bea,
    Unk0xc8a5411c,
    Unk0xd4737a04,
    Unk0xd574bc5b,
    Unk0xef2cc9a6,
    Unk0xfca7e1ca,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumOverLifeMaterialDriver {
    VfxColorOverLifeMaterialDriver(VfxColorOverLifeMaterialDriver),
    VfxFloatOverLifeMaterialDriver(VfxFloatOverLifeMaterialDriver),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumParametricUpdater {
    AttackSpeedParametricUpdater,
    DisplacementParametricUpdater,
    EquippedGearParametricUpdater,
    FacingAndMovementAngleParametricUpdater,
    FacingParametricUpdater,
    IsAllyParametricUpdater,
    IsHomeguardParametricUpdater,
    IsInTerrainParametricUpdater,
    IsMovingParametricUpdater,
    IsRangedParametricUpdater,
    IsTurningParametricUpdater,
    LogicDriverBoolParametricUpdater(LogicDriverBoolParametricUpdater),
    LogicDriverFloatParametricUpdater(LogicDriverFloatParametricUpdater),
    LookAtGoldRedirectTargetAngleParametricUpdater,
    LookAtInterestAngleParametricUpdater,
    LookAtInterestDistanceParametricUpdater,
    LookAtSpellTargetAngleParametricUpdater,
    LookAtSpellTargetDistanceParametricUpdater,
    LookAtSpellTargetHeightOffsetParametricUpdater,
    MoveSpeedParametricUpdater,
    MovementDirectionParametricUpdater,
    ParBarPercentParametricUpdater,
    SkinScaleParametricUpdater,
    SlopeAngleParametricUpdater,
    TotalTurnAngleParametricUpdater,
    TurnAngleParametricUpdater,
    TurnAngleRemainingParametricUpdater,
    Unk0xe7b61183(Unk0xe7b61183),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumRequirement {
    CharacterLevelRequirement(CharacterLevelRequirement),
    HasBuffRequirement(HasBuffRequirement),
    HasSkillPointRequirement,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumResourceMeter {
    ResourceMeterGroupData(ResourceMeterGroupData),
    ResourceMeterIconData(ResourceMeterIconData),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumRigPoseModifierData {
    ConformToPathRigPoseModifierData(ConformToPathRigPoseModifierData),
    SpringPhysicsRigPoseModifierData(SpringPhysicsRigPoseModifierData),
    SyncedAnimationRigPoseModifierData,
    Unk0xe6147387(Unk0xe6147387),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumScoreLineSr {
    ScoreLineSrSpectatorUiData(ScoreLineSrSpectatorUiData),
    ScoreLineSrUiData(ScoreLineSrUiData),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumTakeCamp {
    TakeCamp(TakeCamp),
    TerminatePath,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumTarget {
    TargetHasUnitTagFilter(TargetHasUnitTagFilter),
    TargetTypeFilter(TargetTypeFilter),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumTargeterDefinition {
    TargeterDefinitionAoe(TargeterDefinitionAoe),
    TargeterDefinitionArc(TargeterDefinitionArc),
    TargeterDefinitionCone(TargeterDefinitionCone),
    TargeterDefinitionLine(TargeterDefinitionLine),
    TargeterDefinitionMinimap(TargeterDefinitionMinimap),
    TargeterDefinitionMultiAoe(TargeterDefinitionMultiAoe),
    TargeterDefinitionRange(TargeterDefinitionRange),
    TargeterDefinitionSpline(TargeterDefinitionSpline),
    TargeterDefinitionWall(TargeterDefinitionWall),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumTargetingRangeValue {
    TargetingRangeValue(TargetingRangeValue),
    Unk0x9d62f7e(Unk0x9d62f7e),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumTftGameStartSequenceScene {
    TftGameStartSequenceScene(TftGameStartSequenceScene),
    Unk0xb3674a86(Unk0xb3674a86),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumTipStyle {
    DoubleSidedTipStyle(DoubleSidedTipStyle),
    GlowCenteredOverlayTipStyle(GlowCenteredOverlayTipStyle),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumTransitionData {
    SceneAlphaTransitionData(SceneAlphaTransitionData),
    SceneScreenEdgeTransitionData(SceneScreenEdgeTransitionData),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUiDraggable {
    UiDraggableBasic(UiDraggableBasic),
    UiDraggableElementGroupDrag(UiDraggableElementGroupDrag),
    UiDraggableProxyElementDrag(UiDraggableProxyElementDrag),
    UiDraggableSceneDrag(UiDraggableSceneDrag),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUiMetric {
    UiMetricBattery(UiMetricBattery),
    UiMetricClash(UiMetricClash),
    UiMetricCreepScore(UiMetricCreepScore),
    UiMetricFps(UiMetricFps),
    UiMetricGameTime(UiMetricGameTime),
    UiMetricKda(UiMetricKda),
    UiMetricLaneMinionFlatDamageReductionFromMinion(
        UiMetricLaneMinionFlatDamageReductionFromMinion,
    ),
    UiMetricLaneMinionPercentIncreasedDamageToMinion(
        UiMetricLaneMinionPercentIncreasedDamageToMinion,
    ),
    UiMetricLatencyText(UiMetricLatencyText),
    UiMetricMultiDragonKillsSrX(UiMetricMultiDragonKillsSrX),
    UiMetricTeamGold(UiMetricTeamGold),
    UiMetricTeamKills(UiMetricTeamKills),
    UiMetricTeamScoreMeters(UiMetricTeamScoreMeters),
    UiMetricUnitBounty(UiMetricUnitBounty),
    UiMetricUnitCreepScore(UiMetricUnitCreepScore),
    UiMetricUnitKda(UiMetricUnitKda),
    UiMetricUnitVisionScore(UiMetricUnitVisionScore),
    UiMetricVietnameseRatingLabel(UiMetricVietnameseRatingLabel),
    Unk0x5ab5b20f(Unk0x5ab5b20f),
    Unk0x767adcf7(Unk0x767adcf7),
    Unk0xb62c8675(Unk0xb62c8675),
    Unk0xe228ce4a(Unk0xe228ce4a),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUiPosition {
    UiPositionFullScreen,
    UiPositionPolygon(UiPositionPolygon),
    UiPositionRect(UiPositionRect),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x1a4d18fe {
    Unk0x1a4d18fe(Unk0x1a4d18fe),
    Unk0x3dc0ea14(Unk0x3dc0ea14),
    Unk0x755cf26f(Unk0x755cf26f),
    Unk0x810de4e7(Unk0x810de4e7),
    Unk0xb213e4ff(Unk0xb213e4ff),
    Unk0xc2b1af7f(Unk0xc2b1af7f),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x1aae122 {
    Unk0x1aae122(Unk0x1aae122),
    Unk0x1ddfbeeb,
    Unk0x93adc5b3(Unk0x93adc5b3),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x296c4c00 {
    Unk0x296c4c00(Unk0x296c4c00),
    Unk0xab02008c(Unk0xab02008c),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x2e181f68 {
    Unk0x2e181f68(Unk0x2e181f68),
    Unk0x3f667d7e(Unk0x3f667d7e),
    Unk0x4146f732(Unk0x4146f732),
    Unk0x45d6060(Unk0x45d6060),
    Unk0x46ce0526(Unk0x46ce0526),
    Unk0x5014eb79(Unk0x5014eb79),
    Unk0x51f3b0ef(Unk0x51f3b0ef),
    Unk0x79292790(Unk0x79292790),
    Unk0x7cde3150(Unk0x7cde3150),
    Unk0x7dbb4a4d(Unk0x7dbb4a4d),
    Unk0x8069a433(Unk0x8069a433),
    Unk0x80929fe(Unk0x80929fe),
    Unk0x9506323a(Unk0x9506323a),
    Unk0xa6e992c8(Unk0xa6e992c8),
    Unk0xadfcc498(Unk0xadfcc498),
    Unk0xae56d8c4(Unk0xae56d8c4),
    Unk0xb5f38c13(Unk0xb5f38c13),
    Unk0xb95142cd(Unk0xb95142cd),
    Unk0xc074ba45(Unk0xc074ba45),
    Unk0xc174810b(Unk0xc174810b),
    Unk0xc75640aa(Unk0xc75640aa),
    Unk0xc7b1ec51(Unk0xc7b1ec51),
    Unk0xc8d6dccd(Unk0xc8d6dccd),
    Unk0xcaefc854(Unk0xcaefc854),
    Unk0xda5b233f(Unk0xda5b233f),
    Unk0xdde919c(Unk0xdde919c),
    Unk0xe9b3cb22(Unk0xe9b3cb22),
    Unk0xf0d64228(Unk0xf0d64228),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x32078b36 {
    Unk0x32078b36(Unk0x32078b36),
    Unk0xdf7d294e(Unk0xdf7d294e),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x34f43159 {
    Unk0x34f43159(Unk0x34f43159),
    Unk0x9a7de981(Unk0x9a7de981),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x37841b56 {
    Unk0x37841b56(Unk0x37841b56),
    Unk0x3eed1ba8(Unk0x3eed1ba8),
    Unk0x7709fefa(Unk0x7709fefa),
    Unk0xdb9a90ba(Unk0xdb9a90ba),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x394f5aaf {
    Unk0x394f5aaf(Unk0x394f5aaf),
    Unk0x9e1e8775(Unk0x9e1e8775),
    Unk0xb011f563(Unk0xb011f563),
    Unk0xb5215699(Unk0xb5215699),
    Unk0xd4ac6ed4(Unk0xd4ac6ed4),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x3b8a61ee {
    Unk0x3b8a61ee(Unk0x3b8a61ee),
    Unk0x6c455dac(Unk0x6c455dac),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x3c5f9d3d {
    Unk0x3c5f9d3d(Unk0x3c5f9d3d),
    Unk0xbd659e9b(Unk0xbd659e9b),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x49401c5c {
    Unk0x49401c5c(Unk0x49401c5c),
    Unk0xf2839c7c,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x51445de9 {
    Unk0x51445de9(Unk0x51445de9),
    Unk0x557bb273(Unk0x557bb273),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x51ada002 {
    Unk0x51ada002(Unk0x51ada002),
    Unk0x9c5b78dd(Unk0x9c5b78dd),
    Unk0xeea0bf1,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x6030f7c6 {
    Unk0x6030f7c6(Unk0x6030f7c6),
    Unk0xee8fe512(Unk0xee8fe512),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x6256517d {
    Unk0x6256517d(Unk0x6256517d),
    Unk0xe189202c(Unk0xe189202c),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x6bbc3db6 {
    Unk0x6bbc3db6(Unk0x6bbc3db6),
    Unk0xf00f3333,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x797fe1c2 {
    Unk0x797fe1c2,
    Unk0xcdf661db(Unk0xcdf661db),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0x8873e4c8 {
    Unk0x8873e4c8(Unk0x8873e4c8),
    Unk0xd86db9b6(Unk0xd86db9b6),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0xa8b35a0d {
    Unk0xa8b35a0d(Unk0xa8b35a0d),
    Unk0xab2fec44(Unk0xab2fec44),
    Unk0xf2485b58(Unk0xf2485b58),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0xc96d9140 {
    Unk0xc96d9140(Unk0xc96d9140),
    Unk0xe7ee4f28(Unk0xe7ee4f28),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumUnk0xf4737fbd {
    Unk0xf4737fbd(Unk0xf4737fbd),
    Unk0xfe31ac4d(Unk0xfe31ac4d),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumVars {
    CharacterVars,
    InstanceVars,
    NextBuffVars,
    NextSpellVars,
    WorldVars,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumVfxPrimitive {
    Unk0x8df5fcf7,
    VfxPrimitiveArbitraryQuad,
    VfxPrimitiveArbitraryTrail(VfxPrimitiveArbitraryTrail),
    VfxPrimitiveAttachedMesh(VfxPrimitiveAttachedMesh),
    VfxPrimitiveBeam(VfxPrimitiveBeam),
    VfxPrimitiveCameraSegmentBeam(VfxPrimitiveCameraSegmentBeam),
    VfxPrimitiveCameraTrail(VfxPrimitiveCameraTrail),
    VfxPrimitiveCameraUnitQuad,
    VfxPrimitiveMesh(VfxPrimitiveMesh),
    VfxPrimitivePlanarProjection(VfxPrimitivePlanarProjection),
    VfxPrimitiveRay,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumVfxShape {
    Unk0xee39916f(Unk0xee39916f),
    VfxShapeBox(VfxShapeBox),
    VfxShapeCylinder(VfxShapeCylinder),
    VfxShapeLegacy(VfxShapeLegacy),
    VfxShapeSphere(VfxShapeSphere),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumViewControllerFilter {
    Unk0x349845d9(Unk0x349845d9),
    Unk0xb0afae41(Unk0xb0afae41),
    Unk0xf463e496,
    ViewControllerFilterAnd(ViewControllerFilterAnd),
    ViewControllerFilterChampion(ViewControllerFilterChampion),
    ViewControllerFilterClash,
    ViewControllerFilterMap(ViewControllerFilterMap),
    ViewControllerFilterMobile,
    ViewControllerFilterMode(ViewControllerFilterMode),
    ViewControllerFilterNot(ViewControllerFilterNot),
    ViewControllerFilterOr(ViewControllerFilterOr),
    ViewControllerFilterSpectator,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnumWaveBehavior {
    ConstantWaveBehavior(ConstantWaveBehavior),
    InhibitorWaveBehavior(InhibitorWaveBehavior),
    RotatingWaveBehavior(RotatingWaveBehavior),
    TimedVariableWaveBehavior(TimedVariableWaveBehavior),
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct EsportsBannerData {
    pub banner_name: String,
    pub team: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EsportsBannerMaterialController {}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct EsportsBannerOptions {
    pub default_blank_material: u32,
    pub default_texture_path: u64,
    pub sub_mesh_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct EsportsRotatingBannerConfiguration {
    pub associated_teams: Option<Vec<String>>,
    pub associated_versions: Option<Vec<String>>,
    pub default_banner_texture_path: u64,
    pub event_mutator: Option<u32>,
    pub global_rotation_time_in_seconds: f32,
    pub is_an_event: Option<bool>,
    pub league_rotating_flavors: Vec<LeagueRotatingFlavor>,
    pub name: String,
    pub priority: Option<u32>,
    pub startup_delay_in_seconds: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct EventCardViewController {
    pub base_loadable: u32,
    pub card_background: u32,
    pub countdown_background: u32,
    pub countdown_text: u32,
    pub description_text: u32,
    pub highlight: u32,
    pub icon: u32,
    pub max_alpha: u8,
    pub path_hash_to_self: u64,
    pub scene_handle: u32,
    pub title_text: u32,
    pub transition_time: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EvolutionDescription {
    pub m_flags: Option<u32>,
    pub m_icon_names: Vec<String>,
    pub m_title: Option<String>,
    pub m_tooltips: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceCurveData {
    pub level_difference_experience_multiplier_per_level: Vec<f32>,
    pub m_base_experience_multiplier: Option<f32>,
    pub m_experience_granted_for_kill_per_level: Option<Vec<f32>>,
    pub m_experience_granted_mult_for_shared_kill_per_level: Option<Vec<f32>>,
    pub m_experience_required_per_level: Option<Vec<f32>>,
    pub m_minimum_experience_multiplier: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceModData {
    pub m_player_minion_split_xp: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExponentSubPartsCalculationPart {
    pub part1: NamedDataValueCalculationPart,
    pub part2: NumberCalculationPart,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FaceTargetEventData {
    pub blend_in_time: Option<f32>,
    pub blend_out_time: Option<f32>,
    pub face_target: Option<u8>,
    pub m_end_frame: Option<f32>,
    pub m_fire_if_animation_ends_early: Option<bool>,
    pub m_is_self_only: Option<bool>,
    pub m_name: Option<u32>,
    pub m_start_frame: Option<f32>,
    pub y_rotation_degrees: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FadeEventData {
    pub m_end_frame: Option<f32>,
    pub m_fire_if_animation_ends_early: Option<bool>,
    pub m_name: Option<u32>,
    pub m_start_frame: Option<f32>,
    pub m_target_alpha: f32,
    pub m_time_to_fade: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FadeOverTimeBehavior {
    pub m_end_alpha: Option<f32>,
    pub m_start_alpha: Option<f32>,
    pub m_time_end: f32,
    pub m_time_start: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FadeToExplicitValueBehavior {
    pub m_alpha: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct FeatureAudioDataProperties {
    pub bank_units: Vec<BankUnit>,
    pub feature: u32,
    pub music: MusicAudioDataProperties,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FixedDistanceIgnoringTerrain {
    pub m_maximum_distance: f32,
    pub m_maximum_terrain_walls_to_skip: Option<u32>,
    pub m_minimum_gap_between_terrain_walls: Option<f32>,
    pub m_targeter_definition: TargeterDefinitionSkipTerrain,
    pub scan_width_override: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FixedDurationTriggeredBoolDriver {
    pub m_bool_driver: Box<EnumDriver>,
    pub m_custom_duration: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FixedSpeedMovement {
    pub add_bonus_attack_range_to_cast_range: Option<bool>,
    pub m_infer_direction_from_facing_if_needed: Option<bool>,
    pub m_offset_initial_target_height: Option<f32>,
    pub m_project_target_to_cast_range: Option<bool>,
    pub m_speed: Option<f32>,
    pub m_start_bone_name: Option<String>,
    pub m_start_bone_skin_overrides: Option<HashMap<u32, String>>,
    pub m_target_bone_name: Option<String>,
    pub m_target_height_augment: Option<f32>,
    pub m_tracks_target: Option<bool>,
    pub m_use_ground_height_at_target: Option<bool>,
    pub m_use_height_offset_at_end: Option<bool>,
    pub m_visuals_track_hidden_targets: Option<bool>,
    pub unk_0x3046674: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FixedSpeedSplineMovement {
    pub m_offset_initial_target_height: Option<f32>,
    pub m_speed: f32,
    pub m_spline_info: HermiteSplineInfo,
    pub m_start_bone_name: Option<String>,
    pub m_target_bone_name: Option<String>,
    pub m_target_height_augment: Option<f32>,
    pub m_tracks_target: Option<bool>,
    pub m_use_height_offset_at_end: Option<bool>,
    pub m_use_missile_position_as_origin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FixedTimeMovement {
    pub m_infer_direction_from_facing_if_needed: Option<bool>,
    pub m_offset_initial_target_height: Option<f32>,
    pub m_project_target_to_cast_range: Option<bool>,
    pub m_start_bone_name: Option<String>,
    pub m_start_bone_skin_overrides: Option<HashMap<u32, String>>,
    pub m_target_bone_name: Option<String>,
    pub m_target_height_augment: Option<f32>,
    pub m_tracks_target: Option<bool>,
    pub m_travel_time: f32,
    pub m_use_ground_height_at_target: Option<bool>,
    pub m_use_height_offset_at_end: Option<bool>,
    pub m_visuals_track_hidden_targets: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FixedTimeSplineMovement {
    pub m_spline_info: HermiteSplineInfo,
    pub m_start_bone_name: String,
    pub m_target_bone_name: Option<String>,
    pub m_target_height_augment: Option<f32>,
    pub m_tracks_target: Option<bool>,
    pub m_travel_time: f32,
    pub m_use_missile_position_as_origin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FlexTypeFloat {
    pub m_flex_id: Option<u32>,
    pub m_value: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FlexValueFloat {
    pub m_flex_id: Option<u32>,
    pub m_value: Option<ValueFloat>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FlexValueVector2 {
    pub m_flex_id: Option<u32>,
    pub m_value: Option<ValueVector2>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FlexValueVector3 {
    pub m_value: Option<ValueVector3>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Float4LiteralMaterialDriver {
    pub value: Option<Vec4>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FloatComparisonMaterialDriver {
    pub m_operator: Option<u32>,
    pub m_value_a: Box<EnumDriver>,
    pub m_value_b: Box<EnumDriver>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FloatGet {
    pub value: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FloatGraphMaterialDriver {
    pub driver: Box<EnumDriver>,
    pub graph: VfxAnimatedFloatVariableData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FloatLiteralMaterialDriver {
    pub m_value: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FloatPerSpellLevel {
    pub m_per_level_values: Option<Vec<f32>>,
    pub m_value_type: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FloatTableGet {
    pub table: Option<EnumVars>,
    pub var: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FloatTableSet {
    pub table: Option<EnumVars>,
    pub var: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct FloatTextDisplayOverrides {
    pub alternate_right_left: Option<bool>,
    pub color_offset_b: Option<i32>,
    pub color_offset_g: Option<i32>,
    pub color_offset_r: Option<i32>,
    pub combinable_counter_category: Option<i32>,
    pub combinable_counter_display: Option<bool>,
    pub combinable_text_show_crit: Option<bool>,
    pub continual_force_x: Option<f32>,
    pub continual_force_y: Option<f32>,
    pub decay: Option<f32>,
    pub decay_delay: Option<f32>,
    pub extend_time_on_new_damage: Option<f32>,
    pub follow_source: Option<bool>,
    pub growth_x_scalar: Option<f32>,
    pub growth_y_scalar: Option<f32>,
    pub hang_time: Option<f32>,
    pub ignore_combine_rules: Option<bool>,
    pub is_animated: Option<bool>,
    pub max_x_velocity: Option<f32>,
    pub max_y_velocity: Option<f32>,
    pub min_x_velocity: Option<f32>,
    pub min_y_velocity: Option<f32>,
    pub priority: Option<i32>,
    pub random_offset_max_x: Option<f32>,
    pub random_offset_max_y: Option<f32>,
    pub random_offset_min_x: Option<f32>,
    pub random_offset_min_y: Option<f32>,
    pub relative_offset_max: Option<f32>,
    pub relative_offset_min: Option<f32>,
    pub scale: Option<f32>,
    pub shrink_scale: Option<f32>,
    pub shrink_time: Option<f32>,
    pub start_offset_x: Option<f32>,
    pub start_offset_y: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct FloatTextFormattingData {
    pub alternate_right_left: Option<bool>,
    pub attach_to_health_bar: Option<bool>,
    pub color_offset_b: Option<i32>,
    pub color_offset_g: Option<i32>,
    pub color_offset_r: Option<i32>,
    pub combinable_counter_category: Option<i32>,
    pub combinable_counter_display: Option<bool>,
    pub combinable_negative_number_format: Option<String>,
    pub combinable_number_format: Option<String>,
    pub continual_force_x: Option<f32>,
    pub continual_force_y: Option<f32>,
    pub decay: f32,
    pub decay_delay: f32,
    pub disable_horizontal_reverse: Option<bool>,
    pub disable_vertical_reverse: Option<bool>,
    pub extend_time_on_new_damage: Option<f32>,
    pub follow_source: Option<bool>,
    pub hang_time: Option<f32>,
    pub height: Option<f32>,
    pub icons: Option<Vec<FloatTextIconData>>,
    pub ignore_combine_rules: Option<bool>,
    pub ignore_queue: Option<bool>,
    pub is_animated: Option<bool>,
    pub m_font_description: u32,
    pub m_type_name: u32,
    pub max_instances: Option<i32>,
    pub max_life_time: Option<f32>,
    pub max_x_velocity: Option<f32>,
    pub max_y_velocity: Option<f32>,
    pub min_x_velocity: Option<f32>,
    pub min_y_velocity: Option<f32>,
    pub momentum_from_hit: Option<bool>,
    pub offset_by_bounding_box: Option<bool>,
    pub overwrite_previous_number: Option<bool>,
    pub priority: i32,
    pub random_start_offset_max_x: Option<f32>,
    pub random_start_offset_max_y: Option<f32>,
    pub random_start_offset_min_x: Option<f32>,
    pub random_start_offset_min_y: Option<f32>,
    pub refresh_existing: Option<bool>,
    pub scale: Option<f32>,
    pub shrink_scale: Option<f32>,
    pub shrink_time: Option<f32>,
    pub start_offset_x: Option<f32>,
    pub start_offset_y: Option<f32>,
    pub vertical_alignment: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FloatTextIconData {
    pub m_alignment: Option<u32>,
    pub m_color: Option<[u8; 4]>,
    pub m_display_size: Vec2,
    pub m_icon_file_name: String,
    pub m_offset: Vec2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FloatingHealthBarBurstData {
    pub burst_trigger_percent: f32,
    pub flash_trigger_percent: f32,
    pub shake_trigger_percent: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct FloatingInfoBarViewController {
    pub base_loadable: u32,
    pub info_bar_style_source_map: HashMap<u8, u32>,
    pub path_hash_to_self: u64,
    pub unit_status_priority_list: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FloatingTextDamageDisplayTypeList {
    pub barrack_minion: u32,
    pub default: u32,
    pub dot: u32,
    pub dot_no_combine: u32,
    pub dot_parent_missile: u32,
    pub dot_slow: u32,
    pub impact: u32,
    pub mini: u32,
    pub multistrike: u32,
    pub multistrike_fast: u32,
    pub multistrike_slow: u32,
    pub player_minion: u32,
    pub self_magical_damage_counter: u32,
    pub self_physical_damage_counter: u32,
    pub self_true_damage_counter: u32,
    pub ult: u32,
    pub zone: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct FloatingTextGlobalConfig {
    pub m_damage_display_types: FloatingTextDamageDisplayTypeList,
    pub m_floating_text_types: FloatingTextTypeList,
    pub m_tunables: FloatingTextTunables,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FloatingTextTunables {
    pub m_animated_text_queue_delay: f32,
    pub m_comparison_by_level: Vec<f32>,
    pub m_max_floating_text_items: u32,
    pub m_minion_comparison_multiplier: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FloatingTextTypeList {
    pub absorbed: u32,
    pub countdown: u32,
    pub debug: u32,
    pub disable: u32,
    pub dodge: u32,
    pub enemy_magical_damage: u32,
    pub enemy_magical_damage_critical: u32,
    pub enemy_physical_damage: u32,
    pub enemy_physical_damage_critical: u32,
    pub enemy_true_damage: u32,
    pub enemy_true_damage_critical: u32,
    pub experience: u32,
    pub gold: u32,
    pub heal: u32,
    pub invulnerable: u32,
    pub level: u32,
    pub magical_damage: u32,
    pub magical_damage_critical: u32,
    pub mana_damage: u32,
    pub mana_heal: u32,
    pub omw: u32,
    pub physical_damage: u32,
    pub physical_damage_critical: u32,
    pub practice_tool_dps: u32,
    pub practice_tool_last_hit: u32,
    pub practice_tool_total: u32,
    pub quest_complete: u32,
    pub quest_received: u32,
    pub score: u32,
    pub score_dark_star: u32,
    pub score_project0: u32,
    pub score_project1: u32,
    pub shield_bonus_damage: u32,
    pub special: u32,
    pub tft_unit_label: u32,
    pub true_damage: u32,
    pub true_damage_critical: u32,
    pub unk_0x32383120: u32,
    pub unk_0x3388fb50: u32,
    pub unk_0x4a7b08fa: u32,
    pub unk_0x76599c8b: u32,
    pub unk_0x81f43042: u32,
    pub unk_0x9591979a: u32,
    pub unk_0xa82223b3: u32,
    pub unk_0xa9222546: u32,
    pub unk_0xce96732e: u32,
    pub unk_0xcf504aff: u32,
    pub unk_0xee6adc8c: u32,
    pub unk_0xf816d439: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct FogOfWarProperties {
    pub edge_tint_point: f32,
    pub fade_finish: f32,
    pub fade_start: f32,
    pub mutator_to_texture_map: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowTerrainHeightSolver {
    pub m_height_offset: Option<f32>,
    pub m_max_slope: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FontLocaleResolutions {
    pub locale_name: Option<String>,
    pub resolutions: Vec<FontResolution>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FontLocaleType {
    pub font_file_path_bold: Option<String>,
    pub locale_name: Option<String>,
    pub m_font_file_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FontResolution {
    pub font_size: Option<u32>,
    pub outline_size: Option<u32>,
    pub screen_height: Option<u32>,
    pub shadow_depth_x: Option<i32>,
    pub shadow_depth_y: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct FontResolutionData {
    pub auto_scale: Option<bool>,
    pub locale_resolutions: Vec<FontLocaleResolutions>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct FontType {
    pub locale_types: Vec<FontLocaleType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ForEachInCustomTableBlock {
    pub dest_key: ScriptTableSet,
    pub dest_value: ScriptTableSet,
    pub is_disabled: Option<bool>,
    pub sequence: Option<Box<ScriptSequence>>,
    pub sorted_by_keys: Option<BoolGet>,
    pub table: CustomTableGet,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameCalculation {
    pub m_display_as_percent: Option<bool>,
    pub m_expanded_tooltip_calculation_display: Option<u8>,
    pub m_formula_parts: Option<Vec<EnumAbilityResourceByCoefficientCalculationPart>>,
    pub m_multiplier: Option<EnumAbilityResourceByCoefficientCalculationPart>,
    pub m_precision: Option<i32>,
    pub m_simple_tooltip_calculation_display: Option<u8>,
    pub result_modifier: Option<u8>,
    pub tooltip_only: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameCalculationConditional {
    pub m_conditional_calculation_requirements: HasBuffCastRequirement,
    pub m_conditional_game_calculation: u32,
    pub m_default_game_calculation: u32,
    pub m_expanded_tooltip_calculation_display: Option<u8>,
    pub m_simple_tooltip_calculation_display: Option<u8>,
    pub tooltip_only: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameCalculationModified {
    pub m_expanded_tooltip_calculation_display: Option<u8>,
    pub m_modified_game_calculation: u32,
    pub m_multiplier: EnumAbilityResourceByCoefficientCalculationPart,
    pub m_override_spell_level: Option<i32>,
    pub m_simple_tooltip_calculation_display: Option<u8>,
    pub tooltip_only: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GameFontDescription {
    pub color: Option<[u8; 4]>,
    pub colorblind_color: Option<[u8; 4]>,
    pub colorblind_outline_color: Option<[u8; 4]>,
    pub fill_texture_name: Option<String>,
    pub glow_color: Option<[u8; 4]>,
    pub name: String,
    pub outline_color: Option<[u8; 4]>,
    pub resolution_data: u32,
    pub selection_box_color: Option<[u8; 4]>,
    pub shadow_color: Option<[u8; 4]>,
    pub type_data: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GameModeAutoItemPurchasingConfig {
    pub m_item_group_to_avoid_selling_and_set_cost_ceiling: u32,
    pub m_max_purchase_attempts_per_session: i32,
    pub m_randomly_purchased_rec_item_block_types: Vec<String>,
    pub m_sequentially_purchased_rec_item_block_types: Vec<String>,
    pub m_situational_purchased_rec_item_block_types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GameModeChampionList {
    pub m_champions: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameModeConstantBool {
    pub m_value: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameModeConstantFloat {
    pub m_value: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameModeConstantInteger {
    pub m_value: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameModeConstantString {
    pub m_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameModeConstantStringVector {
    pub m_value: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameModeConstantTraKey {
    pub m_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameModeConstantVector3f {
    pub m_value: Option<Vec3>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GameModeConstants {
    pub m_groups: HashMap<u32, GameModeConstantsGroup>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GameModeConstantsGroup {
    pub m_constants: Option<HashMap<u32, EnumGameModeConstant>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameModeEventDefinition {
    pub unk_0x40a56dcf: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GameModeEventList {
    pub events: HashMap<String, GameModeEventDefinition>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GameModeItemList {
    pub m_items: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GameModeMapData {
    pub additional_property_data_paths: Option<Vec<String>>,
    pub announcements_mapping: Option<u32>,
    pub configs: Option<Vec<u32>>,
    pub configs_client: Option<Vec<u32>>,
    pub default_jungle_path_recommendation: Option<u32>,
    pub default_respawn_points: Option<u32>,
    pub item_lists: Option<Vec<u32>>,
    pub jungle_recommendation_map_information: Option<u32>,
    pub level_controllers: Option<Vec<u32>>,
    pub m_auto_item_purchasing_config: Option<u32>,
    pub m_champion_indicator_enabled: Option<bool>,
    pub m_champion_lists: Option<Vec<u32>>,
    pub m_cursor_config: Option<u32>,
    pub m_cursor_config_update: Option<u32>,
    pub m_death_times: Option<u32>,
    pub m_experience_curve_data: Option<u32>,
    pub m_experience_mod_data: Option<u32>,
    pub m_game_mode_constants: Option<u32>,
    pub m_gameplay_config: Option<u32>,
    pub m_item_shop_data: Option<u32>,
    pub m_map_locators: Option<u32>,
    pub m_mode_name: u32,
    pub m_perk_replacements: Option<PerkReplacementList>,
    pub m_relative_colorization: Option<bool>,
    pub m_stats_ui_data: Option<u32>,
    pub m_surrender_settings: Option<u32>,
    pub unk_0xdc2bc473: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GameModeSpellList {
    pub spells: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GameModeTeamFightViewController {
    pub base_loadable: u32,
    pub parent_scene: u32,
    pub path_hash_to_self: u64,
    pub team_data: Vec<UiTeamFightTeamData>,
    pub tooltip_position: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GameMutatorExpansions {
    pub m_expanded_mutator: String,
    pub m_mutators: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GameStateViewController {
    pub base_loadable: u32,
    pub metrics: Vec<EnumUiMetric>,
    pub path_hash_to_self: u64,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GamepadAxisInputSourceFloat {
    pub axis: u8,
    pub invert: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GamepadButtonInputSourceBool {
    pub button: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GameplayConfig {
    pub ability_haste_max: f32,
    pub extended_visibility_duration: f32,
    pub m_adaptive_force_ability_power_scale: f32,
    pub m_adaptive_force_attack_damage_scale: f32,
    pub m_auto_attack_min_post_cast_lockout_delta_time_sec: f32,
    pub m_auto_attack_min_pre_cast_lockout_delta_time_sec: f32,
    pub m_basic_attack_calculation: GameCalculation,
    pub m_cc_score_multipliers: CcScoreMultipliers,
    pub m_item_root_spell_origination: u32,
    pub m_item_sell_queue_time: f32,
    pub m_legacy_summoner_spells: Vec<u32>,
    pub m_lethality_percent_given_at_level0: f32,
    pub m_lethality_ratio_from_attacker: f32,
    pub m_lethality_ratio_from_target: f32,
    pub m_lethality_scales_caps_at_level: i32,
    pub m_lethality_scales_to_level: i32,
    pub m_minion_aa_helper_limit: f32,
    pub m_minion_auto_leeway: f32,
    pub m_minion_death_delay: f32,
    pub m_perk_root_spell_origination: u32,
    pub m_summoner_spells: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GammaParameters {
    pub brightness: f32,
    pub contrast: f32,
    pub gamma: f32,
    pub level: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GdsMapObject {
    pub box_max: Option<Vec3>,
    pub box_min: Option<Vec3>,
    pub extra_info: Option<Vec<GdsMapObjectBannerInfo>>,
    pub eye_candy: Option<bool>,
    pub m_visibility_flags: Option<u8>,
    pub name: String,
    pub r#type: Option<u8>,
    pub transform: Mat4,
    pub visibility_controller: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GdsMapObjectBannerInfo {
    pub banner_data: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GearData {
    pub enable_override_idle_effects: Option<bool>,
    pub m_character_submeshes_to_hide: Option<Vec<u32>>,
    pub m_character_submeshes_to_show: Option<Vec<u32>>,
    pub m_equip_animation: Option<String>,
    pub m_portrait_icon: Option<String>,
    pub m_self_only_portrait_icon: Option<String>,
    pub m_vfx_resource_resolver: ResourceResolver,
    pub override_idle_effects: Option<Vec<SkinCharacterDataPropertiesCharacterIdleEffect>>,
    pub skin_mesh_properties: Option<SkinMeshDataProperties>,
    pub square_portrait_icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GearSelectionViewController {
    pub cooldown_animations: Vec<u32>,
    pub enabled_animations: Vec<u32>,
    pub gear_buttons: Vec<u32>,
    pub gear_menu_scene: u32,
    pub intro_animations: Vec<u32>,
    pub loadable: u32,
    pub path_hash_to_self: u64,
    pub toggle_button: u32,
    pub toggle_scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GearSkinUpgrade {
    pub m_gear_data: GearData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetGameStartCountdownTime {
    pub dest: FloatTableSet,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetKeyValueInCustomTableBlock {
    pub dest: ScriptTableSet,
    pub is_disabled: Option<bool>,
    pub src_key: Box<EnumAddLevelTimer>,
    pub src_table: CustomTableGet,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetModePreloadFlags {
    pub mode_preload_flags: IntTableSet,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetSizeOfCustomTableBlock {
    pub size: IntTableSet,
    pub table: CustomTableGet,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct GlobalStatsUiData {
    pub base_output_icon_modifier: String,
    pub bonus_output_icon_modifier: String,
    pub char_level_icon_key: String,
    pub formula_part_range_style: String,
    pub formula_part_range_style_bonus: String,
    pub formula_part_range_style_bonus_percent: String,
    pub formula_part_range_style_percent: String,
    pub formula_part_style: String,
    pub formula_part_style_bonus: String,
    pub formula_part_style_bonus_percent: String,
    pub formula_part_style_percent: String,
    pub m_char_level_scaling_tag_key: String,
    pub m_expanded_tooltip_calculation_expansion: u8,
    pub m_mana_icon_key: String,
    pub m_mana_scaling_tag_key: String,
    pub m_number_style: String,
    pub m_number_style_bonus: String,
    pub m_number_style_bonus_percent: String,
    pub m_number_style_percent: String,
    pub m_number_style_total_and_coefficient: String,
    pub m_number_style_total_and_coefficient_percent: String,
    pub m_stat_ui_data: HashMap<u8, StatUiData>,
    pub m_tooltip_calculation_expansion: u8,
    pub new_stat_ui_data: Option<HashMap<u8, StatUiData>>,
    pub number_style_total_and_formula: String,
    pub number_style_total_and_scaling_icons: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GlowCenteredOverlayTipStyle {
    pub directional_tip_elements: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GoldTooltipTraKeys {
    pub body_key: String,
    pub title_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GravityHeightSolver {
    pub m_gravity: Option<f32>,
    pub unk_0x922c17e5: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HasAllSubRequirementsCastRequirement {
    pub m_sub_requirements: Vec<Box<EnumHasAllSubRequirementsCastRequirement>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HasAtleastNSubRequirementsCastRequirement {
    pub m_sub_requirements: Vec<Box<EnumHasAllSubRequirementsCastRequirement>>,
    pub m_successes_required: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HasBuffCastRequirement {
    pub m_buff_name: Option<u32>,
    pub m_from_anyone: Option<bool>,
    pub m_invert_result: Option<bool>,
    pub unk_0x7b66f15d: Option<bool>,
    pub unk_0xd6b6109c: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HasBuffDynamicMaterialBoolDriver {
    pub m_deactivate_early_seconds: Option<f32>,
    pub m_script_name: Option<String>,
    pub spell: Option<u32>,
    pub unk_0x149271dd: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HasBuffOfTypeBoolDriver {
    pub buff_type: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HasBuffRequirement {
    pub m_buff_name: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HasGearDynamicMaterialBoolDriver {
    pub m_gear_index: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HasNNearbyUnitsRequirement {
    pub m_distance_type: Option<u32>,
    pub m_range: f32,
    pub m_units_required: u32,
    pub m_units_requirements: Vec<Box<EnumHasAllSubRequirementsCastRequirement>>,
    pub unk_0x675c02b0: Option<u32>,
    pub unk_0xcec18a6a: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HasNNearbyVisibleUnitsRequirement {
    pub m_distance_type: Option<u32>,
    pub m_range: f32,
    pub m_units_required: u32,
    pub m_units_requirements: Vec<Box<EnumHasAllSubRequirementsCastRequirement>>,
    pub unk_0x990ecf03: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HasTypeAndStatusFlags {
    pub m_affects_type_flags: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HasUnitTagsCastRequirement {
    pub m_unit_tags: ObjectTags,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HealthBarData {
    pub backdrop: Option<u32>,
    pub extra_bars: Option<HealthBarExtraBarsData>,
    pub fade_data: Option<HealthBarFadeData>,
    pub frame: Option<u32>,
    pub health_bar: BarTypeMap,
    pub incoming_damage_bar: Option<u32>,
    pub max_hp_penalty_bar: Option<u32>,
    pub max_hp_penalty_divider: Option<u32>,
    pub text_data: Option<HealthBarTextData>,
    pub tick_style: Option<EnumHealthBarTickStyle>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HealthBarExtraBarsData {
    pub all_shield_bar: u32,
    pub champ_specific_bar: Option<BarTypeMap>,
    pub disguise_health_bar: Option<u32>,
    pub incoming_heal_bar: Option<BarTypeMap>,
    pub magic_shield_bar: Option<u32>,
    pub physical_shield_bar: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HealthBarFadeData {
    pub fade_bar: BarTypeMap,
    pub fade_speed: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HealthBarTextData {
    pub display_mode: Option<u8>,
    pub health_value_text: u32,
    pub include_max_health: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HealthBarTickStyleHero {
    pub micro_tick: u32,
    pub micro_tick_per_standard_tick_data: Vec<MicroTicksPerStandardTickData>,
    pub standard_tick: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HealthBarTickStyleTftCompanion {
    pub standard_tick: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HealthBarTickStyleUnit {
    pub standard_tick: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HealthMeter {
    pub fade_bar: u32,
    pub meter: u32,
    pub value_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HermiteSplineInfo {
    pub m_control_point1: Option<Vec3>,
    pub m_control_point2: Option<Vec3>,
    pub m_start_position_offset: Option<Vec3>,
    pub m_use_missile_position_as_origin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct HeroFloatingInfoBarData {
    pub anchor: u32,
    pub borders: HeroFloatingInfoBorderData,
    pub burst_fade_meter_other: u32,
    pub burst_fade_meter_self: u32,
    pub burst_heal_meter_ally: u32,
    pub burst_heal_meter_enemy: u32,
    pub burst_heal_meter_self: u32,
    pub burst_heal_meter_self_colorblind: u32,
    pub character_state_indicators: Option<HeroFloatingInfoCharacterStateIndicatorData>,
    pub damage_flash_meter: u32,
    pub death_anim_ally: u32,
    pub death_anim_enemy: u32,
    pub divider: u32,
    pub health_bar: HealthBarData,
    pub icons: HeroFloatingInfoIconsData,
    pub par_bar: Option<AbilityResourceBarData>,
    pub sar_bar: Option<AbilityResourceBarData>,
    pub sar_pips: Option<AbilityResourcePipsData>,
    pub scene: u32,
    pub scripted_threshold_types: HashMap<u32, u32>,
    pub unit_status: UiUnitStatusData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HeroFloatingInfoBorderData {
    pub additional_status_icons: Option<HashMap<u32, u32>>,
    pub default_border: HeroFloatingInfoBorderTypeData,
    pub defense_modifier_icons: Option<HeroFloatingInfoBorderDefenseIconData>,
    pub executable_border: HeroFloatingInfoBorderTypeData,
    pub has_attached_ally_border: Option<HeroFloatingInfoBorderTypeData>,
    pub invulnerable_border: Option<HeroFloatingInfoBorderTypeData>,
    pub level_text: u32,
    pub level_text_color_ally: Option<[u8; 4]>,
    pub level_text_color_enemy: Option<[u8; 4]>,
    pub level_text_color_self_colorblind: Option<[u8; 4]>,
    pub spell_shield_border: Option<HeroFloatingInfoBorderTypeData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HeroFloatingInfoBorderDefenseIconData {
    pub defense_down_icons: Vec<HeroFloatingInfoBorderDefenseIconThresholdData>,
    pub defense_up_icon: HeroFloatingInfoBorderDefenseIconThresholdData,
    pub left_icon_region: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HeroFloatingInfoBorderDefenseIconThresholdData {
    pub armor_icon: u32,
    pub combo_icon: u32,
    pub defense_modifier_threshold: f32,
    pub magic_resist_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HeroFloatingInfoBorderTypeData {
    pub anim_in: Option<u32>,
    pub border: u32,
    pub highlight: u32,
    pub level_box_overlay_ally: Option<u32>,
    pub level_box_overlay_enemy: Option<u32>,
    pub level_box_overlay_self: Option<u32>,
    pub level_box_overlay_self_colorblind: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HeroFloatingInfoCharacterStateIndicatorData {
    pub character_states_map: HashMap<u32, HeroFloatingInfoCharacterStateIndicatorList>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HeroFloatingInfoCharacterStateIndicatorList {
    pub state_indicator_list: Vec<Unk0x85a6a05c>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HeroFloatingInfoIconData {
    pub border: u32,
    pub highlight_anim: Option<u32>,
    pub icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HeroFloatingInfoIconsData {
    pub icons: Vec<HeroFloatingInfoIconData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct HighlighterViewController {
    pub ability_links: Vec<u32>,
    pub ability_prompts: Vec<AbilityPrompt>,
    pub additional_level_up_links: Vec<u32>,
    pub base_loadable: u32,
    pub level_up_links: Vec<u32>,
    pub links: Vec<u32>,
    pub path_hash_to_self: u64,
    pub scene_link: u32,
    pub summoner_spell_links: Vec<u32>,
    pub unk_0xf554980c: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct HomeViewController {
    pub base_loadable: u32,
    pub battlepass_button: u32,
    pub battlepass_spacer: u32,
    pub battlepass_toast_view: HomeViewSpecialOffer,
    pub button_vertical_layout: u32,
    pub complete_text: u32,
    pub event_hub_button: u32,
    pub event_timer: TftEventTimer,
    pub level_text: u32,
    pub new_pip: Unk0x6241da2,
    pub pass_text: u32,
    pub path_hash_to_self: u64,
    pub play_game_button: u32,
    pub play_game_button_hint: TftHintUiData,
    pub premium_text: u32,
    pub promo_banner_view: HomeViewPromoBanner,
    pub reward_ready_vfx: u32,
    pub rewards_hint_message: TftHintUiData,
    pub rewards_meter: UiMilestoneProgressMeter,
    pub scene: u32,
    pub special_offer_view: HomeViewSpecialOffer,
    pub store_button: u32,
    pub store_new_pip: Unk0x6241da2,
    pub tablet_override_loadable: u32,
    pub team_planner_button: u32,
    pub theme_music_state: String,
    pub theme_music_state_group: String,
    pub troves_button: u32,
    pub unk_0x2f7e9ed: String,
    pub unk_0x33507ef0: String,
    pub unk_0x3928b393: u32,
    pub unk_0xc2f2bae7: HashMap<String, u32>,
    pub unk_0xc7d21863: u32,
    pub unk_0xd24a7af6: u32,
    pub unk_0xd3eab8e5: Unk0xd3eab8e5,
    pub unk_0xd4e6868d: u32,
    pub unk_0xe5640335: u32,
    pub unk_0xe9431731: u32,
    pub vietnamese_rating_label: u32,
    pub xbox_game_pass_ended_dialog_view_controller: u32,
    pub xbox_game_pass_intro_dialog_view_controller: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeViewPromoBanner {
    pub elements: Vec<EnumElementData>,
    pub fade_duration: f32,
    pub fade_easing: u8,
    pub layout: u32,
    pub scene_view_pane: u32,
    pub unk_0x557095fe: Unk0x42dcedbc,
    pub unk_0x7099ea0e: u8,
    pub unk_0x8190d5ac: f32,
    pub unk_0x82ee166: u32,
    pub unk_0xa3e52539: u8,
    pub unk_0xa658987c: HashMap<u32, Unk0xc3737f3e>,
    pub unk_0xa81cb29a: u32,
    pub unk_0xd81ce014: f32,
    pub unk_0xeb49eb3f: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeViewSpecialOffer {
    pub special_offer_button: u32,
    pub special_offer_toast_scene: Option<u32>,
    pub store_listing_data: u32,
    pub unk_0x9028d1ed: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudAbilityResourceThresholdIndicator {
    pub threshold_indicator_elements: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudBannerData {
    pub pulse_duration: f32,
    pub pulse_offset: Vec2,
    pub pulse_time: f32,
    pub transition_offset: Vec2,
    pub transition_time: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudCalloutIdentifier {
    pub frame: u32,
    pub portrait: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudCenterFrameGlowData {
    pub ease_type: Option<u8>,
    pub icon: u32,
    pub unk_0xcdeba821: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct HudColorData {
    pub ability_resource_threshold_range_high_color: [u8; 4],
    pub ability_resource_threshold_range_low_color: [u8; 4],
    pub ability_resource_threshold_range_medium_color: [u8; 4],
    pub chat_combo_box_option_default_color: [u8; 4],
    pub chat_combo_box_option_highlighted_color: [u8; 4],
    pub kill_callout_bounty_color: [u8; 4],
    pub m_all_chat_color: [u8; 4],
    pub m_chaos_chat_color: [u8; 4],
    pub m_chaos_color: [u8; 4],
    pub m_death_chaos_color: [u8; 4],
    pub m_death_enemy_team_color: [u8; 4],
    pub m_death_friendly_team_color: [u8; 4],
    pub m_death_order_color: [u8; 4],
    pub m_death_recap_magic_damage_color: [u8; 4],
    pub m_death_recap_physical_damage_color: [u8; 4],
    pub m_death_recap_true_damage_color: [u8; 4],
    pub m_enemy_chat_color: [u8; 4],
    pub m_enemy_color: [u8; 4],
    pub m_enemy_lane_minion_bar_color: [u8; 4],
    pub m_evolution_color: [u8; 4],
    pub m_feedback_chat_color: [u8; 4],
    pub m_friendly_chat_color: [u8; 4],
    pub m_friendly_color: [u8; 4],
    pub m_friendly_lane_minion_bar_color: [u8; 4],
    pub m_gold_chat_color: [u8; 4],
    pub m_highlighted_indicator_color: [u8; 4],
    pub m_input_chat_color: [u8; 4],
    pub m_item_callout_body_color: [u8; 4],
    pub m_item_callout_item_color: [u8; 4],
    pub m_item_hot_key_disabled_color: [u8; 4],
    pub m_item_hot_key_enabled_color: [u8; 4],
    pub m_jungle_plant_color: [u8; 4],
    pub m_level_up_color: [u8; 4],
    pub m_marked_indicator_color: [u8; 4],
    pub m_network_chat_color: [u8; 4],
    pub m_neutral_chat_color: [u8; 4],
    pub m_neutral_color: [u8; 4],
    pub m_order_chat_color: [u8; 4],
    pub m_order_color: [u8; 4],
    pub m_party_chat_color: [u8; 4],
    pub m_ping_chat_color: [u8; 4],
    pub m_platform_chat_color: [u8; 4],
    pub m_selected_indicator_color: [u8; 4],
    pub m_self_color: [u8; 4],
    pub m_spell_hot_key_disabled_color: [u8; 4],
    pub m_spell_hot_key_enabled_color: [u8; 4],
    pub m_stat_boosted_color: [u8; 4],
    pub m_stat_normal_color: [u8; 4],
    pub m_summoner_name_dead_color: [u8; 4],
    pub m_summoner_name_default_color: [u8; 4],
    pub m_summoner_name_self_color: [u8; 4],
    pub m_team_chat_color: [u8; 4],
    pub m_timestamp_chat_color: [u8; 4],
    pub m_whisper_color: [u8; 4],
    pub quest_tracker_ongoing_color: [u8; 4],
    pub tft_penalized_stat_color: [u8; 4],
    pub vote_empty_color: [u8; 4],
    pub vote_nocolor: [u8; 4],
    pub vote_yes_color: [u8; 4],
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct HudItemShopData {
    pub all_item_group_definitions: Vec<HudItemShopItemGroupDefinition>,
    pub all_items_header_region: u32,
    pub all_items_header_text: u32,
    pub all_items_header_text_dimmed: u32,
    pub all_items_icon_definition: u32,
    pub all_items_item_region: u32,
    pub assassin_items_filter_button_definition: u32,
    pub attack_items_filter_button_definition: u32,
    pub boots_panel_definition: HudItemShopExpandingPanelDefinition,
    pub build_tree_display_region_definition: u32,
    pub build_tree_icon_definition: u32,
    pub build_tree_top_icon_scale: f32,
    pub builds_into_droplist_backdrop_definition: u32,
    pub builds_into_droplist_item_hover_definition: u32,
    pub builds_into_droplist_scene: u32,
    pub builds_into_icon_definition: u32,
    pub builds_into_overflow_button: u32,
    pub builds_into_slot_definition: Vec<u32>,
    pub builds_into_text_overflow_scale: f32,
    pub bundle_item_list_region_definition: u32,
    pub bundle_item_purchase_button_definition: u32,
    pub bundle_postfix_tra_key: String,
    pub close_button_definition: u32,
    pub commonly_built_icon_definition: u32,
    pub consumables_panel_definition: HudItemShopExpandingPanelDefinition,
    pub default_potion: Vec<u32>,
    pub defense_items_filter_button_definition: u32,
    pub horizontal_build_tree_connector: u32,
    pub inventory_icon_definition: u32,
    pub inventory_panel_definition: HudItemShopExpandingPanelDefinition,
    pub invert_group_display_order_button_definition: u32,
    pub item_description_definition: HudItemShopItemDetailsDefinition,
    pub item_details_scene_view_pane: u32,
    pub item_icon_template_scene: u32,
    pub item_sets_combo_box_definition: u32,
    pub item_sets_header_region: u32,
    pub item_sets_header_text: u32,
    pub item_sets_icon_definition: u32,
    pub item_sets_item_region: u32,
    pub item_shop_search_scene_view_pane: u32,
    pub item_shop_search_view_item_region_definition: u32,
    pub item_shop_search_view_rec_label_definition: u32,
    pub item_shop_sell_overlay_drag_area_definition: u32,
    pub item_shop_sell_overlay_scene: u32,
    pub item_shop_sell_overlay_sell_icon_default_definition: u32,
    pub item_shop_sell_overlay_sell_icon_hover_definition: u32,
    pub item_shop_sell_overlay_sell_item_text_definition: u32,
    pub magical_items_filter_button_definition: u32,
    pub marksman_items_filter_button_definition: u32,
    pub page_template_item_icon_definition: u32,
    pub player_debt_text: u32,
    pub player_gold_icon: u32,
    pub player_gold_text: u32,
    pub popular_items_filter_button_definition: u32,
    pub purchase_button_definition: u32,
    pub quick_buy_icon_definition: u32,
    pub rec_item_card_definition: u32,
    pub rec_item_card_tag_scale: f32,
    pub rec_item_commonly_built_slot_definition: Vec<u32>,
    pub rec_item_slots_definition: Vec<u32>,
    pub search_block_main: u32,
    pub search_clear_text_icon: u32,
    pub search_enable_area: u32,
    pub search_icon_definition: u32,
    pub search_input_edit: u32,
    pub search_input_edit_disabled_text: u32,
    pub search_item_description_definition: HudItemShopItemDetailsDefinition,
    pub sell_button_definition: u32,
    pub show_all_items_filter_button_definition: u32,
    pub stat_filter_button_definitions: u32,
    pub tab_button_definition: u32,
    pub tab_view_all_items_scene_view_pane: u32,
    pub tab_view_item_sets_scene_view_pane: u32,
    pub tab_view_rec_items_scene_view_pane: u32,
    pub tile_item_icon_definition: u32,
    pub undo_button_definition: u32,
    pub unk_0xa8ca51f6: Vec<u32>,
    pub utility_items_filter_button_definition: u32,
    pub vertical_build_tree_connector: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudItemShopExpandingPanelDefinition {
    pub closed: u32,
    pub expanded_region: u32,
    pub hover_vfx: u32,
    pub opened: u32,
    pub pin_button: u32,
    pub scene_data: u32,
    pub slots: Vec<u32>,
    pub tutorial_vfx: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct HudItemShopItemDetailsClickDataDefinition {
    pub click_region_definition: u32,
    pub hover_icon_definition: u32,
    pub item_brief_text_definition: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudItemShopItemDetailsDefinition {
    pub body_definition: u32,
    pub click_data: Option<u32>,
    pub cost_text_definition: u32,
    pub gold_icon_definition: u32,
    pub item_icon_definition: u32,
    pub title_text_definition: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudItemShopItemGroupDefinition {
    pub epicnesses_in_group: Vec<u8>,
    pub group_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct HudItemShopItemIconDefinition {
    pub cooldown_effect_data: Option<CooldownEffectUiData>,
    pub cost_text: Option<u32>,
    pub cost_text_selected: Option<u32>,
    pub cost_text_unpurchaseable: Option<u32>,
    pub frame_icon: u32,
    pub hit_region: u32,
    pub hover_frame_icon: u32,
    pub hover_icon: Option<u32>,
    pub item_icon: u32,
    pub locked_hover_icon: Option<u32>,
    pub locked_icon: Option<u32>,
    pub mythic_frame_icon: u32,
    pub mythic_purchaseable_vfx: Option<u32>,
    pub mythic_purchased_vfx: Option<u32>,
    pub name_text: Option<u32>,
    pub offset_region: Option<u32>,
    pub ornn_frame: u32,
    pub popular_icon: Option<u32>,
    pub purchaseable_vfx: Option<u32>,
    pub purchased_overlay: Option<u32>,
    pub recently_changed_icon: Option<u32>,
    pub selected_icon: u32,
    pub selected_vfx: Option<u32>,
    pub unpurchaseable_overlay: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudItemShopQuickBuyData {
    pub default_index: Option<i32>,
    pub number_of_rows: u32,
    pub sort_popular_to_front: Option<bool>,
    pub unk_0xa4b002d7: Vec<Unk0x68f56a56>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct HudItemShopRecItemCardDefinition {
    pub advice_char_border0: u32,
    pub advice_char_border1: u32,
    pub advice_char_border_hover0: u32,
    pub advice_char_border_hover1: u32,
    pub advice_char_icon0: u32,
    pub advice_char_icon1: u32,
    pub advice_empty_icon_default: u32,
    pub advice_empty_icon_mythic_hover: u32,
    pub advice_empty_icon_non_mythic_hover: u32,
    pub advice_empty_text: u32,
    pub advice_empty_text_hover: u32,
    pub advice_icon_default: u32,
    pub advice_icon_hover_mythic: u32,
    pub advice_icon_hover_non_mythic: u32,
    pub advice_label: u32,
    pub brief_text: u32,
    pub brief_text_backdrop: u32,
    pub bundle_item_frame_hover_icon: u32,
    pub bundle_item_frame_icon: u32,
    pub bundle_item_frame_unpurchasable: u32,
    pub bundle_item_more_tag: u32,
    pub bundle_item_more_tag_hover: u32,
    pub bundle_item_more_text: u32,
    pub bundle_item_primary_icon: u32,
    pub bundle_item_secondary_icon: u32,
    pub bundle_stack_primary_text: u32,
    pub bundle_stack_secondary_text: u32,
    pub card_default: u32,
    pub card_hover_mythic: u32,
    pub card_hover_mythic_vfx: u32,
    pub card_hover_non_mythic: u32,
    pub card_hover_non_mythic_vfx: u32,
    pub card_refresh_mythic_vfx: u32,
    pub card_refresh_non_mythic_vfx: u32,
    pub card_selected_mythic: u32,
    pub card_selected_mythic_vfx: u32,
    pub card_selected_non_mythic: u32,
    pub card_selected_non_mythic_vfx: u32,
    pub cost_text: u32,
    pub cost_text_selected: u32,
    pub cost_text_unpurchaseable: u32,
    pub frame_icon: u32,
    pub hit_region: u32,
    pub hover_frame_icon: u32,
    pub item_icon: u32,
    pub locked_hover_icon: u32,
    pub locked_icon: u32,
    pub mythic_frame_icon: u32,
    pub name_text: u32,
    pub ornn_frame: u32,
    pub purchased_overlay: u32,
    pub recently_changed_icon: u32,
    pub unpurchaseable_overlay: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudKillCalloutTemplateData {
    pub assisters: Vec<HudCalloutIdentifier>,
    pub background: u32,
    pub event_icon_altar_capture: u32,
    pub event_icon_kill_bounty: u32,
    pub event_icon_kill_normal: u32,
    pub event_icon_kill_shutdown: u32,
    pub event_icon_kill_streak: u32,
    pub event_icon_revive_ally: u32,
    pub global_spell_icon: HudCalloutIdentifier,
    pub slot_area: u32,
    pub source: HudCalloutIdentifier,
    pub target: HudCalloutIdentifier,
    pub unk_0x1f03d8c: HudCalloutIdentifier,
    pub unk_0x20e913e7: u32,
    pub unk_0x25799a22: u32,
    pub unk_0x40cfe5a8: u32,
    pub unk_0x848b3faa: u32,
    pub unk_0xcef418bd: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudMenuTransitionData {
    pub easing_type: Option<u8>,
    pub max_alpha: Option<u8>,
    pub min_alpha: Option<u8>,
    pub transition_start_delay_secs: Option<f32>,
    pub transition_time: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudMessageDisplayData {
    pub message_count: Option<u32>,
    pub message_duration_secs: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudPlayerResourceBars {
    pub ar_threshold_indicator: Option<HudAbilityResourceThresholdIndicator>,
    pub experience_bar: u32,
    pub experience_hit_region: u32,
    pub health_animated_meter_skin: UiElementMeterSkin,
    pub health_hit_region: u32,
    pub health_meter: HealthMeter,
    pub health_regen_text: u32,
    pub par_hit_region: u32,
    pub par_meter_data: AbilityResourceBarData,
    pub par_regen_text: u32,
    pub sar_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudReciprocityButton {
    pub button: u32,
    pub hotkey_text: u32,
    pub progress_fill: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudReplayEventLineBaseIcons {
    pub frame_center: u32,
    pub frame_left: u32,
    pub frame_right: u32,
    pub icon_center: u32,
    pub icon_kill: u32,
    pub icon_left: u32,
    pub icon_right: u32,
    pub team_highlight0: u32,
    pub team_highlight1: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudReplayEventTooltip {
    pub event_backdrop: u32,
    pub replay_event_line_base_icons: HudReplayEventLineBaseIcons,
    pub scene_handle: u32,
    pub time_backdrop_handle: u32,
    pub time_text_handle: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudReplaySlider {
    pub slider: u32,
    pub slider_backdrop: u32,
    pub slider_progress_bar: u32,
    pub tooltip: HudReplayEventTooltip,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct HudReplaySliderData {
    pub m_icon_data_priority_list: Vec<HudReplaySliderIconData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudReplaySliderIconData {
    pub element_t0: Option<u32>,
    pub element_t1: Option<u32>,
    pub element_text_t0: Option<u32>,
    pub element_text_t1: Option<u32>,
    pub m_element_alpha_default: Option<f32>,
    pub m_element_alpha_selected: Option<f32>,
    pub m_element_alpha_unselected: Option<f32>,
    pub m_element_spacer: Option<f32>,
    pub m_tooltip_icon_names: Option<Vec<String>>,
    pub m_tooltip_style: Option<u8>,
    pub m_type: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudShopButton {
    pub inactive_icon: u32,
    pub shop_button: u32,
    pub text_link: u32,
    pub unk_0x34a1434b: Option<u32>,
    pub unk_0x697f8b6b: Option<u32>,
    pub unk_0x778e26c6: Option<u32>,
    pub unk_0x7dffe581: Option<String>,
    pub unk_0x8031b7a0: Option<u32>,
    pub unk_0xb77375ae: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HudTipTrackerMessageTemplate {
    pub background_default: u32,
    pub background_hover: u32,
    pub click_region: u32,
    pub icon_default: u32,
    pub icon_hover: u32,
    pub slot_area: u32,
    pub tip_body_text: u32,
    pub tip_title_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IconData {
    pub base_icon: IconLink,
    pub icons_for_currency: HashMap<u8, IconLink>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IconLink {
    pub active_icon: u32,
    pub clicked_icon: u32,
    pub disabled_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IconStateData {
    pub icon_active_hover: u32,
    pub icon_inactive_hover: u32,
    pub unk_0x5786f895: u32,
    pub unk_0xcf3d6a50: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentityInstance {
    pub id: String,
    pub item_vfx: Option<u32>,
    pub m_item_texture_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdleParticlesVisibilityEventData {
    pub m_fire_if_animation_ends_early: Option<bool>,
    pub m_is_self_only: Option<bool>,
    pub m_show: Option<bool>,
    pub m_start_frame: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct IncrementalPurchaseDialog {
    pub base_loadable: u32,
    pub close_button_definition: u32,
    pub confirm_button_definition: u32,
    pub content_scene: u32,
    pub incremental_body_text: u32,
    pub incremental_close_button_region: u32,
    pub incremental_down_button: u32,
    pub incremental_limit_text: u32,
    pub incremental_purchase_item_icon: u32,
    pub incremental_qty_text: u32,
    pub incremental_scene: u32,
    pub incremental_ten_down_button: u32,
    pub incremental_ten_up_button: u32,
    pub incremental_title_text: u32,
    pub incremental_up_button: u32,
    pub path_hash_to_self: u64,
    pub purchase_button: u32,
    pub resizable_backdrop: u32,
    pub store_dialog: u32,
    pub unk_0x385cc32b: u32,
    pub unk_0xf83118bc: UiHyperlink,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndicatorTypeGlobal {
    pub m_is_floating: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InhibitorWaveBehavior {
    pub spawn_count_per_inhibitor_down: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct InputControlSet {
    pub events: Vec<u32>,
    pub mutually_exlusive_sets: Option<Vec<u32>>,
    pub name: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct InputEvent2Axis {
    pub name: u32,
    pub replicated: Option<bool>,
    pub unk_0x7c96ce2b: Option<bool>,
    pub unk_0x8e568fa0: Vec<EnumInputSource>,
    pub unk_0x93cf6a73: Vec<EnumInputSource>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct InputEventBool {
    pub name: u32,
    pub repeat_rate: Option<f32>,
    pub unk_0x8c61159d: Vec<EnumInputSource>,
    pub unk_0xc43abb5a: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InputSourceBoolAsFloat {
    pub bool_source: KeyInputSourceBool,
    pub on_value: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InputSourceFloatAsBool {
    pub float_source: GamepadAxisInputSourceFloat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InsertIntoCustomTableBlock {
    pub dest_index: Option<IntGet>,
    pub dest_table: CustomTableGet,
    pub is_disabled: Option<bool>,
    pub out_index: Option<IntTableSet>,
    pub unk_0xd851ffa3: String,
    pub value: Box<EnumAddLevelTimer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IntGet {
    pub value: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IntTableGet {
    pub table: Option<EnumVars>,
    pub var: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IntTableSet {
    pub table: Option<EnumVars>,
    pub var: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IntegratedValueFloat {
    pub constant_value: Option<f32>,
    pub dynamics: VfxAnimatedFloatVariableData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IntegratedValueVector2 {
    pub constant_value: Option<Vec2>,
    pub dynamics: VfxAnimatedVector2fVariableData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IntegratedValueVector3 {
    pub constant_value: Option<Vec3>,
    pub dynamics: VfxAnimatedVector3fVariableData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IsAnimationPlayingDynamicMaterialBoolDriver {
    pub m_animation_names: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IsCastingBoolDriver {
    pub spell_slot: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IsOptionEnabledBoolDriver {
    pub option: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IsSpecifiedUnitCastRequirement {
    pub m_invert_result: Option<bool>,
    pub m_unit: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemRecOverrideRange {
    pub items: Vec<u32>,
    pub max_completed_items: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemRecommendationChoices {
    pub m_choices: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemRecommendationContext {
    pub m_champion_id: u32,
    pub m_completed_item_matrix: ItemRecommendationMatrix,
    pub m_is_default_position: Option<bool>,
    pub m_map_id: u32,
    pub m_mode_name_string_id: u32,
    pub m_popular_items: Vec<u32>,
    pub m_position: u32,
    pub m_starting_item_bundles: Vec<ItemRecommendationItemList>,
    pub m_starting_item_matrix: ItemRecommendationMatrix,
    pub upgrade_choices: Option<HashMap<u32, ItemRecommendationItemList>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ItemRecommendationContextList {
    pub m_all_recommendable_item_ids: HashMap<u32, ItemRecommendationItemList>,
    pub m_all_starting_item_ids: HashMap<u32, ItemRecommendationItemList>,
    pub m_contexts: Vec<ItemRecommendationContext>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemRecommendationItemList {
    pub m_item_list: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemRecommendationMatrix {
    pub mrows: Vec<ItemRecommendationMatrixRow>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemRecommendationMatrixRow {
    pub m_choices_map: Option<HashMap<String, ItemRecommendationChoices>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ItemRecommendationOverride {
    pub m_core_items: Option<Vec<u32>>,
    pub m_force_override: Option<bool>,
    pub m_override_contexts: Vec<ItemRecommendationOverrideContext>,
    pub m_rec_item_ranges: Option<Vec<ItemRecOverrideRange>>,
    pub starting_item_bundles: Option<Vec<ItemRecommendationOverrideStartingItemBundle>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemRecommendationOverrideContext {
    pub m_map_id: Option<u32>,
    pub m_mode_name_string_id: u32,
    pub m_position: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ItemRecommendationOverrideSet {
    pub m_overrides: Vec<ItemRecommendationOverride>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemRecommendationOverrideStartingItemBundle {
    pub items: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ItemSelectionViewController {
    pub item_slots: Vec<UiItemSelectionSlotData>,
    pub loadable: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ItemShopGameModeData {
    pub all_item_group_definitions_override: Option<Unk0xac27b13a>,
    pub boots_quick_buy_data: HudItemShopQuickBuyData,
    pub completed_items: Vec<u32>,
    pub consumables_quick_buy_data: HudItemShopQuickBuyData,
    pub purchased_item_exclusion_items: Vec<u32>,
    pub rec_fallback_map_id: u32,
    pub rec_fallback_mode_name: u32,
    pub rec_items_swaps: Option<HashMap<u32, u32>>,
    pub rec_override_smite_starting_items: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ItemShopViewController {
    pub base_loadable: u32,
    pub drag_region: u32,
    pub path_hash_to_self: u64,
    pub resize_drag_region: u32,
    pub scene_handle: u32,
    pub unk_0xb9cd03d1: Unk0x39a9452c,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemSlotDetailedUiData {
    pub ammo_fx: Option<u32>,
    pub backdrop: u32,
    pub border_default: Option<u32>,
    pub border_disabled: Option<u32>,
    pub border_enabled: Option<u32>,
    pub border_selected: Option<u32>,
    pub complete_fx: Option<u32>,
    pub cooldown_effects: Option<CooldownEffectUiData>,
    pub hit_area: Option<u32>,
    pub hotkey_text: Option<u32>,
    pub icon: u32,
    pub major_active: Option<u32>,
    pub new_item_fx: Option<u32>,
    pub overlay_disabled: u32,
    pub overlay_hover: Option<u32>,
    pub overlay_loc: Option<u32>,
    pub overlay_oom: Option<u32>,
    pub stack_text: Option<u32>,
    pub toggle_fx: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemSlotSimpleUiData {
    pub backdrop: u32,
    pub cooldown_effects: CooldownEffectUiData,
    pub hit_area: u32,
    pub icon: u32,
    pub stack_text: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JointOrientationEventData {
    pub blend_data: Unk0x125a3586,
    pub m_end_frame: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JointSnapEventData {
    pub m_end_frame: Option<f32>,
    pub m_fire_if_animation_ends_early: Option<bool>,
    pub m_is_self_only: Option<bool>,
    pub m_joint_name_to_override: Option<u32>,
    pub m_joint_name_to_snap_to: Option<u32>,
    pub m_name: Option<u32>,
    pub m_start_frame: Option<f32>,
    pub offset: Option<Vec3>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JungleLocationMapInformation {
    pub index: Option<u8>,
    pub location: Option<u8>,
    pub name: String,
    pub position: Vec3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JunglePath {
    pub recommendation_icons: Vec<RecommendedJunglePathIcons>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct JunglePathRecommendation {
    pub chaos_jungle_path: Vec<EnumTakeCamp>,
    pub order_jungle_path: Vec<EnumTakeCamp>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct JungleRecommendationMapInformation {
    pub jungle_location_information: Vec<JungleLocationMapInformation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyFrameFloatClipReaderDriver {
    pub clip_accessory_to_read: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyFrameFloatMapClipAccessoryData {
    pub key_frame_floatmap: Option<HashMap<u32, f32>>,
    pub name: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyInputSourceBool {
    pub key_sequence: String,
    pub unk_0x371f336e: Option<Unk0xf07517b3>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct KillCalloutsViewController {
    pub base_loadable: u32,
    pub item_scene_template: u32,
    pub message_template: HudKillCalloutTemplateData,
    pub path_hash_to_self: u64,
    pub use_minimap_scale: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LaunchAreaData {
    pub indicator_texture_name: String,
    pub inner_area_target_distance: f32,
    pub inner_radius: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LayoutStyleGrid {
    pub horizontal_fill_direction: Option<u8>,
    pub horizontal_justification: Option<u8>,
    pub row_horizontal_alignment: Option<u8>,
    pub row_vertical_alignment: Option<u8>,
    pub unk_0x2c24fe60: Option<u8>,
    pub vertical_fill_direction: Option<u8>,
    pub vertical_justification: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LayoutStyleHorizontalList {
    pub horizontal_fill_direction: Option<u8>,
    pub horizontal_justification: Option<u8>,
    pub row_vertical_alignment: Option<u8>,
    pub vertical_justification: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LayoutStyleVerticalList {
    pub column_horizontal_alignment: Option<u8>,
    pub horizontal_justification: Option<u8>,
    pub vertical_fill_direction: Option<u8>,
    pub vertical_justification: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeagueRotatingFlavor {
    pub flavor_name: String,
    pub individual_banner_overrides: Vec<RotatingBanner>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LearnedSpellDynamicMaterialBoolDriver {
    pub m_slot: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LerpMaterialDriver {
    pub m_bool_driver: Box<EnumDriver>,
    pub m_off_value: Option<f32>,
    pub m_on_value: Option<f32>,
    pub m_turn_off_time_sec: Option<f32>,
    pub m_turn_on_time_sec: Option<f32>,
    pub m_use_broken_old_interpolation: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LerpVec4LogicDriver {
    pub bool_driver: Box<EnumDriver>,
    pub off_value: Option<Vec4>,
    pub on_value: Option<Vec4>,
    pub turn_off_time_sec: Option<f32>,
    pub turn_on_time_sec: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct LevelControlScript {
    pub functions: HashMap<u32, ScriptFunction>,
    pub guid: String,
    pub path: u32,
    pub script_name: String,
    pub sequences: HashMap<u32, RootScriptSequence>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LevelScriptFunctionLink {
    pub function_name: u32,
    pub level_script_link: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinearTransformProcessorData {
    pub m_increment: Option<f32>,
    pub m_multiplier: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct LoadScreenTipConfiguration {
    pub m_duration_in_game: f32,
    pub m_pbi_tip_duration_on_loading_screen: f32,
    pub m_show_pbi_tips_on_loading_screen: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenBasicViewController {
    pub background: LoadingScreenGameModeBackground,
    pub base_loadable: u32,
    pub path_hash_to_self: u64,
    pub progress_meter: LoadingScreenProgressMeter,
    pub scene: u32,
    pub version: VersionString,
    pub vietnamese_rating_label: UiMetricVietnameseRatingLabel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenChallengeTokenData {
    pub challenge_token_capstone_icon: u32,
    pub challenge_token_group: u32,
    pub challenge_token_hit_region: u32,
    pub challenge_token_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenChallengesData {
    pub challenge_border_crystals_data: HashMap<u32, String>,
    pub challenge_crystals_data: HashMap<u32, String>,
    pub challenge_token_data: LoadingScreenChallengeTokenData,
    pub challenges_token_layout: u32,
    pub summoner_challenge_border_crystal: Vec<u32>,
    pub summoner_challenge_crystal: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenChampionMasteryData {
    pub mastery_only_details_text: Option<u32>,
    pub mastery_only_group: u32,
    pub mastery_only_icon: u32,
    pub mastery_only_title_text: u32,
    pub mastery_side_icon: u32,
    pub slot_details_text: u32,
    pub slot_group: u32,
    pub slot_hit_region: u32,
    pub slot_icon: u32,
    pub slot_layout: u32,
    pub slot_title_text: u32,
    pub unk_0xa846c9e1: u32,
    pub unk_0xdbd4a8f0: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenClash {
    pub chaos_team: LoadingScreenClashTeam,
    pub order_team: LoadingScreenClashTeam,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenClashTeam {
    pub team_logo_icon: u32,
    pub team_name_text: u32,
    pub teamtag_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenClassicViewController {
    pub background: LoadingScreenGameModeBackground,
    pub base_loadable: u32,
    pub clash: LoadingScreenClash,
    pub clash_loadable: u32,
    pub latency: LoadingScreenLatency,
    pub path_hash_to_self: u64,
    pub progress_meter: LoadingScreenProgressMeter,
    pub scene: u32,
    pub tips: LoadingScreenTips,
    pub unk_0xf2af7d00: u32,
    pub version: VersionString,
    pub vietnamese_rating_label: UiMetricVietnameseRatingLabel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenEmblemData {
    pub emblem_elements: HashMap<u32, u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenGameModeBackground {
    pub background: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenHonorFlairData {
    pub flair_icons: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenLatency {
    pub text: u32,
    pub thresholds: Vec<LoadingScreenLatencyThreshold>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenLatencyThreshold {
    pub image: u32,
    pub text_color: [u8; 4],
    pub threshold: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenPlayerCardClassicData {
    pub border: u32,
    pub challenges: LoadingScreenChallengesData,
    pub character_skin_name: Vec<u32>,
    pub character_splash: u32,
    pub character_splash_emblem: LoadingScreenEmblemData,
    pub hit_region: u32,
    pub honor: LoadingScreenHonorFlairData,
    pub main_scene: u32,
    pub mastery: LoadingScreenChampionMasteryData,
    pub perks: Vec<LoadingScreenPlayerCardClassicSpellData>,
    pub ranked_frames: u32,
    pub regalia: LoadingScreenRegaliaData,
    pub state_scenes: Vec<u32>,
    pub summoner_icon: Vec<u32>,
    pub summoner_name: LoadingScreenSummonerNameData,
    pub summoner_spells: Vec<LoadingScreenPlayerCardClassicSpellData>,
    pub summoner_title: LoadingScreenSummonerTitleData,
    pub unk_0x2189347e: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenPlayerCardClassicSpellData {
    pub frame: u32,
    pub icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenPlayerCardSwitcherData {
    pub player_card_switcher_layout: u32,
    pub tab_data: Vec<LoadingScreenPlayerCardTabData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenPlayerCardSwitcherViewController {
    pub base_loadable: u32,
    pub card_switcher: LoadingScreenPlayerCardSwitcherData,
    pub path_hash_to_self: u64,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenPlayerCardTabData {
    pub button: u32,
    pub card_tab: Option<u32>,
    pub hover_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenPlayerCardTftData {
    pub character_banner_skin_name: u32,
    pub character_splash: u32,
    pub character_splash_emblem: LoadingScreenEmblemData,
    pub character_splash_skin_name: u32,
    pub hit_region: u32,
    pub main_scene: u32,
    pub ranked_frames: u32,
    pub rarity: Unk0xb107dfe4,
    pub rating: LoadingScreenRatedData,
    pub regalia: LoadingScreenRegaliaData,
    pub state_scenes: Vec<u32>,
    pub summoner_icon: Vec<u32>,
    pub summoner_name: LoadingScreenSummonerNameData,
    pub unk_0x2ee1834e: Vec<u32>,
    pub unk_0x312f9dd7: Option<u32>,
    pub unk_0xa89213f: Option<Vec<String>>,
    pub unk_0xb86e158b: bool,
    pub unk_0xc1fb53e3: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenPlayerCardsViewController {
    pub base_loadable: u32,
    pub card_template: EnumLoadingScreenPlayerCard,
    pub is_team_based: Option<bool>,
    pub lower_card_region: u32,
    pub path_hash_to_self: u64,
    pub spacer_data: Option<Unk0xacb2dba1>,
    pub upper_card_region: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenProgressMeter {
    pub meter: u32,
    pub text: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenRankedFrames {
    pub chaos_frame: u32,
    pub default_frame: u32,
    pub frame_element_map: HashMap<String, u32>,
    pub order_frame: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenRatedData {
    pub queue_type_to_source_tier_icons: Option<HashMap<String, u32>>,
    pub rated_text: u32,
    pub rated_tier_icon: u32,
    pub unk_0x60d19c3d: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenRatedTierIconData {
    pub rated_tier_source_icons: HashMap<String, u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenRegaliaData {
    pub addons: Vec<u32>,
    pub banner_flag: u32,
    pub banner_trim: u32,
    pub crest_tier: Vec<u32>,
    pub loading_screen_regaliaflags: i32,
    pub prestige_crest_base: u32,
    pub ranked_crest_base: u32,
    pub ranked_regalia_level_text: u32,
    pub summoner_level_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenSummonerNameData {
    pub summoner_name: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenSummonerTitleData {
    pub local_player_title_color: Option<[u8; 4]>,
    pub other_player_title_color: Option<[u8; 4]>,
    pub summoner_title: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadingScreenTips {
    pub condensed_tip_icon: u32,
    pub tip_body_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadoutArenaSkinInfoPanel {
    pub game_pass_icon: u32,
    pub item_icon: u32,
    pub item_icon_text: u32,
    pub item_lock_icon: u32,
    pub scene: u32,
    pub unk_0x8300af56: i32,
    pub unk_0xbd2f7d09: String,
    pub unk_0xd02a6781: Option<u32>,
    pub unk_0xee425ddc: u32,
    pub view_pane_link: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadoutCompanionInfoPanel {
    pub game_pass_icon: u32,
    pub item_icon: u32,
    pub item_icon_text: u32,
    pub just_obtained_icon: Vec<u32>,
    pub lock_icon: Vec<u32>,
    pub scene: u32,
    pub tier2_button_feedback_vfx: u32,
    pub tier3_button_feedback_vfx: u32,
    pub tier_button: Vec<u32>,
    pub unk_0x4b8c1a00: Option<u32>,
    pub unk_0x5683a573: Option<u32>,
    pub unk_0x8300af56: i32,
    pub unk_0x96e6f1ba: Option<u32>,
    pub unk_0xbd2f7d09: String,
    pub unk_0xd02a6781: Option<u32>,
    pub unk_0xee425ddc: u32,
    pub unk_0xf0ed80c8: Option<String>,
    pub unowned_star_shard_icon: Vec<u32>,
    pub unowned_tier_button: Vec<u32>,
    pub upgrade_button_disabled_from_entitlement_text: String,
    pub upgrade_entitlement_error_button: u32,
    pub upgrade_entitlement_error_tooltip_text: String,
    pub upgrade_error_text: u32,
    pub upgrade_info_text: u32,
    pub upgrade_tier_button: Vec<u32>,
    pub view_pane_link: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadoutDamageSkinInfoPanel {
    pub game_pass_icon: u32,
    pub item_icon: u32,
    pub item_icon_text: u32,
    pub lock_icon: Vec<u32>,
    pub scene: u32,
    pub tier_button: Vec<u32>,
    pub unk_0x5683a573: Option<u32>,
    pub unk_0x8300af56: i32,
    pub unk_0xbd2f7d09: String,
    pub unk_0xd02a6781: Option<u32>,
    pub unk_0xee425ddc: u32,
    pub unowned_tier_button: Vec<u32>,
    pub view_pane_link: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadoutEmoteInfoPanel {
    pub emote_start_button: u32,
    pub emote_start_button_icon: u32,
    pub emote_victory_button: u32,
    pub emote_victory_button_icon: u32,
    pub emote_wheel_c_button: u32,
    pub emote_wheel_c_button_icon: u32,
    pub emote_wheel_e_button: u32,
    pub emote_wheel_e_button_icon: u32,
    pub emote_wheel_n_button: u32,
    pub emote_wheel_n_button_icon: u32,
    pub emote_wheel_ne_button: u32,
    pub emote_wheel_ne_button_icon: u32,
    pub emote_wheel_nw_button: u32,
    pub emote_wheel_nw_button_icon: u32,
    pub emote_wheel_s_button: u32,
    pub emote_wheel_s_button_icon: u32,
    pub emote_wheel_se_button: u32,
    pub emote_wheel_se_button_icon: u32,
    pub emote_wheel_sw_button: u32,
    pub emote_wheel_sw_button_icon: u32,
    pub emote_wheel_w_button: u32,
    pub emote_wheel_w_button_icon: u32,
    pub scene: u32,
    pub unk_0xbd2f7d09: Option<String>,
    pub view_pane_link: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadoutItemListPanelData {
    pub button_area: u32,
    pub title_area: u32,
    pub title_template: u32,
    pub title_text_template: u32,
    pub view_panel_scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct LoadoutSelectViewController {
    pub base_loadable: u32,
    pub loadouts_button_data: Vec<LoadoutsButtonData>,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub unk_0x17b25007: u32,
    pub unk_0x3b45db25: String,
    pub unk_0x3f65fe98: u32,
    pub unk_0x46ecbeca: u32,
    pub unk_0x5b0b57db: String,
    pub unk_0xc7ee357: u32,
    pub unk_0xe279bff: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct LoadoutUpgradeDialogViewController {
    pub base_loadable: u32,
    pub close_button_definition: u32,
    pub confirm_button_definition: u32,
    pub content_scene: u32,
    pub error_text: u32,
    pub item_frame: u32,
    pub item_icon: u32,
    pub path_hash_to_self: u64,
    pub star_level2_vfx: u32,
    pub star_level3_vfx: u32,
    pub unk_0xd02a6781: u32,
    pub upgrade_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct LoadoutViewController {
    pub arena_info_panel: LoadoutArenaSkinInfoPanel,
    pub arena_skin_loadout_grid_button_data: ArenaSkinLoadoutGridButtonData,
    pub arena_skin_loadout_item_list_panel_data: LoadoutItemListPanelData,
    pub base_loadable: u32,
    pub close_button: u32,
    pub companion_info_panel: LoadoutCompanionInfoPanel,
    pub companion_loadout_grid_button_data: CompanionLoadoutGridButtonData,
    pub companion_loadout_item_list_panel_data: LoadoutItemListPanelData,
    pub damage_skin_info_panel: LoadoutDamageSkinInfoPanel,
    pub damage_skin_loadout_grid_button_data: DamageSkinLoadoutGridButtonData,
    pub damage_skin_loadout_item_list_panel_data: LoadoutItemListPanelData,
    pub emote_info_panel: LoadoutEmoteInfoPanel,
    pub emote_loadout_grid_button_data: EmoteLoadoutGridButtonData,
    pub emote_loadout_item_list_panel_data: LoadoutItemListPanelData,
    pub equip_button: u32,
    pub grid_item_button: u32,
    pub loadout_scene: u32,
    pub path_hash_to_self: u64,
    pub searchbar_text: u32,
    pub sound_on_activate: String,
    pub sound_on_de_activate: String,
    pub star_shard_currency_button: u32,
    pub star_shard_currency_widget: TftCurrencyWidget,
    pub star_shard_decrease_vfx: u32,
    pub star_shards_store_dialog: u32,
    pub tablet_override_loadable: u32,
    pub unk_0x2eed7e1b: u32,
    pub unk_0x4b90d43d: u32,
    pub unk_0xa95ede35: u32,
    pub unk_0xbfd5b639: u32,
    pub unk_0xcee40baf: u32,
    pub unk_0xd2bd41f4: String,
    pub unk_0xd5846a0a: u32,
    pub unk_0xe0ece3d9: u32,
    pub unk_0xfaf537f0: u32,
    pub upgrade_button: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadoutsButtonData {
    pub button_definition: u32,
    pub display_name_tra_key: String,
    pub illustration_icon: u32,
    pub loadout_type: Option<u32>,
    pub region: u32,
    pub unk_0xe10174d4: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LobbyBottomButtons {
    pub close_button: u32,
    pub invite_button: u32,
    pub restriction_icon: u32,
    pub start_button: u32,
    pub team_planner_button: u32,
    pub unk_0x3d7745f6: u32,
    pub unk_0x599fcd6e: String,
    pub unk_0x5b26202d: u32,
    pub unk_0x7f4e86a0: String,
    pub vietnamese_rating_label: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LobbyLabFields {
    pub popup_view_controller: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LobbyLayout {
    pub custom_button_popup_anchor: Option<u32>,
    pub max_player_count: Option<u32>,
    pub other_player_portrait_data: LobbyPlayerData,
    pub self_portrait_data: LobbyPlayerData,
    pub self_swappable: Option<bool>,
    pub slot_handles: Option<Vec<u32>>,
    pub team_size: Option<u32>,
    pub unk_0x2179027d: Option<Vec<u32>>,
    pub unk_0x46cf8fa8: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LobbyPlayerData {
    pub add_friend_button: Option<u32>,
    pub arena_skin_icon: Option<u32>,
    pub custom_button: Option<u32>,
    pub custom_button_icon: Option<u32>,
    pub edit_button: Option<u32>,
    pub empty_slot_button: Option<u32>,
    pub frame_ranked: Option<u32>,
    pub frame_unranked: Option<u32>,
    pub group: u32,
    pub kick_button: Option<u32>,
    pub little_legend_icon: Option<u32>,
    pub loadouts_button: Option<u32>,
    pub loadouts_frame: Option<u32>,
    pub name_text: u32,
    pub party_leader_icon: u32,
    pub rank_text: u32,
    pub rank_text_line2: Option<u32>,
    pub summoner_icon: u32,
    pub summoner_icon_frame: u32,
    pub swap_button: Option<u32>,
    pub unk_0x4150d546: Option<u32>,
    pub unk_0x44adf547: Option<u32>,
    pub unk_0x602b4f3c: Option<u32>,
    pub unk_0xa3af8181: Option<u32>,
    pub unk_0xa4c75290: Option<u32>,
    pub unk_0xe45fff2c: Option<u32>,
    pub use_leader_controls_modal: Option<bool>,
    pub warning: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LobbyReadyCheck {
    pub accept_button: u32,
    pub accepted_vfx: u32,
    pub decline_button: u32,
    pub default_queue_id_title_tra_key: String,
    pub match_found_text: u32,
    pub match_info_text: u32,
    pub queue_id_title_tra_keys: HashMap<i64, String>,
    pub scene_handle: u32,
    pub timer_text: u32,
    pub timer_vfx: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LockRootOrientationEventData {
    pub blend_out_time: Option<f32>,
    pub joint_name: Option<u32>,
    pub m_end_frame: Option<f32>,
    pub m_start_frame: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogicDriverBoolParametricUpdater {
    pub driver: Option<EnumDriver>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogicDriverConditionalElementsEntry {
    pub enable_condition: EnumDriver,
    pub unk_0x55d13682: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogicDriverFloatParametricUpdater {
    pub driver: EnumDriver,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct LogicDriverViewController {
    pub loadable: u32,
    pub path_hash_to_self: u64,
    pub unk_0x4facf6d1: Option<Vec<LogicDriverViewEntry>>,
    pub unk_0xc7e51dac: Option<Vec<Unk0x26949c34>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogicDriverViewEntry {
    pub enable_condition: EnumDriver,
    pub scene: u32,
    pub unk_0x40c7c429: Option<Vec<Unk0x4f0aa8a0>>,
    pub unk_0x8d424715: Option<Vec<Unk0xfeacedf2>>,
    pub unk_0x9f6debf3: Option<Vec<LogicDriverConditionalElementsEntry>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct LolGameStateViewController {
    pub base_loadable: u32,
    pub draw_area_list: Option<DrawAreaList>,
    pub metrics: Vec<EnumUiMetric>,
    pub path_hash_to_self: u64,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct LolSpellScript {
    pub casting_breaks_stealth: Option<bool>,
    pub functions: Option<HashMap<u32, ScriptFunction>>,
    pub global_properties: Option<ScriptGlobalProperties>,
    pub path: u32,
    pub script_name: String,
    pub sequences: Option<HashMap<u32, RootScriptSequence>>,
    pub unk_0x532c1075: Option<bool>,
    pub unk_0x77e82748: Option<bool>,
    pub unk_0xf491308f: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoopBlock {
    pub dest: IntTableSet,
    pub end: Box<EnumAddLevelTimer>,
    pub is_disabled: Option<bool>,
    pub sequence: Box<ScriptSequence>,
    pub start: Box<EnumAddLevelTimer>,
    pub step: Option<IntGet>,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LooseUiTextureData {
    pub texture_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LooseUiTextureData3SliceH {
    pub edge_sizes_left_right: Option<Vec2>,
    pub texture_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LooseUiTextureData3SliceV {
    pub edge_sizes_top_bottom: Vec2,
    pub texture_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LooseUiTextureData9Slice {
    pub edge_sizes_left_right: Option<Vec2>,
    pub edge_sizes_top_bottom: Option<Vec2>,
    pub texture_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LootItemLineElements {
    pub loot_line_arrow_icon: u32,
    pub loot_line_item_button: u32,
    pub loot_line_item_drop_rate_text: u32,
    pub loot_line_item_group: u32,
    pub loot_line_item_name_text: u32,
    pub loot_line_item_spacing: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct LootTableDialogViewController {
    pub base_loadable: u32,
    pub close_button_definition: u32,
    pub content_scene: u32,
    pub loot_item_line_elements: LootItemLineElements,
    pub loot_line_header_text: u32,
    pub loot_table_title_text: u32,
    pub loot_tablelayout: u32,
    pub no_duplicates_button: u32,
    pub no_duplicates_tra_key: String,
    pub path_hash_to_self: u64,
    pub rollover_protection_button: u32,
    pub rollover_protection_tra_key: String,
    pub root_arrow_collapsed_texture_path: String,
    pub root_arrow_expanded_texture_path: String,
    pub searchbar_text: u32,
    pub sub_arrow_collapsed_texture_path: String,
    pub sub_arrow_expanded_texture_path: String,
    pub tablet_override_loadable: u32,
    pub unk_0x3a79eb8b: String,
    pub unk_0x5ba7b7c9: String,
    pub unk_0x749fcb22: String,
    pub unk_0x791accf7: String,
    pub unk_0xa6124714: String,
    pub unk_0xa9f3c6c8: u32,
    pub unk_0xb43e780f: String,
    pub unk_0xb7d3735d: String,
    pub unk_0xc5e35c96: String,
    pub unk_0xc99290c6: String,
    pub unk_0xead14f26: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    pub character_lists: Vec<u32>,
    pub fog_of_war_properties: u32,
    pub initial_visibility_mask: u8,
    pub map_skins: Vec<u32>,
    pub map_string_id: String,
    pub unk_0x30eafcaa: u8,
    pub unk_0x31af8e97: HashMap<u32, u32>,
    pub unk_0xd31ac6ce: MapVisibilityFlagDefinitions,
    pub visibility_flag_defines: MapVisibilityFlagDefinitions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapAlternateAsset {
    pub audio_bank_units: Vec<BankUnit>,
    pub m_fow_overlay_texture_name: String,
    pub m_grass_tint_texture_name: String,
    pub m_navmesh_fx_mask_name: Option<String>,
    pub m_particle_resource_resolver: u32,
    pub m_visibility_flag_name: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapAlternateAssets {
    pub m_alternate_assets: Vec<MapAlternateAsset>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapAnimatedProp {
    pub idle_animation_name: String,
    pub m_visibility_flags: Option<u8>,
    pub name: String,
    pub play_idle_animation: bool,
    pub prop_name: String,
    pub skin_id: Option<u32>,
    pub transform: Mat4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapAudio {
    pub event_name: String,
    pub name: String,
    pub transform: Mat4,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MapAudioDataProperties {
    pub bank_units: Vec<BankUnit>,
    pub base_data: Option<u32>,
    pub mobile_mix_event: Option<String>,
    pub pc_mix_event: Option<String>,
    pub unk_0xa02b6406: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapBakeProperties {
    pub light_grid_size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MapCharacterList {
    pub characters: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MapContainer {
    pub bounds_max: Vec2,
    pub bounds_min: Option<Vec2>,
    pub chunks: HashMap<u32, u32>,
    pub components: Vec<EnumAddLevelTimer>,
    pub convert_streams_to_half_float: Option<bool>,
    pub lowest_walkable_height: f32,
    pub map_path: String,
    pub mesh_combine_radius: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapCubemapProbe {
    pub cubemap_probe_path: String,
    pub cubemap_probe_scale: f32,
    pub cubemap_region: Option<MapCubemapRegion>,
    pub m_visibility_flags: Option<u8>,
    pub name: String,
    pub transform: Mat4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapCubemapRegion {
    pub max: Vec3,
    pub min: Vec3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapGroup {
    pub name: String,
    pub transform: Mat4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapLocator {
    pub name: String,
    pub transform: Mat4,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MapLocatorArray {
    pub locators: Vec<MapLocator>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapMaterialSwap {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapNavGrid {
    pub nav_grid_config: u32,
    pub nav_grid_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapNavGridOverlay {
    pub name: u32,
    pub nav_grid_file_name: String,
    pub regions_filename: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MapNavGridOverlays {
    pub overlays: Vec<MapNavGridOverlay>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapParticle {
    pub color_modulate: Option<Vec4>,
    pub eye_candy: Option<bool>,
    pub group_name: Option<String>,
    pub m_visibility_flags: Option<u8>,
    pub name: String,
    pub start_disabled: Option<bool>,
    pub system: u32,
    pub transform: Mat4,
    pub transitional: Option<bool>,
    pub visibility_controller: Option<u32>,
    pub visibility_mode: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MapPlaceableContainer {
    pub items: Option<HashMap<u32, EnumAddLevelTimer>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapScriptLocator {
    pub name: String,
    pub script_name: String,
    pub transform: Mat4,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MapSkin {
    pub gamma_parameters: Option<GammaParameters>,
    pub m_alternate_assets: Option<MapAlternateAssets>,
    pub m_colorization_post_effect: Option<MapSkinColorizationPostEffect>,
    pub m_grass_tint_texture: Option<String>,
    pub m_map_container_link: Option<String>,
    pub m_map_objects_cfg: Option<String>,
    pub m_minimap_background_config: MinimapBackgroundConfig,
    pub m_navigation_mesh: Option<String>,
    pub m_object_skin_fallbacks: Option<HashMap<u32, i32>>,
    pub m_resource_resolvers: Vec<u32>,
    pub m_world_particles_ini: Option<String>,
    pub material_swap: Option<MapMaterialSwap>,
    pub name: String,
    pub unk_0x2d3285eb: Option<Vec<Unk0x34a5a0c9>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapSkinColorizationPostEffect {
    pub m_multipliers_rgb: Vec3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapSunProperties {
    pub fog_alternate_color: Option<Vec4>,
    pub fog_color: Option<Vec4>,
    pub fog_enabled: Option<bool>,
    pub fog_start_and_end: Vec2,
    pub ground_color: Vec4,
    pub horizon_color: Vec4,
    pub light_map_color_scale: f32,
    pub sky_light_color: Vec4,
    pub sky_light_scale: f32,
    pub sun_direction: Vec3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapTerrainPaint {
    pub terrain_paint_texture_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapVisibilityFlagDefinition {
    pub bit_index: Option<u8>,
    pub name: u32,
    pub public_name: Option<String>,
    pub transition_time: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapVisibilityFlagDefinitions {
    pub flag_definitions: Vec<MapVisibilityFlagDefinition>,
    pub flag_range: MapVisibilityFlagRange,
    pub select_random_visibility_flag_for_map: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapVisibilityFlagRange {
    pub max_index: u8,
    pub min_index: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MaskData {
    pub m_weight_list: Vec<f32>,
    pub mid: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatchHistoryPlayerTemplate {
    pub augment_container_icon: u32,
    pub augment_container_icon_frame: u32,
    pub augment_icons: Vec<u32>,
    pub augment_tooltip_button: u32,
    pub empty_unit_group: u32,
    pub group: u32,
    pub match_date_text: u32,
    pub match_type_duration_text: u32,
    pub placement_banners: Vec<u32>,
    pub placement_text: u32,
    pub player_icon: u32,
    pub trait_grid: u32,
    pub unit_count_text: u32,
    pub unit_grid: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatchHistoryTraitTemplate {
    pub frame_levels: Vec<u32>,
    pub frame_levels_selected: Vec<u32>,
    pub group: u32,
    pub icon: u32,
    pub tooltip_button: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatchHistoryUnitTemplate {
    pub group: u32,
    pub icon: u32,
    pub icon_frame: u32,
    pub item_icons: Vec<u32>,
    pub one_star: u32,
    pub three_star: u32,
    pub tooltip_button: u32,
    pub two_star: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MaxMaterialDriver {
    pub m_drivers: Vec<Box<EnumDriver>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MessageBoxDialog {
    pub base_loadable: u32,
    pub button_scene: u32,
    pub cancel_button_additional_icons: UiElementGroupButtonAdditionalElements,
    pub cancel_button_definition: u32,
    pub close_button_definition: u32,
    pub confirm_button_additional_icons: UiElementGroupButtonAdditionalElements,
    pub confirm_button_definition: u32,
    pub content_scene: u32,
    pub header_body_text: u32,
    pub header_title_text: u32,
    pub input_error_text: u32,
    pub input_scene: u32,
    pub input_text: u32,
    pub main_body_text: u32,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub placeholder_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MicroTicksPerStandardTickData {
    pub micro_ticks_between: u32,
    pub min_health: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinMaterialDriver {
    pub m_drivers: Vec<Box<EnumDriver>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinimapBackground {
    pub m_size: Vec2,
    pub m_texture_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinimapBackgroundConfig {
    pub m_custom_minimap_backgrounds: Option<HashMap<u32, MinimapBackground>>,
    pub m_default_texture_name: String,
    pub unk_0x47dc1276: Option<HashMap<u32, Unk0xd56fb9cc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MinimapData {
    pub behavior_sets: Option<HashMap<u32, u32>>,
    pub m_custom_icons: HashMap<u32, MinimapIcon>,
    pub m_icons: HashMap<u8, MinimapIcon>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinimapIcon {
    pub m_base_color: Option<MinimapIconColorData>,
    pub m_base_texture: Option<MinimapIconTextureData>,
    pub m_max_scale: Option<f32>,
    pub m_min_scale: Option<f32>,
    pub m_relative_teams: Option<bool>,
    pub m_size: Option<Vec2>,
    pub m_team_colors: Option<HashMap<u8, MinimapIconColorData>>,
    pub m_team_textures: Option<HashMap<u8, MinimapIconTextureData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MinimapIconBehaviorSet {
    pub minimap_behaviors: Vec<EnumMinimapIcon>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinimapIconChangeOpacity {
    pub duration: f32,
    pub ease_type: u8,
    pub start_alpha: Option<f32>,
    pub target_alpha: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinimapIconColorData {
    pub m_base: [u8; 4],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinimapIconRotate {
    pub duration: f32,
    pub rotation_per_second_in_degrees: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinimapIconScale {
    pub duration: f32,
    pub start_scale: Vec2,
    pub target_scale: Vec2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinimapIconTextureData {
    pub m_base: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MinimapPingData {
    pub m_omw_ping_range_cutoffs: HashMap<u8, f32>,
    pub m_pings: Vec<MinimapPingTypeContainer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinimapPingEffectAndTextureData {
    pub m_chaos: TextureAndColorData,
    pub m_default: TextureAndColorData,
    pub m_effect: MinimapPingEffectDefinition,
    pub m_enabled: Option<bool>,
    pub m_order: TextureAndColorData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinimapPingEffectDefinition {
    pub alpha_fade_speed: Option<f32>,
    pub alpha_start: Option<u32>,
    pub life: Option<f32>,
    pub on_death_fade_speed: Option<f32>,
    pub repeat_count: Option<u32>,
    pub scale_speed: Option<f32>,
    pub scale_start: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinimapPingTypeContainer {
    pub ping_effect_list: Vec<MinimapPingEffectAndTextureData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MinimapViewController {
    pub base_loadable: u32,
    pub camera_lock_button: Option<u32>,
    pub draw_area_list: Option<DrawAreaList>,
    pub flipped_override: Option<u32>,
    pub frame: u32,
    pub jungle_path: Option<JunglePath>,
    pub jungle_path_button: Option<u32>,
    pub main_scene: u32,
    pub maximum_scale: Option<f32>,
    pub minimap_content: u32,
    pub minimap_tooltip_anchor: u32,
    pub minimap_voice_chat_anchor: u32,
    pub minimum_scale: Option<f32>,
    pub objective_bounties_status: Option<ObjectiveBountiesStatus>,
    pub options_menu_button: u32,
    pub path_hash_to_self: u64,
    pub ping_button: Option<u32>,
    pub unk_0x3d7e26bc: Option<u32>,
    pub unk_0x719dd682: Option<Vec<u32>>,
    pub unk_0x9bc68be3: Option<u32>,
    pub unk_0xd57e38cd: Option<u32>,
    pub unk_0xd5cb0c0f: Option<Vec<u32>>,
    pub voice_chat_button: u32,
    pub voice_chat_button_glow_fx: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinionUpgradeConfig {
    pub armor_max: Option<f32>,
    pub armor_upgrade: Option<f32>,
    pub armor_upgrade_growth: Option<f32>,
    pub damage_max: f32,
    pub damage_upgrade: Option<f32>,
    pub damage_upgrade_late: Option<f32>,
    pub gold_max: Option<f32>,
    pub hp_max_bonus: f32,
    pub hp_upgrade: f32,
    pub hp_upgrade_late: Option<f32>,
    pub magic_resistance_upgrade: Option<f32>,
    pub unk_0x726ae049: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MissileAttachedTargetingDefinition {
    pub m_end_position_type: u8,
    pub m_line_end_texture_height: f32,
    pub m_line_end_texture_name: String,
    pub m_line_end_texture_width: f32,
    pub m_line_texture_width: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MissileGroupSpawnerSpec {
    pub m_child_missile_spell: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MissileSpecification {
    pub behaviors: Option<Vec<EnumAddLevelTimer>>,
    pub height_solver: Option<EnumHeightSolver>,
    pub m_missile_width: Option<f32>,
    pub missile_group_spawners: Option<Vec<MissileGroupSpawnerSpec>>,
    pub movement_component: EnumMovement,
    pub unk_0xc195fba6: Option<bool>,
    pub vertical_facing: Option<EnumFacing>,
    pub visibility_component: Option<EnumDefaultvisibility>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MissionBuffData {
    pub air_drake: TeamBuffData,
    pub dragon: TeamBuffData,
    pub earth_drake: TeamBuffData,
    pub elder_drake: TeamBuffData,
    pub fire_drake: TeamBuffData,
    pub water_drake: TeamBuffData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MissionCategoryButtonDefinition {
    pub button: u32,
    pub category: Option<u32>,
    pub header_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MissionsPanelViewController {
    pub base_loadable: u32,
    pub empty_group: u32,
    pub event_empty_group: u32,
    pub event_missions_completed_key: String,
    pub event_missions_completed_text: u32,
    pub mission_buttons_layout: u32,
    pub mission_category_buttons: Vec<MissionCategoryButtonDefinition>,
    pub mission_full_managed_layout: u32,
    pub objective_meter_anim_ease_type: u8,
    pub objective_meter_anim_time_secs: f32,
    pub path_hash_to_self: u64,
    pub tablet_override_loadable: u32,
    pub title_text: u32,
    pub view_pane_link: u32,
    pub weekly_mission_template: UiWeeklyMissionTemplate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MobileCatalogData {
    pub active: Option<bool>,
    pub incremental_purchase: Option<bool>,
    pub item_instance_id: String,
    pub purchase_rate_limit: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MobileQualitySettings {
    pub m_default_quality: u32,
    pub unk_0x2c0e972c: Vec<Unk0xf5022dc7>,
    pub unk_0x70a6123f: Vec<Unk0x7ac9c856>,
    pub unk_0x83b62f63: HashMap<u32, u32>,
    pub unk_0xf3842cae: Vec<Unk0x3f1c01bb>,
    pub unk_0xfb849597: HashMap<u32, u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ModalDialogViewController {
    pub base_loadable: u32,
    pub close_button_definition: Option<u32>,
    pub confirm_button_definition: Option<u32>,
    pub content_scene: u32,
    pub path_hash_to_self: u64,
    pub unk_0x38eaaebf: Option<bool>,
    pub unk_0xbfd5b639: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ModalShroudManager {
    pub base_loadable: u32,
    pub input_handler: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModeSelectQueueButtonData {
    pub button: u32,
    pub button_clicked_icon: u32,
    pub button_default_icon: u32,
    pub button_disabled_icon: u32,
    pub button_hover_icon: u32,
    pub illustration_icon: u32,
    pub new_pip: Unk0x6241da2,
    pub subtitle_text: u32,
    pub unk_0x38a08ce2: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ModeSelectViewController {
    pub base_loadable: u32,
    pub clicked_icon_color: [u8; 4],
    pub default_icon_color: [u8; 4],
    pub disabled_icon_color: [u8; 4],
    pub disabled_icon_texture: String,
    pub event_timer_definition: TftEventTimer,
    pub hover_icon_color: [u8; 4],
    pub icon_background: u32,
    pub layout: u32,
    pub mode_select_queue_button_data: ModeSelectQueueButtonData,
    pub path_hash_to_self: u64,
    pub queues: Vec<u32>,
    pub scene: u32,
    pub sound_on_activate: String,
    pub sound_on_de_activate: String,
    pub unk_0x175bb989: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MouseOverEffectData {
    pub m_ally_color: [u8; 4],
    pub m_avatar_color_factor: f32,
    pub m_avatar_size: i32,
    pub m_enemy_color: [u8; 4],
    pub m_interaction_sizes: Vec<i32>,
    pub m_interaction_times: Vec<f32>,
    pub m_mouse_over_blur_pass_count: u32,
    pub m_mouse_over_color_factor: f32,
    pub m_mouse_over_size: i32,
    pub m_neutral_color: [u8; 4],
    pub m_selected_color_factor: f32,
    pub m_selected_size: i32,
    pub m_self_color: [u8; 4],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultiDragonSoulSlotUiData {
    pub soul_icon_slot: u32,
    pub soul_source_empty_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultiDragonTypeSourceUiData {
    pub soul_active_source_icon: Option<u32>,
    pub soul_known_source_icon: Option<u32>,
    pub soul_preview_buff_name: Option<String>,
    pub soul_reward_buff_name: Option<String>,
    pub team_reward_buff_name: Option<String>,
    pub team_slot_source_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MultiPurchaseDialog {
    pub base_loadable: u32,
    pub body_scroll_scene_view_pane: u32,
    pub close_button_definition: u32,
    pub confirm_button_definition: u32,
    pub content_scene: u32,
    pub header_elements: PurchaseDialogSubPurchaseHeaderElements,
    pub more_info_button: u32,
    pub multi_background_region: u32,
    pub multi_body_text: u32,
    pub multi_close_button_region: u32,
    pub multi_purchase_item_icon: u32,
    pub multi_purchase_managed_layout: u32,
    pub multi_scene: u32,
    pub multi_sub_purchase_section_label: u32,
    pub multi_title_text: u32,
    pub path_hash_to_self: u64,
    pub purchase_button: u32,
    pub resizable_backdrop: u32,
    pub store_dialog: u32,
    pub sub_purchase_elements: PurchaseDialogSubPurchaseElements,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MusicAudioDataProperties {
    pub ambient_event: String,
    pub defeat_banner_sound: String,
    pub defeat_music_id: String,
    pub legacy_theme_music_id: Option<String>,
    pub legacy_theme_music_transition_id: Option<String>,
    pub theme_music_id: String,
    pub theme_music_transition_id: Option<String>,
    pub victory_banner_sound: String,
    pub victory_music_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct MutatorMapVisibilityController {
    pub mutator_name: String,
    pub path_hash: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedDataValueCalculationPart {
    pub m_data_value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct NavGridConfig {
    pub region_groups: Vec<Unk0x2bfb084c>,
    pub terrain_config: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct NavGridTerrainConfig {
    pub tags: Vec<Unk0xd82714cc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct NavHeaderViewController {
    pub back_button: u32,
    pub background: u32,
    pub base_loadable: u32,
    pub battery_levels: Vec<u32>,
    pub fiat_store_dialog: u32,
    pub loading_spinner_scene: u32,
    pub missions_badge_vfx: u32,
    pub missions_button: u32,
    pub missions_button_hint: TftHintUiData,
    pub nav_header_scene: u32,
    pub navigation_text: u32,
    pub notifications_badge_vfx: u32,
    pub notifications_button: u32,
    pub path_hash_to_self: u64,
    pub player_icon: u32,
    pub player_name_text_ranked: u32,
    pub player_name_text_unranked: u32,
    pub player_profile_button: u32,
    pub player_rank_text: u32,
    pub profile_background: u32,
    pub profile_widget_scene: u32,
    pub settings_button: u32,
    pub social_badge_vfx: u32,
    pub social_button: u32,
    pub star_shards_decrease_vfx: u32,
    pub star_shards_increase_sink_vfx: u32,
    pub star_shards_increase_source_vfx: u32,
    pub star_shards_store_dialog: u32,
    pub star_shards_widget: TftCurrencyWidget,
    pub tablet_override_loadable: u32,
    pub team_planner_button: u32,
    pub tft_coins_decrease_vfx: u32,
    pub tft_coins_increase_sink_vfx: u32,
    pub tft_coins_increase_source_vfx: u32,
    pub tft_coins_widget: TftCurrencyWidget,
    pub troves_token_store_listing: u32,
    pub troves_tokens_decrease_vfx: u32,
    pub troves_tokens_increase_sink_vfx: u32,
    pub troves_tokens_increase_source_vfx: u32,
    pub troves_tokens_store_dialog: u32,
    pub troves_tokens_widget: TftCurrencyWidget,
    pub unk_0x53e676c5: TftCurrencyWidget,
    pub unk_0x9b36efaf: TftCurrencyWidget,
    pub unk_0xd0349990: u32,
    pub unk_0xe3efc163: u32,
    pub utility_divider: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NeutralTimerData {
    pub source_icons: Option<Vec<NeutralTimerSourceIconData>>,
    pub timer_elements: NeutralTimerElements,
    pub timer_name: u32,
    pub tooltip: String,
    pub tooltip_camp_name: String,
    pub tooltip_chat_name_chaos: Option<String>,
    pub tooltip_chat_name_order: Option<String>,
    pub tooltip_respawn: Option<String>,
    pub unk_0x3995c23b: Option<bool>,
    pub unk_0x5a4ef4e7: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NeutralTimerElements {
    pub alive_icon: Option<u32>,
    pub backdrop: u32,
    pub dead_icon: u32,
    pub timer_text: u32,
    pub unk_0x22fd5e11: Option<u32>,
    pub unk_0x2ff43743: Option<Unk0x89fa197c>,
    pub unk_0x30c86d07: Option<Unk0xcd54aabc>,
    pub unk_0x8b2da88: Option<u32>,
    pub unk_0xc2bdf068: Option<Unk0xcd54aabc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NeutralTimerSourceIconData {
    pub camp_name: u32,
    pub icon: u32,
    pub tooltip_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NeutralTimerState {
    pub unk_0x56841fae: Option<Vec<u32>>,
    pub unk_0x5a4ef4e7: u32,
    pub unk_0xbe0caaff: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NeutralTimerStateManager {
    pub unk_0x715cf2c1: Vec<NeutralTimerState>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct NeutralTimerViewController {
    pub base_loadable: u32,
    pub esports_loadable: Option<u32>,
    pub neutral_timers: Vec<NeutralTimers>,
    pub path_hash_to_self: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NeutralTimers {
    pub neutral_timer_state_manager: Option<NeutralTimerStateManager>,
    pub scene: u32,
    pub timers: Vec<NeutralTimerData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NotMaterialDriver {
    pub m_driver: Box<EnumDriver>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct NotificationDialog {
    pub base_loadable: u32,
    pub path_hash_to_self: u64,
    pub scene_options: Vec<NotificationDialogOption>,
    pub scene_root: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NotificationDialogOption {
    pub scene: u32,
    pub scene_buttons: Vec<u32>,
    pub scene_texts: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NotificationListItemData {
    pub active_game_gradient: u32,
    pub active_game_invite_icon: u32,
    pub add_friend_icon: u32,
    pub alert_icon: u32,
    pub body_text: u32,
    pub click_region: u32,
    pub generic_icon: u32,
    pub group: u32,
    pub inactive_game_invite_icon: u32,
    pub loot_icon: u32,
    pub loot_item_count_background: u32,
    pub loot_item_count_text: u32,
    pub pass_reward_icon: u32,
    pub timestamp_text: u32,
    pub title_text: u32,
    pub unread_icon: u32,
    pub warning_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettings {
    pub customized_sounds: HashMap<u8, String>,
    pub default_sound: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct NotificationsPanelViewController {
    pub base_loadable: u32,
    pub clear_all_button: u32,
    pub empty_state_group: u32,
    pub list_item_data: NotificationListItemData,
    pub path_hash_to_self: u64,
    pub tablet_override_loadable: u32,
    pub title_text: u32,
    pub view_pane_link: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NumberCalculationPart {
    pub m_number: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct NumberFormattingBehavior {
    pub minimum_digit_count_for_thousands_separator: Option<u32>,
    pub western_number_grouping: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct NumberFormattingData {
    pub billion_abbreviation_tra_key: String,
    pub day_abbreviation_tra_key: String,
    pub decimal_separator_tra_key: String,
    pub hour_abbreviation_tra_key: String,
    pub kilometers_abbreviation_tra_key: String,
    pub localized_formatting_behavior: HashMap<u32, u32>,
    pub meters_abbreviation_tra_key: String,
    pub million_abbreviation_tra_key: String,
    pub minute_abbreviation_tra_key: String,
    pub one_hundred_million_abbreviation_tra_key: String,
    pub percentage_format_tra_key: String,
    pub second_abbreviation_tra_key: String,
    pub ten_thousand_abbreviation_tra_key: String,
    pub thousand_abbreviation_tra_key: String,
    pub thousands_separator_tra_key: String,
    pub trillion_abbreviation_tra_key: String,
    pub week_abbreviation_tra_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObjectTags {
    pub m_object_tag_list: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveBannerViewController {
    pub anim_data: HudBannerData,
    pub backdrop: u32,
    pub base_loadable: u32,
    pub mobile_override_loadable: Option<u32>,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveBountiesStatus {
    pub background: u32,
    pub chaos_decaying: u32,
    pub chaos_filling: u32,
    pub order_decaying: u32,
    pub order_filling: u32,
    pub tooltip_ally_loc_key: String,
    pub tooltip_enemy_loc_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveCampDefinition {
    pub objective_camp_name: u32,
    pub vfx_alignment_give: u32,
    pub vfx_alignment_take: u32,
    pub vfx_collapsed_to_expanded: u32,
    pub vfx_expanded_to_collapsed: u32,
    pub vfx_none_to_collapsed: u32,
    pub vfx_none_to_expanded: u32,
    pub vfx_portrait_looping: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveIconDefinition {
    pub objective_name_tra_key: String,
    pub portrait_icon: u32,
    pub timer_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveVotePanelData {
    pub display_group: u32,
    pub for_button: u32,
    pub for_icon: u32,
    pub objective_text: u32,
    pub objective_text_tra_key: String,
    pub pending_icon: u32,
    pub player_for_icon: u32,
    pub player_reject_icon: u32,
    pub reject_button: u32,
    pub reject_icon: u32,
    pub vfx_player_for: u32,
    pub vfx_player_reject: u32,
    pub vote_holder_group: u32,
    pub votes_layout: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveVoteResultData {
    pub background: u32,
    pub bounding_region: u32,
    pub for_icon: u32,
    pub frame: u32,
    pub group: u32,
    pub objective_portrait: u32,
    pub objective_portrait_voting_aligned: u32,
    pub reject_icon: u32,
    pub time_expired_camp_icon: u32,
    pub time_left: u32,
    pub time_left_bg: u32,
    pub time_left_camp_icon: u32,
    pub vfx_group: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveVoteResultsViewController {
    pub alt_layout_region: u32,
    pub auto_activate_time_before_spawn: f32,
    pub base_loadable: u32,
    pub bonus_votes_for_jungler: u32,
    pub disabled_time_before_spawn: f32,
    pub flipped_override: u32,
    pub layout_region: u32,
    pub objective_camp_info: Vec<ObjectiveCampDefinition>,
    pub objective_monster_icons: HashMap<u32, ObjectiveIconDefinition>,
    pub path_hash_to_self: u64,
    pub result_scene: u32,
    pub result_transition_scene: u32,
    pub scene: u32,
    pub vote_auto_close_delay: f32,
    pub vote_panel_template: ObjectiveVotePanelData,
    pub vote_result_template: ObjectiveVoteResultData,
    pub vote_tally_template: ObjectiveVotetTallyData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveVotetTallyData {
    pub count_bg: u32,
    pub for_count: u32,
    pub for_count_icon: u32,
    pub reject_count: u32,
    pub reject_count_icon: u32,
    pub vfx_player_for: u32,
    pub vfx_player_reject: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OffScreenPoiConfigData {
    pub bezier_max_size: f32,
    pub downscale_bezier: Vec4,
    pub downscale_time: f32,
    pub edge_buffer: u32,
    pub intro_bezier: Vec4,
    pub max_displayed_poi: u8,
    pub offscreen_pings: Vec<OffScreenPoiPing>,
    pub outro_time: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OffScreenPoiItemData {
    pub frame: u32,
    pub portrait: u32,
    pub region: u32,
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OffScreenPoiPing {
    pub duration_secs: f32,
    pub frame: u32,
    pub icon: u32,
    pub max_distance_from_center_of_screen: Option<u32>,
    pub particle_effect: u32,
    pub ping_combination_range: f32,
    pub ping_type: u8,
    pub scaling_data: PoiScalingData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct OffScreenPoiViewController {
    pub base_loadable: u32,
    pub champion_portrait_scene: u32,
    pub main_scene: u32,
    pub off_screen_poi_data: OffScreenPoiItemData,
    pub off_screen_poi_settings: OffScreenPoiConfigData,
    pub path_hash_to_self: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OneTrueMaterialDriver {
    pub m_drivers: Option<Vec<Box<EnumDriver>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemBorder {
    pub template: u32,
    pub unk_0xa4b002d7: Vec<Box<EnumOptionItem>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemButton {
    pub action: u16,
    pub description_tra_key: Option<String>,
    pub filter: Option<EnumOptionItemFilter>,
    pub label_tra_key: String,
    pub show_on_platform: u8,
    pub template: u32,
    pub unk_0x6c83351f: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemCheckbox {
    pub description_tra_key: Option<String>,
    pub filter: Option<EnumOptionItemFilter>,
    pub label_tra_key: String,
    pub live_update: Option<bool>,
    pub negate: Option<bool>,
    pub option: u16,
    pub show_on_platform: Option<u8>,
    pub template: u32,
    pub tooltip_tra_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemColumns {
    pub filter: Option<OptionItemFilterGameStyle>,
    pub items_either: Option<Vec<Box<EnumOptionItem>>>,
    pub items_left: Option<Vec<Box<EnumOptionItem>>>,
    pub items_right: Option<Vec<Box<EnumOptionItem>>>,
    pub live_update: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemDropdown {
    pub filter: Option<EnumOptionItemFilter>,
    pub label_tra_key: String,
    pub live_update: Option<bool>,
    pub option: u16,
    pub template: u32,
    pub tooltip_tra_key: Option<String>,
    pub unk_0xa4b002d7: Vec<OptionItemDropdownItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemDropdownItem {
    pub tra_key: String,
    pub value: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemFilterAnd {
    pub filters: Vec<Box<EnumOptionItemFilter>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemFilterFeatureToggle {
    pub toggle_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemFilterGameStyle {
    pub show_in_lol_game: Option<bool>,
    pub show_in_lol_replay: Option<bool>,
    pub show_in_tft_game: Option<bool>,
    pub show_in_tft_pregame: Option<bool>,
    pub show_in_tft_replay: Option<bool>,
    pub unk_0x47600fd: Option<bool>,
    pub unk_0x64bc3430: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemFilterHwRequirement {
    pub requires_alienware: Option<bool>,
    pub requires_shader_model3: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemFilterMutator {
    pub mutator: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemFilterNot {
    pub filter: Box<EnumOptionItemFilter>,
    pub unk_0x93bef4c5: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemGroup {
    pub filter: Option<EnumOptionItemFilter>,
    pub items: Vec<Box<EnumOptionItem>>,
    pub label_tra_key: String,
    pub live_update: Option<bool>,
    pub template: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemHotkeys {
    pub filter: Option<EnumOptionItemFilter>,
    pub template: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemLabel {
    pub label1_tra_key: String,
    pub label2_tra_key: Option<String>,
    pub show_on_platform: Option<u8>,
    pub template: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemResolutionDropdown {
    pub label_tra_key: String,
    pub template: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemSecondaryHotkeys1Column {
    pub filter: Option<EnumOptionItemFilter>,
    pub header: OptionItemSecondaryHotkeys1ColumnHeader,
    pub live_update: Option<bool>,
    pub template: u32,
    pub unk_0x98f86e0: Vec<OptionItemSecondaryHotkeys1ColumnRow>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemSecondaryHotkeys1ColumnHeader {
    pub column0_label_tra_key: Option<String>,
    pub column1_label_tra_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemSecondaryHotkeys1ColumnRow {
    pub event_name: String,
    pub label_tra_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemSecondaryHotkeys2Column {
    pub header: Option<OptionItemSecondaryHotkeys2ColumnHeader>,
    pub template: u32,
    pub unk_0x98f86e0: Vec<OptionItemSecondaryHotkeys2ColumnRow>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemSecondaryHotkeys2ColumnHeader {
    pub column0_label_tra_key: Option<String>,
    pub column1_label_tra_key: String,
    pub column2_label_tra_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemSecondaryHotkeys2ColumnRow {
    pub event_name: String,
    pub filter: Option<EnumOptionItemFilter>,
    pub label_tra_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemSliderFloat {
    pub filter: Option<EnumOptionItemFilter>,
    pub label_tra_key: String,
    pub live_update: Option<bool>,
    pub option: u16,
    pub scale: Option<f32>,
    pub show_on_platform: Option<u8>,
    pub template: u32,
    pub update_on_drag: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemSliderGraphicsQuality {
    pub template: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemSliderInt {
    pub label_tra_key: String,
    pub live_update: bool,
    pub option: u16,
    pub option_scale: Option<u32>,
    pub template: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemSliderVolume {
    pub label_tra_key: String,
    pub live_update: Option<bool>,
    pub mute_button_template: u32,
    pub mute_option: u16,
    pub option: u16,
    pub template: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItemVoiceInputDeviceDropdown {
    pub label_tra_key: String,
    pub template: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct OptionTemplateBorder {
    pub border: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct OptionTemplateButton {
    pub bounds_element: u32,
    pub button_definition: u32,
    pub description_definition: u32,
    pub unk_0x244362ff: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct OptionTemplateCheckbox {
    pub bounds_element: u32,
    pub button_definition: u32,
    pub label_element: u32,
    pub unk_0x60e0943: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct OptionTemplateDropdown {
    pub bounds_element: u32,
    pub combo_box_definition: u32,
    pub label_element: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct OptionTemplateGroup {
    pub bounds_element: u32,
    pub expand_button_definition: Option<u32>,
    pub label_element: u32,
    pub post_group_padding_region: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct OptionTemplateHotkeys {
    pub bounds_element: u32,
    pub cast_all_button_definition: u32,
    pub hotkey_button_definition: u32,
    pub hotkey_button_text_small: u32,
    pub hotkey_modifier_text: u32,
    pub hotkey_quick_cast_button_definition: u32,
    pub labels: Vec<OptionTemplateHotkeysLabel>,
    pub normal_cast_button_pos: u32,
    pub quick_cast_button_pos: u32,
    pub unk_0xd8b966a9: Vec<OptionTemplateHotkeysKey>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionTemplateHotkeysKey {
    pub event_name: String,
    pub event_name_tra_key: String,
    pub position: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionTemplateHotkeysLabel {
    pub label: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct OptionTemplateLabel {
    pub bounds_element: u32,
    pub label1: u32,
    pub label2: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct OptionTemplateMuteButton {
    pub button_definition: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct OptionTemplateSecondaryHotkeys1Column {
    pub bounds_element: u32,
    pub heading_row_label0: OptionTemplateSecondaryHotkeysLabel,
    pub heading_row_label1: OptionTemplateSecondaryHotkeysLabel,
    pub row_button_column1: OptionTemplateSecondaryHotkeysButton,
    pub row_label_column0: OptionTemplateSecondaryHotkeysLabel,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct OptionTemplateSecondaryHotkeys2Column {
    pub bounds_element: u32,
    pub heading_row_label0: OptionTemplateSecondaryHotkeysLabel,
    pub heading_row_label1: OptionTemplateSecondaryHotkeysLabel,
    pub heading_row_label2: OptionTemplateSecondaryHotkeysLabel,
    pub row_button_column1: OptionTemplateSecondaryHotkeysButton,
    pub row_button_column2: OptionTemplateSecondaryHotkeysButton,
    pub row_label_column0: OptionTemplateSecondaryHotkeysLabel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionTemplateSecondaryHotkeysButton {
    pub button_definition: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionTemplateSecondaryHotkeysLabel {
    pub background_element: u32,
    pub text_element: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct OptionTemplateSlider {
    pub bounds_element: u32,
    pub label_element: u32,
    pub slider_bar_definition: u32,
    pub value_element: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct OptionsTab {
    pub add_padding_after_last_item: Option<bool>,
    pub filter: Option<EnumOptionItemFilter>,
    pub show_on: Option<u8>,
    pub tab_name_tra_key: String,
    pub unk_0xa4b002d7: Vec<EnumOptionItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct OptionsViewController {
    pub base_loadable: u32,
    pub button1_definition: u32,
    pub button2_definition: u32,
    pub cancel_hit_region: u32,
    pub close_button_definition: u32,
    pub default_menu_button_tra_keys: Vec<String>,
    pub exit_hit_region: u32,
    pub korea_ratings_icon_element: u32,
    pub last_item_padding: u32,
    pub mobile_menu_button_tra_keys: Vec<String>,
    pub mobile_override_loadable: u32,
    pub okay_hit_region: u32,
    pub path_hash_to_self: u64,
    pub restore_defaults_hit_region: u32,
    pub surrender_hit_region: u32,
    pub tab_button_definition: u32,
    pub tabs: Vec<u32>,
    pub view_pane_link: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OverrideAttackTimeData {
    pub m_cast_time_percent: Option<f32>,
    pub m_total_attack_time_secs: Option<GameCalculation>,
    pub set_override_attack_delay: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OverrideAutoAttackCastTimeData {
    pub m_override_autoattack_cast_time_calculation: GameCalculation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PackFormationData {
    pub formation_positions: Vec<Vec2>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PackManagerData {
    pub attack_move_target_forgiveness_range: Option<f32>,
    pub buff_overrides: Option<Vec<u32>>,
    pub follower_crossover_animation: u32,
    pub leash_distance: Option<f32>,
    pub on_leader_move_follower_animation: u32,
    pub order_trailing_delay: Option<f32>,
    pub rank_to_formation_map: Option<HashMap<u32, PackFormationData>>,
    pub ui_target_forgiveness_range: Option<f32>,
    pub unk_0x377491e8: EnumUnk0x1aae122,
    pub unk_0xb97a9b92: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParallelClipData {
    pub m_clip_name_list: Vec<u32>,
    pub m_flags: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParametricClipData {
    pub m_animation_interruption_group_names: Option<Vec<u32>>,
    pub m_event_data_map: Option<HashMap<u32, EnumEventData>>,
    pub m_flags: Option<u32>,
    pub m_mask_data_name: Option<u32>,
    pub m_parametric_pair_data_list: Vec<ParametricPairData>,
    pub m_sync_group_data_name: Option<u32>,
    pub m_track_data_name: u32,
    pub unk_0x69de8fca: Option<bool>,
    pub updater: EnumParametricUpdater,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParametricMovement {
    pub m_offset_initial_target_height: f32,
    pub m_start_bone_name: String,
    pub m_target_height_augment: f32,
    pub movement_entries: Vec<ParametricMovementEntry>,
    pub parametric_movement_type: ParametricMovementTypeAngleFromTarget,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParametricMovementEntry {
    pub movement_spec: FixedSpeedSplineMovement,
    pub value: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParametricMovementTypeAngleFromTarget {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParametricPairData {
    pub m_clip_name: u32,
    pub m_value: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParticleEventData {
    pub m_effect_key: Option<u32>,
    pub m_effect_name: Option<String>,
    pub m_end_frame: Option<f32>,
    pub m_enemy_effect_key: Option<u32>,
    pub m_fire_if_animation_ends_early: Option<bool>,
    pub m_is_detachable: Option<bool>,
    pub m_is_kill_event: Option<bool>,
    pub m_is_loop: Option<bool>,
    pub m_is_self_only: Option<bool>,
    pub m_name: Option<u32>,
    pub m_particle_event_data_pair_list: Option<Vec<ParticleEventDataPair>>,
    pub m_scale_play_speed_with_animation: Option<bool>,
    pub m_start_frame: Option<f32>,
    pub scale: Option<f32>,
    pub skip_if_past_end_frame: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParticleEventDataPair {
    pub m_bone_name: Option<u32>,
    pub m_target_bone_name: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartnerGroupPlacements {
    pub partner_group_banners: Vec<u32>,
    pub placement_banners: Vec<u32>,
    pub placement_texts: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct PauseDialogViewController {
    pub base_loadable: u32,
    pub meter: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PerLocaleTooltipAdjustments {
    pub bottom_hr_y_post_adjustment: Option<i32>,
    pub bottom_hr_y_pre_adjustment: Option<i32>,
    pub bottom_y_padding_adjustment: Option<i32>,
    pub title_y_adjustment: Option<i32>,
    pub top_hr_y_post_adjustment: Option<i32>,
    pub top_hr_y_pre_adjustment: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PercentageOfBuffNameElapsed {
    pub buff_name: u32,
    pub coefficient: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PerkReplacement {
    pub m_replace_target: u32,
    pub m_replace_with: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PerkReplacementList {
    pub m_replacements: Vec<PerkReplacement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersistentEffectConditionData {
    pub force_render_vfx: Option<bool>,
    pub owner_condition: Option<EnumDriver>,
    pub persistent_vfxs: Option<Vec<PersistentVfxData>>,
    pub source_condition: Option<EnumDriver>,
    pub submeshes_to_hide: Option<Vec<u32>>,
    pub submeshes_to_show: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVfxData {
    pub attach_to_camera: Option<bool>,
    pub bone_name: Option<String>,
    pub effect_key: u32,
    pub effect_key_for_other_team: Option<u32>,
    pub play_speed_modifier: Option<f32>,
    pub scale: Option<f32>,
    pub show_to_owner_only: Option<bool>,
    pub specific_team: Option<u32>,
    pub target_bone_name: Option<String>,
    pub target_pos_is_owner: Option<bool>,
    pub use_different_key_for_other_team: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PhysicsMovement {
    pub m_drag: f32,
    pub m_initial_speed: f32,
    pub m_lifetime: f32,
    pub m_offset_initial_target_height: Option<f32>,
    pub m_start_bone_name: String,
    pub m_target_height_augment: Option<f32>,
    pub m_tracks_target: bool,
    pub m_wall_sliding: bool,
    pub m_wall_sliding_friction_multiplier: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PingRadialBaseCategoryDefinition {
    pub category_text: u32,
    pub ping_category: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PingRadialBaseDefaultCommandDefinition {
    pub command_center_icon: u32,
    pub command_center_minimap_icon: u32,
    pub command_text: String,
    pub ping_category: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct PingRadialViewController {
    pub active_wheel_scene: u32,
    pub background_cooldown_effect: u32,
    pub base_loadable: u32,
    pub category_text_on_hover_only: Option<bool>,
    pub center_overrides: Option<Vec<PingRadialBaseDefaultCommandDefinition>>,
    pub enable_center_activate_default_ping: Option<bool>,
    pub minimap_active_wheel_scene: u32,
    pub minimap_background_cooldown_effect: u32,
    pub minimap_radial_buttons: Vec<RadialMenuButtonDefinitionBase>,
    pub minimap_selection_line: u32,
    pub minimap_selection_rotating_icon: Option<u32>,
    pub override_center_button_region: Option<u32>,
    pub override_minimap_center_button_region: Option<u32>,
    pub path_hash_to_self: u64,
    pub ping_categories: Vec<PingRadialBaseCategoryDefinition>,
    pub radial_buttons: Vec<RadialMenuHoverButtonDefinition>,
    pub selection_line: u32,
    pub selection_rotating_icon: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct PingSecondaryRadialViewController {
    pub active_wheel_scene: u32,
    pub background_cooldown_effect: u32,
    pub base_loadable: u32,
    pub category_text_on_hover_only: bool,
    pub enable_center_activate_default_ping: bool,
    pub horizontally_center_first_wedge: bool,
    pub minimap_active_wheel_scene: u32,
    pub minimap_background_cooldown_effect: u32,
    pub minimap_radial_buttons: Vec<RadialMenuButtonDefinitionBase>,
    pub minimap_selection_line: u32,
    pub minimap_selection_rotating_icon: u32,
    pub override_center_button_region: u32,
    pub override_minimap_center_button_region: u32,
    pub path_hash_to_self: u64,
    pub ping_categories: Vec<PingRadialBaseCategoryDefinition>,
    pub radial_buttons: Vec<RadialMenuHoverButtonDefinition>,
    pub selection_line: u32,
    pub selection_rotating_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlatformSpellInfo {
    pub m_avatar_level_required: Option<i32>,
    pub m_game_modes: Option<Vec<String>>,
    pub m_platform_enabled: Option<bool>,
    pub m_spell_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookAugmentItemData {
    pub description: u32,
    pub frame: u32,
    pub group: u32,
    pub icon: u32,
    pub name: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookAugmentPanelData {
    pub late_augment_grid: u32,
    pub mid_augment_grid: u32,
    pub unk_0xa233b0ab: u32,
    pub view_panel_scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookGridButtonData {
    pub equipped_icon: u32,
    pub icon: u32,
    pub name: u32,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookInfoPanel {
    pub description: u32,
    pub icon: u32,
    pub name: u32,
    pub scene: u32,
    pub subtitle: u32,
    pub view_pane_link: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookItemListPanelData {
    pub button_area: u32,
    pub view_panel_scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookViewController {
    pub augment_item_data: PlaybookAugmentItemData,
    pub augment_panel_data: PlaybookAugmentPanelData,
    pub back_button: u32,
    pub base_loadable: u32,
    pub close_button: u32,
    pub details_button: u32,
    pub equip_button: u32,
    pub grid_button_data: PlaybookGridButtonData,
    pub grid_item_button: u32,
    pub help_button: u32,
    pub info_panel: PlaybookInfoPanel,
    pub item_list_panel_data: PlaybookItemListPanelData,
    pub path_hash_to_self: u64,
    pub playbook_help_view_controller: u32,
    pub scene: u32,
    pub selection_scene: u32,
    pub sound_on_activate: String,
    pub sound_on_de_activate: String,
    pub unk_0x2eed7e1b: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAugmentsViewController {
    pub base_loadable: u32,
    pub flipped_minimap_override: u32,
    pub panel_bg: u32,
    pub panel_bg_alt_side: u32,
    pub panel_bg_alt_top: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub unk_0x73dd5ab3: Unk0x4a7922fb,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFrameViewController {
    pub abilities_ui_data: AbilitiesUiData,
    pub base_loadable: u32,
    pub center_tooltip_region: u32,
    pub draw_area_list: Option<DrawAreaList>,
    pub hud_center_frame_glow_data: HudCenterFrameGlowData,
    pub level_up_display: UiLevelUp,
    pub level_up_links: Vec<u32>,
    pub negative_buffs: BuffDisplayList,
    pub path_hash_to_self: u64,
    pub player_buffs_scene: u32,
    pub portrait_ui_data: PlayerPortraitUiData,
    pub positive_buffs: BuffDisplayList,
    pub resource_bars: HudPlayerResourceBars,
    pub root_scene: u32,
    pub spell_cast_message: u32,
    pub status_message: u32,
    pub summoner_spell_specialist: UiPerkSummonerSpecialistToggles,
    pub unk_0x1c05ee9d: Vec<Unk0xc3f95838>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct PlayerHeroStatsViewController {
    pub advanced_stats: u32,
    pub background: u32,
    pub base_loadable: u32,
    pub basic_stats: u32,
    pub flipped_minimap_override: u32,
    pub path_hash_to_self: u64,
    pub stats_ui_data: UnitStatsUiData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInventoryViewController {
    pub base_loadable: u32,
    pub draw_area_list: DrawAreaList,
    pub item_slot_ui_data: Vec<ItemSlotDetailedUiData>,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub shop_button: HudShopButton,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMuteViewController {
    pub background_link: u32,
    pub base_loadable: u32,
    pub champion_icon_link: u32,
    pub champion_name_link: u32,
    pub main_scene: u32,
    pub menu_close_button: u32,
    pub mute_all_button: u32,
    pub mute_chat_button: u32,
    pub mute_emotes_button: u32,
    pub mute_pings_button: u32,
    pub path_hash_to_self: u64,
    pub summoner_name_link: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPerksViewController {
    pub base_loadable: u32,
    pub flipped_minimap_override: u32,
    pub panel_bg: u32,
    pub panel_bg_alt_side: u32,
    pub panel_bg_alt_top: u32,
    pub path_hash_to_self: u64,
    pub player_perks_stats: UiPerksStats,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPortraitUiData {
    pub icon: u32,
    pub level_text: u32,
    pub respawn_timer: u32,
    pub tooltip_region: u32,
    pub voice_chat_halo: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProfileCategoryButtonDefinition {
    pub button: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProfileMatchHistoryView {
    pub loading_spinner_vfx: u32,
    pub main_content_view_pane: u32,
    pub match_history_grid: u32,
    pub match_history_list_scene: u32,
    pub match_history_player_template: MatchHistoryPlayerTemplate,
    pub match_history_scene: u32,
    pub match_history_trait_template: MatchHistoryTraitTemplate,
    pub match_history_unit_template: MatchHistoryUnitTemplate,
    pub no_match_history_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProfileViewController {
    pub base_loadable: u32,
    pub match_history_view: PlayerProfileMatchHistoryView,
    pub path_hash_to_self: u64,
    pub player_profile_category_buttons: Vec<PlayerProfileCategoryButtonDefinition>,
    pub player_profile_scene: u32,
    pub tablet_override_loadable: u32,
    pub unk_0x2eed7e1b: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct PlayerReportViewController {
    pub background_link: u32,
    pub base_loadable: u32,
    pub champion_icon_link: u32,
    pub champion_name_link: u32,
    pub close_region: u32,
    pub close_region_scene: u32,
    pub drag_region: u32,
    pub main_scene: u32,
    pub menu_close_button: u32,
    pub path_hash_to_self: u64,
    pub report_cheating_button: u32,
    pub report_intentional_feeding_button: u32,
    pub report_leaving_the_game_button: u32,
    pub report_negative_attitude_button: u32,
    pub report_offensive_name_button: u32,
    pub report_verbal_abuse_button: u32,
    pub submit_report_button: u32,
    pub summoner_name_link: u32,
    pub unk_0xa50499ac: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStatStonesViewController {
    pub base_loadable: u32,
    pub flipped_minimap_override: u32,
    pub frame_adv_side: u32,
    pub frame_adv_top: u32,
    pub frame_default: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub stat_stones_stats: UiStatStonesStats,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoiScalingData {
    pub max_distance: u32,
    pub max_distance_scale: f32,
    pub min_distance: u32,
    pub min_distance_scale: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostGameLabFields {
    pub rated_icons: HashMap<u32, u32>,
    pub rated_loading_icon_vfx: u32,
    pub rated_tier_loc_keys: HashMap<u32, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostGamePlayerTemplate {
    pub group: u32,
    pub hidden_unit_count_text: u32,
    pub name_tag: u32,
    pub placement_text: u32,
    pub summoner_icon: u32,
    pub summoner_icon_frame: u32,
    pub unk_0x403907ce: u32,
    pub unk_0x43ba4914: u8,
    pub unk_0x85e44456: u8,
    pub unk_0x88f8477a: u32,
    pub unk_0x9c422946: u32,
    pub unk_0xa8651970: String,
    pub unk_0xdd2b5b5d: u32,
    pub unk_0xddc2fcb5: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostGameUnitTemplate {
    pub group: u32,
    pub icon: u32,
    pub icon_frame: u32,
    pub item_icons: Vec<u32>,
    pub one_star: u32,
    pub three_star: u32,
    pub tooltip_button: u32,
    pub two_star: u32,
    pub unk_0x49bd8929: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct PostGameViewController {
    pub background_texture: u32,
    pub base_loadable: u32,
    pub current_exp_text: u32,
    pub current_player_highlight: u32,
    pub demotion_protection_text: u32,
    pub gained_exp_text: u32,
    pub labs: HashMap<u32, PostGameLabFields>,
    pub lost_exp_text: u32,
    pub partner_group_placements: PartnerGroupPlacements,
    pub path_hash_to_self: u64,
    pub placement_text_right: u32,
    pub play_again_button: u32,
    pub player_placement_banners: Vec<u32>,
    pub player_slots: Vec<u32>,
    pub player_template: PostGamePlayerTemplate,
    pub provisional_text_right: u32,
    pub rank_icons: HashMap<u32, u32>,
    pub rank_text: u32,
    pub scene: u32,
    pub scene_divider: u32,
    pub tablet_override_loadable: u32,
    pub unit_template: PostGameUnitTemplate,
    pub unk_0x1e916517: u32,
    pub unk_0x2eed7e1b: u32,
    pub unk_0x3e488bdb: Unk0x77f812a6,
    pub unk_0x638add82: Unk0xf8828af9,
    pub unk_0x7617429d: u32,
    pub unk_0x78e53d0d: Unk0x5af2b668,
    pub unk_0x791d8674: u32,
    pub unk_0x7d52ddad: u32,
    pub unk_0xa081e6b5: Unk0x1b3a73f0,
    pub unk_0xa3df78a2: u32,
    pub unk_0xb7d2df48: Unk0x272b222f,
    pub unk_0xb84044af: String,
    pub unk_0xdfccb347: u32,
    pub unk_0xeb85bfc2: u32,
    pub unk_0xfbf67a11: String,
    pub win_streak_text: u32,
    pub win_streak_vfx: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PracticeToolButtonDefinition {
    pub button: u32,
    pub portrait: u32,
    pub title_text: u32,
    pub toggle_vfx: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct PracticeToolViewController {
    pub base_loadable: u32,
    pub button_definition: PracticeToolButtonDefinition,
    pub cheat_icon_map: HashMap<u32, u32>,
    pub cheat_set: u32,
    pub menu_scene: u32,
    pub menu_title_scene: u32,
    pub menu_toggle_scene: u32,
    pub page_down: u32,
    pub page_up: u32,
    pub path_hash_to_self: u64,
    pub slot_height_ref: u32,
    pub toggle_menu_button: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PreloadCharacter {
    pub character_name: Box<EnumAddLevelTimer>,
    pub is_disabled: Option<bool>,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PreloadCharacterWithSkinId {
    pub character_name: StringGet,
    pub skin_id: IntGet,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductOfSubPartsCalculationPart {
    pub m_part1: Box<EnumAbilityResourceByCoefficientCalculationPart>,
    pub m_part2: Box<EnumAbilityResourceByCoefficientCalculationPart>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProgressBarDisplayData {
    pub meter: u32,
    pub timer_text: u32,
    pub title_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ProgressBarViewController {
    pub base_loadable: u32,
    pub cast_bar: ProgressBarDisplayData,
    pub layout: u32,
    pub path_hash_to_self: u64,
    pub progress_bar: ProgressBarDisplayData,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct PropertyLoadable {
    pub filepath_hash: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseDialogSubPurchaseElements {
    pub sub_purchase_divider: u32,
    pub sub_purchase_group: u32,
    pub sub_purchase_icon: u32,
    pub sub_purchase_icon_frame: u32,
    pub sub_purchase_title: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseDialogSubPurchaseHeaderElements {
    pub header_group: u32,
    pub header_space: u32,
    pub header_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct QualitySetting {
    pub m_back_buffer_scale: Option<f32>,
    pub m_charater_quality: Option<u32>,
    pub m_effects_quality: Option<u32>,
    pub m_environment_quality: Option<u32>,
    pub m_frame_cap: Option<u32>,
    pub m_name: String,
    pub m_shadow_quality: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuestTrackerMessageTemplate {
    pub background: u32,
    pub body_icon: u32,
    pub pulse_background: u32,
    pub slot_area: u32,
    pub swosh_anim: u32,
    pub text: u32,
    pub text_by_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct QuestTrackerViewController {
    pub base_loadable: u32,
    pub header_text_template: u32,
    pub item_scene_template: u32,
    pub message_display_data: HudMessageDisplayData,
    pub message_template: QuestTrackerMessageTemplate,
    pub path_hash_to_self: u64,
    pub scene_template: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RadialMenuButtonDefinitionBase {
    pub default_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RadialMenuHoverButtonDefinition {
    pub default_icon: Option<u32>,
    pub hover_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RatioConversion {
    pub m_source_stat_output: Option<u8>,
    pub m_source_stat_type: u8,
    pub m_source_to_result_conversion_ratio: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecSpellRankUpInfo {
    pub is_default_recommendation: Option<bool>,
    pub m_default_priority: Option<Vec<u32>>,
    pub m_early_level_overrides: Option<Vec<u32>>,
    pub map_id: Option<u32>,
    pub mode_name_string_id: Option<u32>,
    pub position: Option<u32>,
    pub unk_0x5b968ffb: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct RecSpellRankUpInfoList {
    pub rec_spell_rank_up_infos: Vec<RecSpellRankUpInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ReciprocityConfig {
    pub throttle_limit: u8,
    pub unk_0x8e26fcb2: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecommendedJunglePathIcons {
    pub scoreboard_disabled_icon: Option<MinimapIcon>,
    pub scoreboard_enabled_final_icon: MinimapIcon,
    pub scoreboard_enabled_icon: MinimapIcon,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ReconnectDialogViewController {
    pub base_loadable: u32,
    pub meter: u32,
    pub mobile_override: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct RefundDialogViewController {
    pub base_loadable: u32,
    pub close_button_definition: u32,
    pub confirm_button_definition: u32,
    pub content_scene: u32,
    pub path_hash_to_self: u64,
    pub refund_body_text: u32,
    pub refund_button: u32,
    pub refund_title_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemapFloatMaterialDriver {
    pub m_driver: Box<EnumDriver>,
    pub m_max_value: Option<f32>,
    pub m_min_value: Option<f32>,
    pub m_output_max_value: Option<f32>,
    pub m_output_min_value: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemapVec4MaterialDriver {
    pub driver: AbilityResourceDynamicMaterialFloatDriver,
    pub output_max_value: Vec4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFromCustomTableBlock {
    pub index: Option<Box<EnumAddLevelTimer>>,
    pub key: Option<ScriptTableGet>,
    pub table: CustomTableGet,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ReplayCameraControlsViewController {
    pub base_loadable: u32,
    pub camera_controls_combo_box_definition: u32,
    pub directed_camera_status_text: u32,
    pub directed_camera_toggle_button: u32,
    pub fow_toggle_button: u32,
    pub path_hash_to_self: u64,
    pub scene_link: u32,
    pub team_fow_combo_box_definition: u32,
    pub tooltip_link: u32,
    pub visibility_menu_button: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReplayControlsButton {
    pub button: u32,
    pub default_tooltip_key: String,
    pub hotkey_event_string: String,
    pub selected_tooltip_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ReplayControlsViewController {
    pub backdrop_full: u32,
    pub backdrop_lite: u32,
    pub base_loadable: u32,
    pub message_text_handle: u32,
    pub path_hash_to_self: u64,
    pub play: ReplayControlsButton,
    pub playback_speed_text_handle: u32,
    pub record: ReplayControlsButton,
    pub recording_active_message: String,
    pub recording_complete_to_path_message: String,
    pub recording_error_message: String,
    pub replay_slider: HudReplaySlider,
    pub scene_handle: u32,
    pub skip_back: ReplayControlsButton,
    pub speed_down: ReplayControlsButton,
    pub speed_up: ReplayControlsButton,
    pub tooltip_handle: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ReplayGameStateViewController {
    pub base_loadable: u32,
    pub chaos_objective_bounties_data: Option<SbObjectiveBounties>,
    pub metrics: Vec<EnumUiMetric>,
    pub order_objective_bounties_data: Option<SbObjectiveBounties>,
    pub path_hash_to_self: u64,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ReplayTeamFightViewController {
    pub base_loadable: u32,
    pub offscreen_differentiation: UiTeamFightOffScreenDifferentiationData,
    pub parent_scene: u32,
    pub path_hash_to_self: u64,
    pub resizing_backdrop: u32,
    pub scene_transition: HudMenuTransitionData,
    pub team_data: Vec<UiTeamFightTeamData>,
    pub tooltip_position: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ReplayTeamFramesViewController {
    pub base_loadable: u32,
    pub chaos_team_data: UiReplayTeamFramesData,
    pub order_team_data: UiReplayTeamFramesData,
    pub path_hash_to_self: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ReplayVisibilityMenuViewController {
    pub all_visibility_key: String,
    pub base_loadable: u32,
    pub chat_visibility_key: String,
    pub checkbox_template: u32,
    pub kill_callouts_visibility_key: String,
    pub layout: u32,
    pub minimap_visibility_key: String,
    pub objective_timers_visibility_key: String,
    pub path_hash_to_self: u64,
    pub quests_visibility_key: String,
    pub scene_handle: u32,
    pub scoreboard_visibility_key: String,
    pub target_frame_visibility_key: String,
    pub team_frames_blue_visibility_key: String,
    pub team_frames_purple_visibility_key: String,
    pub team_score_visibility_key: String,
    pub time_controle_visibility_key: String,
    pub unk_0x39781626: Unk0x2db7365f,
    pub unk_0x5c32c63c: String,
    pub unk_0x82624f61: String,
    pub unk_0xa661ee40: String,
    pub unk_0xfdc7b782: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RerollButtonData {
    pub reroll_button: u32,
    pub reroll_button_text: Option<u32>,
    pub reroll_button_text_color: [u8; 4],
    pub reroll_button_text_disabled_color: [u8; 4],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResimulateTrailVfxOnEnterVisibility {
    pub cycles: u32,
    pub simulation_frames: u32,
    pub time_to_resimulate: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMeterGroupData {
    pub meter: u32,
    pub meter_skins: ResourceMeterSkinData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMeterIconData {
    pub additional_bar_types: Option<HashMap<u32, u32>>,
    pub default_bar: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMeterSkinData {
    pub additional_meter_skins: HashMap<u32, UiElementMeterSkin>,
    pub default_meter_skin: UiElementMeterSkin,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ResourceResolver {
    pub resource_map: Option<HashMap<u32, u32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RespawnPointData {
    pub first_spawn_position_offset: Vec3,
    pub team: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct RespawnPointDataList {
    pub respawn_points: Vec<RespawnPointData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReturnToCasterOnMovementComplete {
    pub m_override_spec: AcceleratingMovement,
    pub m_preserve_speed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RichBackgroundGameModeBackground {
    pub background: u32,
    pub background_element_map: HashMap<u32, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct RichBackgroundViewController {
    pub background: RichBackgroundGameModeBackground,
    pub base_loadable: u32,
    pub main_scene: u32,
    pub path_hash_to_self: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RootScriptSequence {
    pub blocks: Option<Vec<EnumAddLevelTimer>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RotatingBanner {
    pub banner: u32,
    pub banner_set: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RotatingBannerEntry {
    pub sponsor_texture_path: u64,
    pub times_shown_per_cycle: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct RotatingBannerSet {
    pub banner_entries: Vec<RotatingBannerEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RotatingWaveBehavior {
    pub spawn_counts_by_wave: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SameTeamCastRequirement {
    pub m_invert_result: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SbObjectiveBounties {
    pub baron_icon: u32,
    pub baron_score: u32,
    pub baron_value: u32,
    pub dragons_icon: u32,
    pub dragons_score: u32,
    pub dragons_value: u32,
    pub rift_herald_camp_name: u32,
    pub rift_herald_icon: u32,
    pub rift_herald_score: u32,
    pub rift_herald_value: u32,
    pub tower_icon: u32,
    pub tower_score: u32,
    pub towers_value: u32,
    pub unk_0x47dac7e3: u32,
    pub unk_0x7681fe9f: u32,
    pub unk_0x90fce2a4: u32,
    pub unk_0xaec651cc: u32,
    pub unk_0xcfe173b: u32,
    pub unk_0xd631c93e: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SceneAlphaTransitionData {
    pub easing_type: Option<u8>,
    pub end_alpha: Option<u8>,
    pub start_alpha: Option<u8>,
    pub transition_start_delay_secs: Option<f32>,
    pub transition_time: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SceneScreenEdgeTransitionData {
    pub easing_type: Option<u8>,
    pub edge: Option<u8>,
    pub end_alpha: Option<u8>,
    pub start_alpha: Option<u8>,
    pub transition_start_delay_secs: Option<f32>,
    pub transition_time: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScoreLineCherryUiData {
    pub augment_slots: Unk0xfc331f53,
    pub champion_name: ChampionNameUiData,
    pub drag_bar_slot: u32,
    pub item_slots: DetailedItemSlots,
    pub metrics: HashMap<u8, UiMetricUnitKda>,
    pub music_button: u32,
    pub mute_all_button: u32,
    pub mute_modal_anchor: u32,
    pub mute_self_button: u32,
    pub open_mute_modal_button: u32,
    pub open_report_modal_button: u32,
    pub portrait: CherryUiPlayerPortraitData,
    pub portrait_region: u32,
    pub scene: u32,
    pub social_tooltip_anchor: u32,
    pub spell_slots: ScoreLineSpellSlots,
    pub summoner_name: SummonerNameUiData,
    pub ult_cooldown_gem: CooldownGemUiData,
    pub unit_level: UnitLevelUiData,
    pub unk_0x9f82652b: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScoreLineSpellSlots {
    pub summoner_spell1: SpellSlotSimpleUiDefinition,
    pub summoner_spell2: SpellSlotSimpleUiDefinition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScoreLineSrSpectatorUiData {
    pub champion_gold: ChampionGoldUiData,
    pub dead_slot_highlight: u32,
    pub drag_bar_slot: u32,
    pub fow_slot_highlight: u32,
    pub item_slots: DetailedItemSlots,
    pub metrics: HashMap<u8, EnumUiMetric>,
    pub portrait: UiPlayerPortraitData,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScoreLineSrUiData {
    pub augment_slots: Option<Unk0xfc331f53>,
    pub dead_slot_highlight: u32,
    pub drag_bar_slot: u32,
    pub fow_slot_highlight: u32,
    pub item_slots: SimpleItemSlots,
    pub keystone: ChampionPerkKeystoneUiData,
    pub metrics: HashMap<u8, EnumUiMetric>,
    pub music_button: u32,
    pub mute_all_button: u32,
    pub mute_modal_anchor: u32,
    pub mute_self_button: u32,
    pub open_mute_modal_button: u32,
    pub open_report_modal_button: u32,
    pub portrait: UiPlayerPortraitData,
    pub portrait_region: u32,
    pub scene: u32,
    pub self_slot_highlight: u32,
    pub social_tooltip_anchor: u32,
    pub spell_slots: ScoreLineSpellSlots,
    pub summoner_name: SummonerNameUiData,
    pub ult_cooldown_gem: CooldownGemUiData,
    pub unit_level: UnitLevelUiData,
    pub unk_0x9f82652b: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ScoreboardGameModeConfig {
    pub scoreboard_team_score_types: u32,
    pub unk_0x7ed80272: Option<u32>,
    pub unk_0xc72e2257: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScoreboardTeamScoresDefinitions {
    pub champion_kills: UiMetricTeamKills,
    pub dragon_tracker: UiMetricMultiDragonKillsSrX,
    pub tower_kills: UiMetricTeamTowerKills,
    pub unk_0x6faf0b56: Option<Unk0x114c210c>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ScoreboardViewController {
    pub base_loadable: u32,
    pub chaos_objective_bounties_data: Option<SbObjectiveBounties>,
    pub chaos_score_line_ui_data: EnumScoreLineSr,
    pub chaos_team_identity: Option<UiTeamIdentityData>,
    pub drag_overlay_scene: u32,
    pub dragon_ui_loadable: Option<u32>,
    pub main_tooltip_anchor: u32,
    pub order_objective_bounties_data: Option<SbObjectiveBounties>,
    pub order_score_line_ui_data: EnumScoreLineSr,
    pub order_team_identity: Option<UiTeamIdentityData>,
    pub overlay_scene: u32,
    pub path_hash_to_self: u64,
    pub player_slot_height_reference: u32,
    pub scoreboard_scene: u32,
    pub summoner_social_card: Option<UiSummonerSocialCardData>,
    pub team1_ally_glow: u32,
    pub team1_enemy_glow: u32,
    pub team1_report_modal_anchor: Option<u32>,
    pub team1_swap_drag_target: u32,
    pub team1_swap_hover_icon: u32,
    pub team2_ally_glow: u32,
    pub team2_enemy_glow: u32,
    pub team2_report_modal_anchor: Option<u32>,
    pub team2_swap_drag_target: u32,
    pub team2_swap_hover_icon: u32,
    pub team_scores_definitions: Option<ScoreboardTeamScoresDefinitions>,
    pub unk_0x330f7f89: Option<u32>,
    pub unk_0x8817ae1a: Option<u32>,
    pub unk_0xb01c9387: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScriptCommentBlock {
    pub is_disabled: Option<bool>,
    pub sequence: Option<Box<ScriptSequence>>,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ScriptDataObject {
    pub m_constants: Option<HashMap<String, EnumGameModeConstant>>,
    pub m_name: String,
    pub m_required_constants_group: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScriptFunction {
    pub blocks: Vec<EnumAddLevelTimer>,
    pub function_def: Option<Unk0xeca2da9a>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScriptGlobalProperties {
    pub auto_buff_activate_attach_bone_names: Option<Vec<String>>,
    pub auto_buff_activate_effects: Option<Vec<String>>,
    pub buff_name: Option<String>,
    pub buff_texture_name: Option<String>,
    pub display_name: Option<String>,
    pub is_death_recap_source: Option<bool>,
    pub non_dispellable: Option<bool>,
    pub persists_through_death: Option<bool>,
    pub popup_messages: Option<Vec<String>>,
    pub spell_fx_override_skins: Option<Vec<String>>,
    pub spell_vo_override_skins: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScriptSequence {
    pub blocks: Vec<Box<EnumAddLevelTimer>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScriptTableGet {
    pub table: Option<EnumVars>,
    pub var: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScriptTableSet {
    pub table: Option<EnumVars>,
    pub var: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SeasonalSinglePurchaseDialog {
    pub background: u32,
    pub base_loadable: u32,
    pub close_button_definition: u32,
    pub confirm_button_definition: u32,
    pub content_scene: u32,
    pub path_hash_to_self: u64,
    pub purchase_button: u32,
    pub resizable_backdrop: u32,
    pub single_body_text: u32,
    pub single_close_button_region: u32,
    pub single_purchase_item_icon: u32,
    pub single_scene: u32,
    pub single_title_text: u32,
    pub tablet_override_loadable: u32,
    pub unk_0x3fe8aeb9: u32,
    pub unk_0x720a4eb5: [u8; 4],
    pub unk_0x7ca0e9d0: [u8; 4],
    pub unk_0x9b9a3c69: Unk0x36a5593c,
    pub unk_0xbc03fb23: String,
    pub unk_0xd02a6781: u32,
    pub unk_0xd1519ad2: u32,
    pub unk_0xea0e4ad5: u32,
    pub unk_0xee4e2158: String,
    pub unk_0xfff98f6a: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryResourceDisplayFractional {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SelectorClipData {
    pub m_flags: Option<u32>,
    pub m_selector_pair_data_list: Vec<SelectorPairData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SelectorPairData {
    pub m_clip_name: u32,
    pub m_probability: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Sequence {
    pub actions: Vec<EnumUnk0x1a4d18fe>,
    pub category: u32,
    pub path: String,
    pub path_hash: u32,
    pub phase_overrides: Option<Vec<SequencePhaseOverride>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SequenceCategory {
    pub hash: u32,
    pub name: String,
    pub phases: Vec<SequencePhase>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SequenceObjectSelector {
    pub hash: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SequencePhase {
    pub default_duration: Option<f32>,
    pub name: String,
    pub r#type: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SequencePhaseOverride {
    pub duration: Option<f32>,
    pub override_duration: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SequenceTiming {
    pub offset: Option<f32>,
    pub phase_index: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SequencerClipData {
    pub m_clip_name_list: Vec<u32>,
    pub m_event_data_map: Option<HashMap<u32, EnumEventData>>,
    pub m_flags: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetBit {
    pub bit_field: IntTableSet,
    pub bit_index: IntTableGet,
    pub is_disabled: Option<bool>,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetInvulnerableBlock {
    pub unk_0xd851ffa3: String,
    pub value: BoolGet,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetKeyValueInCustomTableBlock {
    pub dest_table: CustomTableGet,
    pub is_disabled: Option<bool>,
    pub key: Box<EnumAddLevelTimer>,
    pub unk_0xd851ffa3: String,
    pub value: Box<EnumAddLevelTimer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetModePreloadFlags {
    pub is_disabled: Option<bool>,
    pub mode_preload_flags: IntTableGet,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetTargetableBlock {
    pub unk_0xd7899f68: Option<BoolGet>,
    pub unk_0xd851ffa3: String,
    pub value: BoolGet,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetVarInTableBlock {
    pub dest: ScriptTableSet,
    pub is_disabled: Option<bool>,
    pub src: Box<EnumAddLevelTimer>,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimpleItemSlots {
    pub item0: ItemSlotSimpleUiData,
    pub item1: ItemSlotSimpleUiData,
    pub item2: ItemSlotSimpleUiData,
    pub item3: ItemSlotSimpleUiData,
    pub item4: ItemSlotSimpleUiData,
    pub item5: ItemSlotSimpleUiData,
    pub item6: ItemSlotSimpleUiData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SineMaterialDriver {
    pub m_bias: Option<f32>,
    pub m_driver: Box<EnumDriver>,
    pub m_frequency: Option<f32>,
    pub m_scale: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SinglePurchaseDialog {
    pub base_loadable: u32,
    pub close_button_definition: u32,
    pub confirm_button_definition: u32,
    pub content_scene: u32,
    pub path_hash_to_self: u64,
    pub purchase_button: u32,
    pub resizable_backdrop: u32,
    pub single_body_text: u32,
    pub single_close_button_region: u32,
    pub single_purchase_item_icon: u32,
    pub single_scene: u32,
    pub single_title_text: u32,
    pub store_dialog: u32,
    pub unk_0x385cc32b: u32,
    pub unk_0x3fe8aeb9: u32,
    pub unk_0x720a4eb5: [u8; 4],
    pub unk_0x95a147d3: u32,
    pub unk_0x9b9a3c69: Unk0x36a5593c,
    pub unk_0x9e218839: u32,
    pub unk_0xad26124e: u32,
    pub unk_0xd02a6781: u32,
    pub unk_0xd6f3c03: String,
    pub unk_0xf83118bc: UiHyperlink,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SinusoidalHeightSolver {
    pub m_amplitude: f32,
    pub m_number_of_periods: f32,
    pub m_vertical_offset: Option<f32>,
    pub unk_0x827af87a: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkinAnimationProperties {
    pub animation_graph_data: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkinAudioProperties {
    pub bank_units: Option<Vec<BankUnit>>,
    pub plays_vo: Option<bool>,
    pub tag_event_list: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkinAugmentCategories {
    pub basic_augments: Option<Vec<Unk0xe1555e0a>>,
    pub border_augments: Vec<Unk0x4a70b12c>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SkinCharacterDataProperties {
    pub alternate_icons_circle: Option<Vec<String>>,
    pub alternate_icons_square: Option<Vec<String>>,
    pub armor_material: Option<String>,
    pub attribute_flags: Option<u32>,
    pub can_share_theme_music: Option<bool>,
    pub champion_skin_name: Option<String>,
    pub default_animations: Option<Vec<String>>,
    pub emote_buffbone: Option<String>,
    pub emote_loadout: Option<u32>,
    pub emote_y_offset: Option<f32>,
    pub extra_action_button_count: Option<u32>,
    pub extra_character_preloads: Option<Vec<String>>,
    pub godray_f_xbone: Option<String>,
    pub health_bar_data: Option<CharacterHealthBarDataRecord>,
    pub hud_mute_event: Option<String>,
    pub hud_unmute_event: Option<String>,
    pub icon_avatar: Option<String>,
    pub icon_circle: Option<String>,
    pub icon_circle_scale: Option<f32>,
    pub icon_square: Option<String>,
    pub idle_particles_effects: Option<Vec<SkinCharacterDataPropertiesCharacterIdleEffect>>,
    pub loadscreen: Option<CensoredImage>,
    pub loadscreen_vintage: Option<CensoredImage>,
    pub m_contextual_action_data: Option<u32>,
    pub m_emblems: Option<Vec<SkinEmblem>>,
    pub m_resource_resolver: Option<u32>,
    pub m_spawn_particle_name: Option<String>,
    pub meta_data_tags: Option<String>,
    pub override_on_screen_name: Option<String>,
    pub particle_override_champion_kill_death_particle: Option<String>,
    pub particle_override_death_particle: Option<String>,
    pub persistent_effect_conditions: Option<Vec<EnumAddLevelTimer>>,
    pub secondary_resource_hud_display_data: Option<SecondaryResourceDisplayFractional>,
    pub skin_animation_properties: SkinAnimationProperties,
    pub skin_audio_properties: Option<SkinAudioProperties>,
    pub skin_classification: Option<u32>,
    pub skin_mesh_properties: Option<SkinMeshDataProperties>,
    pub skin_parent: Option<i32>,
    pub skin_upgrade_data: Option<SkinUpgradeData>,
    pub theme_music: Option<Vec<String>>,
    pub uncensored_icon_circles: Option<HashMap<u32, String>>,
    pub uncensored_icon_squares: Option<HashMap<u32, String>>,
    pub unk_0x2ac577e2: Option<bool>,
    pub unk_0xb67a2dd8: Option<Vec<Unk0x9c1d99c0>>,
    pub unk_0xc3a944e7: Option<EnumUnk0xc96d9140>,
    pub unk_0xe484edc4: Option<u32>,
    pub unk_0xeda7817e: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkinCharacterDataPropertiesCharacterIdleEffect {
    pub bone_name: Option<String>,
    pub effect_key: Option<u32>,
    pub effect_name: Option<String>,
    pub position: Option<Vec3>,
    pub target_bone_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SkinCharacterMetaDataProperties {
    pub e_sport_character: Option<bool>,
    pub e_sport_league_table: Option<Vec<ESportLeagueEntry>>,
    pub e_sport_team_table: Option<Vec<ESportTeamEntry>>,
    pub relative_color_swap_table: Option<Vec<i32>>,
    pub skin_based_relative_color_scheme: Option<bool>,
    pub spawning_skin_offsets: Option<Vec<SkinCharacterMetaDataPropertiesSpawningSkinOffset>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkinCharacterMetaDataPropertiesSpawningSkinOffset {
    pub offset: i32,
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkinEmblem {
    pub m_emblem_data: u32,
    pub m_loading_screen_anchor: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkinFilterData {
    pub filter_type: Option<u32>,
    pub skin_ids: Vec<u32>,
    pub use_valid_parent_for_chroma: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkinMeshDataProperties {
    pub bounding_cylinder_height: Option<f32>,
    pub bounding_cylinder_radius: Option<f32>,
    pub bounding_sphere_radius: Option<f32>,
    pub brush_alpha_override: Option<f32>,
    pub cast_shadows: Option<bool>,
    pub emissive_texture: Option<String>,
    pub emitter_submesh_avatar_to_hide: Option<String>,
    pub enable_picking: Option<bool>,
    pub force_draw_last: Option<bool>,
    pub fresnel: Option<f32>,
    pub fresnel_color: Option<[u8; 4]>,
    pub gloss_texture: Option<String>,
    pub initial_submesh_avatar_to_hide: Option<String>,
    pub initial_submesh_mouse_overs_to_hide: Option<String>,
    pub initial_submesh_shadows_to_hide: Option<String>,
    pub initial_submesh_to_hide: Option<String>,
    pub material: Option<u32>,
    pub material_controller: Option<EsportsBannerMaterialController>,
    pub material_override: Option<Vec<SkinMeshDataPropertiesMaterialOverride>>,
    pub override_bounding_box: Option<Vec3>,
    pub reduced_bone_skinning: Option<bool>,
    pub reflection_fresnel: Option<f32>,
    pub reflection_fresnel_color: Option<[u8; 4]>,
    pub reflection_map: Option<String>,
    pub reflection_opacity_direct: Option<f32>,
    pub reflection_opacity_glancing: Option<f32>,
    pub rig_pose_modifier_data: Option<Vec<EnumRigPoseModifierData>>,
    pub self_illumination: Option<f32>,
    pub simple_skin: Option<String>,
    pub skeleton: Option<String>,
    pub skin_scale: Option<f32>,
    pub submesh_render_order: Option<String>,
    pub texture: Option<String>,
    pub uses_skin_vo: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkinMeshDataPropertiesMaterialOverride {
    pub gloss_texture: Option<String>,
    pub material: Option<u32>,
    pub submesh: String,
    pub texture: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkinUpgradeData {
    pub m_gear_skin_upgrades: Option<Vec<u32>>,
    pub skin_augment_categories: Option<SkinAugmentCategories>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SmartPingData {
    pub unk_0x1e98bb0e: HashMap<u8, u8>,
    pub unk_0x5aa830e8: f32,
    pub unk_0xaa8eb438: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SocialActionsDialogViewController {
    pub base_loadable: u32,
    pub buttons_region: u32,
    pub content_scene: u32,
    pub invite_button: u32,
    pub path_hash_to_self: u64,
    pub player_portrait: u32,
    pub social_status_icons: SocialStatusIcons,
    pub summoner_name_text: u32,
    pub unk_0x3b5dbe09: u32,
    pub unk_0xbceaaa2a: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SocialChatViewController {
    pub base_loadable: u32,
    pub close_button: u32,
    pub friend_icon: u32,
    pub friend_name: u32,
    pub friend_options_button: u32,
    pub input_text_field_link: u32,
    pub path_hash_to_self: u64,
    pub received_message_color: [u8; 4],
    pub scene: u32,
    pub send_button: u32,
    pub sent_message_color: [u8; 4],
    pub today_tra_key: String,
    pub ui_chat_pane_definition: UiChatPaneDefinition,
    pub ui_social_chat_message_template: UiSocialChatMessageTemplate,
    pub yesterday_tra_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SocialPanelFriendsListItemData {
    pub active_invite_button: u32,
    pub click_region: u32,
    pub group: u32,
    pub name_text: u32,
    pub player_icon: u32,
    pub selected_highlight_texture: u32,
    pub status_icons: SocialStatusIcons,
    pub status_text: u32,
    pub unread_messages_badge_vfx: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SocialStatusIcons {
    pub away_status_icon: u32,
    pub ingame_status_icon: u32,
    pub mobile_app_status_icon: u32,
    pub offline_status_icon: u32,
    pub online_status_icon: u32,
    pub open_party_status_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SortCustomTableBlock {
    pub table: CustomTableGet,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SoundEventData {
    pub condition_clip_transition_type: Option<u16>,
    pub m_end_frame: Option<f32>,
    pub m_fire_if_animation_ends_early: Option<bool>,
    pub m_is_kill_event: Option<bool>,
    pub m_is_loop: Option<bool>,
    pub m_is_self_only: Option<bool>,
    pub m_name: Option<u32>,
    pub m_skip_if_past_end_frame: Option<bool>,
    pub m_sound_name: Option<String>,
    pub m_start_frame: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpawningUiDefinition {
    pub buff_name_filter: String,
    pub max_number_of_units: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpecificColorMaterialDriver {
    pub m_color: Option<Vec4>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellChoiceTemplate {
    pub button: u32,
    pub description: u32,
    pub icon: u32,
    pub title: u32,
    pub top_level_group: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SpellDataResource {
    pub always_snap_facing: Option<bool>,
    pub audio_bank_paths: Option<Vec<String>>,
    pub b_have_hit_bone: Option<bool>,
    pub b_have_hit_effect: Option<bool>,
    pub b_is_toggle_spell: Option<bool>,
    pub can_cast_or_queue_while_casting: Option<bool>,
    pub can_cast_while_disabled: Option<bool>,
    pub can_only_cast_while_dead: Option<bool>,
    pub cannot_be_suppressed: Option<bool>,
    pub cant_cast_while_rooted: Option<bool>,
    pub cast_cone_angle: Option<f32>,
    pub cast_cone_distance: Option<f32>,
    pub cast_frame: Option<f32>,
    pub cast_radius: Option<Vec<f32>>,
    pub cast_radius_secondary: Option<Vec<f32>>,
    pub cast_range: Option<Vec<f32>>,
    pub cast_range_display_override: Option<Vec<f32>>,
    pub cast_range_use_bounding_boxes: Option<bool>,
    pub cast_target_additional_units_radius: Option<f32>,
    pub cooldown_time: Option<Vec<f32>>,
    pub data_values: Option<Vec<SpellDataValue>>,
    pub data_values_mode_override: Option<HashMap<u32, SpellDataValueVector>>,
    pub delay_cast_offset_percent: Option<f32>,
    pub delay_total_time_percent: Option<f32>,
    pub flags: Option<u32>,
    pub img_icon_path: Option<String>,
    pub loadable: Option<u32>,
    pub lua_on_missile_update_distance_interval: Option<f32>,
    pub m_affects_status_flags: Option<u32>,
    pub m_affects_type_flags: Option<u32>,
    pub m_after_effect_key: Option<u32>,
    pub m_after_effect_name: Option<String>,
    pub m_ai_data: Option<AiSpellData>,
    pub m_alternate_name: Option<String>,
    pub m_ammo_count_hidden_in_ui: Option<bool>,
    pub m_ammo_not_affected_by_cdr: Option<bool>,
    pub m_ammo_recharge_time: Option<Vec<f32>>,
    pub m_ammo_used: Option<Vec<i32>>,
    pub m_animation_loop_name: Option<String>,
    pub m_animation_name: Option<String>,
    pub m_animation_winddown_name: Option<String>,
    pub m_apply_attack_damage: Option<bool>,
    pub m_apply_attack_effect: Option<bool>,
    pub m_apply_material_on_hit_sound: Option<bool>,
    pub m_belongs_to_avatar: Option<bool>,
    pub m_can_move_while_channeling: Option<bool>,
    pub m_can_trigger_charge_spell_while_disabled: Option<bool>,
    pub m_cancel_charge_on_recast_time: Option<f32>,
    pub m_cant_cancel_while_channeling: Option<bool>,
    pub m_cant_cancel_while_winding_up: Option<bool>,
    pub m_cant_cancel_while_winding_up_targeting_champion: Option<bool>,
    pub m_cast_range_growth_duration: Option<Vec<f32>>,
    pub m_cast_range_growth_max: Option<Vec<f32>>,
    pub m_cast_range_growth_start_time: Option<Vec<f32>>,
    pub m_cast_requirements_caster: Option<Vec<EnumHasAllSubRequirementsCastRequirement>>,
    pub m_cast_requirements_target: Option<Vec<EnumHasAllSubRequirementsCastRequirement>>,
    pub m_cast_time: Option<f32>,
    pub m_cast_type: Option<u32>,
    pub m_caster_position_end_of_cast_update: Option<u8>,
    pub m_casting_breaks_stealth: Option<bool>,
    pub m_casting_breaks_stealth_while_attached: Option<bool>,
    pub m_casting_reveals_caster_stealth: Option<bool>,
    pub m_channel_duration: Option<Vec<f32>>,
    pub m_channel_is_interrupted_by_attacking: Option<bool>,
    pub m_channel_is_interrupted_by_disables: Option<bool>,
    pub m_character_passive_buffs: Option<Vec<CharacterPassiveData>>,
    pub m_charge_update_interval: Option<f32>,
    pub m_client_data: Option<SpellDataResourceClient>,
    pub m_coefficient: Option<f32>,
    pub m_coefficient2: Option<f32>,
    pub m_considered_as_auto_attack: Option<bool>,
    pub m_cooldown_not_affected_by_cdr: Option<bool>,
    pub m_cost_always_shown_in_ui: Option<bool>,
    pub m_cursor_changes_in_grass: Option<bool>,
    pub m_cursor_changes_in_terrain: Option<bool>,
    pub m_dimension_behavior: Option<u8>,
    pub m_disable_cast_bar: Option<bool>,
    pub m_do_not_need_to_face_target: Option<bool>,
    pub m_does_not_consume_cooldown: Option<bool>,
    pub m_does_not_consume_mana: Option<bool>,
    pub m_doesnt_break_channels: Option<bool>,
    pub m_effect_amount: Option<Vec<SpellEffectAmount>>,
    pub m_excluded_unit_tags: Option<ObjectTags>,
    pub m_float_vars_decimals: Option<Vec<i32>>,
    pub m_hide_range_indicator_when_casting: Option<bool>,
    pub m_hit_bone_name: Option<String>,
    pub m_hit_effect_key: Option<u32>,
    pub m_hit_effect_name: Option<String>,
    pub m_hit_effect_orient_type: Option<u32>,
    pub m_hit_effect_player_key: Option<u32>,
    pub m_hit_effect_player_name: Option<String>,
    pub m_ignore_anim_continue_until_cast_frame: Option<bool>,
    pub m_ignore_range_check: Option<bool>,
    pub m_img_icon_name: Option<Vec<String>>,
    pub m_is_delayed_by_cast_locked: Option<bool>,
    pub m_is_disabled_while_dead: Option<bool>,
    pub m_keyword_when_acquired: Option<String>,
    pub m_line_drag_length: Option<f32>,
    pub m_line_width: Option<f32>,
    pub m_locked_spell_origination_cast_id: Option<bool>,
    pub m_look_at_policy: Option<u32>,
    pub m_max_ammo: Option<Vec<i32>>,
    pub m_minimap_icon_display_flag: Option<u16>,
    pub m_minimap_icon_name: Option<String>,
    pub m_minimap_icon_rotation: Option<bool>,
    pub m_missile_effect_enemy_key: Option<u32>,
    pub m_missile_effect_key: Option<u32>,
    pub m_missile_effect_name: Option<String>,
    pub m_missile_effect_player_key: Option<u32>,
    pub m_missile_effect_player_name: Option<String>,
    pub m_missile_spec: Option<MissileSpecification>,
    pub m_no_winddown_if_cancelled: Option<bool>,
    pub m_orient_radius_texture_from_player: Option<bool>,
    pub m_override_attack_time: Option<OverrideAttackTimeData>,
    pub m_particle_start_offset: Option<Vec3>,
    pub m_pingable_while_disabled: Option<bool>,
    pub m_platform_spell_info: Option<PlatformSpellInfo>,
    pub m_post_cast_lockout_delta_time: Option<f32>,
    pub m_pre_cast_lockout_delta_time: Option<f32>,
    pub m_pre_cast_lockout_delta_time_data: Option<SpellLockDeltaTimeData>,
    pub m_project_target_to_cast_range: Option<bool>,
    pub m_required_unit_tags: Option<ObjectTags>,
    pub m_resource_resolvers: Option<Vec<u32>>,
    pub m_roll_for_critical_hit: Option<bool>,
    pub m_show_channel_bar: Option<bool>,
    pub m_spell_calculations: Option<HashMap<u32, EnumGameCalculation>>,
    pub m_spell_cooldown_or_sealed_queue_threshold: Option<f32>,
    pub m_spell_reveals_champion: Option<bool>,
    pub m_spell_tags: Option<Vec<String>>,
    pub m_start_cooldown: Option<f32>,
    pub m_targeting_type_data: Option<EnumArea>,
    pub m_turn_speed_scalar: Option<f32>,
    pub m_update_rotation_when_casting: Option<bool>,
    pub m_use_autoattack_cast_time_data: Option<UseAutoattackCastTimeData>,
    pub m_use_charge_channeling: Option<bool>,
    pub m_use_minimap_targeting: Option<bool>,
    pub mana: Option<Vec<f32>>,
    pub mana_ui_override: Option<Vec<f32>>,
    pub missile_effect_max_turn_speed_degrees_per_second: Option<f32>,
    pub missile_effect_maximum_angle_degrees: Option<f32>,
    pub missile_speed: Option<f32>,
    pub passive_spell_affected_by_cooldown: Option<bool>,
    pub selection_priority: Option<u32>,
    pub should_receive_input_events: Option<bool>,
    pub show_channel_bar_per_spell_level_override: Option<Vec<bool>>,
    pub spell_event_to_audio_event_suffix: Option<HashMap<u32, String>>,
    pub targeting_forgiveness_definitions: Option<Vec<TargetingForgivenessDefinitions>>,
    pub unk_0x288b8edc: Option<EnumUnk0x6bbc3db6>,
    pub unk_0x48201b0d: Option<f32>,
    pub unk_0x66769fb4: Option<bool>,
    pub unk_0x8958fee2: Option<Unk0x8958fee2>,
    pub unk_0xabe507b9: Option<u32>,
    pub unk_0xb08bc498: Option<HashMap<u32, SpellEffectAmount>>,
    pub unk_0xf4ca428f: Option<u8>,
    pub unk_0xf9c2333e: Option<HashMap<u32, SpellEffectAmount>>,
    pub use_animator_framerate: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellDataResourceClient {
    pub m_custom_targeter_definitions: Option<HashMap<u32, CustomTargeterDefinitions>>,
    pub m_left_click_spell_action: Option<u32>,
    pub m_missile_targeter_definitions: Option<Vec<MissileAttachedTargetingDefinition>>,
    pub m_right_click_spell_action: Option<u32>,
    pub m_spawning_ui_definition: Option<SpawningUiDefinition>,
    pub m_targeter_definitions: Option<Vec<EnumTargeterDefinition>>,
    pub m_tooltip_data: Option<TooltipInstanceSpell>,
    pub m_use_death_recap_tooltip_from_another_spell: Option<u32>,
    pub m_use_tooltip_from_another_spell: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellDataValue {
    pub m_name: String,
    pub m_values: Option<Vec<f32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellDataValueVector {
    pub spell_data_values: Option<Vec<SpellDataValue>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellEffectAmount {
    pub value: Option<Vec<f32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellEvolutionUiData {
    pub button: u32,
    pub icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellLevelUpInfo {
    pub m_requirements: Vec<SpellRankUpRequirements>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellLevelUpUiData {
    pub ability_fx_in: u32,
    pub button_fx_in: u32,
    pub button_fx_out_selected: u32,
    pub button_fx_out_unselected: u32,
    pub button_idle_glow_fx: u32,
    pub button_idle_sheen_fx: u32,
    pub button_post_fx_in: u32,
    pub evolution: Option<SpellEvolutionUiData>,
    pub skill_up_button: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellLockDeltaTimeData {
    pub m_spell_lock_delta_time_calculation: GameCalculation,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SpellModifier {
    pub description_append_priority: Option<u32>,
    pub description_append_tra: Option<String>,
    pub m_calculation_stat_conversions: Option<Vec<RatioConversion>>,
    pub m_modifier_id: u32,
    pub m_spell_does_not_include_stat_scaling: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SpellObject {
    pub bot_data: Option<BotsSpellData>,
    pub cc_behavior_data: Option<CcBehaviorData>,
    pub m_buff: Option<BuffData>,
    pub m_script_name: String,
    pub m_spell: Option<SpellDataResource>,
    pub object_name: String,
    pub script: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SpellPickChoiceViewController {
    pub base_loadable: u32,
    pub confirm_button: u32,
    pub countdown_meter: u32,
    pub grid: u32,
    pub max_num_choices: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub spell_choice_template: SpellChoiceTemplate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellPipsUiData {
    pub empty_pips: Vec<u32>,
    pub full_pips: Vec<u32>,
    pub pip_target_rect: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellRankIntDriver {
    pub spell_slot: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellRankPipsUiData {
    pub rank_pips: Vec<EnumAddLevelTimer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellRankTextUiData {
    pub rank_text: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellRankUpRequirements {
    pub m_requirements: Option<Vec<EnumRequirement>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellSlotBuffTimerData {
    pub timer_bar_bg: u32,
    pub timer_bar_fill: u32,
    pub timer_border_bg: u32,
    pub timer_border_fx: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellSlotDetailedUiDefinition {
    pub ammo_fx: Option<u32>,
    pub ammo_text: Option<u32>,
    pub border_disabled: Option<u32>,
    pub border_enabled: Option<u32>,
    pub buff_timer: Option<SpellSlotBuffTimerData>,
    pub content_element: Option<u32>,
    pub cooldown_gem: Option<CooldownGemUiData>,
    pub cooldown_ui_data: Option<CooldownEffectUiData>,
    pub cost: Option<u32>,
    pub cost_bg: Option<u32>,
    pub hotkey: Option<u32>,
    pub mouseover_region: Option<u32>,
    pub overlay_cced: Option<u32>,
    pub overlay_disabled: Option<u32>,
    pub overlay_oom: Option<u32>,
    pub reset_flash_fx_attention: Option<u32>,
    pub toggle_fx: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellSlotSimpleUiDefinition {
    pub content_element: u32,
    pub cooldown_fx: u32,
    pub overlay_handle: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpringPhysicsEventData {
    pub blend_out_time: Option<f32>,
    pub m_end_frame: Option<f32>,
    pub m_fire_if_animation_ends_early: Option<bool>,
    pub spring_to_affect: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpringPhysicsRigPoseModifierData {
    pub damping: Option<f32>,
    pub default_on: Option<bool>,
    pub do_rotation: bool,
    pub do_translation: Option<bool>,
    pub joint: u32,
    pub mass: Option<f32>,
    pub max_angle: Option<f32>,
    pub max_distance: Option<f32>,
    pub name: Option<u32>,
    pub spring_stiffness: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatByCoefficientCalculationPart {
    pub m_coefficient: Option<f32>,
    pub m_stat: Option<u8>,
    pub m_stat_formula: Option<u8>,
    pub unk_0xa8cb9c14: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatByNamedDataValueCalculationPart {
    pub m_data_value: u32,
    pub m_stat: Option<u8>,
    pub m_stat_formula: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatBySubPartCalculationPart {
    pub m_stat: Option<u8>,
    pub m_stat_formula: Option<u8>,
    pub m_subpart: Box<EnumAbilityResourceByCoefficientCalculationPart>,
    pub unk_0xa8cb9c14: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatEfficiencyPerHundred {
    pub m_bonus_stat_for_efficiency: f32,
    pub m_data_value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct StatFilterButtonDefinitions {
    pub ability_haste: StatFilterDefinition,
    pub ability_power: StatFilterDefinition,
    pub armor: StatFilterDefinition,
    pub armor_penetration: StatFilterDefinition,
    pub attack_speed: StatFilterDefinition,
    pub critical_strike: StatFilterDefinition,
    pub disable_stat_filters: StatFilterDefinition,
    pub health: StatFilterDefinition,
    pub life_steal_and_vamp: StatFilterDefinition,
    pub magic_penetration: StatFilterDefinition,
    pub magic_resist: StatFilterDefinition,
    pub mana: StatFilterDefinition,
    pub move_speed: StatFilterDefinition,
    pub on_hit: StatFilterDefinition,
    pub physical_damage: StatFilterDefinition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatFilterDefinition {
    pub button_definition: u32,
    pub matching_categories: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct StatStoneData {
    pub catalog_entry: CatalogEntry,
    pub category: u32,
    pub data_collection_only: Option<bool>,
    pub epic_stat_stone: Option<bool>,
    pub events_to_track: Vec<StatStoneEventToTrack>,
    pub is_retired: Option<bool>,
    pub m_description_tra_key: String,
    pub m_name_tra_key: String,
    pub milestones: Vec<u64>,
    pub stone_name: String,
    pub tracking_type: Option<u8>,
    pub triggered_from_script: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatStoneEventToTrack {
    pub event_to_track: u32,
    pub stat_filters: Option<Vec<EnumTarget>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct StatStoneMilestoneCalloutViewController {
    pub ally_frame: u32,
    pub base_loadable: u32,
    pub champ_icon_min_alpha: u8,
    pub champ_icon_native_offset: f32,
    pub common_effect_elements: StatStoneMilestoneVfxDefinition,
    pub enemy_frame: u32,
    pub max_number_of_milestones_to_show_per_stone_per_game: u32,
    pub milestone_bg_effect_tinted: u32,
    pub milestone_bg_frame_other: u32,
    pub milestone_bg_frame_self: u32,
    pub milestone_content_scene: u32,
    pub milestone_count: u32,
    pub milestone_display_time: f32,
    pub milestone_fg_effect_tinted: u32,
    pub milestone_fg_uncolored: u32,
    pub milestone_name: u32,
    pub milestone_other_intro_time: f32,
    pub milestone_owner_icon: u32,
    pub milestone_owner_scene: u32,
    pub milestone_personal_best_effect: u32,
    pub milestone_personal_best_scene: u32,
    pub milestone_self_intro_time: f32,
    pub milestone_stone_icon: u32,
    pub milestone_text_position: u32,
    pub milestone_transition_in_min_alpha: u8,
    pub milestone_transition_in_time: f32,
    pub milestone_transition_out_time: f32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub self_frame: u32,
    pub ui_sound: String,
    pub ui_sound_for_local_player: String,
    pub ui_sound_for_personal_best: String,
    pub ui_sound_for_personal_best_for_local_player: String,
    pub unique_effect_elements: StatStoneMilestoneVfxDefinition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatStoneMilestoneVfxDefinition {
    pub flame_effect_element: u32,
    pub glow_effect_element: u32,
    pub rekindled_effect_element: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct StatStoneSet {
    pub catalog_entry: CatalogEntry,
    pub name: String,
    pub stat_stones: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatUiData {
    pub m_abbreviation: String,
    pub m_display_type: Option<u8>,
    pub m_icon_key: String,
    pub m_name: String,
    pub m_scaling_tag_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StaticMaterialChildTechniqueDef {
    pub name: String,
    pub parent_name: String,
    pub shader_macros: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct StaticMaterialDef {
    pub child_techniques: Option<Vec<StaticMaterialChildTechniqueDef>>,
    pub dynamic_material: Option<DynamicMaterialDef>,
    pub name: String,
    pub param_values: Option<Vec<StaticMaterialShaderParamDef>>,
    pub r#type: Option<u32>,
    pub sampler_values: Option<Vec<StaticMaterialShaderSamplerDef>>,
    pub shader_macros: Option<HashMap<String, String>>,
    pub switches: Option<Vec<StaticMaterialSwitchDef>>,
    pub techniques: Vec<StaticMaterialTechniqueDef>,
    pub unk_0xe251b20a: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StaticMaterialPassDef {
    pub blend_enable: Option<bool>,
    pub cull_enable: Option<bool>,
    pub depth_compare_func: Option<u32>,
    pub depth_enable: Option<bool>,
    pub depth_offset_bias: Option<f32>,
    pub depth_offset_slope: Option<f32>,
    pub dst_alpha_blend_factor: Option<u32>,
    pub dst_color_blend_factor: Option<u32>,
    pub polygon_depth_bias_enable: Option<bool>,
    pub shader: u32,
    pub shader_macros: Option<HashMap<String, String>>,
    pub src_alpha_blend_factor: Option<u32>,
    pub src_color_blend_factor: Option<u32>,
    pub stencil_compare_func: Option<u32>,
    pub stencil_enable: Option<bool>,
    pub stencil_mask: Option<u32>,
    pub stencil_reference_val: Option<u8>,
    pub winding_to_cull: Option<u32>,
    pub write_mask: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StaticMaterialShaderParamDef {
    pub name: String,
    pub value: Option<Vec4>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StaticMaterialShaderSamplerDef {
    pub address_u: Option<u32>,
    pub address_v: Option<u32>,
    pub address_w: Option<u32>,
    pub sampler_name: Option<String>,
    pub texture_name: String,
    pub texture_path: Option<String>,
    pub uncensored_textures: Option<HashMap<u32, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StaticMaterialSwitchDef {
    pub group: Option<String>,
    pub name: String,
    pub on: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StaticMaterialTechniqueDef {
    pub name: String,
    pub passes: Vec<StaticMaterialPassDef>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StopAnimationEventData {
    pub m_end_frame: Option<f32>,
    pub m_name: Option<u32>,
    pub m_start_frame: Option<f32>,
    pub m_stop_animation_name: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StoreCategoryButtonDefinition {
    pub button: u32,
    pub category: u32,
    pub new_pip: Option<Unk0x6241da2>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StoreCategoryInventoryFilter {
    pub m_inventory_filters: Option<Vec<Box<StoreCategoryInventoryFilter>>>,
    pub m_inventory_type: String,
    pub m_item_i_ds: Option<Vec<i32>>,
    pub m_order: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct StoreCategoryPageData {
    pub m_inventory_filters: Vec<StoreCategoryInventoryFilter>,
    pub m_name: String,
    pub m_order: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct StoreDialogViewController {
    pub base_loadable: u32,
    pub close_button_definition: u32,
    pub content_scene: u32,
    pub footer_hyperlinks: Option<Vec<UiHyperlink>>,
    pub grid_max_size: Option<u32>,
    pub offer_grid: u32,
    pub path_hash_to_self: u64,
    pub ui_store_item_tile_data: UiStoreItemTileData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StoreElementData {
    pub id: String,
    pub instance_data: Vec<StoreInstanceData>,
    pub preferred_order: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StoreInstanceData {
    pub enabled: Option<bool>,
    pub end_date: String,
    pub foreground_vfx: Option<TextureOverride>,
    pub id: String,
    pub region_data: Option<EnabledRegionData>,
    pub store_category: Option<u32>,
    pub title_text_tra: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct StoreListingData {
    pub m_identity_instance: IdentityInstance,
    pub mobile_catalog_data: Option<MobileCatalogData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct StoreViewController {
    pub base_loadable: u32,
    pub catalog_background: u32,
    pub filter_arena_toggle: u32,
    pub filter_featured_toggle: u32,
    pub filter_tactician_toggle: u32,
    pub footer_grid: u32,
    pub footer_hyperlinks: Vec<UiHyperlink>,
    pub mythic_view: StoreViewMythic,
    pub offer_grid: u32,
    pub path_hash_to_self: u64,
    pub purchases_view: StoreViewPurchases,
    pub searchbar_text: u32,
    pub store_category_buttons: Vec<StoreCategoryButtonDefinition>,
    pub tablet_override_loadable: u32,
    pub ui_store_item_tile_data: UiStoreItemTileData,
    pub unk_0x1d9b78e8: Unk0xb2430347,
    pub unk_0x20f2a4c4: u32,
    pub unk_0x26546c1d: Unk0x7296321a,
    pub unk_0x2d63ee49: u32,
    pub unk_0x2eed7e1b: u32,
    pub unk_0x42cb32fc: u32,
    pub unk_0x4b90d43d: u32,
    pub unk_0x52656dba: u32,
    pub unk_0x58104add: Vec<Unk0x69bf9d05>,
    pub unk_0x6528ad2f: u32,
    pub unk_0x7b557bdc: u32,
    pub unk_0x7ee661cb: u32,
    pub unk_0x815aaee7: u32,
    pub unk_0x834e2e8d: Unk0x834e2e8d,
    pub unk_0x9a183364: HashMap<u8, Unk0xd07fc429>,
    pub unk_0xaf6acce9: Unk0xaf6acce9,
    pub unk_0xb94f84c3: u32,
    pub unk_0xd5846a0a: u32,
    pub unk_0xe0ece3d9: u32,
    pub unk_0xe2deb7ce: HashMap<u8, Unk0x3c2230b7>,
    pub unk_0xf2c9b3be: u32,
    pub view_pane_link: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StoreViewMythic {
    pub background_texture: u32,
    pub managed_layout: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StoreViewPurchases {
    pub no_purchases_text: u32,
    pub purchase_footer: u32,
    pub purchase_grid: u32,
    pub purchase_header: u32,
    pub purchases_loading_vfx: u32,
    pub ui_store_purchase_row_data: UiStorePurchaseRowData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StrawberryAugmentDisplayTagData {
    pub augment_display_tag_frame: u32,
    pub augment_display_tag_row: u32,
    pub augment_display_tag_spacer: u32,
    pub augment_display_tag_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct StrawberryAugmentSelectionViewController {
    pub augment_selection_grid: u32,
    pub augment_slot_data: StrawberryAugmentSlotData,
    pub base_loadable: u32,
    pub main_scene: u32,
    pub path_hash_to_self: u64,
    pub reroll_button_data: Unk0x7f644206,
    pub timeout_prevent_clicks_on_show: f32,
    pub unk_0x56de9f99: u32,
    pub unk_0x583ec19b: String,
    pub unk_0x6da1a863: Unk0xfc6af367,
    pub unk_0x88f0f56e: Vec<Unk0xea7bb717>,
    pub unk_0x9c2732f2: u32,
    pub unk_0x9f720c: u32,
    pub unk_0xaf65bd1: f32,
    pub unk_0xce64b8ab: Unk0xfc6af367,
    pub unk_0xd93a5b65: Unk0xfc6af367,
    pub unk_0xf8d5ccda: Unk0xfc6af367,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StrawberryAugmentSlotData {
    pub augment_button: u32,
    pub augment_description: u32,
    pub augment_display_tag_data: StrawberryAugmentDisplayTagData,
    pub augment_group: u32,
    pub augment_hover_vfx: u32,
    pub augment_icon: u32,
    pub augment_idle_vfx: u32,
    pub augment_level_title: u32,
    pub augment_level_up_data: u32,
    pub augment_name: u32,
    pub augment_not_picked_vfx: u32,
    pub augment_picked_vfx: u32,
    pub augment_refresh_overlay_vfx: u32,
    pub augment_refresh_vfx: u32,
    pub unk_0x100d59ce: u32,
    pub unk_0x4ddb7d67: u32,
    pub unk_0x508e2ebe: Unk0xc62b91e4,
    pub unk_0xba750138: u32,
    pub unk_0xc9531aa3: u32,
    pub unk_0xcf83fb1e: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct StrawberryHeroStatsViewController {
    pub augment_slots: Unk0xfc331f53,
    pub background: u32,
    pub base_loadable: u32,
    pub basic_stats: u32,
    pub flipped_minimap_override: u32,
    pub path_hash_to_self: u64,
    pub stats_ui_data: DisplayStatsUiData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct StrawberryRoundsViewController {
    pub base_loadable: u32,
    pub current_phase_icon: Unk0x43aaf187,
    pub left_phase_icons: Vec<Unk0x43aaf187>,
    pub path_hash_to_self: u64,
    pub right_phase_icons: Vec<Unk0x43aaf187>,
    pub round_label: u32,
    pub round_label_tra: String,
    pub scene: u32,
    pub timer_left_bar: Unk0xbe081d2c,
    pub timer_right_bar: Unk0xbe081d2c,
    pub timer_text: u32,
    pub timer_text_default_color: [u8; 4],
    pub tooltip_anchor: u32,
    pub unk_0x3aa6852c: u32,
    pub unk_0xb03d7e4b: [u8; 4],
    pub unk_0xb1f34e3f: Unk0x45f140fc,
    pub unk_0xcf3fe190: u32,
    pub unk_0xd24a0877: HashMap<u8, String>,
    pub unk_0xe527f39d: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StringGet {
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StringTableGet {
    pub table: Option<EnumVars>,
    pub var: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StringTableSet {
    pub var: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct StructureFloatingInfoBarData {
    pub aggro: u32,
    pub anchor: u32,
    pub border: u32,
    pub burst_data: FloatingHealthBarBurstData,
    pub burst_fade_meter: u32,
    pub damage_flash_meter: u32,
    pub death_anim_ally: u32,
    pub death_anim_enemy: u32,
    pub health_bar: HealthBarData,
    pub highlight: u32,
    pub objective_bounty_ally: u32,
    pub objective_bounty_enemy: u32,
    pub par_bar: Option<AbilityResourceBarData>,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubPartScaledProportionalToStat {
    pub m_ratio: f32,
    pub m_stat: Option<u8>,
    pub m_style_tag: Option<String>,
    pub m_style_tag_if_scaled: Option<String>,
    pub m_subpart: Box<EnumAbilityResourceByCoefficientCalculationPart>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubmeshVisibilityBoolDriver {
    pub any_submesh: Option<bool>,
    pub submeshes: Vec<u32>,
    pub visible: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubmeshVisibilityEventData {
    pub apply_only_to_avatar_meshes: Option<bool>,
    pub m_end_frame: Option<f32>,
    pub m_fire_if_animation_ends_early: Option<bool>,
    pub m_hide_submesh_list: Option<Vec<u32>>,
    pub m_is_self_only: Option<bool>,
    pub m_name: Option<u32>,
    pub m_show_submesh_list: Option<Vec<u32>>,
    pub m_start_frame: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SubteamSurrenderViewController {
    pub base_loadable: u32,
    pub default_title_tra: String,
    pub done_voting_title_tra: String,
    pub main_scene: u32,
    pub no_button: u32,
    pub path_hash_to_self: u64,
    pub status_text: u32,
    pub timer_bar: u32,
    pub title_text: u32,
    pub vote_icon: u32,
    pub yes_button: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SumOfSubPartsCalculationPart {
    pub m_subparts: Vec<Box<EnumAbilityResourceByCoefficientCalculationPart>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SummonerNameUiData {
    pub summoner_name_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SurrenderData {
    pub fountain_detection_duration_secs: f32,
    pub m_min_afk_time_for_no_vote: f32,
    pub m_min_time_between_surrenders: Option<f32>,
    pub m_type_data: HashMap<u8, SurrenderTypeData>,
    pub time_inactive_for_early_surrender_secs: Option<f32>,
    pub unk_0x4987e889: Option<f32>,
    pub unk_0xd9d17df4: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SurrenderTypeData {
    pub percentage_required: Option<f32>,
    pub start_time: Option<f32>,
    pub start_time_with_afk: Option<f32>,
    pub vote_timeout: Option<f32>,
    pub window_length: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct SurrenderViewController {
    pub base_loadable: u32,
    pub default_text: u32,
    pub early_text: u32,
    pub main_scene: u32,
    pub no_button: u32,
    pub path_hash_to_self: u64,
    pub remake_text: u32,
    pub status_text: u32,
    pub timer_bar: u32,
    pub vote_icon: u32,
    pub yes_button: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwitchMaterialDriver {
    pub m_default_value: Box<EnumDriver>,
    pub m_elements: Option<Vec<Box<SwitchMaterialDriverElement>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwitchMaterialDriverElement {
    pub m_condition: Box<EnumDriver>,
    pub m_value: Box<EnumDriver>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SyncCircleMovement {
    pub m_angular_velocity: f32,
    pub m_axis_of_rotation: Option<u8>,
    pub m_lifetime: Option<f32>,
    pub m_offset_initial_target_height: Option<f32>,
    pub m_rotate_around_caster_facing_direction: Option<bool>,
    pub m_start_bone_name: Option<String>,
    pub m_target_bone_name: Option<String>,
    pub m_visuals_track_hidden_targets: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SyncGroupData {
    pub m_type: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SyncedAnimationEventData {
    pub m_lerp_time: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TableValueExistsScriptCondition {
    pub table_value: ScriptTableSet,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TakeCamp {
    pub camp: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub m_can_complete_cast_without_target: Option<bool>,
    pub unk_0xfb5bbd7: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargetFrameMeterSkinData {
    pub meter_ally_skin: UiElementMeterSkin,
    pub meter_enemy_skin: UiElementMeterSkin,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TargetFrameViewController {
    pub abilities_data: Option<AbilitiesUiData>,
    pub augment_slots: Option<Unk0xfc331f53>,
    pub base_loadable: u32,
    pub draw_area_list: Option<DrawAreaList>,
    pub experience_meter: Option<u32>,
    pub general_metrics: Vec<UiMetricVietnameseRatingLabel>,
    pub health_meter: HealthMeter,
    pub health_meter_skins: Option<TargetFrameMeterSkinData>,
    pub inventory_data: Option<SimpleItemSlots>,
    pub level_data: UnitLevelUiData,
    pub metrics: Option<Vec<EnumUiMetric>>,
    pub negative_buffs: BuffDisplayList,
    pub par_bar_data: AbilityResourceBarData,
    pub path_hash_to_self: u64,
    pub portrait: u32,
    pub positive_buffs: BuffDisplayList,
    pub scene: u32,
    pub stats_ui_data: UnitStatsUiData,
    pub target_closed_scene: u32,
    pub target_stats_menu: Option<UiTargetStatsMenu>,
    pub tooltip_region: u32,
    pub use_square_portrait: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargetHasUnitTagFilter {
    pub unit_tags: ObjectTags,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargetLaserComponentEffects {
    pub beam_effect_definition: SkinCharacterDataPropertiesCharacterIdleEffect,
    pub champ_targeting_effect_definition: Option<SkinCharacterDataPropertiesCharacterIdleEffect>,
    pub tower_targeting_effect_definition: Option<SkinCharacterDataPropertiesCharacterIdleEffect>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargetTypeFilter {
    pub champions_are_valid: Option<bool>,
    pub minions_are_valid: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargeterDefinitionAoe {
    pub center_locator: Option<DrawablePositionLocator>,
    pub constraint_pos_locator: Option<DrawablePositionLocator>,
    pub constraint_range: Option<FloatPerSpellLevel>,
    pub dynamic_game_calc_size_scalar: Option<GameCalculationModified>,
    pub is_constrained_to_range: Option<bool>,
    pub max_range_size_scalar: Option<TargeterDefinitionAoeScalar>,
    pub override_radius: Option<FloatPerSpellLevel>,
    pub texture_orientation: Option<u32>,
    pub texture_radius_override_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargeterDefinitionAoeScalar {
    pub scalar: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargeterDefinitionArc {
    pub constraint_range: FloatPerSpellLevel,
    pub end_locator: DrawablePositionLocator,
    pub is_constrained_to_range: bool,
    pub override_radius: FloatPerSpellLevel,
    pub start_locator: Option<DrawablePositionLocator>,
    pub texture_arc_override_name: Option<String>,
    pub thickness_offset: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargeterDefinitionCone {
    pub cone_angle_degrees: Option<f32>,
    pub cone_follows_end: Option<bool>,
    pub cone_range: Option<f32>,
    pub end_locator: DrawablePositionLocator,
    pub fallback_direction: Option<u32>,
    pub start_locator: Option<DrawablePositionLocator>,
    pub texture_cone_override_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargeterDefinitionLine {
    pub always_draw: Option<bool>,
    pub arrow_size: Option<f32>,
    pub end_locator: Option<DrawablePositionLocator>,
    pub facing_line: Option<bool>,
    pub fade: Option<bool>,
    pub fallback_direction: Option<u32>,
    pub indicator_type: Option<EnumIndicatorType>,
    pub line_stops_at_end_position: Option<bool>,
    pub line_width: Option<FloatPerSpellLevel>,
    pub m_center_arrow_to_end_point: Option<bool>,
    pub m_fade_behavior: Option<FadeOverTimeBehavior>,
    pub max_angle: Option<f32>,
    pub minimum_displayed_range: Option<f32>,
    pub override_base_range: Option<FloatPerSpellLevel>,
    pub range_growth_duration: Option<FloatPerSpellLevel>,
    pub range_growth_max: Option<FloatPerSpellLevel>,
    pub range_growth_start_time: Option<FloatPerSpellLevel>,
    pub start_locator: Option<DrawablePositionLocator>,
    pub texture_base_max_grow_name: Option<String>,
    pub texture_base_override_name: Option<String>,
    pub texture_target_max_grow_name: Option<String>,
    pub texture_target_override_name: Option<String>,
    pub use_global_line_indicator: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargeterDefinitionMinimap {
    pub center_locator: Option<DrawablePositionLocator>,
    pub override_base_range: Option<FloatPerSpellLevel>,
    pub use_caster_bounding_box: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargeterDefinitionMultiAoe {
    pub angel_offset_radian: Option<f32>,
    pub center_locator: DrawablePositionLocator,
    pub inner_texture_name: String,
    pub left_texture_name: String,
    pub num_of_inner_aoe: Option<u32>,
    pub override_aoe_radius: Option<FloatPerSpellLevel>,
    pub override_max_cast_range: FloatPerSpellLevel,
    pub override_min_cast_range: FloatPerSpellLevel,
    pub right_texture_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargeterDefinitionRange {
    pub center_locator: Option<DrawablePositionLocator>,
    pub has_max_grow_range: Option<bool>,
    pub hide_with_line_indicator: Option<bool>,
    pub m_fade_behavior: Option<EnumBehavior>,
    pub override_base_range: Option<FloatPerSpellLevel>,
    pub range_growth_duration: Option<FloatPerSpellLevel>,
    pub range_growth_max: Option<FloatPerSpellLevel>,
    pub range_growth_start_time: Option<FloatPerSpellLevel>,
    pub texture_max_grow_name: Option<String>,
    pub texture_orientation: Option<u32>,
    pub texture_override_name: Option<String>,
    pub use_caster_bounding_box: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargeterDefinitionSkipTerrain {
    pub m_base_texture_name: String,
    pub m_end_locator: DrawablePositionLocator,
    pub m_terrain_texture_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargeterDefinitionSpline {
    pub base_texture_name: String,
    pub constraint_range: FloatPerSpellLevel,
    pub end_locator: DrawablePositionLocator,
    pub front_texture_name: String,
    pub is_constrained_to_range: bool,
    pub override_spline: HermiteSplineInfo,
    pub spline_width: FloatPerSpellLevel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargeterDefinitionWall {
    pub center_locator: DrawablePositionLocator,
    pub length: FloatPerSpellLevel,
    pub texture_wall_override_name: Option<String>,
    pub thickness: FloatPerSpellLevel,
    pub wall_rotation: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargetingForgivenessDefinitions {
    pub caster_forgiveness_definitions: Option<Vec<SameTeamCastRequirement>>,
    pub forgiveness_range: f32,
    pub m_affects_type_override: Option<u32>,
    pub override_affects_flags: Option<bool>,
    pub target_forgiveness_definitions: Option<Vec<EnumHasAllSubRequirementsCastRequirement>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargetingParameters {
    pub exit_conditions: Option<Vec<u8>>,
    pub m_affects_status_flags: Option<u32>,
    pub m_affects_type_flags: u32,
    pub m_spell_flags: Option<u32>,
    pub range_value: EnumTargetingRangeValue,
    pub targeting_param_name: Option<String>,
    pub unit_object_tags: Option<ObjectTags>,
    pub unk_0x791c5fa3: Option<bool>,
    pub unk_0x9845aa67: Option<bool>,
    pub unk_0xfc462d60: Option<Vec<Unk0xe90af953>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargetingPriorityList {
    pub m_spell_flags: u32,
    pub targeting_parameters_list: Vec<TargetingParameters>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargetingRangeValue {
    pub range: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamBuffData {
    pub m_buff_name: String,
    pub m_globally_visible: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TeamFramesViewController {
    pub base_loadable: u32,
    pub flipped_minimap_override: u32,
    pub layout: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub team_frame_template: UiTeamMemberData,
    pub unk_0x25f3de6e: u32,
    pub unk_0xd24680d: Option<HudReciprocityButton>,
    pub unk_0xfd9ba272: Option<HudReciprocityButton>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TerrainType {
    pub m_brush_cursor: CursorData,
    pub m_river_cursor: CursorData,
    pub m_wall_cursor: CursorData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextureAndColorData {
    pub color: Option<[u8; 4]>,
    pub colorblind_color: Option<[u8; 4]>,
    pub colorblind_texture_file: Option<String>,
    pub texture_file: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextureOverride {
    pub texture_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TextureResource {
    pub texture_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftAnnouncementsViewController {
    pub animated_icon_link: u32,
    pub base_loadable: u32,
    pub center_icon_link: u32,
    pub center_title_link: u32,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub scene_link: u32,
    pub scene_transition_in: HudMenuTransitionData,
    pub scene_transition_out: HudMenuTransitionData,
    pub unk_0x9f1b4f78: u32,
    pub unk_0xbafb3268: u32,
    pub unk_0xbe81aa92: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftArmoryActionButtonData {
    pub action_button: u32,
    pub disable_on_action: Option<bool>,
    pub dynamic_numeric_text: Option<u32>,
    pub dynamic_numeric_text_color: Option<[u8; 4]>,
    pub dynamic_numeric_text_disabled_color: Option<[u8; 4]>,
    pub is_active_by_default: Option<bool>,
    pub numeric_text_default: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftArmoryBadgeDisplayData {
    pub group: u32,
    pub icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftArmoryButtonStyleData {
    pub description: Option<u32>,
    pub item_button: u32,
    pub subtitle: Option<u32>,
    pub title: Option<u32>,
    pub unk_0x63df7bd3: Option<Vec<u32>>,
    pub unk_0x70927b29: Option<Vec<u32>>,
    pub use_bordered_icon: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftArmoryItemTraitData {
    pub backgrounds: Option<HashMap<u32, u32>>,
    pub icon: u32,
    pub name: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftArmorySlotData {
    pub badge_data: Option<TftArmoryBadgeDisplayData>,
    pub button_styles: Option<HashMap<String, TftArmoryButtonStyleData>>,
    pub category_text: Option<u32>,
    pub character_tier_groups: Option<Vec<u32>>,
    pub default_button_style: TftArmoryButtonStyleData,
    pub desaturate_on_inactive: Option<Vec<u32>>,
    pub element_group: u32,
    pub icon_mask_region: Option<u32>,
    pub item_action: Option<u8>,
    pub item_action_button: Option<u32>,
    pub item_drag_region: Option<u32>,
    pub item_footprint_selected_vfx: u32,
    pub item_footprint_unselected_vfx: u32,
    pub item_icon: u32,
    pub item_icon_bordered: u32,
    pub recipe_hint_button: Option<u32>,
    pub scene: Option<u32>,
    pub selection_cost_text: Option<u32>,
    pub traits: Option<Vec<TftArmoryItemTraitData>>,
    pub unk_0x1475bab7: Option<u32>,
    pub unk_0x19d35524: Option<u32>,
    pub unk_0x351e8202: Option<Vec<u32>>,
    pub unk_0x4bdd71fb: Option<u32>,
    pub unk_0x692794b6: Option<u32>,
    pub unk_0xa51a4d37: Option<u32>,
    pub unk_0xb5063433: Option<HashMap<u8, u32>>,
    pub unk_0xbc8f3dd5: Option<u8>,
    pub unk_0xcd2b3ba4: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftArmoryViewController {
    pub allow_single_item_selection: Option<bool>,
    pub armory_action_buttons: Option<HashMap<u8, TftArmoryActionButtonData>>,
    pub armory_style: Option<u8>,
    pub badge_display_property: Option<u32>,
    pub base_loadable: u32,
    pub can_sell_during_armory: Option<bool>,
    pub click_show_recipe_hint_tra_key: String,
    pub current_gold_icon: Option<u32>,
    pub current_gold_text: Option<u32>,
    pub full_description_tra_key: String,
    pub gold_scene: Option<u32>,
    pub has_timer_bar: Option<bool>,
    pub hide_button_scene: u32,
    pub item_slot_data: TftArmorySlotData,
    pub item_space: u32,
    pub main_scene: u32,
    pub max_item_slot: Option<u32>,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub pre_armory_vfx: Option<u32>,
    pub recipe_hint_tra_key: String,
    pub sell_areas: Option<Vec<TftSellArea>>,
    pub should_hide_buttons_on_choice: Option<bool>,
    pub timeout_double_click_prevention: Option<f32>,
    pub timeout_prevent_clicks_on_show: Option<f32>,
    pub timeout_vfx_duration: f32,
    pub timeout_vfx_on_toggle_button: u32,
    pub timer_bar_background: Option<u32>,
    pub timer_element_group: Option<u32>,
    pub timer_fill_texture: Option<u32>,
    pub timer_text: Option<u32>,
    pub title_text: u32,
    pub toggle_main_view_button: u32,
    pub unk_0x284d353b: Option<u32>,
    pub unk_0x2cf5bb7a: Option<u8>,
    pub unk_0x2fd3d3c2: Option<u32>,
    pub unk_0x56dd541c: Option<HashMap<String, u8>>,
    pub unk_0x7812e733: Option<Vec<EnumUnk0x3b8a61ee>>,
    pub unk_0x8992bd5d: Option<u32>,
    pub unk_0xb1e401dc: Option<u32>,
    pub unk_0xbe1f6391: Option<bool>,
    pub unk_0xe23ea3ba: Option<Unk0xf96d9400>,
    pub use_sequencer: Option<bool>,
    pub use_toggle_button_on_pc: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftAugmentDisplayData {
    pub augment_button: u32,
    pub icon: u32,
    pub placeholder: u32,
    pub texts: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftAugmentInfoViewController {
    pub background: u32,
    pub base_loadable: u32,
    pub default_title_tra: String,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub slots: Vec<TftAugmentDisplayData>,
    pub title_icon: u32,
    pub title_text: u32,
    pub unk_0x885bb7de: Vec<Unk0x1f0bbd6>,
    pub unk_0xcc709614: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftBannerIconData {
    pub button: u32,
    pub group: u32,
    pub icon: u32,
    pub new_pip: Unk0x6241da2,
    pub unk_0x8bdf5cc4: Option<u32>,
    pub unk_0xd02a6781: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftBaseTraitSetData {
    pub activated_buff_name: Option<String>,
    pub constants: Option<Unk0xd65315ee>,
    pub unk_0x86b5abcb: Option<Vec<Unk0x76621fa6>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftBattlepassSelectedRewardBannerUiData {
    pub claim_ready_group: u32,
    pub claim_text: u32,
    pub lock_group: u32,
    pub lock_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftBattlepassSelectedRewardUiData {
    pub body_text: u32,
    pub claim_ceremony_vfx: u32,
    pub frame_available: u32,
    pub frame_locked: u32,
    pub premium_bg_vfx: u32,
    pub reward_icon: u32,
    pub title_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftBattlepassViewController {
    pub background_image: u32,
    pub base_loadable: u32,
    pub default_theme_data: Unk0x5cb6b755,
    pub loot_table_button: u32,
    pub milestone_progress_meter: UiMilestoneProgressMeter,
    pub milestone_timeline_end: UiMilestoneTimelineEnd,
    pub milestones_view_pane: u32,
    pub modal_dialog_view_controller: u32,
    pub pass_subtitle: u32,
    pub pass_title: u32,
    pub path_hash_to_self: u64,
    pub selected_milestone_highlight: u32,
    pub selected_reward: TftBattlepassSelectedRewardUiData,
    pub selected_reward_banner: TftBattlepassSelectedRewardBannerUiData,
    pub selected_reward_banner_scene: u32,
    pub tablet_override_loadable: u32,
    pub theme_data_map: HashMap<String, Unk0x5cb6b755>,
    pub ui_milestone_mission_template: UiMilestoneMissionTemplate,
    pub unk_0x2eed7e1b: u32,
    pub unk_0x74fcfeb2: u32,
    pub unk_0xd1373982: u32,
    pub unk_0xd6e52a1f: u32,
    pub unk_0xe53aff6c: u32,
    pub unk_0xf3399e0e: u32,
    pub unk_0xfbf302db: Unk0x41bdce89,
    pub upgrade_pass_button: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftCharacterPositioningData {
    pub icon: u32,
    pub text_tra: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftCharacterRecord {
    pub attack_range: Option<f32>,
    pub attack_speed: Option<f32>,
    pub attack_speed_ratio: Option<f32>,
    pub base_armor: Option<f32>,
    pub base_crit_chance: Option<f32>,
    pub base_damage: Option<f32>,
    pub base_hp: Option<f32>,
    pub base_move_speed: Option<f32>,
    pub base_spell_block: Option<f32>,
    pub base_static_hp_regen: Option<f32>,
    pub basic_attack: Option<AttackSlotData>,
    pub board_count_contribution: Option<u8>,
    pub character_role: Option<u32>,
    pub crit_attacks: Option<Vec<AttackSlotData>>,
    pub crit_damage_multiplier: Option<f32>,
    pub exp_given_on_death: Option<f32>,
    pub extra_attacks: Option<Vec<AttackSlotData>>,
    pub extra_spells: Option<Vec<String>>,
    pub flags: Option<u32>,
    pub gold_given_on_death: Option<f32>,
    pub health_bar_full_parallax: Option<bool>,
    pub health_bar_height: Option<f32>,
    pub hp_per_level: Option<f32>,
    pub linked_units: Option<Vec<u32>>,
    pub m_character_name: String,
    pub m_initial_mana: Option<f32>,
    pub m_is_discoverable: Option<bool>,
    pub m_linked_traits: Option<Vec<TftTraitContributionData>>,
    pub m_mana_per_attack: Option<f32>,
    pub m_move_height: Option<f32>,
    pub m_move_interval: Option<f32>,
    pub m_move_proximity: Option<f32>,
    pub m_move_range: Option<f32>,
    pub m_shop_data: Option<u32>,
    pub m_unit_info_card_attach_to_bone: Option<String>,
    pub m_unit_info_card_world_offset: Option<Vec3>,
    pub m_uses_ability_power: Option<bool>,
    pub mobile_health_bar_height_override: Option<f32>,
    pub num_to_combine: Option<u8>,
    pub omit_from_combat_recap: Option<bool>,
    pub omit_from_match_history: Option<bool>,
    pub passive_range: Option<f32>,
    pub primary_ability_resource: Option<AbilityResourceSlotInfo>,
    pub secondary_ability_resource: Option<AbilityResourceSlotInfo>,
    pub selection_height: Option<f32>,
    pub selection_radius: Option<f32>,
    pub show_items_unsorted: Option<bool>,
    pub spell_names: Option<Vec<String>>,
    pub spells: Option<Vec<u32>>,
    pub tier: Option<u8>,
    pub unit_info_card_display_type: Option<u32>,
    pub unit_tags_string: Option<String>,
    pub unk_0xc1984296: Option<Vec<u32>>,
    pub unk_0xf94987a3: Option<bool>,
    pub unk_0xfe0d6477: Option<Vec<u32>>,
    pub useable_data: Option<UseableData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftCharacterRoleCardHeader {
    pub description: u32,
    pub icon: u32,
    pub title: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftCharacterRoleCardItem {
    pub component_icons: Vec<u32>,
    pub group: u32,
    pub item_icon: u32,
    pub recipe_group: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftCharacterRoleCardViewController {
    pub base_loadable: u32,
    pub grid: u32,
    pub header_data: TftCharacterRoleCardHeader,
    pub item_data: TftCharacterRoleCardItem,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftCharacterRoleData {
    pub character_role_name_tra: String,
    pub description_tra: String,
    pub icon: u32,
    pub item_role_name_tra: String,
    pub items: Vec<u32>,
    pub name: String,
    pub unk_0x116bd9b: String,
    pub unk_0x886be411: String,
    pub unk_0xa1ad92a7: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftCombatRecapLineTemplate {
    pub hit_region: u32,
    pub meter_fill_heal_casted: u32,
    pub meter_fill_magic_blocked: u32,
    pub meter_fill_magic_dealt: u32,
    pub meter_fill_physical_blocked: u32,
    pub meter_fill_physical_dealt: u32,
    pub meter_fill_shield_granted: u32,
    pub meter_fill_true_blocked: u32,
    pub meter_fill_true_dealt: u32,
    pub scene: u32,
    pub selected_icon: u32,
    pub star_badges: Vec<u32>,
    pub star_borders: Vec<u32>,
    pub text_amount_total: u32,
    pub tooltip_region: u32,
    pub unit_empty_slot: u32,
    pub unit_icon: u32,
    pub unit_slot_regions: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftCombatRecapPanelTemplate {
    pub left_view_panelink: u32,
    pub mode_label: u32,
    pub opponent_background: u32,
    pub opponent_fill_meter: u32,
    pub opponent_frame: u32,
    pub opponent_health_text: u32,
    pub opponent_icon: u32,
    pub player_name: u32,
    pub scene: u32,
    pub view_pane_link: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftCombatRecapViewController {
    pub base_loadable: u32,
    pub button_scene: u32,
    pub combat_recap_line_template: TftCombatRecapLineTemplate,
    pub combat_recap_panel_template: TftCombatRecapPanelTemplate,
    pub defensive_mode_button: u32,
    pub mobile_override_loadable: u32,
    pub offensive_mode_button: u32,
    pub panel_transition: HudMenuTransitionData,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub toggle_left_panel_button: u32,
    pub toggle_left_panel_button_scene: u32,
    pub utility_mode_button: u32,
    pub vs_badge_scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftCondensedItemCombineTooltipData {
    pub component_icons: u32,
    pub item_icon_one: u32,
    pub item_icon_two: u32,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftConditionalTraitSetData {
    pub activated_buff_name: Option<String>,
    pub constants: Option<Unk0xd65315ee>,
    pub max_units: Option<u32>,
    pub min_units: u32,
    pub required_unit_property: Option<TftRequiredUnitProperty>,
    pub should_always_display: Option<bool>,
    pub style: Option<u8>,
    pub unk_0x22d8f6bc: Option<String>,
    pub unk_0x420ca0b2: Option<bool>,
    pub unk_0x86b5abcb: Option<Vec<Unk0x76621fa6>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftCraftingCelebrationViewController {
    pub base_loadable: u32,
    pub continue_button: u32,
    pub crafting_sequence: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub tablet_override_loadable: u32,
    pub unk_0x22c15c3b: String,
    pub unk_0x729f2ae: String,
    pub unk_0x7cbd3f05: u32,
    pub unk_0x923b6b2e: u32,
    pub unk_0x9395be56: String,
    pub unk_0xd36d5f6e: HashMap<u32, String>,
    pub unk_0xfde8ebb2: HashMap<u32, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftCraftingDialogViewController {
    pub background_texture: u32,
    pub base_loadable: u32,
    pub body_text: u32,
    pub cancel_button_definition: u32,
    pub close_button_definition: u32,
    pub confirm_button_definition: u32,
    pub content_scene: u32,
    pub error_body_tra: String,
    pub error_title_text: u32,
    pub path_hash_to_self: u64,
    pub tablet_override_loadable: u32,
    pub title_text: u32,
    pub unk_0xe6a5be46: String,
    pub unk_0xeccf9ffa: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftCurrency {
    pub currency_id: String,
    pub description_tra_key: Option<String>,
    pub name_tra_key: String,
    pub reward_texture_path: Option<String>,
    pub troves_reward_texture_path: Option<String>,
    pub unk_0x2853d34a: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftCurrencyWidget {
    pub currency_button: u32,
    pub tooltip_button: Option<u32>,
    pub tooltip_description_tra: String,
    pub tooltip_offset_y: f32,
    pub tooltip_title_tra: String,
    pub unk_0x3b061cb9: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftCustomAnnouncementData {
    pub player: Option<u32>,
    pub scene: u32,
    pub subtitle: Option<u32>,
    pub title: Option<u32>,
    pub unk_0x2fd3d3c2: Option<u32>,
    pub unk_0xbf98d308: Option<u32>,
    pub unk_0xeb01e454: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftCustomAnnouncementViewController {
    pub announcement_data: TftCustomAnnouncementData,
    pub base_loadable: u32,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub style: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftDropRateSlot {
    pub text: u32,
    pub tier_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftEliminationViewController {
    pub background: u32,
    pub base_loadable: u32,
    pub elimination_highlight_vfx: u32,
    pub leave_button: u32,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub rank_text: u32,
    pub scene: u32,
    pub spectate_button: u32,
    pub title_text: u32,
    pub victory_highlight_vfx: u32,
    pub waiting_highlight_vfx: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftEmotePanelViewController {
    pub base_loadable: u32,
    pub main_view_scene: u32,
    pub path_hash_to_self: u64,
    pub play_dance_animation_button: u32,
    pub play_joke_animation_button: u32,
    pub play_laugh_animation_button: u32,
    pub play_taunt_animation_button: u32,
    pub summoner_emote_buttons: Vec<u32>,
    pub summoner_emote_icons: Vec<u32>,
    pub toggle_emotes_button: u32,
    pub toplevel_scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftEncounterPanelViewController {
    pub base_loadable: u32,
    pub encounter_panel_scene: u32,
    pub encounter_slot_template: TftEncounterSlotData,
    pub encounter_toggle_button: u32,
    pub encounter_vertical_list: u32,
    pub mobile_override: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftEncounterSlotData {
    pub active_background: u32,
    pub description: u32,
    pub group_link: u32,
    pub icon: u32,
    pub title: u32,
    pub unk_0x9a800d92: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftEventHubLargeViewController {
    pub base_loadable: u32,
    pub close_button: u32,
    pub event_pass_thumbnail: Unk0xbf5c4715,
    pub event_timer: TftEventTimer,
    pub info_button: u32,
    pub mission_scroller: Unk0xdd8ea5ae,
    pub path_hash_to_self: u64,
    pub scene_root: u32,
    pub tablet_override_loadable: u32,
    pub troves_thumbnail: Unk0x22dd5ebf,
    pub unk_0x2eed7e1b: u32,
    pub unk_0x7d200cc1: Unk0x61902388,
    pub unk_0x96dd8579: u32,
    pub unk_0xe5388f19: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftEventHubViewController {
    pub base_loadable: u32,
    pub close_button: u32,
    pub event_timer: TftEventTimer,
    pub info_button: u32,
    pub path_hash_to_self: u64,
    pub scene_root: u32,
    pub tablet_override_loadable: u32,
    pub unk_0x2eed7e1b: u32,
    pub unk_0x45b34874: Unk0x1668b3e5,
    pub unk_0x5e0fc30b: Unk0xba9f6aca,
    pub unk_0xa0dd31e5: Unk0x4e16b860,
    pub unk_0xe5388f19: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftEventHubXsViewController {
    pub base_loadable: u32,
    pub close_button: u32,
    pub event_timer: TftEventTimer,
    pub info_button: u32,
    pub mission_scroller: Unk0xdd8ea5ae,
    pub path_hash_to_self: u64,
    pub scene_root: u32,
    pub tablet_override_loadable: u32,
    pub title_text: u32,
    pub troves_thumbnail: Unk0x22dd5ebf,
    pub unk_0x2eed7e1b: u32,
    pub unk_0x797fd50b: u32,
    pub unk_0x96dd8579: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftEventTimer {
    pub greyed_timer_text: Option<u32>,
    pub timer_frame: u32,
    pub timer_icon: u32,
    pub timer_text: u32,
    pub unk_0x2374a39f: String,
    pub unk_0x8938c438: String,
    pub unk_0xe1c84837: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftFloatingInfoBarViewController {
    pub base_loadable: u32,
    pub info_bar_style_source_map: HashMap<u8, u32>,
    pub mobile_loadable: u32,
    pub path_hash_to_self: u64,
    pub unit_status_priority_list: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftGameHeaderViewController {
    pub badge_bg: u32,
    pub badge_text: u32,
    pub badge_vfx: u32,
    pub base_loadable: u32,
    pub metrics: Vec<EnumUiMetric>,
    pub mobile_override_loadable: u32,
    pub music_source_button: u32,
    pub music_source_info: u32,
    pub options_menu_tooltip_tra_key: String,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub team_planner_disabled_tooltip_tra_key: String,
    pub team_planner_tooltip_tra_key: String,
    pub tft_game_settings_button: u32,
    pub tft_game_team_planner_button: u32,
    pub tft_traits_items_toggle_button: u32,
    pub tooltip_anchor: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftGameStartSequenceScene {
    pub ui_scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftGameStartSequenceSimple {
    pub base_loadable: u32,
    pub events: Option<Vec<TftGameStartSequenceSimpleEvent>>,
    pub main_scene: u32,
    pub mobile_override_loadable: u32,
    pub total_duration_secs: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftGameStartSequenceSimpleEvent {
    pub duration_secs: f32,
    pub start_time_secs: f32,
    pub ui_object: EnumTftGameStartSequenceScene,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftGameStartSequenceTutorial {
    pub base_loadable: u32,
    pub game_welcome_loc_scene: u32,
    pub game_welcome_loc_vn_vn: u32,
    pub game_welcome_loc_zh_cn: u32,
    pub game_welcome_loc_zh_tw: u32,
    pub game_welcome_scene: u32,
    pub game_welcome_vfx: u32,
    pub game_welcome_vfx_show_duration_secs: f32,
    pub game_welcome_vfx_start_time_secs: f32,
    pub mobile_override_loadable: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftGameStartViewController {
    pub game_start_table: HashMap<u32, u32>,
    pub path_hash_to_self: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftHintMessageData {
    pub message_anchor: Option<u8>,
    pub message_trakey: String,
    pub target_element: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftHintMessageViewController {
    pub arrow_icons: Vec<u32>,
    pub background: u32,
    pub base_loadable: u32,
    pub message_anchor_elements: Option<HashMap<u32, u32>>,
    pub mobile_override_loadable: Option<u32>,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub text: u32,
    pub vfx_scene: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftHintUiData {
    pub message: Option<TftHintMessageData>,
    pub vfx: TftHintVfxData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftHintVfxData {
    pub target_element: u32,
    pub vfx_scale: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftHudColorData {
    pub tft_chosen_color: Option<[u8; 4]>,
    pub tft_shop_drop_rate_text_colors_list: Vec<[u8; 4]>,
    pub unk_0x3fdee6e4: Vec<[u8; 4]>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftHudExpBar {
    pub backdrop: u32,
    pub exp_bar: u32,
    pub exp_bar_divider: u32,
    pub exp_scene: u32,
    pub radial_backdrop: u32,
    pub radial_exp_bar: u32,
    pub unk_0x234f4a0c: String,
    pub value_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftHudStreakUi {
    pub scene: u32,
    pub streak_hit_target: u32,
    pub streak_icons: Vec<u32>,
    pub streak_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftHudTraitUnitSlotBorders {
    pub item_slot_border: u32,
    pub unit_slot_borders: HashMap<u8, u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftHudTraitUnitSlotData {
    pub border: u32,
    pub overlay: u32,
    pub padding: u32,
    pub region: u32,
    pub unit_group_link: u32,
    pub unit_icon: u32,
    pub unit_shade: u32,
    pub unk_0x417ae5d6: Option<u32>,
    pub unk_0xe96f72ce: Unk0x2b365a82,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftHudUnitShopDropRate {
    pub drop_rate_slots_list: Vec<TftDropRateSlot>,
    pub unk_0x69b0a839: Option<u32>,
    pub unk_0xe2c6ffc9: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftItemCodexHeaderData {
    pub group: u32,
    pub main_item_brief: u32,
    pub main_item_icon: u32,
    pub main_item_main_text: u32,
    pub main_item_name: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftItemCodexViewController {
    pub base_bin_path: String,
    pub base_loadable: u32,
    pub condensed_item_combine_tooltip_data: TftCondensedItemCombineTooltipData,
    pub group_framed: u32,
    pub header_data: TftItemCodexHeaderData,
    pub horizontal_rule: u32,
    pub item_combine_tooltip_data: TftItemCombineTooltipData,
    pub layout_region: u32,
    pub mobile_bin_path: String,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub pc_mouse_offset_x: i32,
    pub pc_mouse_offset_y: i32,
    pub scene: u32,
    pub tft_item_codex_entry_data: TftItemCodexViewEntryData,
    pub unk_0x84907d75: Unk0x886b77ed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftItemCodexViewEntryData {
    pub arrow_icon: u32,
    pub component_icon: u32,
    pub full_item_icon: u32,
    pub full_item_name_text: u32,
    pub group: u32,
    pub highlight_icon: u32,
    pub hover_region: u32,
    pub plus_icon: u32,
    pub selected_icon: u32,
    pub unk_0x21cfd37c: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftItemCombineTooltipData {
    pub champion_icon: u32,
    pub champion_name: u32,
    pub component_icons: u32,
    pub fourstar_badge: u32,
    pub item_icon_one: u32,
    pub item_icon_two: u32,
    pub onestar_badge: u32,
    pub scene: u32,
    pub threestar_badge: u32,
    pub twostar_badge: u32,
    pub unk_0x21cfd37c: u32,
    pub unk_0x363f1e09: u32,
    pub unk_0x909e7f19: u32,
    pub unk_0xb0b19a20: u32,
    pub unk_0xb2f1de46: u32,
    pub unk_0xb4e6bd8d: u32,
    pub unk_0xc060ff55: u32,
    pub unk_0xc896d4da: u32,
    pub unk_0xcca3bd21: u32,
    pub unk_0xda650f10: u32,
    pub unk_0xf6e17fdc: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftItemData {
    pub constants: Unk0xd65315ee,
    pub icon_material: Option<u32>,
    pub incompatible_traits: Option<Vec<u32>>,
    pub item_tags: Vec<u32>,
    pub m_color: Option<[u8; 4]>,
    pub m_description_name_tra: String,
    pub m_display_name_tra: String,
    pub m_icon_path: String,
    pub m_is_unique: Option<bool>,
    pub m_name: String,
    pub mutually_exclusive_items: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftItemPanelViewController {
    pub base_loadable: u32,
    pub mobile_override_loadable: u32,
    pub panel_scene: u32,
    pub path_hash_to_self: u64,
    pub slot_regions: Vec<u32>,
    pub slot_template: TftItemSlotDisplayTemplate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftItemSlotDisplayTemplate {
    pub frame: u32,
    pub icon: u32,
    pub item_transition_in: HudMenuTransitionData,
    pub scene: u32,
    pub selected_frame: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftLobbyCustomAssets {
    pub custom_background_texture_path: String,
    pub unk_0x19e7962f: String,
    pub unk_0x6349b77b: String,
    pub unk_0x90b42524: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftLobbyCustomButtonPopupViewController {
    pub base_loadable: u32,
    pub confirm_button_definition: u32,
    pub content_scene: u32,
    pub path_hash_to_self: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftLobbyViewController {
    pub base_loadable: u32,
    pub bottom_buttons: LobbyBottomButtons,
    pub custom_button_popup_view_controller: u32,
    pub event_type_fields: HashMap<String, LobbyLabFields>,
    pub game_type_fields: HashMap<u32, LobbyLabFields>,
    pub in_queue_music_state: String,
    pub lab_popup_hit_region: u32,
    pub lobby_layouts: HashMap<u8, LobbyLayout>,
    pub lobby_music_state: String,
    pub lobby_scene: u32,
    pub path_hash_to_self: u64,
    pub rank_and_rated_tra_key: String,
    pub rank_only_tra_key: String,
    pub rated_only_tra_key: String,
    pub ready_check: LobbyReadyCheck,
    pub ready_check_accept_vfx: u32,
    pub ready_check_decline_vfx: u32,
    pub ready_check_timer_vfx: u32,
    pub theme_music_state_group: String,
    pub unk_0x2eed7e1b: u32,
    pub unk_0x354d2b95: u32,
    pub unk_0x47aa5f76: u32,
    pub unk_0x8963e73a: HashMap<u8, Unk0x470b636c>,
    pub unk_0xb7d07cd2: String,
    pub unk_0xd3665aac: u32,
    pub unk_0xe31300f: u32,
    pub unk_0xe397d18b: HashMap<u32, TftLobbyCustomAssets>,
    pub unk_0xf0a2677: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftMatchupBannerUiContainer {
    pub tactician_icon: u32,
    pub text: u32,
    pub unk_0xe291ee04: Vec<Unk0x84c56837>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftMatchupBannerViewController {
    pub banner_duration_secs: f32,
    pub base_loadable: u32,
    pub left_player: TftMatchupBannerUiContainer,
    pub main_scene: u32,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub right_player: TftMatchupBannerUiContainer,
    pub round_text: u32,
    pub unk_0x6e9023c0: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftOptionsViewController {
    pub active_tab_scroll_region: u32,
    pub active_tab_with_footer_scroll_region: u32,
    pub base_loadable: u32,
    pub build_version_text: u32,
    pub build_version_text_footer: u32,
    pub button1_definition: u32,
    pub button2_definition: u32,
    pub cancel_hit_region: u32,
    pub close_button_definition: u32,
    pub default_menu_button_tra_keys: Vec<String>,
    pub exit_hit_region: u32,
    pub footer_scene: u32,
    pub korea_ratings_icon_element: u32,
    pub last_item_padding: u32,
    pub mobile_menu_button_tra_keys: Vec<String>,
    pub mobile_override_loadable: u32,
    pub okay_hit_region: u32,
    pub options_style: u8,
    pub path_hash_to_self: u64,
    pub restore_defaults_hit_region: u32,
    pub surrender_hit_region: u32,
    pub tab_button_definition: u32,
    pub tabs: Vec<u32>,
    pub view_pane_link: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftPayoutViewController {
    pub banner_duration_secs: f32,
    pub base_loadable: u32,
    pub main_scene: u32,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub progress_bar: u32,
    pub reward_bars: Vec<Unk0x209fa685>,
    pub reward_number_display: u32,
    pub reward_trakey: String,
    pub unk_0x87eb3d0: f32,
    pub unk_0xb2a69226: [u8; 4],
    pub unk_0xc60338b8: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftPlayerChoiceViewController {
    pub base_loadable: u32,
    pub mobile_override_loadable: u32,
    pub option_display_data: Unk0x34cca270,
    pub option_hover_region: u32,
    pub option_modal_data: Unk0xf3cf86a3,
    pub option_region: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftRequiredUnitProperty {
    pub active_condition: Option<bool>,
    pub property: u32,
    pub unk_0x10fedaa0: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftReturnToBoardViewController {
    pub armory_timeout_vfx: u32,
    pub base_loadable: u32,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub scene_handle: u32,
    pub unk_0xa3dafbbb: u32,
    pub unk_0xc8c453d3: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftScoreboardNotificationTemplate {
    pub notification_bottom_icon1: u32,
    pub notification_bottom_icon2: u32,
    pub notification_bottom_icon3: u32,
    pub notification_bottomline: u32,
    pub notification_icon: u32,
    pub notification_scene: u32,
    pub notification_title: u32,
    pub notification_topline: u32,
    pub transition_in: HudMenuTransitionData,
    pub transition_out: HudMenuTransitionData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftScoreboardPlayerTemplate {
    pub bounds: u32,
    pub click_region: u32,
    pub combat_state_background: u32,
    pub combat_state_dots_fx: u32,
    pub combat_state_loss_fx: u32,
    pub combat_state_win_fx: u32,
    pub custom_button_data: Option<Unk0xf84a5b90>,
    pub fill_meter: u32,
    pub frame: u32,
    pub future_sight_vfx: u32,
    pub health_text: u32,
    pub icon: u32,
    pub matchmaking_vfx: u32,
    pub mute_button: u32,
    pub name_backdrop: u32,
    pub name_text: u32,
    pub player_scene: u32,
    pub shared_draft_group_background: u32,
    pub shared_draft_group_text: u32,
    pub unk_0xd27deaf6: Option<bool>,
    pub viewing_icon: u32,
    pub win_streak_intro: u32,
    pub win_streak_loop_level1: u32,
    pub win_streak_loop_level2: u32,
    pub win_streak_outro: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftScoreboardViewController {
    pub activate_combat_recap_button: u32,
    pub activate_scoreboard_button: u32,
    pub background: u32,
    pub base_loadable: u32,
    pub fill_meter_self_color: Option<[u8; 4]>,
    pub main_scene: u32,
    pub mobile_override_loadable: u32,
    pub notification_template: TftScoreboardNotificationTemplate,
    pub opponent_bounds: u32,
    pub path_hash_to_self: u64,
    pub player_opponent_template: TftScoreboardPlayerTemplate,
    pub player_self_template: TftScoreboardPlayerTemplate,
    pub toggle_button: u32,
    pub unk_0x2d85c640: Option<u32>,
    pub unk_0x520cfbb9: Option<Unk0xff55db60>,
    pub unk_0x7214dd12: u32,
    pub unk_0xa5bec403: Option<u32>,
    pub unk_0xbae662ae: Option<TftScoreboardPlayerTemplate>,
    pub unk_0xbfe15a6a: Option<Unk0xd2b529c>,
    pub unk_0xdd7ddab0: Option<bool>,
    pub unk_0xe26f4055: Option<Unk0x138c3d23>,
    pub unk_0xe71e9783: Option<u32>,
    pub viewing_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftSellArea {
    pub default_scene_handle: u32,
    pub hit_target_handle: u32,
    pub hover_scene_handle: u32,
    pub scene_handle: u32,
    pub sell_text_handle: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftShopData {
    pub ability_icon_path: Option<String>,
    pub base_cost: Option<i32>,
    pub m_ability_name_tra: Option<String>,
    pub m_description_tra: Option<String>,
    pub m_display_name_tra: Option<String>,
    pub m_name: String,
    pub m_rarity: Option<u8>,
    pub pc_splash_path: Option<String>,
    pub sell_overwrite_tra: Option<String>,
    pub square_splash_path: Option<String>,
    pub team_planner_portrait_path: String,
    pub team_planner_splash_path: Option<String>,
    pub unk_0x3451d6e4: Option<String>,
    pub unk_0x4d4e5cf5: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftSocialPanelViewController {
    pub add_friend_button: u32,
    pub base_friends_list_scissor_region: u32,
    pub base_loadable: u32,
    pub empty_state_group: u32,
    pub expanded_friends_list_scissor_region: u32,
    pub friends_list_item: SocialPanelFriendsListItemData,
    pub friends_list_scene_view_pane: u32,
    pub invite_button: u32,
    pub path_hash_to_self: u64,
    pub slot_one_region: u32,
    pub slot_zero_region: u32,
    pub tablet_override_loadable: u32,
    pub unk_0x2eed7e1b: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftSoulFighterTournamentBracketsViewController {
    pub base_loadable: u32,
    pub duration: f32,
    pub main_scene: u32,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub stage_configs: Vec<Unk0x1181085f>,
    pub title: u32,
    pub unk_0x17baadb7: u32,
    pub unk_0xa9affd87: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftStageRoundDataTemplate {
    pub active_icon: u32,
    pub group: u32,
    pub hit_region: u32,
    pub round_state_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftStageSceneData {
    pub background_top_center: u32,
    pub background_top_right: u32,
    pub overtime_background_top_right: u32,
    pub scene: u32,
    pub stage_left_group: u32,
    pub stage_number: u32,
    pub stage_right_group: u32,
    pub timer: u32,
    pub timer_bar_background: u32,
    pub timer_fill_texture: u32,
    pub timing_out_background_top_right: u32,
    pub unk_0xcadd164e: u32,
    pub unk_0xcaf9d408: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftStageViewController {
    pub base_loadable: u32,
    pub collapse_button: u32,
    pub collapsed_stage_scene: TftStageSceneData,
    pub expand_button: u32,
    pub expanded_stage_scene: TftStageSceneData,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub phase_icons: Vec<u32>,
    pub round_template: TftStageRoundDataTemplate,
    pub stage_icon: u32,
    pub unk_0xa0b12922: u32,
    pub unk_0xc67a7d17: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftSurrenderViewController {
    pub base_loadable: u32,
    pub default_title_tra: String,
    pub done_voting_title_tra: String,
    pub main_scene: u32,
    pub mobile_override_loadable: u32,
    pub no_button: u32,
    pub path_hash_to_self: u64,
    pub status_text: u32,
    pub timer_bar: u32,
    pub title_text: u32,
    pub vote_icon: u32,
    pub yes_button: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftTraitContributionData {
    pub amount: Option<i32>,
    pub show_trait_nub: Option<bool>,
    pub trait_data: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftTraitData {
    pub innate_trait_sets: Option<Vec<TftBaseTraitSetData>>,
    pub m_conditional_trait_sets: Vec<TftConditionalTraitSetData>,
    pub m_description_name_tra: String,
    pub m_display_name_tra: String,
    pub m_icon_path: String,
    pub m_name: String,
    pub trait_progression_display: Option<u8>,
    pub trait_type: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftTraitInfoCard {
    pub card_scene: u32,
    pub card_scene_anchor: u32,
    pub horizontal_rule: Option<u32>,
    pub hud_trait_unit_slot_borders: TftHudTraitUnitSlotBorders,
    pub hud_trait_unit_slot_template: TftHudTraitUnitSlotData,
    pub portrait_region: u32,
    pub resizable_backdrop: u32,
    pub trait_description: u32,
    pub trait_footer: Option<u32>,
    pub trait_footer_padding: Option<u32>,
    pub trait_icon: u32,
    pub trait_name: u32,
    pub unit_grid_link: u32,
    pub unk_0x3eedfe4f: Option<bool>,
    pub unk_0x43703702: Option<u32>,
    pub unk_0x4374a356: Option<u32>,
    pub unk_0x71aea619: Option<Unk0xc88d8b75>,
    pub unk_0x7b11850b: Option<Unk0xa3129b41>,
    pub unk_0x7b313643: Option<bool>,
    pub unk_0x7f4a4a4: Option<bool>,
    pub unk_0xc3002c22: Option<u32>,
    pub unk_0xfffd8771: Unk0x1f5bef59,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftTraitInfoCardViewController {
    pub base_loadable: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub unk_0x6de5e0b8: HashMap<u8, TftTraitInfoCard>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftTraitTrackerTraitDataTemplate {
    pub augmented_vfx: u32,
    pub count: u32,
    pub group: u32,
    pub icon: u32,
    pub icon_backgrounds: Vec<TftTraitTrackerTraitIconData>,
    pub mobile_tap_region: u32,
    pub name: u32,
    pub name_background: u32,
    pub total_count: u32,
    pub total_count_background: u32,
    pub unk_0x406a0588: u32,
    pub unk_0x7f167d1d: Vec<Unk0x65dbf6d0>,
    pub unk_0x81e12d36: HashMap<u32, u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftTraitTrackerTraitIconData {
    pub default_icon: u32,
    pub hover_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftTraitTrackerViewController {
    pub base_loadable: u32,
    pub mobile_override_loadable: u32,
    pub page_button: u32,
    pub path_hash_to_self: u64,
    pub resizable_backdrop: u32,
    pub resizable_enemy_backdrop: u32,
    pub scene: u32,
    pub trait_height_ref: u32,
    pub trait_info_card_hover_region: u32,
    pub trait_info_card_region: u32,
    pub trait_page_region: u32,
    pub trait_template: TftTraitTrackerTraitDataTemplate,
    pub unk_0xa826ecce: u32,
    pub unk_0xaeece919: u32,
    pub unk_0xefcbc89c: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftTrovesBannerData {
    pub activation_date_time: String,
    pub background_texture_path: String,
    pub banner_currency: u32,
    pub banner_currency_id: String,
    pub celebration_theme: u32,
    pub deactivation_date_time: String,
    pub id: String,
    pub name_tra_key: String,
    pub pity_counter_id: String,
    pub pity_threshold: Option<u32>,
    pub thumbnail_texture_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftTrovesCelebrationChestSegmentData {
    pub chest_asset_multi_vfx: u32,
    pub chest_asset_single_vfx: u32,
    pub chest_background_asset_vfx: u32,
    pub multi_pull_primary_tier_hint_stagger: f32,
    pub multi_pull_primary_tier_hint_vfx: HashMap<u32, u32>,
    pub multi_pull_secondary_hint_count: u8,
    pub multi_pull_secondary_tier_hint_stagger: f32,
    pub multi_pull_secondary_tier_hint_vfx: HashMap<u32, u32>,
    pub multi_pull_tier_hint_offset: f32,
    pub multi_sequence: u32,
    pub single_pull_tier_hint_offset: f32,
    pub single_pull_tier_hint_vfx: HashMap<u32, u32>,
    pub single_sequence: u32,
    pub total_duration: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftTrovesCelebrationCurrencySegmentData {
    pub gem_multi_asset_vfx: u32,
    pub gem_mythic_asset_vfx: u32,
    pub gem_single_asset_vfx: u32,
    pub portal_transition_offset: f32,
    pub sequence: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftTrovesCelebrationHighlightSegmentData {
    pub chase_content_vfx: u32,
    pub highlight_end_timing_offset: f32,
    pub highlight_end_vfx: u32,
    pub highlights_sequence: u32,
    pub no_highlights_sequence: u32,
    pub unk_0xb6160f23: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftTrovesCelebrationStandardSegmentData {
    pub background_timing_offset: f32,
    pub entry_echo_sfx: String,
    pub entry_reveal_sfx: String,
    pub first_item_timing_offset: f32,
    pub inter_item_timing_offset: f32,
    pub sequence: u32,
    pub standard_content_background: u32,
    pub unk_0xfdfabc7d: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftTrovesCelebrationThemeData {
    pub currency_segment_data: TftTrovesCelebrationCurrencySegmentData,
    pub highlight_segment_data: TftTrovesCelebrationHighlightSegmentData,
    pub portal_segment_data: TftTrovesCelebrationChestSegmentData,
    pub scene: u32,
    pub standard_segment_data: TftTrovesCelebrationStandardSegmentData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftTrovesCelebrationViewControllerV2 {
    pub base_loadable: u32,
    pub common_scene: u32,
    pub continue_button: u32,
    pub default_standard_item_rarity_texture_path: String,
    pub default_standard_item_star_level_texture_path: String,
    pub default_standard_item_thumbnail_texture_path: String,
    pub default_theme: u32,
    pub parent_scene: u32,
    pub path_hash_to_self: u64,
    pub single_standard_currency_vfx: u32,
    pub single_standard_item_legendary_vfx: u32,
    pub single_standard_item_text: u32,
    pub single_standard_item_vfx: u32,
    pub single_standard_quantity_text: u32,
    pub standard_content_layout: u32,
    pub standard_item_rarity_texture_paths: HashMap<u32, String>,
    pub standard_item_star_level_texture_paths: Vec<String>,
    pub standard_item_template: Unk0x8190bc9f,
    pub tablet_override_loadable: u32,
    pub unk_0x104fe04e: u32,
    pub unk_0x108492b7: String,
    pub unk_0x27edf69a: Vec<u32>,
    pub unk_0x2e05d5a9: u32,
    pub unk_0x3fdcfd10: HashMap<String, Unk0xbaf9ac75>,
    pub unk_0x52fa1831: u32,
    pub unk_0x58134905: u32,
    pub unk_0x5db7f820: HashMap<String, Unk0x9ab8b8e6>,
    pub unk_0x629d7fe5: u32,
    pub unk_0x65774e46: String,
    pub unk_0x783a2d93: u32,
    pub unk_0x7b14669a: u32,
    pub unk_0x880040f3: u32,
    pub unk_0x9395be56: String,
    pub unk_0xa7b45c64: u32,
    pub unk_0xa940b93d: u32,
    pub unk_0xaad46f66: u32,
    pub unk_0xc1d80535: u32,
    pub unk_0xc91cc065: u32,
    pub unk_0xd7ce36cb: u32,
    pub unk_0xdf4d96db: u32,
    pub unk_0xeed1cb06: u32,
    pub unk_0xf9ae2168: u32,
    pub unk_0xf9f0eed6: u32,
    pub vignette_title: u32,
    pub vignette_title_tra_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftUnitAttachmentSlotsData {
    pub standard_slots: Vec<TftUnitAttachmentStandardSlotData>,
    pub unk_0xec2cd82f: Option<Unk0xdd57ce5e>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftUnitAttachmentStandardSlotData {
    pub frame: u32,
    pub icon: u32,
    pub overlay: Option<u32>,
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftUnitFloatingInfoBarData {
    pub anchor: u32,
    pub attachment_slots: TftUnitAttachmentSlotsData,
    pub border: Option<u32>,
    pub health_bar: Option<HealthBarData>,
    pub inspect_button: Option<u32>,
    pub par_bar: Option<AbilityResourceBarData>,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftUnitInfoCharacterRoleData {
    pub button: u32,
    pub icon: u32,
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftUnitInfoCustomButtonData {
    pub activate_unit_property: u32,
    pub button: u32,
    pub enabled_unit_property: u32,
    pub item_tooltip_data: TftUnitInfoCustomButtonItemTooltipData,
    pub plain_text_tooltip_data: TftUnitInfoCustomButtonPlainTextTooltipData,
    pub selected_unit_property: u32,
    pub tooltip_region: Option<u32>,
    pub tooltip_upper_right_anchor: u32,
    pub unk_0xfb7904ba: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftUnitInfoCustomButtonItemTooltipData {
    pub is_item_tooltip_unit_property: u32,
    pub item_name_unit_property: u32,
    pub unk_0x63ce1d5: u32,
    pub unk_0x7fe49de2: Option<u32>,
    pub unk_0xa6daa6f0: u32,
    pub unk_0xb583f85b: u32,
    pub unk_0xd5169d40: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftUnitInfoCustomButtonPlainTextTooltipData {
    pub main_text_tra: String,
    pub title_tra: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftUnitInfoItemDisplayData {
    pub button: u32,
    pub icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TftUnitInfoTraitDisplayData {
    pub highlight: u32,
    pub icon: u32,
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftUnitInfoViewController {
    pub ability_button: u32,
    pub ability_icon: u32,
    pub ability_tooltip_upper_right_anchor: u32,
    pub base_loadable: u32,
    pub champ_inspect_vfx: u32,
    pub character_role_data: TftUnitInfoCharacterRoleData,
    pub custom_button_data: TftUnitInfoCustomButtonData,
    pub frame: u32,
    pub gold_icon: u32,
    pub gold_text: u32,
    pub health_bar_data: HealthBarData,
    pub item_codex_upper_right_anchor: u32,
    pub item_tooltip_upper_right_anchor: u32,
    pub items: Vec<TftUnitInfoItemDisplayData>,
    pub mobile_override_loadable: u32,
    pub par_bar_data: AbilityResourceBarData,
    pub par_regen_text: u32,
    pub path_hash_to_self: u64,
    pub positioning_button: u32,
    pub positioning_data: HashMap<u8, TftCharacterPositioningData>,
    pub positioning_hex_vfx: u32,
    pub positioning_icon: u32,
    pub positioning_text: u32,
    pub range_button: u32,
    pub range_hex_vfx: u32,
    pub range_text: u32,
    pub range_text_tra: String,
    pub rarity_underlays: Vec<u32>,
    pub scene: u32,
    pub sell_unit_button: u32,
    pub sell_unit_button_text: u32,
    pub sell_unit_button_text_tra: String,
    pub star_level_overlays: Vec<u32>,
    pub stat_tooltip_upper_right_anchor: u32,
    pub stats_ui_data: UnitStatsUiData,
    pub trait_info_card_hover_region: u32,
    pub trait_info_card_region: u32,
    pub traits: Vec<TftUnitInfoTraitDisplayData>,
    pub unit_name: u32,
    pub unit_portrait: u32,
    pub unk_0x517ee83f: Vec<Unk0x629f5938>,
    pub unk_0x913437bb: String,
    pub unk_0xa5cdc4c3: Option<String>,
    pub unk_0xb11cb7d: Option<Unk0x2b365a82>,
    pub unk_0xeeea5fa1: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftUnitShopViewController {
    pub base_loadable: u32,
    pub buy_exp_button: u32,
    pub buy_exp_button_scene: u32,
    pub buy_exp_button_text_handle: u32,
    pub buy_exp_click_default_vfx: u32,
    pub buy_exp_click_element: u32,
    pub buy_exp_cost_text_handle: u32,
    pub buy_exp_icon_data: IconData,
    pub close_shop_reminder_vfx_handle: u32,
    pub drag_area_handle: u32,
    pub drop_rate: TftHudUnitShopDropRate,
    pub exp_bar: TftHudExpBar,
    pub gold_display_scene_handle: u32,
    pub gold_frame: u32,
    pub gold_gain_vfx_handle: u32,
    pub gold_text_handle: u32,
    pub gold_tooltip_tra_keys: HashMap<u32, GoldTooltipTraKeys>,
    pub info_nub_button: u32,
    pub info_nub_tooltip_region: u32,
    pub item_slot_handles: Vec<u32>,
    pub item_template: UnitShopItemData,
    pub level_text_handle: u32,
    pub lock_button: u32,
    pub main_shop_scene_handle: u32,
    pub mobile_override_loadable: u32,
    pub modal_shroud_handle: u32,
    pub path_hash_to_self: u64,
    pub reroll_button: u32,
    pub reroll_button_text_handle: u32,
    pub reroll_click_default_vfx: u32,
    pub reroll_click_element: u32,
    pub reroll_cost_text_handle: u32,
    pub reroll_trait_counter_text: u32,
    pub sell_areas: Vec<TftSellArea>,
    pub shop_click_rejected_vfx: u32,
    pub shop_drag_area: u32,
    pub streak_ui: TftHudStreakUi,
    pub toggle_shop_button: u32,
    pub toggle_shop_button_scene: u32,
    pub toggle_shop_reminder_vfx_handle: u32,
    pub top_level_scene_handle: u32,
    pub unk_0x44358b5: TftHudUnitShopDropRate,
    pub unk_0x52f434cd: HashMap<u32, Unk0xb609aae3>,
    pub unk_0x579e71b1: Unk0xf96d9400,
    pub unk_0x59b4e436: u32,
    pub unk_0x5c25a39e: u32,
    pub unk_0x5e800761: u32,
    pub unk_0x8bf5e123: u32,
    pub unk_0xa9e4a8cb: HashMap<u32, u32>,
    pub unk_0xb78bcf1a: Unk0x9925c3c2,
    pub unk_0xe3f7224b: u32,
    pub zoom_transition_in: HudMenuTransitionData,
    pub zoom_transition_out: HudMenuTransitionData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TftUxTunables {
    pub tft_future_sight_icon_color: [u8; 4],
    pub tft_matchmaking_icon_color: [u8; 4],
    pub tft_partner_group_colors: Vec<[u8; 4]>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeBlendData {
    pub m_time: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeMaterialDriver {
    pub loop_duration: Option<f32>,
    pub loop_time_as_fraction: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimedVariableWaveBehavior {
    pub behaviors: Vec<Box<TimedWaveBehaviorInfo>>,
    pub default_spawn_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimedWaveBehaviorInfo {
    pub behavior: Box<EnumWaveBehavior>,
    pub start_time_secs: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TipTrackerViewController {
    pub base_loadable: u32,
    pub item_scene_template: u32,
    pub message_display_data: HudMessageDisplayData,
    pub message_template: HudTipTrackerMessageTemplate,
    pub path_hash_to_self: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ToastNotificationsViewController {
    pub base_loadable: u32,
    pub body_text: u32,
    pub click_region: u32,
    pub event_icon: u32,
    pub friend_invite_icon: u32,
    pub pass_reward_icon: u32,
    pub path_hash_to_self: u64,
    pub queue_type_to_game_invite_icon: HashMap<String, u32>,
    pub scene: u32,
    pub title_text: u32,
    pub toast_duration_secs: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ToolAiPresence {
    pub easy: Option<bool>,
    pub hard: Option<bool>,
    pub intro: Option<bool>,
    pub medium: Option<bool>,
    pub unk_0x42ac598e: Option<bool>,
    pub unk_0x6175bb7b: Option<bool>,
    pub unk_0xb66d0e47: Option<bool>,
    pub unk_0xb75b2ab8: Option<bool>,
    pub unk_0xca762bfc: Option<bool>,
    pub unk_0xeba3cb5: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ToolAlternateForm {
    pub champion: Option<String>,
    pub name: String,
    pub spells: Option<Vec<String>>,
    pub the_switch: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ToolEducationData {
    pub first_item: i32,
    pub skill_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ToolInheritsData {
    pub recommended: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ToolPassiveData {
    pub effect: Option<Vec<String>>,
    pub level: Option<Vec<i32>>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ToolSoundData {
    pub attack: Option<Vec<String>>,
    pub click: Option<Vec<String>>,
    pub death: Option<String>,
    pub r#move: Option<Vec<String>>,
    pub ready: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ToolSpellDesc {
    pub desc: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TooltipFormat {
    pub m_input_loc_keys_with_defaults: HashMap<String, String>,
    pub m_list_grid_postfix: Option<String>,
    pub m_list_grid_prefix: Option<String>,
    pub m_list_grid_separator: Option<String>,
    pub m_list_names: Option<Vec<String>>,
    pub m_list_styles: Option<HashMap<u32, String>>,
    pub m_list_type_choices: Option<HashMap<String, String>>,
    pub m_list_value_separator: Option<String>,
    pub m_object_name: String,
    pub m_output_strings: HashMap<String, String>,
    pub m_uses_list_values: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TooltipInstanceBuff {
    pub m_format: u32,
    pub m_loc_keys: Option<HashMap<String, String>>,
    pub m_object_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TooltipInstanceList {
    pub elements: Option<Vec<TooltipInstanceListElement>>,
    pub level_count: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TooltipInstanceListElement {
    pub multiplier: Option<f32>,
    pub name_override: Option<String>,
    pub r#type: String,
    pub style: Option<u32>,
    pub type_index: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TooltipInstanceSpell {
    pub enable_extended_tooltip: Option<bool>,
    pub m_format: u32,
    pub m_lists: Option<HashMap<String, TooltipInstanceList>>,
    pub m_loc_keys: Option<HashMap<String, String>>,
    pub m_object_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TooltipViewController {
    pub base_loadable: u32,
    pub path_hash_to_self: u64,
    pub per_locale_adjustments: HashMap<String, PerLocaleTooltipAdjustments>,
    pub tablet_override_loadable: Option<u32>,
    pub tooltip_popup_delay_time: f32,
    pub tooltip_popup_timeout: f32,
    pub unk_0x56716e4a: String,
    pub unk_0x7c7147eb: Option<u32>,
    pub unk_0xf0ae6ff1: Unk0xf0ae6ff1,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct TouchOverlayViewController {
    pub base_loadable: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub touch_feedback_element: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TrackData {
    pub m_blend_mode: Option<u8>,
    pub m_blend_weight: Option<f32>,
    pub m_priority: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TrackMouseMovement {
    pub m_acceleration: f32,
    pub m_anti_lag_delay: f32,
    pub m_initial_speed: f32,
    pub m_max_speed: f32,
    pub m_min_speed: f32,
    pub m_start_bone_name: String,
    pub m_target_bone_name: String,
    pub m_tracks_target: bool,
    pub m_turn_radius_by_level: Vec<f32>,
    pub m_use_ground_height_at_target: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransitionClipBlendData {
    pub m_clip_name: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TreatAutoAttacksAsNormalSpells {
    pub auto_attack_spells_use_spell_source: bool,
    pub override_queryable_attack_range: GameCalculation,
    pub skip_sequenced_attack_events: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TriggerFromScript {
    pub m_actions: Vec<EnumAttackEvents>,
    pub m_delay: Option<f32>,
    pub m_trigger_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TriggerOnDelay {
    pub m_actions: Vec<EnumAttackEvents>,
    pub m_delay: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TriggerOnHit {
    pub m_actions: Vec<EnumAttackEvents>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TriggerOnMovementComplete {
    pub m_actions: Vec<EnumAttackEvents>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TroveElementData {
    pub id: String,
    pub instance_data: Vec<TroveInstanceData>,
    pub preferred_order: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TroveInstanceData {
    pub banner_data: u32,
    pub enabled: Option<bool>,
    pub end_date: String,
    pub foreground_vfx: Option<TextureOverride>,
    pub id: String,
    pub region_data: Option<EnabledRegionData>,
    pub title_text_tra: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiChampionSelectionSlotData {
    pub button: u32,
    pub ex_button: u32,
    pub portrait: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiChatPaneDefinition {
    pub max_messages: u32,
    pub message_list: u32,
    pub view_pane: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiClashTeam {
    pub logo_icon: u32,
    pub tag_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiComboBoxDefinition {
    pub button_definition: u32,
    pub dropdown_backdrop_element_data: u32,
    pub dropdown_display_tra_key: Option<String>,
    pub dropdown_hover_element_data: u32,
    pub list_display_direction: Option<u8>,
    pub list_option_hit_area_element_data: u32,
    pub list_option_text_element_data: u32,
    pub object_path: u32,
    pub selected_highlight_element_data: u32,
    pub sound_events: Option<UiComboBoxSoundEvents>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiComboBoxSoundEvents {
    pub on_selection_event: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiDraggableBasic {
    pub use_sticky_drag: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiDraggableElementGroupDrag {
    pub use_sticky_drag: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiDraggableProxyElementDrag {
    pub proxy_element: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiDraggableSceneDrag {
    pub use_sticky_drag: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementEffectAmmoData {
    pub layer: u32,
    pub m_effect_color0: [u8; 4],
    pub m_effect_color1: [u8; 4],
    pub name: String,
    pub position: UiPositionRect,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementEffectAnimatedRotatingIconData {
    pub frames_per_second: f32,
    pub name: String,
    pub number_of_frames_per_row_in_atlas: f32,
    pub position: UiPositionRect,
    pub scene: u32,
    pub texture_data: AtlasData,
    pub total_number_of_frames: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementEffectAnimationData {
    pub enabled: Option<bool>,
    pub frames_per_second: Option<f32>,
    pub layer: Option<u32>,
    pub m_finish_behavior: Option<u8>,
    pub m_per_pixel_uvs_x: Option<bool>,
    pub name: String,
    pub number_of_frames_per_row_in_atlas: Option<f32>,
    pub position: UiPositionRect,
    pub scene: u32,
    pub texture_data: EnumData,
    pub total_number_of_frames: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementEffectArcFillData {
    pub enabled: Option<bool>,
    pub layer: u32,
    pub m_flip_x: Option<bool>,
    pub name: String,
    pub position: UiPositionRect,
    pub scene: u32,
    pub texture_data: EnumData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementEffectCircleMaskCooldownData {
    pub layer: u32,
    pub m_effect_color0: [u8; 4],
    pub m_effect_color1: [u8; 4],
    pub name: String,
    pub position: UiPositionRect,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementEffectCircleMaskDesaturateData {
    pub enabled: Option<bool>,
    pub layer: u32,
    pub m_flip_x: Option<bool>,
    pub minimum_saturation: Option<f32>,
    pub name: String,
    pub position: UiPositionRect,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementEffectCooldownData {
    pub layer: Option<u32>,
    pub m_effect_color0: [u8; 4],
    pub m_effect_color1: [u8; 4],
    pub name: String,
    pub position: UiPositionRect,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementEffectCooldownRadialData {
    pub enabled: Option<bool>,
    pub layer: Option<u32>,
    pub m_flip_x: Option<bool>,
    pub m_flip_y: Option<bool>,
    pub m_is_fill: Option<bool>,
    pub name: String,
    pub position: UiPositionRect,
    pub scene: u32,
    pub texture_data: EnumData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementEffectDesaturateData {
    pub enabled: Option<bool>,
    pub layer: Option<u32>,
    pub minimum_saturation: Option<f32>,
    pub name: String,
    pub position: UiPositionRect,
    pub scene: u32,
    pub texture_data: Option<EnumData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementEffectFillPercentageData {
    pub enabled: Option<bool>,
    pub layer: u32,
    pub m_per_pixel_uvs_x: bool,
    pub name: String,
    pub position: UiPositionRect,
    pub scene: u32,
    pub texture_data: AtlasData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementEffectGlowData {
    pub base_scale: f32,
    pub cycle_time: f32,
    pub layer: u32,
    pub minimum_alpha: Option<f32>,
    pub name: String,
    pub position: UiPositionRect,
    pub scene: u32,
    pub texture_data: EnumData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementEffectGlowingRotatingIconData {
    pub brightness_mod: f32,
    pub cycle_time: f32,
    pub name: String,
    pub position: UiPositionRect,
    pub scene: u32,
    pub texture_data: AtlasData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementEffectInstancedData {
    pub enabled: Option<bool>,
    pub layer: u32,
    pub m_color: Option<[u8; 4]>,
    pub m_per_pixel_uvs_x: Option<bool>,
    pub name: String,
    pub position: UiPositionRect,
    pub scene: u32,
    pub texture_data: Option<AtlasData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementEffectLineData {
    pub enabled: bool,
    pub layer: u32,
    pub m_thickness: f32,
    pub name: String,
    pub position: UiPositionRect,
    pub scene: u32,
    pub texture_data: AtlasData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementEffectRotatingIconData {
    pub enabled: Option<bool>,
    pub layer: u32,
    pub name: String,
    pub position: UiPositionRect,
    pub scene: u32,
    pub texture_data: EnumData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiElementGroupButtonAdditionalElements {
    pub clicked_state_elements: Vec<u32>,
    pub default_state_elements: Vec<u32>,
    pub hover_state_elements: Vec<u32>,
    pub inactive_state_elements: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementGroupButtonData {
    pub active_tooltip_tra_key: Option<String>,
    pub add_text_size_to_hit_region: Option<bool>,
    pub click_release_particle_element: Option<u32>,
    pub clicked_state_elements: Option<UiElementGroupButtonState>,
    pub default_state_elements: Option<UiElementGroupButtonState>,
    pub elements: Vec<u32>,
    pub hit_region_element: u32,
    pub hover_state_elements: Option<UiElementGroupButtonState>,
    pub inactive_selected_state_elements: Option<UiElementGroupButtonState>,
    pub inactive_state_elements: Option<UiElementGroupButtonState>,
    pub inactive_tooltip_tra_key: Option<String>,
    pub is_active: Option<bool>,
    pub is_enabled: Option<bool>,
    pub is_focusable: Option<bool>,
    pub is_selected: Option<bool>,
    pub name: String,
    pub scene: u32,
    pub selected_clicked_state_elements: Option<UiElementGroupButtonState>,
    pub selected_hover_state_elements: Option<UiElementGroupButtonState>,
    pub selected_state_elements: Option<UiElementGroupButtonState>,
    pub selected_tooltip_tra_key: Option<String>,
    pub sound_events: Option<UiElementGroupButtonSoundEvents>,
    pub tab_order: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiElementGroupButtonSoundEvents {
    pub mouse_down_event: Option<String>,
    pub mouse_down_on_inactive: Option<String>,
    pub mouse_down_selected: Option<String>,
    pub mouse_up_event: Option<String>,
    pub mouse_up_selected: Option<String>,
    pub roll_over_event: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiElementGroupButtonState {
    pub display_element_list: Option<Vec<u32>>,
    pub text_element: Option<u32>,
    pub text_frame_element: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementGroupData {
    pub elements: Option<Vec<u32>>,
    pub name: String,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementGroupFramedData {
    pub elements: Vec<u32>,
    pub frame_element: u32,
    pub name: String,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementGroupManagedLayoutData {
    pub elements: Vec<u32>,
    pub flip_for_rtl: Option<bool>,
    pub ignore_disabled_elements: Option<bool>,
    pub layout_style: EnumLayoutStyle,
    pub name: String,
    pub region: u32,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementGroupMeterData {
    pub bar_elements: Vec<u32>,
    pub elements: Vec<u32>,
    pub fill_direction: Option<u8>,
    pub is_enabled: Option<bool>,
    pub name: String,
    pub scene: u32,
    pub start_percentage: Option<f32>,
    pub tip_style: Option<EnumTipStyle>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementGroupSliderData {
    pub bar_hit_region: u32,
    pub bar_hovered_state: UiElementGroupSliderState,
    pub default_state: UiElementGroupSliderState,
    pub direction: Option<u8>,
    pub elements: Vec<u32>,
    pub is_enabled: Option<bool>,
    pub name: String,
    pub scene: u32,
    pub slider_clicked_state: UiElementGroupSliderState,
    pub slider_hit_region: u32,
    pub slider_hovered_state: UiElementGroupSliderState,
    pub sound_events: Option<UiElementGroupSliderSoundEvents>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiElementGroupSliderSoundEvents {
    pub on_bar_clicked_event: String,
    pub on_drag_end_event: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiElementGroupSliderState {
    pub bar_backdrop: Option<u32>,
    pub bar_fill: Option<u32>,
    pub slider_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementIconData {
    pub block_input_events: Option<bool>,
    pub color: Option<[u8; 4]>,
    pub drag_type: Option<EnumUiDraggable>,
    pub enabled: Option<bool>,
    pub extension: Option<EnumIconElement>,
    pub fill_type: Option<u32>,
    pub flip_x: Option<bool>,
    pub flip_y: Option<bool>,
    pub layer: Option<u32>,
    pub material: Option<u32>,
    pub name: String,
    pub per_pixel_uvs_x: Option<bool>,
    pub position: EnumUiPosition,
    pub scene: u32,
    pub texture_data: Option<EnumData>,
    pub use_alpha: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementMeterSkin {
    pub bar_elements: Vec<u32>,
    pub reverse_directional_tip_elements: Option<Vec<u32>>,
    pub sliver: Option<u32>,
    pub tip_elements: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementParticleSystemData {
    pub enabled: Option<bool>,
    pub layer: Option<u32>,
    pub max_play_count: Option<u32>,
    pub name: String,
    pub play_during_transition: Option<bool>,
    pub position: EnumUiPosition,
    pub scene: u32,
    pub texture_overrides: Option<HashMap<u32, String>>,
    pub vfx_adjustment_scale: Option<f32>,
    pub vfx_system: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiElementRect {
    pub position: Option<Vec2>,
    pub size: Option<Vec2>,
    pub source_resolution_height: Option<u16>,
    pub source_resolution_width: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementRegionData {
    pub block_input_events: Option<bool>,
    pub drag_type: Option<EnumUiDraggable>,
    pub enabled: Option<bool>,
    pub layer: Option<u32>,
    pub name: String,
    pub position: Option<EnumUiPosition>,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementScissorRegionData {
    pub enabled: Option<bool>,
    pub layer: Option<u32>,
    pub name: String,
    pub position: UiPositionRect,
    pub scene: u32,
    pub scene_to_scissor: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiElementTextData {
    pub block_input_events: Option<bool>,
    pub color: Option<[u8; 4]>,
    pub enabled: Option<bool>,
    pub flip_for_rtl: Option<bool>,
    pub font_description: u32,
    pub html_style_sheet: Option<u32>,
    pub icon_scale: Option<f32>,
    pub layer: Option<u32>,
    pub name: String,
    pub position: Option<UiPositionRect>,
    pub scene: u32,
    pub text_alignment_horizontal: Option<u8>,
    pub text_alignment_vertical: Option<u8>,
    pub tra_key: Option<String>,
    pub unk_0xc1d0e91a: Option<f32>,
    pub wrapping_mode: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiHyperlink {
    pub display_text: u32,
    pub url_tra_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiItemSelectionSlotData {
    pub button: u32,
    pub item_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiLevelUp {
    pub buttons_scene: u32,
    pub fx_in_scene: u32,
    pub spells: Option<Vec<EnumAddLevelTimer>>,
    pub title: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricBattery {
    pub battery_level_icons: Vec<u32>,
    pub device_ux: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricClash {
    pub clash_frame: u32,
    pub clash_frame_mirror: u32,
    pub clash_round_icon: u32,
    pub clash_round_text: u32,
    pub device_ux: i32,
    pub team1: UiClashTeam,
    pub team2: UiClashTeam,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricCreepScore {
    pub device_ux: i32,
    pub icon: u32,
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricFps {
    pub device_ux: i32,
    pub fps_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricGameTime {
    pub device_ux: i32,
    pub time_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricKda {
    pub device_ux: i32,
    pub icon: u32,
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricLaneMinionFlatDamageReductionFromMinion {
    pub icon: u32,
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricLaneMinionPercentIncreasedDamageToMinion {
    pub icon: u32,
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricLatencyText {
    pub device_ux: i32,
    pub latency_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricMultiDragonKillsSrX {
    pub device_ux: i32,
    pub dragon_type_icons: Option<HashMap<u32, MultiDragonTypeSourceUiData>>,
    pub soul_slot: Option<MultiDragonSoulSlotUiData>,
    pub team1_icon_slots: Vec<u32>,
    pub team2_icon_slots: Vec<u32>,
    pub team_slot_disabled_icon: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricTeamGold {
    pub device_ux: i32,
    pub team1_gold_icon: u32,
    pub team1_gold_text: u32,
    pub team2_gold_icon: u32,
    pub team2_gold_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricTeamKills {
    pub device_ux: i32,
    pub team1_kill_text: u32,
    pub team2_kill_text: u32,
    pub team_kills_icon: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricTeamScoreMeters {
    pub device_ux: Option<i32>,
    pub frame: u32,
    pub team1_meter: u32,
    pub team1_meter_blue_skin: u32,
    pub team1_meter_red_skin: u32,
    pub team2_meter: u32,
    pub team2_meter_blue_skin: u32,
    pub team2_meter_red_skin: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricTeamTowerKills {
    pub device_ux: i32,
    pub team1_tower_kills_icon: u32,
    pub team1_tower_kills_text: u32,
    pub team2_tower_kills_icon: u32,
    pub team2_tower_kills_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricUnitBounty {
    pub icon: u32,
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricUnitCreepScore {
    pub icon: Option<u32>,
    pub show_disguise: Option<bool>,
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricUnitKda {
    pub icon: Option<u32>,
    pub show_disguise: Option<bool>,
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricUnitVisionScore {
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMetricVietnameseRatingLabel {
    pub device_ux: i32,
    pub label: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMilestoneFrameData {
    pub available_default_icon: u32,
    pub available_hover_icon: u32,
    pub claimable_default_icon: u32,
    pub claimable_hover_icon: u32,
    pub complete_default_icon: u32,
    pub complete_hover_icon: u32,
    pub locked_default_icon: Option<u32>,
    pub locked_hover_icon: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMilestoneMissionTemplate {
    pub claimable_icon: u32,
    pub claimable_vfx: u32,
    pub claimed_icon: u32,
    pub end_region: u32,
    pub free_bg: u32,
    pub free_frame_data: UiMilestoneFrameData,
    pub free_text: u32,
    pub group: u32,
    pub hit_region: u32,
    pub keystone_frame_data: UiMilestoneFrameData,
    pub level_pip_active_default: u32,
    pub level_pip_active_hover: u32,
    pub level_pip_available_default: u32,
    pub level_pip_available_hover: u32,
    pub level_pip_bonus_default: u32,
    pub level_pip_bonus_hover: u32,
    pub level_pip_complete_default: u32,
    pub level_pip_complete_hover: u32,
    pub level_text_active: u32,
    pub level_text_available: u32,
    pub level_text_complete: u32,
    pub meter_available_icon: u32,
    pub meter_bonus_icon: u32,
    pub meter_complete_icon: u32,
    pub mission_icon: u32,
    pub mission_icon_background: u32,
    pub premium_frame_data: UiMilestoneFrameData,
    pub unk_0xac1b0adb: UiMilestoneFrameData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMilestoneProgressMeter {
    pub bar_vfx_group: u32,
    pub bonus_skin: UiElementMeterSkin,
    pub exp_text: Option<u32>,
    pub level_text: u32,
    pub level_up_vfx_group: Option<u32>,
    pub meter_easing_type: Option<u8>,
    pub meter_transition_time_secs: f32,
    pub normal_skin: UiElementMeterSkin,
    pub progress_bar_delay_time: f32,
    pub progress_meter: u32,
    pub progress_meter_segments: Option<Vec<u32>>,
    pub show_level_up: Option<bool>,
    pub xp_value_to_segment_index: Vec<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMilestoneTimelineEnd {
    pub background: u32,
    pub frame_available: u32,
    pub frame_complete: u32,
    pub group: u32,
    pub meter_available: u32,
    pub meter_complete: u32,
    pub milestone_region: u32,
    pub pip_available: u32,
    pub pip_complete: u32,
    pub text_available: u32,
    pub text_complete: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMissionObjectiveData {
    pub alert_icon: u32,
    pub alert_text: u32,
    pub body_text: u32,
    pub expiry_text: u32,
    pub group: u32,
    pub progress_complete: u32,
    pub progress_fill: u32,
    pub progress_meter_bg: u32,
    pub progress_text: u32,
    pub separator_icon: u32,
    pub separator_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiMultiKillIconData {
    pub icon: u32,
    pub region: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiPerkSummonerSpecialistSelector {
    pub layout: u32,
    pub scene: u32,
    pub selector_button_template: UiPerkSummonerSpecialistSelectorButtonData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiPerkSummonerSpecialistSelectorButtonData {
    pub button: u32,
    pub spell_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiPerkSummonerSpecialistToggles {
    pub scene: u32,
    pub selector_menu: UiPerkSummonerSpecialistSelector,
    pub toggle_buttons: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiPerksStatData {
    pub hit_region: u32,
    pub icon: u32,
    pub r#type: Option<u32>,
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiPerksStats {
    pub stats: Vec<UiPerksStatData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiPlayerPortraitData {
    pub frame: u32,
    pub frame_while_dead: u32,
    pub portrait_icon: u32,
    pub portrait_icon_shape: Option<u8>,
    pub respawn_timer_text: u32,
    pub single_click_to_ping: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiPositionPolygon {
    pub anchors: AnchorSingle,
    pub polygon_vertices: Vec<Vec2>,
    pub ui_rect: UiElementRect,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiPositionRect {
    pub anchors: Option<EnumAddLevelTimer>,
    pub disable_pixel_snapping_x: Option<bool>,
    pub disable_pixel_snapping_y: Option<bool>,
    pub disable_resolution_downscale: Option<bool>,
    pub ignore_global_scale: Option<bool>,
    pub ignore_safe_zone: Option<bool>,
    pub ui_rect: Option<UiElementRect>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiPropertyLoadable {
    pub filepath_hash: u64,
    pub unk_0xe50d4da6: Option<EnumUnk0x797fe1c2>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiPropertyOverrideLoadable {
    pub filepath_hash: u64,
    pub override_src_folder: u64,
    pub unk_0xe50d4da6: Option<Unk0xcdf661db>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiReplayTeamFramesData {
    pub layout: u32,
    pub member_template: UiTeamMemberData,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiRotationalStoreItemTileData {
    pub background: Option<u32>,
    pub button: u32,
    pub group: u32,
    pub icon: u32,
    pub price_text: u32,
    pub timer: TftEventTimer,
    pub unk_0x1a24d83: Option<u32>,
    pub unk_0x276c0797: [u8; 4],
    pub unk_0x3987396d: Option<u32>,
    pub unk_0x5b1c302e: Option<u32>,
    pub unk_0x5baac24d: Option<u32>,
    pub unk_0x77aee5a6: Option<u32>,
    pub unk_0x860c9b32: [u8; 4],
    pub unk_0xa6805f0: [u8; 4],
    pub unk_0xaef2e781: Option<u32>,
    pub unk_0xb9cd82c6: Option<u32>,
    pub unk_0xd02a6781: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiSceneData {
    pub enabled: Option<bool>,
    pub handle_input_during_pause: Option<bool>,
    pub inherit_scissoring: Option<bool>,
    pub layer: Option<u32>,
    pub name: String,
    pub parent_scene: Option<u32>,
    pub scene_transition_in: Option<EnumTransitionData>,
    pub scene_transition_out: Option<EnumTransitionData>,
    pub unk_0x49d8f2c4: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UiSceneViewPaneData {
    pub drag_region_element: Option<u32>,
    pub enabled: Option<bool>,
    pub handle_input_during_pause: Option<bool>,
    pub inherit_scissoring: Option<bool>,
    pub layer: u32,
    pub name: String,
    pub parent_scene: Option<u32>,
    pub scissor_region_element: u32,
    pub scroll_direction: Option<u8>,
    pub scroll_region_element: Option<u32>,
    pub scrolling_scene: u32,
    pub slider: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiSocialChatMessageTemplate {
    pub bottom_padding: u32,
    pub date_divider: u32,
    pub date_spacing_over: u32,
    pub date_spacing_under: u32,
    pub date_stamp: u32,
    pub group: u32,
    pub message_background: u32,
    pub message_row: u32,
    pub message_text: u32,
    pub timestamp: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiStatStonesStats {
    pub button_next: u32,
    pub button_prev: u32,
    pub series_title_handle: u32,
    pub stat_stone_count: u32,
    pub stat_stone_hover_targets: Vec<u32>,
    pub stat_stone_icon: u32,
    pub stat_stone_template_scene: u32,
    pub stat_stone_title: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiStoreItemTileData {
    pub balance_header: Option<u32>,
    pub balance_value: Option<u32>,
    pub button: u32,
    pub group: u32,
    pub icon: u32,
    pub price_text: u32,
    pub unk_0x1a24d83: Option<u32>,
    pub unk_0x3987396d: Option<u32>,
    pub unk_0x5baac24d: Option<u32>,
    pub unk_0x6fc8eb8a: Option<u32>,
    pub unk_0x77aee5a6: Option<u32>,
    pub unk_0xaef2e781: Option<u32>,
    pub unk_0xb9cd82c6: Option<u32>,
    pub unk_0xd02a6781: Option<u32>,
    pub vfx: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiStorePurchaseRowData {
    pub action_button: u32,
    pub action_text: u32,
    pub amount: u32,
    pub arena_icon_path: String,
    pub bundle_icon_path: String,
    pub date: u32,
    pub divider: u32,
    pub group: u32,
    pub icon: u32,
    pub item: u32,
    pub misc_icon_path: String,
    pub quantity: u32,
    pub tactician_icon_path: String,
    pub transaction_id: u32,
    pub treasure_trove_token_icon_path: String,
    pub unk_0xe95d41cf: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiSummonerSocialCardData {
    pub backdrop_link: u32,
    pub champion_name_link: u32,
    pub scene_link: u32,
    pub summoner_icon_link: u32,
    pub summoner_name_link: u32,
    pub tagline_link: u32,
    pub unk_0x3da285d8: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiTargetStatsMenu {
    pub parent_scene: u32,
    pub target_perks: UiPerksStats,
    pub toggle_button: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiTeamFightOffScreenDifferentiationData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiTeamFightTeamData {
    pub team_health_meter: Option<UiTeamFightTeamHealthMeterData>,
    pub team_member: UiTeamFightTeamMemberData,
    pub team_member_slots: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiTeamFightTeamHealthMeterData {
    pub backdrop: u32,
    pub onscreen_health_fill: u32,
    pub total_health_fill: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiTeamFightTeamMemberData {
    pub frame: UiTeamMemberData,
    pub multikill_icon: Option<UiMultiKillIconData>,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiTeamIdentityData {
    pub logo_link: u32,
    pub name_link: u32,
    pub tag_link: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiTeamMemberData {
    pub group: u32,
    pub health_meter: Option<HealthBarData>,
    pub hit_region: u32,
    pub keystone_icon: Option<u32>,
    pub level: Option<UnitLevelUiData>,
    pub objective_planning_icon: Option<Unk0x6c84152e>,
    pub par_meter: Option<AbilityResourceBarData>,
    pub portrait: UiPlayerPortraitData,
    pub status_effects: Option<UiUnitStatusData>,
    pub summoner_name: Option<SummonerNameUiData>,
    pub summoner_spell_slots: Option<ScoreLineSpellSlots>,
    pub ultimate_cooldown_gem: Option<CooldownGemUiData>,
    pub ultimate_spell_slot: Option<SpellSlotSimpleUiDefinition>,
    pub unk_0x1d924766: Option<Unk0x37a8385c>,
    pub unk_0x213b14bb: Option<u32>,
    pub unk_0x4cee435: Option<u32>,
    pub unk_0x67531bf1: Option<u32>,
    pub unk_0x84b13172: Option<u32>,
    pub unk_0xed8127c4: Option<u32>,
    pub voice_chat_halo: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiUnitStatusData {
    pub center_justify_status_icon_and_text: Option<bool>,
    pub name_text: Option<u32>,
    pub status_duration_bar_data: Option<UiUnitStatusDurationBarData>,
    pub status_icon: u32,
    pub status_text: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiUnitStatusDurationBarData {
    pub status_duration_bar: u32,
    pub tenacity_bar: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiWeeklyMissionTemplate {
    pub completion_vfx: u32,
    pub description_text: u32,
    pub divder: u32,
    pub footer_group: u32,
    pub helper_text: u32,
    pub horizontal_spacer: u32,
    pub layout_group: u32,
    pub left_managed_layout: u32,
    pub mission_layout: u32,
    pub mission_objective_data: UiMissionObjectiveData,
    pub new_mission_vfx: u32,
    pub objective_layout: u32,
    pub objective_list_layout: u32,
    pub reward_group: u32,
    pub reward_icon: u32,
    pub reward_icon_frame: u32,
    pub reward_layout: u32,
    pub reward_text: u32,
    pub title_group: u32,
    pub title_text: u32,
    pub vertical_spacer: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UnitFloatingInfoBarData {
    pub aggro: u32,
    pub anchor: u32,
    pub border: u32,
    pub death_anim_ally: Option<u32>,
    pub death_anim_enemy: Option<u32>,
    pub health_bar: HealthBarData,
    pub highlight: u32,
    pub icons: Option<HeroFloatingInfoIconsData>,
    pub objective_bounty_ally: Option<u32>,
    pub objective_bounty_enemy: Option<u32>,
    pub par_bar: Option<AbilityResourceBarData>,
    pub scene: u32,
    pub scripted_threshold_types: Option<HashMap<u32, u32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnitLevelUiData {
    pub level_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnitShopDisplayTraitData {
    pub styles: HashMap<u32, UnitShopItemTraitData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnitShopItemData {
    pub click_rejected_vfx: u32,
    pub description: u32,
    pub frames: HashMap<u8, Unk0x6c66eae4>,
    pub gold_cost_icon: u32,
    pub gold_cost_icon_disabled: u32,
    pub health_cost_icon: u32,
    pub health_cost_icon_disabled: u32,
    pub highlight_onestar: u32,
    pub highlight_threestar: u32,
    pub highlight_twostar: u32,
    pub hit_target: u32,
    pub rarity6_cost_health_vfx: u32,
    pub scene: u32,
    pub styles: HashMap<u32, u32>,
    pub team_plan_reminder: u32,
    pub traits: Vec<UnitShopDisplayTraitData>,
    pub unit_cost_text: u32,
    pub unit_name_text: u32,
    pub unit_portrait: u32,
    pub unk_0xa6d1796b: Option<Unk0xc73631da>,
    pub unk_0xb11cb7d: Option<Unk0x2b365a82>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnitShopItemTraitData {
    pub background: u32,
    pub highlight: u32,
    pub icon: u32,
    pub name_text: u32,
    pub trait_nub: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnitStatUiData {
    pub text: u32,
    pub unk_0x1fe7164e: EnumUnk0x32078b36,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnitStatsUiData {
    pub advanced_stats: Option<HashMap<u8, UnitStatUiData>>,
    pub basic_stats: HashMap<u8, UnitStatUiData>,
    pub new_stats: Option<HashMap<u8, UnitStatUiData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnitStatusData {
    pub attackable_unit_status_type: Option<u32>,
    pub loc_type: Option<u32>,
    pub status_name: String,
    pub texture_uvs: Vec4,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UnitStatusPriorityList {
    pub m_prioritized_unit_status_data: Vec<UnitStatusData>,
    pub texture_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1003c990 {
    pub texture_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x104afcda {
    pub animation_items: Vec<Unk0xd31bbf89>,
    pub block_input_events: Option<bool>,
    pub enabled: Option<bool>,
    pub layer: Option<u32>,
    pub material: Option<u32>,
    pub name: String,
    pub per_attachment_material: Option<HashMap<u32, u32>>,
    pub position: EnumUiPosition,
    pub scene: u32,
    pub spine_atlas_file: String,
    pub spine_skel_file: String,
    pub unk_0x2181f0dd: Option<bool>,
    pub unk_0x4fc07890: Option<String>,
    pub unk_0x68dace96: Option<Unk0x7acf50f9>,
    pub unk_0xc31d847f: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x10df77cc {
    pub unk_0x3d671f4c: u32,
    pub unk_0x74fb3eb0: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x114c210c {
    pub device_ux: i32,
    pub unk_0x2b40381c: String,
    pub unk_0x4ede7d51: u32,
    pub unk_0x9051620d: HashMap<u32, Unk0x4576713f>,
    pub unk_0xfd73ebe0: HashMap<u32, Unk0x58ae628f>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1181085f {
    pub stage: u8,
    pub title_tra: String,
    pub unk_0x77cef4e0: Vec<Unk0xf2bc55fb>,
    pub visible_elements: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x11a27f9e {
    pub styles: Vec<Unk0x3c52b66>,
    pub visibility: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x125a3586 {
    pub unk_0xe61bf09e: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x12b12bdf {
    pub sub_team_name: u32,
    pub unk_0xa41734ff: u32,
    pub unk_0xa9d41f94: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x131f5725 {
    pub static_texture: Option<u32>,
    pub unk_0xe50e4f4f: String,
    pub vfx: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x132b63da {
    pub table: Option<EnumVars>,
    pub var: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x135a0579 {
    pub result_grid_item: u32,
    pub result_icon: u32,
    pub result_region: u32,
    pub result_text: u32,
    pub tooltip_offset: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x138c3d23 {
    pub description: u32,
    pub scene: u32,
    pub subtitle: u32,
    pub subtitle_tra: String,
    pub title: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x143f10a8 {
    pub unk_0x4facf6d1: Vec<Unk0x46a048a9>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1493959a {
    pub event_pass_button: u32,
    pub event_pass_completed_frame: u32,
    pub event_pass_completed_text: u32,
    pub event_pass_exp_text: u32,
    pub event_pass_name_text: u32,
    pub event_pass_next_reward_icon: u32,
    pub event_pass_next_reward_text: u32,
    pub unk_0x1bb7131a: Unk0x990115ea,
    pub unk_0xed10fe9a: UiMilestoneProgressMeter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x150d1b92 {
    pub unk_0x717e686: Option<bool>,
    pub unk_0xe38f54f7: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x15ebaa9c {
    pub dest: ScriptTableSet,
    pub formula: String,
    pub is_disabled: Option<bool>,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x165c0117 {
    pub filter: EnumOptionItemFilter,
    pub template: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1668b3e5 {
    pub event_pass_button: u32,
    pub event_pass_completed_frame: u32,
    pub event_pass_completed_text: u32,
    pub event_pass_exp_text: u32,
    pub event_pass_name_text: u32,
    pub event_pass_next_reward_icon: u32,
    pub event_pass_next_reward_text: u32,
    pub unk_0x1bb7131a: Unk0x8d8b1535,
    pub unk_0xed10fe9a: UiMilestoneProgressMeter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x16e6e17a {
    pub unk_0xa525bc7a: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1739a4e6 {
    pub unk_0x9a9ef17d: u32,
    pub unk_0xa9a04dd9: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x190ed0b9 {
    pub button: u32,
    pub button_text: Option<String>,
    pub event_id: String,
    pub event_timer: Option<TftEventTimer>,
    pub group: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x19da44b2 {
    pub unk_0x44146c9d: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1a4d18fe {
    pub disabled: Option<bool>,
    pub element: Unk0x37841b56,
    pub enable_on_end: Option<bool>,
    pub end: Option<SequenceTiming>,
    pub start: Option<SequenceTiming>,
    pub unk_0x76df2a8b: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1a4d9bd {
    pub spell_buff: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1aae122 {
    pub max_distance: f32,
    pub max_offset_delta: Option<f32>,
    pub min_distance: f32,
    pub unk_0x7863785e: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1b3a73f0 {
    pub unk_0x693979f: String,
    pub unk_0xd5cba936: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1b85de12 {
    pub base_loadable: u32,
    pub path_hash_to_self: u64,
    pub scene_link: u32,
    pub visibility_menu_button: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1c91dc02 {
    pub base_loadable: u32,
    pub champion_template: Unk0xd7ec4ad6,
    pub close_button: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub snapshot_button: u32,
    pub tablet_override_loadable: Option<u32>,
    pub unk_0x10262f0d: u32,
    pub unk_0x27a5bc4b: String,
    pub unk_0x2b70d681: Option<Unk0x48eda36d>,
    pub unk_0x4191b710: u32,
    pub unk_0x4da5487d: String,
    pub unk_0x50664e3b: String,
    pub unk_0x5d127174: u32,
    pub unk_0x5d40ba6f: HashMap<u32, u32>,
    pub unk_0x5d473c96: Unk0x7442c4c0,
    pub unk_0x68e102ec: f32,
    pub unk_0x7a2ae3db: String,
    pub unk_0x8985222b: String,
    pub unk_0x98572c3: String,
    pub unk_0xa53f4255: u32,
    pub unk_0xa579b2bc: u32,
    pub unk_0xb73bf774: u32,
    pub unk_0xc44cfe37: u32,
    pub unk_0xc79fed23: Vec<String>,
    pub unk_0xcb923d3b: u32,
    pub unk_0xccb89314: String,
    pub unk_0xdd7e1f42: String,
    pub unk_0xe588b2b2: String,
    pub unk_0xeaeb6903: String,
    pub unk_0xf0247475: String,
    pub unk_0xf9d5a86b: u32,
    pub unk_0xfa0e33c7: u32,
    pub unk_0xfd5f88f0: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1ca3eb78 {
    pub unk_0x2b04f589: Option<f32>,
    pub unk_0x8628e0ff: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1d1499b7 {
    pub group: Box<EnumAddLevelTimer>,
    pub sequence: Box<ScriptSequence>,
    pub unk_0x421aee08: Unk0x851d2116,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1d452085 {
    pub m_stat: Option<u8>,
    pub unk_0x137cf12a: u32,
    pub unk_0xa519b194: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1e26bfa8 {
    pub live_update: bool,
    pub show_on_platform: u8,
    pub template: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1e2d1428 {
    pub button: u32,
    pub stat_page_view_controller: u32,
    pub unk_0x5b5f63b5: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1e8e7b49 {
    pub check: u32,
    pub favorite_icon: u32,
    pub game_pass: u32,
    pub group: u32,
    pub icon: u32,
    pub lock: u32,
    pub unk_0xd02a6781: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1f0bbd6 {
    pub unk_0x2c2ba982: String,
    pub unk_0xa8c1b67d: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1f1f50f2 {
    pub definition: Unk0x8ad25772,
    pub name: u32,
    pub transform: Mat4,
    pub unk_0xbbe68da1: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1f2e5fd0 {
    pub group: Option<u32>,
    pub unk_0x752ff961: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1f5bef59 {
    pub unk_0x7d661894: HashMap<u8, u32>,
    pub unk_0xc895f7ad: HashMap<u8, u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1f8480d8 {
    pub found_index: IntTableSet,
    pub input_string: StringTableGet,
    pub target_string: StringGet,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x1ff0e246 {
    pub unk_0x5ba4ad0f: u32,
    pub unk_0x960dcbff: u32,
    pub unk_0xd1e56ce4: u32,
    pub unk_0xe999961a: u32,
    pub unk_0xf34f5793: u32,
    pub unk_0xf39f8571: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x20194a16 {
    pub resource_resolver: u32,
    pub unk_0xfcea40c4: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x209fa685 {
    pub reward_sound_event: String,
    pub ui_elements: Vec<u32>,
    pub unk_0xcebbf565: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x2152c2a1 {
    pub unk_0x5e3d5ec0: Vec<u32>,
    pub unk_0xfd36c650: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x215f4776 {
    pub display_name: String,
    pub unk_0x1b2d687d: Vec<Unk0xbb778e9b>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x21814e41 {
    pub path_hash_to_self: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x22a6b567 {
    pub background_image: u32,
    pub base_loadable: u32,
    pub continue_button: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub unk_0x7f7e87bc: Unk0x239ebd66,
    pub unk_0x95c5b3e8: Unk0x239ebd66,
    pub unk_0xbcb67e65: u32,
    pub unk_0xc1c6ede2: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x22d93e23 {
    pub base_loadable: u32,
    pub cancel_button_definition: u32,
    pub checkbox: u32,
    pub close_button_definition: u32,
    pub confirm_button_definition: u32,
    pub content_scene: u32,
    pub path_hash_to_self: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x22dd5ebf {
    pub unk_0xd9306080: Unk0xdc24bc6f,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x2330a302 {
    pub input: EnumAddLevelTimer,
    pub unk_0x3f074682: Option<EnumAddLevelTimer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x2363fb10 {
    pub animation_name: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x239ebd66 {
    pub background: u32,
    pub frame: u32,
    pub group: u32,
    pub icon: u32,
    pub level_icon: u32,
    pub name_text: Option<u32>,
    pub unk_0x8d0e3f5e: u32,
    pub unk_0xd02a6781: u32,
    pub unk_0xd0596065: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x24e1cb45 {
    pub unk_0xb8ee5cef: f32,
    pub unk_0xc080021c: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x253eba55 {
    pub base_loadable: u32,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub unk_0x279a1b43: Unk0xf68386bb,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x25bfa52 {
    pub unk_0x4397646b: u8,
    pub unk_0xd851ffa3: String,
    pub value: FloatGet,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x25e3f5d0 {
    pub definition: Unk0xf775806c,
    pub name: u32,
    pub transform: Mat4,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x26435b25 {
    pub base_loadable: u32,
    pub card_background: u32,
    pub countdown_background: u32,
    pub countdown_text: u32,
    pub description_text: u32,
    pub highlight: u32,
    pub icon: u32,
    pub max_alpha: u8,
    pub path_hash_to_self: u64,
    pub scene_handle: u32,
    pub title_text: u32,
    pub transition_time: f32,
    pub unk_0xa077200a: u32,
    pub unk_0xaa391f82: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x26949c34 {
    pub button_element: u32,
    pub button_type: Option<u8>,
    pub id: Option<u32>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x2697432e {
    pub unk_0x12279fa7: f32,
    pub unk_0x46e108ef: Option<f32>,
    pub unk_0x552282e2: u32,
    pub unk_0x909e7f19: u32,
    pub unk_0xb4e6bd8d: u32,
    pub unk_0xc5cbeaa4: String,
    pub unk_0xdb58f09e: String,
    pub unk_0xf681a485: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x272b222f {
    pub continue_button: u32,
    pub info_button: u32,
    pub unk_0x1a627229: u32,
    pub unk_0x22673072: u32,
    pub unk_0x359a88b9: Unk0x34bf0406,
    pub unk_0xab5cfd6e: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x2743f2b3 {
    pub button: u32,
    pub group: u32,
    pub icon: u32,
    pub textures: Unk0x895afb93,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x276246d8 {
    pub unk_0x90ebcab0: HashMap<String, Unk0x72553f91>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x27d533fe {
    pub background_missing_texture: u32,
    pub background_texture: u32,
    pub base_loadable: u32,
    pub confirm_button_definition: u32,
    pub content_scene: u32,
    pub description_text: u32,
    pub disabled_text: u32,
    pub name_text: u32,
    pub owned_trakey: String,
    pub path_hash_to_self: u64,
    pub price_text: u32,
    pub redeem_trakey: String,
    pub tablet_override_loadable: u32,
    pub unk_0x1558a5dd: u32,
    pub unk_0x1e3e07ee: f32,
    pub unk_0x27b7800b: String,
    pub unk_0x3928b393: u32,
    pub unk_0x3aecdaf0: u32,
    pub unk_0x3bc27cba: u32,
    pub unk_0x3f5f98b7: String,
    pub unk_0x4367434e: String,
    pub unk_0x4c24968a: u32,
    pub unk_0x5719b3c0: String,
    pub unk_0x6807f689: Unk0xd37b1519,
    pub unk_0x9829caab: u32,
    pub unk_0xaa95fec8: u32,
    pub unk_0xad82e2a2: u32,
    pub unk_0xb08308c8: u32,
    pub unk_0xb372bc4c: u32,
    pub unk_0xb61addc7: u32,
    pub unk_0xbc03fb23: String,
    pub unk_0xbcbb9921: u32,
    pub unk_0xbe25799e: u32,
    pub unk_0xc5317d2a: u32,
    pub unk_0xde99e0d8: String,
    pub unk_0xf4019c2e: u32,
    pub unk_0xfbf302db: Unk0x41bdce89,
    pub unk_0xfff98f6a: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x280745b1 {
    pub params: Unk0xc7e628b9,
    pub unk_0x50aad250: Vec<Unk0xdd661aab>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x28628b50 {
    pub position: VectorTableGet,
    pub target: Unk0x132b63da,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x292991be {
    pub unk_0x2e104d7: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x294650d7 {
    pub output: EnumAddLevelTimer,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x29619231 {
    pub unk_0x2ac8249e: f32,
    pub unk_0x9044f758: f32,
    pub unk_0xfdf7c618: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x296c4c00 {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x2a2c82dc {
    pub unk_0x8c54e274: Option<HashMap<u32, Box<EnumAddLevelTimer>>>,
    pub unk_0xde888209: Option<HashMap<u32, Box<EnumAddLevelTimer>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x2b365a82 {
    pub lock_icon: u32,
    pub unk_0xc2116a86: Option<u32>,
    pub unk_0xfd6a4413: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x2b686c3d {
    pub unk_0x1cf2c30f: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x2bb66cd6 {
    pub unk_0xbc526c1f: String,
    pub unk_0xdb5c861b: String,
    pub unk_0xf1f242a0: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x2bfb084c {
    pub group_name: String,
    pub tags: Vec<Unk0xf6f4bb5f>,
    pub unk_0xec01928c: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x2cc33a4b {
    pub base_loadable: u32,
    pub main_scene: u32,
    pub path_hash_to_self: u64,
    pub respawn_timer_text: u32,
    pub shop_button: u32,
    pub total_damage_radial_chart: Unk0x5cafc4e3,
    pub unk_0x11b79106: u32,
    pub unk_0x126c76f5: String,
    pub unk_0x26539d4d: u32,
    pub unk_0x27a1df9b: u32,
    pub unk_0x2e192913: u32,
    pub unk_0x3062cfa: u32,
    pub unk_0x33095f20: u32,
    pub unk_0x35bfddff: bool,
    pub unk_0x3c4a9633: String,
    pub unk_0x3dce1bf3: u32,
    pub unk_0x40cf14f9: u32,
    pub unk_0x47483a1: u32,
    pub unk_0x4a255f53: u32,
    pub unk_0x630b9e09: u32,
    pub unk_0x8c804934: bool,
    pub unk_0x9c56227: u32,
    pub unk_0xae8418d9: u32,
    pub unk_0xb84d80fb: Unk0x2d965f1c,
    pub unk_0xb8e3898: u32,
    pub unk_0xbef7045a: String,
    pub unk_0xc060792d: u32,
    pub unk_0xc2368229: u32,
    pub unk_0xca290d44: u32,
    pub unk_0xcc6fee26: Unk0xb1edf879,
    pub unk_0xd3547072: u32,
    pub unk_0xd44a200a: u32,
    pub unk_0xde91dbcb: u32,
    pub unk_0xe08bc4b7: Unk0x4e7f16fe,
    pub unk_0xeb57fe3d: u32,
    pub unk_0xf44069a6: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x2d965f1c {
    pub group: u32,
    pub killer_icon: u32,
    pub portrait: u32,
    pub unk_0x993f753a: Unk0x2743f2b3,
    pub unk_0xad0c960e: u32,
    pub unk_0xb80df257: Unk0x3094abd6,
    pub unk_0xcc8608fe: u32,
    pub unk_0xd90a6600: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x2db7365f {
    pub group: u32,
    pub label: u32,
    pub slider: u32,
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x2e181f68 {
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x2ea23974 {
    pub frame: u32,
    pub group: u32,
    pub icon: u32,
    pub item_transition_in: HudMenuTransitionData,
    pub scene: u32,
    pub vfx: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x303c66f4 {
    pub buff_type: u8,
    pub unk_0xe93cd19c: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3094abd6 {
    pub meter_elements: u32,
    pub unk_0x22bfe860: Vec<u32>,
    pub unk_0x364bf7c: u32,
    pub unk_0xb7cd7f6b: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x30ed049a {
    pub event_name: String,
    pub event_name_tra_key: String,
    pub position: u32,
    pub unk_0x1cb3d492: String,
    pub unk_0xa7429aa5: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x313c0076 {
    pub enabled: bool,
    pub icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x313da04c {
    pub item_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x31bf21b0 {
    pub bounds_element: u32,
    pub unk_0x25b31bc: u32,
    pub unk_0x2a781f11: u32,
    pub unk_0x2d1115b1: u32,
    pub unk_0xbaa5364a: u32,
    pub unk_0xcaacc388: u32,
    pub unk_0xf6b809cd: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x31fc4adc {
    pub base_loadable: u32,
    pub close_button: u32,
    pub emote_info_panel: LoadoutEmoteInfoPanel,
    pub emote_loadout_grid_button_data: EmoteLoadoutGridButtonData,
    pub emote_loadout_item_list_panel_data: LoadoutItemListPanelData,
    pub equip_button: u32,
    pub grid_item_button: u32,
    pub loadout_scene: u32,
    pub path_hash_to_self: u64,
    pub searchbar_text: u32,
    pub sound_on_activate: String,
    pub sound_on_de_activate: String,
    pub tablet_override_loadable: u32,
    pub unk_0x1aa55fe1: u32,
    pub unk_0x2eed7e1b: u32,
    pub unk_0x4b90d43d: u32,
    pub unk_0xa95ede35: u32,
    pub unk_0xbfd5b639: u32,
    pub unk_0xd5846a0a: u32,
    pub unk_0xe0ece3d9: u32,
    pub upgrade_button: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x32078b36 {
    pub region: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x321b2b1c {
    pub base_loadable: u32,
    pub path_hash_to_self: u64,
    pub tablet_override_loadable: Option<u32>,
    pub unk_0x491204c2: Unk0xa9253585,
    pub unk_0xbce152b: u32,
    pub unk_0xd7cb25a0: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3283e2d6 {
    pub unk_0x5f2346f6: f32,
    pub unk_0x8a0cc660: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x349845d9 {
    pub tft_set_data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x34a5a0c9 {
    pub character: u32,
    pub skin_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x34bf0406 {
    pub score_value: u32,
    pub unk_0x1741a1c: u32,
    pub unk_0xabbacbb6: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x34cca270 {
    pub button: u32,
    pub button_icons: Vec<u32>,
    pub group: u32,
    pub modal_upper_left_anchor: u32,
    pub subtitle: Option<u32>,
    pub subtitle_background: Option<u32>,
    pub title: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x34f43159 {
    pub unk_0x3a302e74: EnumDriver,
    pub value_array: Vec<Unk0x9a7de981>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x35317d3f {
    pub buffer_region_element: u32,
    pub drag_region_element: u32,
    pub enabled: Option<bool>,
    pub layer: u32,
    pub name: String,
    pub scissor_region_element: u32,
    pub scrolling_scene: u32,
    pub slider: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3562d439 {
    pub source: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x36a5593c {
    pub unk_0x27b7800b: String,
    pub unk_0x3f5f98b7: String,
    pub unk_0x4367434e: String,
    pub unk_0x5719b3c0: String,
    pub unk_0xde99e0d8: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x37841b56 {
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x37a8385c {
    pub buff_display: Unk0x3aee5d80,
    pub expanded_region: u32,
    pub group: u32,
    pub item_slots: DetailedItemSlots,
    pub summoner_name: SummonerNameUiData,
    pub team_frame_region: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x37ee6882 {
    pub unk_0x38f34447: f32,
    pub unk_0xc1d0e91a: f32,
    pub unk_0xf842ed21: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x382277da {
    pub m_subparts: Vec<Box<EnumAbilityResourceByCoefficientCalculationPart>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x38d7429 {
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x393df345 {
    pub slot_template: Unk0x5841e5bd,
    pub slots_layout: u32,
    pub unk_0xd04a0fe6: u32,
    pub unk_0xd3a91466: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x394f5aaf {
    pub unk_0x55a6d9a: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x39a9452c {
    pub unk_0x24e31fac: String,
    pub unk_0x3518798d: String,
    pub unk_0x400cac9: String,
    pub unk_0x8322a528: String,
    pub unk_0x97dec6cf: String,
    pub unk_0x9cc6881: String,
    pub unk_0xec824f6b: String,
    pub unk_0xf2714556: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3a8e6763 {
    pub bounds_element: u32,
    pub cast_all_button_definition: u32,
    pub hotkey_button_definition: u32,
    pub hotkey_button_text_small: u32,
    pub hotkey_modifier_text: u32,
    pub hotkey_quick_cast_button_definition: u32,
    pub labels: Vec<Unk0x6a04facb>,
    pub normal_cast_button_pos: u32,
    pub quick_cast_button_pos: u32,
    pub unk_0xb712cd3d: bool,
    pub unk_0xbda33073: u32,
    pub unk_0xd8b966a9: Vec<Unk0x30ed049a>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3aa63a24 {
    pub target: Unk0x132b63da,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3aee5d80 {
    pub buff_display_template: BuffDisplayData,
    pub buff_layout: u32,
    pub max_buff_display_count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3b2ba6c0 {
    pub icon_override_texture_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3b6f488e {
    pub button: u32,
    pub frame_icon: u32,
    pub trait_icons: Vec<u32>,
    pub unk_0x12cef16b: Unk0x2b365a82,
    pub unk_0x233b0b50: HashMap<u32, u32>,
    pub unk_0x8f8b64b6: HashMap<u32, u32>,
    pub unk_0xc6668613: u32,
    pub unk_0xca62500c: u32,
    pub unk_0xe58d38df: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3b8a61ee {
    pub unk_0xa0fe8b04: TftArmorySlotData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3bd6ea88 {
    pub id: String,
    pub instance_data: Vec<Unk0x8f92cfab>,
    pub preferred_order: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3c2230b7 {
    pub unk_0x1693e860: String,
    pub unk_0x542585b5: String,
    pub unk_0xf701308d: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3c52b66 {
    pub icon: Option<u32>,
    pub start_time: f32,
    pub text: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3c5f9d3d {
    pub data: Unk0x4b26ffad,
    pub key: Unk0x3562d439,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3c995caf {
    pub name: String,
    pub segments: Vec<Vec3>,
    pub transform: Mat4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3dc0ea14 {
    pub element: EnumUnk0xf4737fbd,
    pub end: SequenceTiming,
    pub start: SequenceTiming,
    pub text: EnumUnk0x296c4c00,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3eed1ba8 {
    pub key_type: EnumUnk0x3c5f9d3d,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3f040c8e {
    pub group: u32,
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3f04641e {
    pub camp_name: String,
    pub unk_0x7511a599: Option<Vec<Unk0xdb26669a>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3f1c01bb {
    pub quality_setting: u32,
    pub unk_0x2f7e46c8: f32,
    pub unk_0xc58ea052: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x3f667d7e {
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4146f732 {
    pub unk_0xf248d989: u32,
    pub unk_0xfa27d217: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x418af145 {
    pub bit_field: IntTableGet,
    pub position: IntTableGet,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x41bdce89 {
    pub unk_0x137bf219: Option<u32>,
    pub unk_0x1bbefc93: Vec<Unk0xf45e04e1>,
    pub unk_0x5bf8755d: Option<u32>,
    pub unk_0x6152a19f: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x420e433d {
    pub game_pass_icon: u32,
    pub item_icon: u32,
    pub item_icon_text: u32,
    pub item_lock_icon: u32,
    pub scene: u32,
    pub unk_0x8300af56: i32,
    pub unk_0xbd2f7d09: String,
    pub unk_0xd02a6781: u32,
    pub unk_0xee425ddc: u32,
    pub view_pane_link: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x429a2180 {
    pub camp_level: Option<u16>,
    pub minimap_icon: Option<String>,
    pub minimap_icon_offset: Option<Vec3>,
    pub reveal_event: Option<u16>,
    pub scoreboard_timer: Option<u16>,
    pub stop_spawn_time_secs: Option<f32>,
    pub tags: Option<Vec<u32>>,
    pub team: u32,
    pub unk_0x1f2e5fd0: Option<Unk0x1f2e5fd0>,
    pub unk_0x5a4ef4e7: u32,
    pub unk_0x7d27af7f: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x42dcedbc {
    pub border: u32,
    pub group: u32,
    pub static_icon: u32,
    pub timer_vfx: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x43aaf187 {
    pub phase_hit_region: u32,
    pub phase_icon: u32,
    pub unk_0x39b2057a: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4576713f {
    pub icon_data: Vec<Unk0x64aa69ad>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x45d6060 {
    pub ping_category: Option<u8>,
    pub unk_0x4d5476d2: Option<u32>,
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x45f140fc {
    pub title_defeat_tra: String,
    pub title_future_tra: String,
    pub title_next_tra: String,
    pub title_victory_tra: String,
    pub unk_0x10074827: String,
    pub unk_0x3fca802: String,
    pub unk_0x4c453e79: String,
    pub unk_0x8e3d338b: String,
    pub unk_0xd0133f4a: String,
    pub unk_0xe130f1de: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x46a048a9 {
    pub group: u32,
    pub time_text: u32,
    pub unk_0xdd58f41d: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x46ce0526 {
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x470b636c {
    pub priority: Option<i8>,
    pub unk_0xe9103252: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x47f13ab0 {
    pub unk_0xcf19cb5d: Unk0x770f7888,
    pub unk_0xe4f7105d: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x487b1677 {
    pub unk_0x4d8cab3e: Option<Vec<Unk0xa50ab26>>,
    pub unk_0x6406d8f7: Vec<Unk0xa50ab26>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x48eda36d {
    pub unk_0x17043336: u32,
    pub unk_0x28465cea: String,
    pub unk_0x772e2c8c: u32,
    pub unk_0x909f73c5: u32,
    pub unk_0xc51cf463: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x48f3fe52 {
    pub unk_0xb9cb9ce8: Vec<Unk0x7084628f>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x49401c5c {
    pub unk_0x5e3d5ec0: Vec<u32>,
    pub unk_0xfd36c650: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x496d22af {
    pub base_loadable: u32,
    pub close_button: u32,
    pub description_text: u32,
    pub error_color: [u8; 4],
    pub grid: u32,
    pub level_text: u32,
    pub path_hash_to_self: u64,
    pub purchase_button: u32,
    pub scene: u32,
    pub template: Unk0x9efb6a51,
    pub unk_0x12c1bc1: [u8; 4],
    pub unk_0x19cb36b2: u32,
    pub unk_0x215bac2a: UiHyperlink,
    pub unk_0x21ce7cca: u32,
    pub unk_0x3812e73e: u32,
    pub unk_0x424f9539: u32,
    pub unk_0x54b57536: u32,
    pub unk_0x626c7955: u32,
    pub unk_0x996a8f3f: u32,
    pub unk_0xc01c3f65: u32,
    pub unk_0xc2aa4bfd: u32,
    pub unk_0xc3c5d21: u32,
    pub unk_0xd5d75a69: i32,
    pub unk_0xddf728ec: u32,
    pub unk_0xe2c11a81: u32,
    pub unk_0xebf79b40: u32,
    pub unk_0xed599fd8: u32,
    pub unk_0xfff98f6a: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x49f51d24 {
    pub base_loadable: u32,
    pub mobile_override: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub unk_0x395f0ee5: u32,
    pub unk_0x451a36ef: u32,
    pub unk_0x459e8e56: u32,
    pub unk_0x765d6f7: u32,
    pub unk_0xad211785: u32,
    pub unk_0xb1a780bf: u32,
    pub unk_0xc98119b: Unk0x8948ed01,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4a0e85bf {
    pub unk_0x38ad90f3: f32,
    pub unk_0xbc94aba4: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4a70b12c {
    pub augment_group: Vec<u32>,
    pub unk_0x9a676645: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4a7922fb {
    pub augment_slots: Unk0xfc331f53,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4a929a90 {
    pub unk_0xb2811129: Option<u8>,
    pub unk_0xd8b7cfd7: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4ab36eb5 {
    pub unk_0x93f0c42c: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4af7e9f2 {
    pub compare_op: u8,
    pub unk_0x55848081: Unk0x7463e786,
    pub unk_0x6caa1bcc: Unk0x7463e786,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4b150334 {
    pub unk_0x11784e5b: String,
    pub unk_0x17d6be37: Option<String>,
    pub unk_0xc2079a3f: String,
    pub unk_0xd58eab1d: String,
    pub unk_0xf8ee7183: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4b26ffad {
    pub value: HashMap<u32, u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4b30a085 {
    pub target: Unk0x132b63da,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4c1fe46e {
    pub source: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4cf74021 {
    pub definition: Unk0xfbbe5989,
    pub name: u32,
    pub transform: Mat4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4d80ee3d {
    pub unk_0x29d7590a: String,
    pub unk_0xf7fe534f: String,
    pub width: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4d909a30 {
    pub bounds_element: u32,
    pub label: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4e16b860 {
    pub cosmetic_icons: Vec<u32>,
    pub expanded_holder_icon: u32,
    pub holder_icon: u32,
    pub managed_layout: u32,
    pub mission_group: u32,
    pub primary_mission: Unk0xd19d72ee,
    pub progress_icons: Vec<Unk0xb4517220>,
    pub secondary_mission: Unk0xd19d72ee,
    pub unk_0x3a95996f: Vec<u32>,
    pub unk_0x51e9dd09: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4e7f16fe {
    pub group: u32,
    pub spell_layout: u32,
    pub unk_0x16f1763f: Unk0x6098cd78,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4f0aa8a0 {
    pub element: u32,
    pub element_anchor: Option<u8>,
    pub tooltip_text: EnumUnk0x34f43159,
    pub unk_0x36e9d151: Option<bool>,
    pub unk_0xa620b98f: Option<Vec<Unk0xd5c5318a>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x4fbb3f5d {
    pub cosmetic_icons: Vec<u32>,
    pub expanded_holder_icon: u32,
    pub holder_icon: u32,
    pub managed_layout: u32,
    pub mission_group: u32,
    pub primary_mission: Unk0xece8d41b,
    pub secondary_mission: Unk0xece8d41b,
    pub separator_icon: u32,
    pub separator_text: u32,
    pub tracker_icon_template: Option<Unk0xd65c937c>,
    pub unk_0x20c75747: String,
    pub unk_0x233c17d3: [u8; 4],
    pub unk_0x2585bb19: [u8; 4],
    pub unk_0x25a7b1dc: Option<bool>,
    pub unk_0x2ebc4f20: [u8; 4],
    pub unk_0x3a95996f: Vec<u32>,
    pub unk_0x49df37c2: [u8; 4],
    pub unk_0x4b634d73: [u8; 4],
    pub unk_0x51e9dd09: Vec<u32>,
    pub unk_0x5865eb60: [u8; 4],
    pub unk_0x64761191: String,
    pub unk_0x64df6b08: [u8; 4],
    pub unk_0x65ae83cd: [u8; 4],
    pub unk_0x7169294e: String,
    pub unk_0x71ced82e: [u8; 4],
    pub unk_0x74113c2e: [u8; 4],
    pub unk_0x7f15daa1: [u8; 4],
    pub unk_0x853847b9: Option<String>,
    pub unk_0x9520ea9e: [u8; 4],
    pub unk_0xa09cd543: [u8; 4],
    pub unk_0xa26a2763: [u8; 4],
    pub unk_0xa8b5db8d: [u8; 4],
    pub unk_0xaa4d7f80: Option<String>,
    pub unk_0xae789376: String,
    pub unk_0xaf82dc30: Option<String>,
    pub unk_0xb2121311: [u8; 4],
    pub unk_0xbd63565e: String,
    pub unk_0xcc492242: String,
    pub unk_0xce9b588e: [u8; 4],
    pub unk_0xcebb9f25: [u8; 4],
    pub unk_0xcecdd9eb: String,
    pub unk_0xcf48be81: Option<u32>,
    pub unk_0xd08d9750: String,
    pub unk_0xd1f4da53: [u8; 4],
    pub unk_0xd5ca9b39: Option<u32>,
    pub unk_0xf35849d9: [u8; 4],
    pub unk_0xf689bf83: [u8; 4],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5014eb79 {
    pub unk_0x24344877: Option<u32>,
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x51445de9 {
    pub value: Vec4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x51ada002 {
    pub unk_0x6f18a0d: Option<u8>,
    pub unk_0x75d39a3b: String,
    pub unk_0xa5912d83: bool,
    pub unk_0xf618789b: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x51db35d3 {
    pub claimable_default_icon: String,
    pub claimable_hover_icon: String,
    pub complete_default_icon: String,
    pub complete_hover_icon: String,
    pub locked_default_icon: String,
    pub locked_hover_icon: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x51f3b0ef {
    pub spell_slot: Option<Unk0x5f011348>,
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5206ee88 {
    pub owner_champion: u32,
    pub spell: u32,
    pub unk_0xf054015a: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5307f5e1 {
    pub tra_key: String,
    pub unk_0x6bfb316f: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x53dfc5b5 {
    pub subtract: FloatLiteralMaterialDriver,
    pub value: Box<Unk0xcc35f742>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x557bb273 {
    pub value: Option<Vec4>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x55f6bf86 {
    pub effect_key: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x56bb851 {
    pub unk_0xe6d60f41: Option<HashMap<u8, Unk0xc76c1b9a>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5727ed42 {
    pub custom_table: CustomTableGet,
    pub key: ScriptTableGet,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5841e5bd {
    pub description: u32,
    pub group: u32,
    pub icon: u32,
    pub subtitle: u32,
    pub title: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5858e503 {
    pub event_lists: HashMap<String, u32>,
    pub unk_0x8a68bc86: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5860a1dc {
    pub base_loadable: u32,
    pub champion_listing_data: Unk0xc8c47da,
    pub close_button_definition: u32,
    pub confirm_button_definition: u32,
    pub content_scene: u32,
    pub path_hash_to_self: u64,
    pub placeholder_text: u32,
    pub team_name_text: u32,
    pub title_text: u32,
    pub unk_0x21148b40: HashMap<u32, u32>,
    pub unk_0x27afb7ac: u32,
    pub unk_0x3bc6a68: u32,
    pub unk_0x46bcd3de: u32,
    pub unk_0x4f0e44f5: u32,
    pub unk_0x57dc813a: u32,
    pub unk_0x653ba7b0: u32,
    pub unk_0x780e4425: u32,
    pub unk_0x80a5b0b0: String,
    pub unk_0x8d90e83f: u32,
    pub unk_0x90bbe05d: u32,
    pub unk_0x9cf04786: u32,
    pub unk_0x9ed937e5: u32,
    pub unk_0xaf45567c: String,
    pub unk_0xb555559d: String,
    pub unk_0xcdfd3bca: u32,
    pub unk_0xd5240093: u32,
    pub unk_0xee832608: String,
    pub unk_0xfc331c21: String,
    pub unk_0xffcad394: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x58ae628f {
    pub unk_0x165df089: Option<String>,
    pub unk_0x514b064: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5a92b195 {
    pub unk_0x68d95a00: HashMap<String, String>,
    pub unk_0x911d1120: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5ab5b20f {
    pub device_ux: i32,
    pub time_text: u32,
    pub unk_0xadbcc5ee: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5af2b668 {
    pub unk_0xc02ad6d8: String,
    pub unk_0xd5cba936: String,
    pub unk_0xf68db2ae: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5b2fdd66 {
    pub add: FloatLiteralMaterialDriver,
    pub value: Box<EnumDriver>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5b4f4df6 {
    pub path_hash: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5b5e6994 {
    pub timer_default_fill: u32,
    pub unk_0x81cbec5f: u32,
    pub unk_0x9ba9ed34: u32,
    pub unk_0xbcde5149: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5c8aed6 {
    pub event_icon: u32,
    pub unk_0x1a3d3933: Option<Unk0xfa33a427>,
    pub unk_0x1ff99d7f: Unk0xfa33a427,
    pub unk_0x6229f94d: Option<Unk0xfa33a427>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5cafc4e3 {
    pub data: Vec<Unk0xad9f1783>,
    pub hover_region: u32,
    pub unk_0x4b8df5f4: u32,
    pub unk_0x9f4f0ed8: u32,
    pub unk_0xebd43853: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5cb6b755 {
    pub background_texture_path: String,
    pub background_vfx: Option<u32>,
    pub unk_0x16e21209: String,
    pub unk_0x20b77151: String,
    pub unk_0x2abb026a: String,
    pub unk_0x2e09a7c: String,
    pub unk_0x2ee329e: String,
    pub unk_0x314a63bb: String,
    pub unk_0x3392c041: String,
    pub unk_0x36b9f511: String,
    pub unk_0x4026cb15: String,
    pub unk_0x414d5717: String,
    pub unk_0x4cbd596a: String,
    pub unk_0x58766309: String,
    pub unk_0x63d105a5: String,
    pub unk_0x765c4429: String,
    pub unk_0x7e458980: String,
    pub unk_0x96349f57: String,
    pub unk_0x9a0caad2: String,
    pub unk_0xa2026b64: Option<bool>,
    pub unk_0xa807a4b2: String,
    pub unk_0xc1d8474a: String,
    pub unk_0xd0666cc4: String,
    pub unk_0xde84022d: String,
    pub unk_0xdf83f0d3: String,
    pub unk_0xe98a6e08: String,
    pub unk_0xf1cfe676: String,
    pub unk_0xfa3657e2: String,
    pub unk_0xfbf4286e: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5d0f7d97 {
    pub background_icon: Option<u32>,
    pub base_loadable: u32,
    pub main_scene: u32,
    pub page_button: Option<u32>,
    pub path_hash_to_self: u64,
    pub selected_frame: u32,
    pub unk_0x109da077: u32,
    pub unk_0x2ea23974: Unk0x2ea23974,
    pub unk_0x3db64b22: String,
    pub unk_0x4da6efff: String,
    pub unk_0x54051c58: u32,
    pub unk_0x54b1eab: String,
    pub unk_0x56f014d4: String,
    pub unk_0x59fc480: Option<u32>,
    pub unk_0x6c2b545: String,
    pub unk_0x6d62bdb4: u32,
    pub unk_0x76d78bbe: u32,
    pub unk_0x83c4b267: Unk0x2697432e,
    pub unk_0x9203914b: u32,
    pub unk_0x963ad310: Option<u32>,
    pub unk_0x97b20086: u32,
    pub unk_0xb5a2f478: u32,
    pub unk_0xba92cb0e: u32,
    pub unk_0xbc6e5ae5: u32,
    pub unk_0xc40b91eb: Option<u32>,
    pub unk_0xdfb5ef06: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5d85166e {
    pub value: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5e73536 {
    pub arena_info_panel: LoadoutArenaSkinInfoPanel,
    pub arena_skin_loadout_grid_button_data: ArenaSkinLoadoutGridButtonData,
    pub arena_skin_loadout_item_list_panel_data: LoadoutItemListPanelData,
    pub base_loadable: u32,
    pub close_button: u32,
    pub companion_info_panel: LoadoutCompanionInfoPanel,
    pub companion_loadout_grid_button_data: CompanionLoadoutGridButtonData,
    pub companion_loadout_item_list_panel_data: LoadoutItemListPanelData,
    pub damage_skin_info_panel: LoadoutDamageSkinInfoPanel,
    pub damage_skin_loadout_grid_button_data: DamageSkinLoadoutGridButtonData,
    pub damage_skin_loadout_item_list_panel_data: LoadoutItemListPanelData,
    pub equip_button: u32,
    pub grid_item_button: u32,
    pub loadout_scene: u32,
    pub path_hash_to_self: u64,
    pub searchbar_text: u32,
    pub sound_on_activate: String,
    pub sound_on_de_activate: String,
    pub star_shard_currency_button: u32,
    pub star_shard_currency_widget: TftCurrencyWidget,
    pub star_shard_decrease_vfx: u32,
    pub star_shards_store_dialog: u32,
    pub tablet_override_loadable: u32,
    pub unk_0x1aa55fe1: u32,
    pub unk_0x1e8e7b49: Unk0x1e8e7b49,
    pub unk_0x2256cd1f: u32,
    pub unk_0x2eed7e1b: u32,
    pub unk_0x4b90d43d: u32,
    pub unk_0x4fc30b29: u32,
    pub unk_0x5f2cd4eb: Unk0x420e433d,
    pub unk_0x70e416ac: u32,
    pub unk_0x899e7ee7: u32,
    pub unk_0xa1d79c7c: LoadoutItemListPanelData,
    pub unk_0xa95ede35: u32,
    pub unk_0xbfd5b639: u32,
    pub unk_0xcee40baf: u32,
    pub unk_0xd2bd41f4: String,
    pub unk_0xd5846a0a: u32,
    pub unk_0xe0ece3d9: u32,
    pub unk_0xfaf537f0: u32,
    pub upgrade_button: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5f011348 {
    pub index: Option<i32>,
    pub slot_type: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x5f33c34 {
    pub button: u32,
    pub frame_icon: u32,
    pub portrait_icon: u32,
    pub trait_icons: Vec<u32>,
    pub unk_0x12cef16b: Unk0x2b365a82,
    pub unk_0x233b0b50: HashMap<u32, u32>,
    pub unk_0xca62500c: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6030f7c6 {
    pub unk_0x3cb3392e: f32,
    pub unk_0x4ec6dd3c: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6055e037 {
    pub source: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6098cd78 {
    pub group: u32,
    pub hotkey_text: u32,
    pub icon: u32,
    pub textures: Unk0x895afb93,
    pub value_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x60e2ec74 {
    pub load_screen_tip_configuration: u32,
    pub loading_screen_background: String,
    pub mutator_controlled_loading_screen_backgrounds: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x610a14d0 {
    pub id: String,
    pub unk_0x604bf65: f32,
    pub unk_0x72729cf8: Unk0x8e11ad0c,
    pub unk_0xf66fb6b0: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x61902388 {
    pub unk_0xc70f6a1e: u32,
    pub unk_0xea27ff5b: Unk0xf1fd1323,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x61b427f {
    pub unk_0x20941997: Option<u32>,
    pub unk_0x262bfa2e: Option<u32>,
    pub unk_0xaa8fd0c5: Option<bool>,
    pub unk_0xdba9e788: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6241da2 {
    pub new_pipvfx: u32,
    pub unk_0x114ce1e2: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6256517d {
    pub unk_0x85c97330: f32,
    pub unk_0x9b28c1fa: f32,
    pub unk_0xa3d51b05: f32,
    pub unk_0xb19f2894: f32,
    pub unk_0xcb2ea8af: f32,
    pub unk_0xdc300691: f32,
    pub unk_0xe552fcd7: Unk0x3283e2d6,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x629f5938 {
    pub ui_element: u32,
    pub unit_property_toggle: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x634a3a64 {
    pub color: [u8; 4],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6355dd6f {
    pub chunk: u32,
    pub visibility_controller: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x635d04b7 {
    pub champion_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x63fbd3de {
    pub description: u32,
    pub frame: u32,
    pub icon: u32,
    pub unk_0x3845cab: Vec<u32>,
    pub unk_0x5bd8b396: String,
    pub unk_0x829c2960: HashMap<i32, String>,
    pub unk_0xca62500c: u32,
    pub unk_0xcb084e46: u32,
    pub unk_0xcd3fd8ac: String,
    pub unk_0xee10ba86: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x64aa69ad {
    pub icons: Vec<Unk0xecd03686>,
    pub unk_0xb2811129: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x64ee2fb1 {
    pub color: Option<Unk0x634a3a64>,
    pub id: u32,
    pub max_scale: Option<f32>,
    pub min_scale: Option<f32>,
    pub size: Option<Vec2>,
    pub texture: Unk0x1003c990,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6548f43 {
    pub function: LevelScriptFunctionLink,
    pub function_definition: Option<Box<Unk0x2a2c82dc>>,
    pub is_disabled: Option<bool>,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x65955db8 {
    pub bounds_element: u32,
    pub unk_0x25b31bc: u32,
    pub unk_0x2a781f11: u32,
    pub unk_0xb02e568a: u32,
    pub unk_0xcaacc388: u32,
    pub unk_0xd3d87bbf: u32,
    pub unk_0xf0692ff1: u32,
    pub unk_0xf60f3af2: u32,
    pub unk_0xf6b809cd: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x65dbf6d0 {
    pub unit_property_toggle: u32,
    pub unk_0x2fd3d3c2: u32,
    pub unk_0xb10e2d1e: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6653bfda {
    pub owner_champion: u32,
    pub spell: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x667eafac {
    pub dest: VectorTableSet,
    pub target: Unk0x132b63da,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x671b7351 {
    pub vfx_group_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x67406e7f {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x67bfa45e {
    pub unk_0x217f76f0: Option<bool>,
    pub unk_0x241124fd: f32,
    pub unk_0xaf1ce7ab: Option<bool>,
    pub unk_0xec06ab24: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x67d3ab82 {
    pub button: u32,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x68f56a56 {
    pub item: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x69057401 {
    pub divider: u32,
    pub team_banner: u32,
    pub team_bye_text: Option<u32>,
    pub team_name: u32,
    pub team_position: u32,
    pub unk_0xf933c90d: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x692082f8 {
    pub unk_0x11784e5b: Option<Vec<Unk0xd31bbf89>>,
    pub unk_0x17d6be37: Option<Vec<Unk0xd31bbf89>>,
    pub unk_0x1a24d83: u32,
    pub unk_0xc2079a3f: Vec<Unk0xd31bbf89>,
    pub unk_0xd58eab1d: Option<Vec<Unk0xd31bbf89>>,
    pub unk_0xf8ee7183: Option<Vec<Unk0xd31bbf89>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x69680d {
    pub alive_icon: u32,
    pub dead_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x69bf9d05 {
    pub button: u32,
    pub filter: Option<u32>,
    pub r#type: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6a04facb {
    pub label: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6a50b5d7 {
    pub unk_0x25a09a75: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6a68b4f1 {
    pub bye_fx: u32,
    pub defeat_fx: u32,
    pub fighting_fx: u32,
    pub victory_fx: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6b863734 {
    pub path_hash: u32,
    pub vis_flags: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6b91544a {
    pub blur_textures: bool,
    pub unk_0x46edf5aa: f32,
    pub upscale_textures: bool,
    pub vfx_systems: Vec<VfxPrimitiveCameraSegmentSeriesBeam>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6bbc3db6 {
    pub spell_objects: Vec<u32>,
    pub unk_0xda28e4c: [u8; 4],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6c455dac {
    pub unk_0x23908b24: u32,
    pub unk_0x3323ae7f: UnitShopItemData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6c66eae4 {
    pub button: u32,
    pub unk_0xf688ec43: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6c84152e {
    pub against_vote: u32,
    pub for_vote: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6c93a1b4 {
    pub callback: Unk0xf6e1bec7,
    pub function_definition: Option<Box<Unk0x2a2c82dc>>,
    pub is_disabled: Option<bool>,
    pub target: Unk0x132b63da,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6db33e0c {
    pub frame: u32,
    pub name: u32,
    pub portrait_icon: u32,
    pub tooltip_region: u32,
    pub unk_0x12cef16b: Unk0x2b365a82,
    pub unk_0x491204c2: Unk0xa9253585,
    pub unk_0x507c22b2: u32,
    pub unk_0x829c2960: HashMap<i32, String>,
    pub unk_0xca62500c: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6e97f980 {
    pub unk_0x37ee6882: HashMap<u32, Unk0x37ee6882>,
    pub unk_0xe41a3709: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6f44928c {
    pub active_background: u32,
    pub group_link: u32,
    pub icon: u32,
    pub unk_0x9a800d92: u32,
    pub unk_0x9df2c4cd: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x6fb748e3 {
    pub files: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7084628f {
    pub bottom_hr_momentum_post: Vec<u32>,
    pub custom_announcement_style: u32,
    pub unk_0x9d6e31fd: u32,
    pub unk_0xc742ceb4: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x72553f91 {
    pub description_tra: String,
    pub speaker_icon: u32,
    pub time_length: f32,
    pub title_tra: String,
    pub unk_0xa077200a: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x72651449 {
    pub unk_0x989015d3: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7296321a {
    pub background_texture: u32,
    pub scene: u32,
    pub unk_0x30151f4f: u32,
    pub unk_0x395435fc: u32,
    pub unk_0x643afb52: u32,
    pub unk_0x81dd4303: u32,
    pub unk_0x834809f8: u32,
    pub unk_0xcfaa3075: u32,
    pub unk_0xe934082f: UiRotationalStoreItemTileData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x72e9216e {
    pub var: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x72f86c81 {
    pub unk_0xccfd27e6: Option<f32>,
    pub unk_0xdc9124b1: Option<[u8; 4]>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7442c4c0 {
    pub edit_button: u32,
    pub title: u32,
    pub unk_0x38823ae2: u32,
    pub unk_0x4f0e44f5: u32,
    pub unk_0x5f0e2f53: u32,
    pub unk_0x8a663f35: u32,
    pub unk_0x8fba2ead: u32,
    pub unk_0xc549c782: u32,
    pub unk_0xca62500c: u32,
    pub unk_0xcc043494: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7463e786 {
    pub unk_0xdf38ac45: u8,
    pub unk_0xf1cea006: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x755cf26f {
    pub end: Option<SequenceTiming>,
    pub sound_event: Unk0x67406e7f,
    pub start: Option<SequenceTiming>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x75e34c40 {
    pub unk_0x1dcc5270: Vec<Unk0xd5c9eb1>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7638f87c {
    pub parent: u32,
    pub reward_background: u32,
    pub reward_day_text: u32,
    pub reward_frame: u32,
    pub reward_hit_region: u32,
    pub reward_icon: u32,
    pub reward_icon_frame: Unk0x51db35d3,
    pub unk_0x56c607bc: u32,
    pub unk_0x9dcff647: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x76621fa6 {
    pub buff_name: String,
    pub target_strategy: Option<u8>,
    pub team_to_buff: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x767adcf7 {
    pub device_ux: i32,
    pub frame: u32,
    pub time_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7709fefa {
    pub data: Unk0x4c1fe46e,
    pub index: Unk0xc8879e6c,
    pub unk_0xb848c7d8: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x770f7888 {
    pub armor_per_level: Option<f32>,
    pub attack_speed_per_level: Option<f32>,
    pub base_armor: Option<f32>,
    pub base_hp: Option<f32>,
    pub damage_per_level: Option<f32>,
    pub hp_per_level: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x77f812a6 {
    pub tooltip_tra_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x77fb37dd {
    pub divider: u32,
    pub group: u32,
    pub spectate_button: u32,
    pub team_one_icon: u32,
    pub team_two_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x79292790 {
    pub unk_0xbcd3fffb: Option<u8>,
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x79efeb7c {
    pub base_loadable: u32,
    pub close_button: u32,
    pub path_hash_to_self: u64,
    pub retry_button: u32,
    pub scene: u32,
    pub tablet_override_loadable: u32,
    pub unk_0x1ea15738: Vec<u32>,
    pub unk_0xe50186a0: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7a19656 {
    pub detail_panel: u32,
    pub detail_text_t1: u32,
    pub detail_text_t2: u32,
    pub timer_panel: u32,
    pub timer_text: u32,
    pub unk_0x6188e7b7: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7a1a2d27 {
    pub absorbed_damage_format: u32,
    pub combinable_damage_format: u32,
    pub critical_magical_damage_format: u32,
    pub critical_physical_damage_format: u32,
    pub critical_true_damage_format: u32,
    pub default_magical_damage_format: u32,
    pub default_physical_damage_format: u32,
    pub default_true_damage_format: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7ac9c856 {
    pub name: u32,
    pub quality_setting: u32,
    pub unk_0x249f7223: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7aca1f4d {
    pub unk_0x2d96f463: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7acf50f9 {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7b40445f {
    pub is_optional: Option<bool>,
    pub participant_champion: u32,
    pub quest_objective_list: Vec<CharacterQuestObjective>,
    pub tooltip: String,
    pub unk_0xa62e6d72: Vec<CharacterInitRequirement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7cde3150 {
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7dbb4a4d {
    pub unk_0x815f6e2: Option<u8>,
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7dc4f3ec {
    pub player_slot: u32,
    pub player_slot_icon: u32,
    pub player_slot_icon_frame: u32,
    pub player_slot_shroud: u32,
    pub player_slot_splash: u32,
    pub player_slot_title_data: LoadingScreenSummonerTitleData,
    pub sub_team_button: u32,
    pub sub_team_logo: u32,
    pub sub_team_name_hovered: u32,
    pub sub_team_name_inactive: u32,
    pub sub_team_name_pressed: u32,
    pub sub_team_name_selected: u32,
    pub sub_team_player_row: u32,
    pub subteam: u32,
    pub subteam_column: u32,
    pub unk_0x18620678: u32,
    pub unk_0x3fd5018c: u32,
    pub unk_0x65aaa935: u32,
    pub unk_0x75ef08f0: u32,
    pub unk_0x9d8c63d0: u32,
    pub unk_0xb2d1e4ec: u32,
    pub unk_0xc030cc99: u32,
    pub unk_0xc9d18858: u32,
    pub unk_0xd1dbeee9: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7dcd3672 {
    pub table: EnumVars,
    pub var: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7e0ad0f9 {
    pub base_loadable: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub unk_0x356e8ea0: u32,
    pub unk_0x4fa7c67f: Unk0xe35f9399,
    pub unk_0x60cce187: f32,
    pub unk_0x8862947d: Unk0x8df505ab,
    pub unk_0x88dfbb7a: Vec<Unk0xf85d936e>,
    pub unk_0x8c7172: Unk0xe35f9399,
    pub unk_0x9f2f4bdb: HashMap<i8, u32>,
    pub unk_0xa3e06fe2: HashMap<i8, u32>,
    pub unk_0xb0177e69: Vec<Unk0xf85d936e>,
    pub unk_0xcee650d6: Unk0xe35f9399,
    pub unk_0xf0464464: Unk0x8df505ab,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7ebc692d {
    pub unk_0x7f5a3f02: Option<f32>,
    pub unk_0xb99eae1d: bool,
    pub unk_0xbdb8cd08: Option<f32>,
    pub unk_0xcb49a225: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7efceaff {
    pub background_icon: u32,
    pub button: u32,
    pub trait_icon: u32,
    pub unk_0xbf8afda0: HashMap<u8, u32>,
    pub unk_0xca62500c: u32,
    pub unk_0xd1f18af8: u32,
    pub unk_0xf7674ec0: HashMap<u8, u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7f644206 {
    pub reroll_button: u32,
    pub reroll_button_text: u32,
    pub reroll_button_text_color: [u8; 4],
    pub reroll_button_text_disabled_color: [u8; 4],
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7f796784 {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7faa90a0 {
    pub character_record: String,
    pub idle_animation_name: String,
    pub play_idle_animation: Option<bool>,
    pub skin: String,
    pub team: Option<u32>,
    pub unk_0x86f3c70: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x7fb92f53 {
    pub unk_0x28de30d6: f32,
    pub unk_0x3c475337: f32,
    pub unk_0xc865acd9: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8069a433 {
    pub unk_0x5ff6cd50: Vec<EnumUnk0xa8b35a0d>,
    pub unk_0x9165a7c4: Option<u8>,
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x80929fe {
    pub spell_slot: Option<Unk0x5f011348>,
    pub unk_0x3a3c1fee: Option<u32>,
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x810de4e7 {
    pub end: SequenceTiming,
    pub start: SequenceTiming,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x81580a34 {
    pub filter: EnumOptionItemFilter,
    pub template: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8190bc9f {
    pub currency_template: u32,
    pub item_template: u32,
    pub legendary_vfx: u32,
    pub parent: u32,
    pub quantity_text: u32,
    pub unk_0xea4b69c5: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x82bdb583 {
    pub unk_0xf1bc6341: HashMap<u32, Unk0x92c8c778>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x82cab1b3 {
    pub lane: u16,
    pub position: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x82ece567 {
    pub item_vfx: u32,
    pub parent: u32,
    pub unk_0xc27d6478: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x834e2e8d {
    pub error: u32,
    pub group: u32,
    pub large_square: Vec<UiRotationalStoreItemTileData>,
    pub long: Vec<UiRotationalStoreItemTileData>,
    pub region: u32,
    pub single: Vec<UiRotationalStoreItemTileData>,
    pub tall: Vec<UiRotationalStoreItemTileData>,
    pub timer: TftEventTimer,
    pub title: u32,
    pub unk_0xbbfd8261: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8444401a {
    pub layout: u32,
    pub matchup_slot: Unk0x135a0579,
    pub scene: u32,
    pub unk_0x3bee2696: u32,
    pub unk_0x6daf08ed: String,
    pub unk_0x91bad98c: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x84c56837 {
    pub empty: Vec<u32>,
    pub loss: Vec<u32>,
    pub win: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x851b8dc5 {
    pub is_disabled: Option<bool>,
    pub unk_0xd851ffa3: String,
    pub var: Box<EnumAddLevelTimer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x851d2116 {
    pub table: Option<EnumVars>,
    pub var: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x858de500 {
    pub path_hash_to_self: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x85a6a05c {
    pub unk_0xffeb3531: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x85bf79cf {
    pub unk_0x7aa36d0: u32,
    pub unk_0xc5b58261: HashMap<u32, u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x886b77ed {
    pub description: String,
    pub unit_property: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8873e4c8 {
    pub character_name_override: Option<String>,
    pub character_record: String,
    pub lua_script: Option<String>,
    pub skins: Option<HashMap<u32, Unk0xb979c2f>>,
    pub unk_0x397fe037: Option<bool>,
    pub unk_0xbd69e44b: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8948ed01 {
    pub active_background: u32,
    pub description: u32,
    pub group_link: u32,
    pub icon: u32,
    pub title: u32,
    pub unk_0x9a800d92: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8958fee2 {
    pub unk_0x79a2e7aa: Option<f32>,
    pub unk_0xffcbd9e2: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x895afb93 {
    pub basic_attack_icon_texture_name: String,
    pub item_damage_icon_texture_name: String,
    pub rune_damage_icon_texture_name: String,
    pub unknown_damage_icon_texture_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x89fa197c {
    pub tooltips: Vec<String>,
    pub unk_0x2f6cd7a5: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8a96ea3c {
    pub m_subparts: Vec<Box<EnumAbilityResourceByCoefficientCalculationPart>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8ad25772 {
    pub system: u32,
    pub unk_0x63176011: Option<bool>,
    pub unk_0x86f3c70: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8b04b4cb {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8b232009 {
    pub button: u32,
    pub header_key: String,
    pub tab: Option<u32>,
    pub unk_0xe0ee1744: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8beb0550 {
    pub default_value: f32,
    pub unk_0xb9562e5b: HashMap<u32, f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8c31df6a {
    pub active_banner_parent: u32,
    pub banner_background: u32,
    pub banner_list: u32,
    pub banner_timer: TftEventTimer,
    pub base_loadable: u32,
    pub craft_button: u32,
    pub craft_subtitle: u32,
    pub craft_text: u32,
    pub craft_text_clicked: u32,
    pub craft_text_disable: u32,
    pub failure_icon: u32,
    pub failure_subtitle_tra_key: String,
    pub failure_text: u32,
    pub failure_title_tra_key: String,
    pub milestone_rewards_template: Unk0x93e412e0,
    pub milestone_tooltip_button: u32,
    pub milestone_tooltip_icon: u32,
    pub more_info_button: u32,
    pub no_active_banners_icon: u32,
    pub no_content_subtitle_tra_key: String,
    pub no_content_title_tra_key: String,
    pub owned_trakey: String,
    pub path_hash_to_self: u64,
    pub redeem_trakey: String,
    pub roll10_button: u32,
    pub roll10_text: u32,
    pub roll10_text_clicked: u32,
    pub roll10_text_disable: u32,
    pub roll1_button: u32,
    pub roll1_text: u32,
    pub roll1_text_clicked: u32,
    pub roll1_text_disable: u32,
    pub scene: u32,
    pub tablet_override_loadable: u32,
    pub tft_banner_icon_data: TftBannerIconData,
    pub title_text: u32,
    pub unk_0x13dbfb76: u32,
    pub unk_0x18f5bf4: u32,
    pub unk_0x1c9cb0d7: u32,
    pub unk_0x1cd13dc6: u32,
    pub unk_0x1e3e07ee: f32,
    pub unk_0x1f818d49: u32,
    pub unk_0x20f6463d: u32,
    pub unk_0x22a4ad45: u32,
    pub unk_0x27fe4326: u32,
    pub unk_0x2eed7e1b: u32,
    pub unk_0x3747115a: u32,
    pub unk_0x387b3b43: u32,
    pub unk_0x3966bee3: u32,
    pub unk_0x3aecdaf0: u32,
    pub unk_0x4076de6a: u32,
    pub unk_0x40810d53: u32,
    pub unk_0x423baeda: u32,
    pub unk_0x50db72a2: u32,
    pub unk_0x5d5e69e5: u32,
    pub unk_0x5d864609: u32,
    pub unk_0x602e7f2a: u32,
    pub unk_0x64c2cb47: u32,
    pub unk_0x70fa4e8c: u32,
    pub unk_0x7d88be7d: u32,
    pub unk_0x86be36a3: u32,
    pub unk_0x8e2e1f5e: u32,
    pub unk_0x9ab63a0b: u32,
    pub unk_0x9f950eae: u32,
    pub unk_0xa0f010ba: u32,
    pub unk_0xa4419c92: HashMap<String, String>,
    pub unk_0xab22be6d: u32,
    pub unk_0xbc03fb23: String,
    pub unk_0xc0755406: u32,
    pub unk_0xc0e6b9de: u32,
    pub unk_0xc3193cb0: u32,
    pub unk_0xcaebb3c7: u32,
    pub unk_0xe00d8f17: String,
    pub unk_0xec61c212: u32,
    pub unk_0xeceb0f26: String,
    pub unk_0xfa36b211: u32,
    pub unk_0xfbf302db: Unk0x41bdce89,
    pub unk_0xfbff5912: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8d8b1535 {
    pub icon_frame: u32,
    pub unk_0x3392c041: String,
    pub unk_0x58766309: String,
    pub unk_0x720e4297: u32,
    pub unk_0x96349f57: String,
    pub unk_0xa807a4b2: String,
    pub unk_0xd0666cc4: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8df505ab {
    pub team_icon: u32,
    pub unk_0x4c07c447: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8e11ad0c {}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8e31f800 {
    pub base_loadable: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub unk_0x2e5727c8: Option<f32>,
    pub unk_0x7a721ae7: String,
    pub unk_0x9b8bea73: u32,
    pub unk_0xa8ea735a: f32,
    pub unk_0xe5d3d1fb: Unk0xd5d2b377,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8e65fb6b {
    pub text_format: String,
    pub value_driver: EnumDriver,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8f92cfab {
    pub end_date: String,
    pub foreground_image: TextureOverride,
    pub id: String,
    pub start_date: String,
    pub title_text_tra: String,
    pub url_tra: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x8ffd7c61 {
    pub elements: Vec<u32>,
    pub name: String,
    pub position: UiPositionRect,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9051620d {
    pub icon_data: Vec<Unk0x4a929a90>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x90f7282b {
    pub unk_0x116cb40d: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x92024c11 {
    pub max: Vec3,
    pub min: Vec3,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x92309121 {
    pub unk_0x17659fc6: Unk0x67bfa45e,
    pub unk_0x1e88b9d4: f32,
    pub unk_0x20fc97e7: u32,
    pub unk_0x2253423: bool,
    pub unk_0x30b50544: bool,
    pub unk_0x433be97f: f32,
    pub unk_0x4721facd: f32,
    pub unk_0x497e6121: bool,
    pub unk_0x4fb25d9f: bool,
    pub unk_0x51274048: f32,
    pub unk_0x52d1e5c3: f32,
    pub unk_0x65a68710: Option<f32>,
    pub unk_0x6df5dc15: bool,
    pub unk_0x70e43c7e: f32,
    pub unk_0x81998a00: f32,
    pub unk_0x83f73f14: f32,
    pub unk_0x8eb174a9: u32,
    pub unk_0x8fcc8a74: bool,
    pub unk_0x9c8d113e: Unk0x67bfa45e,
    pub unk_0xb3da60e0: Option<f32>,
    pub unk_0xbc9ba03a: f32,
    pub unk_0xd7396d73: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x92b6b7fa {
    pub max_value: IntTableGet,
    pub min_value: IntGet,
    pub output_table: CustomTableSet,
    pub unk_0xd851ffa3: String,
    pub unk_0xdec56440: Vec<IntTableGet>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x92c8c778 {
    pub font_size: u16,
    pub unk_0x2bf77ed0: Option<u8>,
    pub unk_0x51c7ac80: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x93adc5b3 {
    pub distance_between: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x93e412e0 {
    pub frame_available_icon: u32,
    pub frame_complete_icon: u32,
    pub group: u32,
    pub hit_region: u32,
    pub level_pip_active: u32,
    pub level_pip_claimed: u32,
    pub level_pip_default: u32,
    pub level_text: u32,
    pub managed_layout: u32,
    pub meter_complete_left: u32,
    pub meter_complete_right: u32,
    pub meter_holder_left: u32,
    pub meter_holder_right: u32,
    pub meter_holder_start: u32,
    pub milestone_rewards_icon: u32,
    pub milestone_rewards_icon_background: u32,
    pub quantity_text: u32,
    pub unk_0x4e5f0d62: [u8; 4],
    pub unk_0x77ea633d: [u8; 4],
    pub unk_0xcc3c5927: [u8; 4],
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9401f2e2 {
    pub base_loadable: u32,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub player_template: Unk0xa9593c80,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9506323a {
    pub ping_category: Option<u8>,
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x96260714 {
    pub unk_0x4b9489d8: Vec<u32>,
    pub unk_0xdb3512a6: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x968459a8 {
    pub base_loadable: u32,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub unk_0x3783e997: u32,
    pub unk_0x518dd41e: u32,
    pub unk_0x747d2301: Vec<u32>,
    pub unk_0xa16cf741: Unk0x3f040c8e,
    pub unk_0xa2101a11: u32,
    pub unk_0xcb232eb8: Unk0x3f040c8e,
    pub unk_0xea13124a: Unk0x3f040c8e,
    pub unk_0xf772f204: Vec<u32>,
    pub unk_0xfd597a5a: Unk0x3f040c8e,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9784901f {
    pub phase_hit_region: u32,
    pub phase_icon: u32,
    pub unk_0x2ea2ef00: Option<u32>,
    pub unk_0x39b2057a: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x990115ea {
    pub icon_frame: u32,
    pub unk_0x3392c041: String,
    pub unk_0x58766309: String,
    pub unk_0x720e4297: u32,
    pub unk_0x96349f57: String,
    pub unk_0xa807a4b2: String,
    pub unk_0xd0666cc4: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9925c3c2 {
    pub background: u32,
    pub hit_target: u32,
    pub tooltip_region: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9984b4a {
    pub bounds_element: u32,
    pub unk_0x40e1645d: u32,
    pub unk_0x60858432: u32,
    pub unk_0x73cb42f6: u32,
    pub unk_0xb02e568a: u32,
    pub unk_0xd3d87bbf: u32,
    pub unk_0xf0692ff1: u32,
    pub unk_0xf60f3af2: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x99c5a706 {
    pub description: String,
    pub title: String,
    pub unk_0x62e1b0aa: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9a4ba494 {
    pub button: u32,
    pub name: u32,
    pub portrait_icon: u32,
    pub progress_text: u32,
    pub scene: u32,
    pub unk_0x12cef16b: Unk0x2b365a82,
    pub unk_0x3c4b5c83: String,
    pub unk_0x44f732bc: String,
    pub unk_0x829c2960: HashMap<i32, String>,
    pub unk_0x98759e1a: u32,
    pub unk_0xaf128762: f32,
    pub unk_0xca62500c: u32,
    pub unk_0xfbe13398: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9a722730 {
    pub unk_0x146b3900: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9a7de981 {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9aa5b4bc {
    pub definition: Unk0x7faa90a0,
    pub name: u32,
    pub transform: Mat4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9ab8b8e6 {
    pub description: String,
    pub name: String,
    pub vfx: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9bc366ca {
    pub skin_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9be57ed9 {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9c1d99c0 {
    pub spells: Vec<u32>,
    pub unk_0x80cf3335: Unk0x7a1a2d27,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9c33070d {
    pub condition: Box<ComparisonScriptCondition>,
    pub sequence: Box<ScriptSequence>,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9c5b78dd {
    pub resource_resolver: Unk0x20194a16,
    pub unk_0xb4222185: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9c8a0477 {
    pub base_loadable: u32,
    pub frame: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub title: u32,
    pub unk_0x3c0154a1: Vec<Unk0x8b232009>,
    pub unk_0x66fb858d: u32,
    pub unk_0x8daff178: u32,
    pub unk_0xb7be8b79: Unk0x6db33e0c,
    pub unk_0xbc2ce5f6: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9cd4ec31 {
    pub unk_0xfed597b0: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9d62f7e {
    pub named_data_value: u32,
    pub spell: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9d9f60d2 {
    pub character_record: String,
    pub r#type: Option<u16>,
    pub skin: String,
    pub tags: Option<Vec<u32>>,
    pub team: Option<u32>,
    pub unk_0x33c6fd60: Option<Unk0x313c0076>,
    pub unk_0x397fe037: Option<bool>,
    pub unk_0xdbde2288: Option<Vec<Unk0x82cab1b3>>,
    pub unk_0xde46f1d8: Option<String>,
    pub unk_0xf1d3a034: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9e1e8775 {
    pub text_element: u32,
    pub text_format: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9e9e2e5c {
    pub source_object: u32,
    pub unk_0x137cf12a: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9eba3f83 {
    pub duration_driver: FloatLiteralMaterialDriver,
    pub easing: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9ef1e737 {
    pub filter: OptionItemFilterGameStyle,
    pub template: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9efb6a51 {
    pub button: u32,
    pub group: u32,
    pub icon: u32,
    pub level: u32,
    pub region: u32,
    pub title: u32,
    pub unk_0xd02a6781: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0x9f30b8a6 {
    pub assist_duration_override: f32,
    pub base_gold: Vec<f32>,
    pub first_blood_bonus: Option<f32>,
    pub negative_bounty_config: Option<Unk0x24e1cb45>,
    pub objective_bounty_config: Option<EnumUnk0x6030f7c6>,
    pub unk_0x11f10a40: f32,
    pub unk_0x1eacb90a: f32,
    pub unk_0x3cf326fa: Option<bool>,
    pub unk_0x40b0d0ff: Option<bool>,
    pub unk_0x4e0d7b75: Option<bool>,
    pub unk_0x54ccd262: f32,
    pub unk_0x7c29d4b9: u32,
    pub unk_0x8c1b120: f32,
    pub unk_0x907442e7: f32,
    pub unk_0x937cc95a: f32,
    pub unk_0xa7e2cc37: Option<bool>,
    pub unk_0xa966473c: Option<Unk0xb53e2f1a>,
    pub unk_0xc29d06b9: f32,
    pub unk_0xce073fe8: f32,
    pub unk_0xd1f9fb6a: Option<f32>,
    pub unk_0xec211346: f32,
    pub unk_0xfa93507d: f32,
    pub unk_0xfd43d59f: f32,
    pub unk_0xfe1b406e: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xa21d6491 {
    pub frame: u32,
    pub scene: u32,
    pub unk_0xc8140e7: Vec<Unk0x11a27f9e>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xa3129b41 {
    pub padding: u32,
    pub region: u32,
    pub unit_group_link: u32,
    pub unit_icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xa495afda {
    pub recipient_champion: u32,
    pub spell: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xa50ab26 {
    pub owner_champion: u32,
    pub tooltip: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xa534365c {
    pub group: u32,
    pub timer_vfx: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xa6e992c8 {
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xa8b35a0d {
    pub unk_0xfd36c650: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xa8c6f5f0 {
    pub unk_0x1793d323: u32,
    pub unk_0x4297f4f9: u32,
    pub unk_0x5329572e: u32,
    pub unk_0xb80015ba: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xa9253585 {
    pub header_group: u32,
    pub layout_group: u32,
    pub unk_0x241c258b: [u8; 4],
    pub unk_0x3c4b5c83: String,
    pub unk_0x3ddb00: f32,
    pub unk_0x44f732bc: String,
    pub unk_0x81d55cb7: u32,
    pub unk_0x88ee5bd9: Unk0x63fbd3de,
    pub unk_0x8a73adc8: [u8; 4],
    pub unk_0x9d8818ba: [u8; 4],
    pub unk_0xb80d6251: [u8; 4],
    pub unk_0xbec348d3: u32,
    pub unk_0xf3c1ea09: [u8; 4],
    pub unk_0xfbe13398: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xa9593c80 {
    pub player_scene: u32,
    pub unk_0x19d88086: String,
    pub unk_0x4cd9da85: String,
    pub unk_0x8aea4585: String,
    pub unk_0x90599dbe: u32,
    pub unk_0x94a951ce: u32,
    pub unk_0x9f56fb21: u32,
    pub unk_0xa7eb3089: f32,
    pub unk_0xa806eda2: u32,
    pub unk_0xab0328f6: String,
    pub unk_0xc598d1cd: Unk0x313da04c,
    pub unk_0xf2f97ffc: u32,
    pub unk_0xfbb04bd5: String,
    pub unk_0xfc4f6c98: Unk0x313da04c,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xa9d60c77 {
    pub column1_label_tra_key: String,
    pub filter: EnumOptionItemFilter,
    pub unk_0x3e58e16: u32,
    pub unk_0xd7150c53: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xab02008c {
    pub source: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xab066fe {
    pub base_loadable: u32,
    pub mobile_override: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub unk_0x369fbcaa: Unk0x6f44928c,
    pub unk_0x395f0ee5: u32,
    pub unk_0x765d6f7: u32,
    pub unk_0x8c333492: u32,
    pub unk_0x8fde6981: u32,
    pub unk_0xc0f9389a: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xab2fec44 {
    pub unk_0xfd36c650: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xab66439a {
    pub base_loadable: u32,
    pub leave_button: u32,
    pub main_scene: u32,
    pub path_hash_to_self: u64,
    pub unk_0x6b4d6c6b: Unk0xf86c7aa6,
    pub unk_0x8ead4a1a: HashMap<u8, u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xac27b13a {
    pub groups: Vec<HudItemShopItemGroupDefinition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xacb2dba1 {
    pub region: u32,
    pub unk_0xba819369: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xad65d8c4 {
    pub definition: Unk0x9d9f60d2,
    pub name: u32,
    pub transform: Option<Mat4>,
    pub unk_0xbbe68da1: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xad9f1783 {
    pub unk_0x13927cb2: u32,
    pub unk_0x2fe884a1: u32,
    pub unk_0x6a5b6251: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xadfcc498 {
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xae56d8c4 {
    pub unk_0xf248d989: u32,
    pub unk_0xf92d1cec: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xaf6acce9 {
    pub error: u32,
    pub group: u32,
    pub large_square: UiRotationalStoreItemTileData,
    pub long: UiRotationalStoreItemTileData,
    pub region: u32,
    pub single: UiRotationalStoreItemTileData,
    pub tall: UiRotationalStoreItemTileData,
    pub timer: TftEventTimer,
    pub title: u32,
    pub unk_0x163b66d4: u32,
    pub unk_0x28fbbad9: u32,
    pub unk_0x4be28a06: u32,
    pub unk_0x57d57c1: u32,
    pub unk_0x6eb6c46: u32,
    pub unk_0x84daa05a: u32,
    pub unk_0x88b23162: u32,
    pub unk_0x9371d126: u32,
    pub unk_0xafa28b02: u32,
    pub unk_0xb88c7538: u32,
    pub unk_0xbbfd8261: String,
    pub unk_0xf54d1c3a: u32,
    pub unk_0xfa1a27e6: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb011f563 {
    pub button_element: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb09016f6 {
    pub effect_calculation: GameCalculation,
    pub effect_tag: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb0afae41 {
    pub tft_game_type: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb107dfe4 {
    pub unk_0x8e39efb3: u32,
    pub unk_0xd02a6781: u32,
    pub unk_0xeb3bae8f: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb1edf879 {
    pub scene: u32,
    pub spectate_button: u32,
    pub unk_0x3e23d344: u32,
    pub unk_0xbfd6c710: u32,
    pub unk_0xcebabe5e: u32,
    pub unk_0xd51ffbe2: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb213e4ff {
    pub disable_on_end: Option<bool>,
    pub disabled: Option<bool>,
    pub element: Unk0x38d7429,
    pub enable_on_start: Option<bool>,
    pub end: SequenceTiming,
    pub priority: Option<i8>,
    pub start: Option<SequenceTiming>,
    pub unk_0x25fdb38f: Unk0xd31bbf89,
    pub unk_0x457338e5: Option<bool>,
    pub unk_0x76df2a8b: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb22609db {
    pub unk_0x91d404a5: u32,
    pub unk_0xb2cd0eb0: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb2430347 {
    pub background_texture: u32,
    pub managed_layout: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb26bd951 {
    pub units: HashMap<u32, EnumUnk0x8873e4c8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb2fe8cea {
    pub base_loadable: u32,
    pub header_text_template: u32,
    pub item_scene_template: u32,
    pub message_template: QuestTrackerMessageTemplate,
    pub path_hash_to_self: u64,
    pub scene_template: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb3674a86 {
    pub ui_element: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb4517220 {
    pub active: u32,
    pub complete: u32,
    pub inactive: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb5215699 {
    pub color_easing: u8,
    pub colors: Vec<Unk0xd1b1f16>,
    pub radial_element: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb53e2f1a {
    pub unk_0x490b09: f32,
    pub unk_0x4b733ea3: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb5d9d3a0 {
    pub name: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb5f38c13 {
    pub spell_slot: Option<Unk0x5f011348>,
    pub unk_0x3a3c1fee: Option<u32>,
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb602063b {
    pub filter: Unk0xec0f1b18,
    pub label_tra_key: String,
    pub show_on_platform: u8,
    pub template: u32,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb609aae3 {
    pub effect: u32,
    pub unk_0x8f6df5b2: HudMenuTransitionData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb62c8675 {
    pub base_loadable: u32,
    pub crown_icons: Unk0xa8c6f5f0,
    pub details_panel: Unk0x7a19656,
    pub device_ux: i32,
    pub meters_panel: Unk0xf43ad1ce,
    pub scene: u32,
    pub soraka_icons: Unk0xa8c6f5f0,
    pub tower_icons: Unk0xa8c6f5f0,
    pub unk_0x462800b7: Unk0xa8c6f5f0,
    pub unk_0xb057cf4b: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb6da23cb {
    pub unk_0x63508947: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb7b2875 {
    pub base_loadable: u32,
    pub close_button: u32,
    pub path_hash_to_self: u64,
    pub reward_layout: u32,
    pub scene_root: u32,
    pub tablet_override_loadable: u32,
    pub unk_0x2eed7e1b: u32,
    pub unk_0x304f33f0: String,
    pub unk_0xcbfa2678: Unk0x7638f87c,
    pub unk_0xd1373982: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb7b43e1d {
    pub bool_driver: IsAnimationPlayingDynamicMaterialBoolDriver,
    pub percentage: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb8a49c96 {
    pub blue_skin: u32,
    pub meter: u32,
    pub red_skin: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb95142cd {
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xb979c2f {
    pub skin: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xba007871 {
    pub source_object: u32,
    pub unk_0x3de6ce2d: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xba138ae3 {
    pub definition: Unk0xfde6a2d7,
    pub name: u32,
    pub transform: Mat4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xba9f6aca {
    pub failure_text: String,
    pub failure_texture_path: String,
    pub trove_banner_icon: u32,
    pub trove_button: u32,
    pub unk_0x439b26dc: String,
    pub unk_0xabd0de07: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xbaf9ac75 {
    pub unk_0xb490894c: Vec<u32>,
    pub unk_0xf128c9be: HashMap<u32, Unk0x3b2ba6c0>,
    pub unk_0xfab10bb2: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xbafc3e15 {
    pub spell: u32,
    pub spell_level: Box<EnumDriver>,
    pub value_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xbb56e8ed {
    pub unk_0xb08b3c5f: HashMap<u8, u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xbb778e9b {
    pub display_name: String,
    pub style: Option<u8>,
    pub unk_0x75000be4: String,
    pub weight: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xbcbc4216 {
    pub base_loadable: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub unk_0x36ba7fe6: HashMap<String, Unk0x143f10a8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xbd659e9b {
    pub data: Unk0xbef9baa7,
    pub key: Unk0x6055e037,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xbe081d2c {
    pub timer_default_fill: u32,
    pub unk_0x81cbec5f: u32,
    pub unk_0x9ba9ed34: u32,
    pub unk_0xbcde5149: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xbef9baa7 {
    pub value: HashMap<u8, u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xbf431a5a {
    pub base_loadable: u32,
    pub confirm_button: u32,
    pub countdown_meter: u32,
    pub grid: u32,
    pub max_num_choices: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub spell_choice_template: SpellChoiceTemplate,
    pub unk_0xb39d177e: String,
    pub unk_0xd5038016: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xbf5c4715 {
    pub unk_0x16ea3d9: Unk0x1493959a,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc074ba45 {
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc174810b {
    pub unk_0x63e5d2ff: Option<u16>,
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc2afdb3d {
    pub icon: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc2b1af7f {
    pub disable_on_end: Option<bool>,
    pub disabled: Option<bool>,
    pub element: EnumUnk0x37841b56,
    pub end: Option<SequenceTiming>,
    pub start: Option<SequenceTiming>,
    pub unk_0x76df2a8b: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc3737f3e {
    pub background_assets: Unk0x131f5725,
    pub button: u32,
    pub foreground_assets: Unk0x131f5725,
    pub title_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc3a44766 {
    pub unk_0x22728a51: f32,
    pub unk_0x44735f30: String,
    pub unk_0x95823356: f32,
    pub unk_0xa86fc2ef: f32,
    pub unk_0xeb6c72dd: String,
    pub unk_0xed23ad91: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc3f95838 {
    pub button: u32,
    pub stat_page_view_controller: u32,
    pub unk_0xcfb90a94: Option<HashMap<u32, Unk0x1e2d1428>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc406a533 {
    pub default_visible: bool,
    pub name: u32,
    pub path_hash: u32,
    pub unk_0x27639032: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc565e640 {
    pub unk_0x207e8859: Option<bool>,
    pub unk_0x2399f883: EnumUnk0x6256517d,
    pub unk_0x288baac2: bool,
    pub unk_0x42bc76e8: Option<u8>,
    pub unk_0x5793fe9f: Unk0x7ebc692d,
    pub unk_0x5f4ed4a5: f32,
    pub unk_0x61366c38: bool,
    pub unk_0x87a84ec7: bool,
    pub unk_0x89214322: Option<bool>,
    pub unk_0x94625153: Option<bool>,
    pub unk_0xc63d17df: Option<f32>,
    pub unk_0xc77cbba0: Option<Unk0x16e6e17a>,
    pub unk_0xd429a631: Option<f32>,
    pub unk_0xd4521cf1: bool,
    pub unk_0xda429209: Option<f32>,
    pub unk_0xdd7d7c58: Option<bool>,
    pub unk_0xe538859e: f32,
    pub unk_0xf1381710: Option<bool>,
    pub unk_0xf9995077: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc5e719b6 {
    pub unk_0x7cf5e3eb: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc62b91e4 {
    pub unk_0x3aaecc43: u32,
    pub unk_0x896acc29: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc73631da {
    pub unk_0x127c55f0: u32,
    pub unk_0x23908b24: u32,
    pub unk_0x41752f5a: u32,
    pub unk_0x583f66ba: String,
    pub unk_0xa51a4d37: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc75640aa {
    pub unk_0x9bcbd1a9: Option<u8>,
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc76c1b9a {
    pub modifiers: Vec<EnumUnk0x51445de9>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc77834ec {
    pub base_loadable: u32,
    pub defeat_group_handle: u32,
    pub game_over_scene: u32,
    pub leave_button: u32,
    pub path_hash_to_self: u64,
    pub unk_0x6d28c8a1: u32,
    pub victory_group_handle: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc7b1ec51 {
    pub unk_0x5ff6cd50: Vec<EnumUnk0xa8b35a0d>,
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc7ca4925 {
    pub frame_ally: u32,
    pub frame_enemy: u32,
    pub frame_self: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc7e628b9 {
    pub spell: u32,
    pub unk_0x877e4953: u32,
    pub unk_0xa2877ddb: u32,
    pub unk_0xd00e123a: u32,
    pub unk_0xe5bc4229: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc8400f38 {
    pub tra_key: String,
    pub unk_0x2ec9dee5: Option<i32>,
    pub unk_0x5ff6cd50: Vec<EnumUnk0x49401c5c>,
    pub unk_0x6bfb316f: String,
    pub unk_0x82d6dcb0: Vec<EnumUnk0x2e181f68>,
    pub unk_0x89ba4a26: u32,
    pub unk_0xe77fefb: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc8879e6c {
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc88d8b75 {
    pub unk_0xc201e: HashMap<u8, u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc8c47da {
    pub button: u32,
    pub frame_icon: u32,
    pub portrait_icon: u32,
    pub unk_0xca62500c: u32,
    pub unk_0xedd2d807: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc8d6dccd {
    pub unk_0x1d6318d: Option<u32>,
    pub unk_0x5ff6cd50: Option<Vec<Unk0x2152c2a1>>,
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc964f53a {
    pub base_loadable: u32,
    pub clear_button: u32,
    pub close_button: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub tablet_override_loadable: Option<u32>,
    pub unk_0x6e40a6f6: [u8; 4],
    pub unk_0x71dbfe7a: [u8; 4],
    pub unk_0x791fec87: Unk0xdbe05e33,
    pub unk_0x832f1b77: [u8; 4],
    pub unk_0xeecf7312: [u8; 4],
    pub unk_0xfc0c2359: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xc96d9140 {
    pub unk_0x1418c47f: u32,
    pub unk_0xa2cb8e03: Option<HashMap<String, u32>>,
    pub unk_0xc19c58be: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xcaefc854 {
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xcc2c0827 {
    pub base_hover_glow: u32,
    pub unk_0xd3467f57: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xcc35f742 {
    pub denominator: Box<Unk0xbafc3e15>,
    pub value: Box<EnumDriver>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xcc6491d4 {
    pub back_button: u32,
    pub base_loadable: u32,
    pub clear_all_button: u32,
    pub close_button: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub snapshot_button: u32,
    pub tablet_override_loadable: Option<u32>,
    pub team_name_text: u32,
    pub unk_0x13a3c8aa: f32,
    pub unk_0x1733060c: u32,
    pub unk_0x18a6e0b1: String,
    pub unk_0x299d1eee: u32,
    pub unk_0x2eed7e1b: u32,
    pub unk_0x2f144da6: String,
    pub unk_0x36155e8c: u32,
    pub unk_0x38a49e67: String,
    pub unk_0x397d10d8: u32,
    pub unk_0x4d5e815f: Option<String>,
    pub unk_0x546f17ba: u32,
    pub unk_0x63356fc5: u32,
    pub unk_0x7788d034: String,
    pub unk_0x7993a621: u32,
    pub unk_0xa579b2bc: u32,
    pub unk_0xb172e3c7: u32,
    pub unk_0xc096e7d3: u32,
    pub unk_0xc0f4b3bf: u32,
    pub unk_0xc973af15: Option<u32>,
    pub unk_0xcc043494: u32,
    pub unk_0xcf948d45: Unk0xd88ce199,
    pub unk_0xe588b2b2: String,
    pub unk_0xe9d16d12: Option<u32>,
    pub unk_0xf9d5a86b: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xcd54aabc {
    pub icons: Vec<Unk0x69680d>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xcd5a34f5 {
    pub animation_name: Option<String>,
    pub mesh_name: Option<String>,
    pub mesh_scale: Option<f32>,
    pub skeleton_name: Option<String>,
    pub submeshes: Option<Vec<u32>>,
    pub use_surface_normal_for_birth_physics: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xcdb1c8f6 {
    pub unk_0x6355dd6f: Vec<Unk0x6355dd6f>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xcdf661db {
    pub category: String,
    pub unk_0x2de18da: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xce762bab {
    pub unk_0x45258bde: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xcf47a9a3 {
    pub cost_text: u32,
    pub trait_icon: u32,
    pub unk_0x4f0e44f5: u32,
    pub unk_0x7bef87dc: u32,
    pub unk_0xca62500c: u32,
    pub unk_0xeaefb0c9: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xcf4a55da {
    pub overlays: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd07fc429 {
    pub unk_0x2fd3d3c2: Option<Unk0xd31bbf89>,
    pub unk_0x34e801f5: String,
    pub unk_0x37c0686c: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd178749c {
    pub definition: Unk0x429a2180,
    pub name: u32,
    pub transform: Mat4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd19d72ee {
    pub description_text: u32,
    pub progress_complete_icon: u32,
    pub progress_text: u32,
    pub reward_icon: u32,
    pub reward_text: u32,
    pub title_text: u32,
    pub unk_0xedc62dad: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd1b1f16 {
    pub color: SwitchMaterialDriver,
    pub starting_value: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd2b529c {
    pub health_bar: u32,
    pub scene: u32,
    pub unk_0x25acb4ca: u32,
    pub unk_0x3088251: u32,
    pub unk_0x37c8b4fa: u32,
    pub unk_0x471d14f0: u32,
    pub unk_0x54f575dd: u32,
    pub unk_0x75dbb937: u32,
    pub unk_0x91cef9f4: u32,
    pub unk_0xda7fe7a3: u32,
    pub unk_0xed0d075f: u32,
    pub unk_0xf24af8ac: u32,
    pub unk_0xff86fe59: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd31bbf89 {
    pub animation_name: Option<String>,
    pub r#loop: Option<bool>,
    pub speed_scale: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd35c6ec8 {
    pub active_banner_parent: u32,
    pub banner_background: u32,
    pub banner_list: u32,
    pub banner_timer: TftEventTimer,
    pub base_loadable: u32,
    pub craft_button: u32,
    pub craft_subtitle: u32,
    pub craft_text: u32,
    pub craft_text_clicked: u32,
    pub craft_text_disable: u32,
    pub failure_icon: u32,
    pub failure_subtitle_tra_key: String,
    pub failure_text: u32,
    pub failure_title_tra_key: String,
    pub milestone_rewards_template: Unk0x93e412e0,
    pub milestone_tooltip_button: u32,
    pub milestone_tooltip_icon: u32,
    pub more_info_button: u32,
    pub no_active_banners_icon: u32,
    pub no_content_subtitle_tra_key: String,
    pub no_content_title_tra_key: String,
    pub owned_trakey: String,
    pub path_hash_to_self: u64,
    pub redeem_trakey: String,
    pub roll10_button: u32,
    pub roll10_text: u32,
    pub roll10_text_clicked: u32,
    pub roll10_text_disable: u32,
    pub roll1_button: u32,
    pub roll1_text: u32,
    pub roll1_text_clicked: u32,
    pub roll1_text_disable: u32,
    pub scene: u32,
    pub tablet_override_loadable: u32,
    pub tft_banner_icon_data: TftBannerIconData,
    pub title_text: u32,
    pub unk_0x1316649a: u32,
    pub unk_0x18f5bf4: u32,
    pub unk_0x1cd13dc6: u32,
    pub unk_0x1e3e07ee: f32,
    pub unk_0x1e76c3bf: u32,
    pub unk_0x2d6b78e2: u32,
    pub unk_0x2eed7e1b: u32,
    pub unk_0x387b3b43: u32,
    pub unk_0x3aecdaf0: u32,
    pub unk_0x40810d53: u32,
    pub unk_0x6389cd6f: u32,
    pub unk_0x64c2cb47: u32,
    pub unk_0x67468dbe: u32,
    pub unk_0x70fa4e8c: u32,
    pub unk_0x7d88be7d: u32,
    pub unk_0x8e2e1f5e: u32,
    pub unk_0xa0f010ba: u32,
    pub unk_0xa2127147: u32,
    pub unk_0xa4419c92: HashMap<String, String>,
    pub unk_0xab22be6d: u32,
    pub unk_0xbc03fb23: String,
    pub unk_0xcaebb3c7: u32,
    pub unk_0xce25ab39: u32,
    pub unk_0xe00d8f17: String,
    pub unk_0xea254026: u32,
    pub unk_0xeceb0f26: String,
    pub unk_0xfa36b211: u32,
    pub unk_0xfbf302db: Unk0x41bdce89,
    pub unk_0xfbff5912: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd37b1519 {
    pub group: u32,
    pub icon: u32,
    pub text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd3eab8e5 {
    pub scene_view_pane: u32,
    pub unk_0x190ed0b9: Vec<Unk0x190ed0b9>,
    pub unk_0x42dcedbc: Unk0xa534365c,
    pub unk_0xa81cb29a: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd4ac6ed4 {
    pub fill_direction: Option<u8>,
    pub unk_0x9d636613: u16,
    pub unk_0xd65a8b5c: u8,
    pub unk_0xf359eded: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd56fb9cc {
    pub texture_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd5a52c2b {
    pub base_loadable: u32,
    pub leave_button: u32,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub unk_0x548c43a4: u32,
    pub unk_0x87616d16: u32,
    pub unk_0x8d61d6d3: u32,
    pub unk_0xa2d26cd7: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd5c5318a {
    pub key: String,
    pub value_driver: Unk0x8e65fb6b,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd5c9eb1 {
    pub event_name: u32,
    pub unk_0x1004c9c8: HashMap<u32, Unk0x56bb851>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd5d2b377 {
    pub group: u32,
    pub subtitle: u32,
    pub title: u32,
    pub unk_0x19147e06: u32,
    pub unk_0x2fd3d3c2: u32,
    pub unk_0xa51a4d37: u32,
    pub unk_0xf9cb518f: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd65315ee {
    pub unk_0xdf085b93: Option<HashMap<u32, EnumGameModeConstant>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd65c937c {
    pub active_default: String,
    pub active_hover: String,
    pub active_selected: String,
    pub complete_default: String,
    pub complete_hover: String,
    pub complete_selected: String,
    pub hit_region: u32,
    pub inactive_default: String,
    pub inactive_hover: String,
    pub inactive_selected: String,
    pub tracker_group: u32,
    pub tracker_icon: u32,
    pub tracker_layout: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd75bce6a {
    pub layer: u32,
    pub name: String,
    pub position: UiPositionRect,
    pub scene: u32,
    pub texture_data: LooseUiTextureData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd7ec4ad6 {
    pub frame_icon: u32,
    pub portrait_icon: u32,
    pub unk_0x12cef16b: Unk0x2b365a82,
    pub unk_0xca62500c: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd82714cc {
    pub color: Option<[u8; 4]>,
    pub flags: Option<u16>,
    pub name: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd86db9b6 {
    pub character_record: String,
    pub skins: HashMap<u32, Unk0xb979c2f>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd88ce199 {
    pub champion_template: Unk0x5f33c34,
    pub unk_0x2414834f: Unk0xcf47a9a3,
    pub unk_0x7bf984ca: Unk0x3b6f488e,
    pub unk_0xece283b: Unk0x7efceaff,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd8a0b4e6 {
    pub base_loadable: u32,
    pub option_display_data: Unk0xd8a0e424,
    pub option_region: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub unk_0x9fb37b48: String,
    pub unk_0xafafe728: Unk0xf9aa519e,
    pub unk_0xb818e45a: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd8a0e424 {
    pub button: u32,
    pub button_icon: u32,
    pub group: u32,
    pub subtitle: u32,
    pub title: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xd91a223 {
    pub unk_0x68309c0b: bool,
    pub unk_0xe2e5b6dd: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xda5b233f {
    pub unk_0x534b6cc4: Option<u32>,
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xdb26669a {
    pub unk_0x19a9984e: String,
    pub unk_0x90373b91: Option<Vec<Unk0xc2afdb3d>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xdb9a90ba {
    pub source: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xdbe05e33 {
    pub button: u32,
    pub trait_icon: u32,
    pub unk_0xca62500c: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xdc24bc6f {
    pub failure_text: String,
    pub failure_texture_path: String,
    pub trove_banner_icon: u32,
    pub trove_button: u32,
    pub unk_0x439b26dc: String,
    pub unk_0xabd0de07: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xdc53945d {
    pub base_loadable: u32,
    pub close_button: u32,
    pub path_hash_to_self: u64,
    pub retry_button: u32,
    pub scene: u32,
    pub unk_0x1ea15738: Vec<u32>,
    pub unk_0x2eed7e1b: u32,
    pub unk_0x3d6d8631: u32,
    pub unk_0x4e976ea0: HashMap<String, u32>,
    pub unk_0xd64694f4: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xdd57ce5e {
    pub characters_nunu_skins_skin8_particles_nunu_skin08_recall_winddown_bird_sparkle: u32,
    pub frame: u32,
    pub icon: u32,
    pub unk_0xfd33c40: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xdd661aab {
    pub override_params: Option<Unk0xc7e628b9>,
    pub trigger_spells: Vec<u32>,
    pub unk_0x6cd45762: bool,
    pub unk_0x77cff90e: Unk0xd91a223,
    pub unk_0x8f7842e4: Vec<Unk0x55f6bf86>,
    pub unk_0x96e77860: u32,
    pub unk_0xda1ee5bc: bool,
    pub unk_0xe4ecb00c: Unk0xfb16e4be,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xdd8ea5ae {
    pub time_remaining_text: u32,
    pub top_bar_icon: Option<u32>,
    pub unk_0x20e7d7b2: Option<bool>,
    pub unk_0x550cba72: u32,
    pub unk_0x715799f9: u32,
    pub unk_0x720b91e8: u32,
    pub unk_0x88131f9a: Unk0x4fbb3f5d,
    pub unk_0xe2f95b32: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xdde919c {
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xddfb3ce9 {
    pub base_loadable: u32,
    pub icon: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub unk_0xebfad7f: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xde07f33b {
    pub base_loadable: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub unk_0x155cac23: Vec<u32>,
    pub unk_0x9051620d: HashMap<u32, Unk0x9051620d>,
    pub unk_0x96260714: Unk0x96260714,
    pub unk_0xfd73ebe0: HashMap<u32, u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xdf4d7688 {
    pub base_loadable: u32,
    pub mobile_override_loadable: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub unk_0x67492b34: Vec<u32>,
    pub unk_0x8a7ef9d0: Vec<u32>,
    pub unk_0xe0bf6ea7: u32,
    pub unk_0xea4aa27c: u32,
    pub unk_0xf9300747: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xdf6e7fa5 {
    pub unk_0xa3627eca: Vec<Unk0x2b686c3d>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xdf7d294e {
    pub button: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xe07edfa4 {
    pub default_visible: Option<bool>,
    pub name: u32,
    pub path_hash: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xe1555e0a {
    pub augment_group: Vec<u32>,
    pub unk_0x9a676645: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xe189202c {
    pub unk_0x36ca632c: f32,
    pub unk_0x6c1cc7b5: f32,
    pub unk_0x7934f534: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xe228ce4a {
    pub device_ux: i32,
    pub frame: u32,
    pub team1_text: u32,
    pub team2_text: u32,
    pub unk_0x3a568777: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xe2b34203 {
    pub custom_sequences: Option<HashMap<u32, RootScriptSequence>>,
    pub functions: Option<HashMap<u32, ScriptFunction>>,
    pub path: u32,
    pub script_name: String,
    pub sequences: Option<HashMap<u32, RootScriptSequence>>,
    pub unk_0x4c19c9cb: Option<i32>,
    pub unk_0xee26c73f: Option<HashMap<u32, EnumAddLevelTimer>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xe2ef74d0 {
    pub slot: u8,
    pub unk_0xc073c624: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xe2ff8b22 {
    pub complete_reward_list: Vec<Unk0x5206ee88>,
    pub general_condition: Unk0x487b1677,
    pub is_enabled: Option<bool>,
    pub quest_name: String,
    pub unk_0x6c8f6dcf: Vec<Unk0x6653bfda>,
    pub unk_0x7829e090: Vec<Unk0x6653bfda>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xe348122d {
    pub unk_0x5c333500: Unk0xce762bab,
    pub unk_0xf2c97a22: Unk0x29619231,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xe35f9399 {
    pub transition_time: f32,
    pub unk_0x5b237830: Option<u8>,
    pub unk_0x66048e3b: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xe524b2fc {
    pub condition: Option<Box<EnumAddLevelTimer>>,
    pub invert: Option<bool>,
    pub is_disabled: Option<bool>,
    pub r#type: Option<u32>,
    pub sequence: Option<Box<ScriptSequence>>,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xe6147387 {
    pub default_on: bool,
    pub joints: Vec<u32>,
    pub orientation_source: Unk0x19da44b2,
    pub orientation_type: u8,
    pub unk_0x1a30a486: bool,
    pub unk_0x420b233d: f32,
    pub unk_0xa57f0269: u8,
    pub unk_0xab2e032a: u8,
    pub unk_0xae1cbd5f: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xe7b61183 {
    pub unk_0x44146c9d: Vec<u32>,
    pub unk_0x8f149e18: f32,
    pub unk_0xe1795243: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xe7ee4f28 {
    pub unk_0x7dd33afb: u32,
    pub unk_0xa2cb8e03: Option<HashMap<String, u32>>,
    pub unk_0xc19c58be: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xe90af953 {
    pub buff: u32,
    pub unk_0xbe161d6e: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xe92f8b6c {
    pub base_loadable: u32,
    pub card_template: LoadingScreenPlayerCardClassicData,
    pub lower_card_region: u32,
    pub path_hash_to_self: u64,
    pub sub_teams_loadable: u32,
    pub unk_0x147429f5: String,
    pub unk_0x1b7ff930: Unk0x12b12bdf,
    pub unk_0x23ff90f5: String,
    pub unk_0x42324784: Unk0x7dc4f3ec,
    pub unk_0x52a578b9: Unk0x12b12bdf,
    pub unk_0xadf9c6d7: Unk0x7dc4f3ec,
    pub unk_0xc2e4a761: String,
    pub unk_0xfff63be6: String,
    pub upper_card_region: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xe9b3cb22 {
    pub unk_0xf248d989: u32,
    pub unk_0xfbeff0: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xea69d6f3 {
    pub elements: Vec<StringGet>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xea7bb717 {
    pub unk_0x384d47fc: u32,
    pub unk_0x8f40c4d9: f32,
    pub unk_0xfc67d839: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xeacb459e {
    pub base_loadable: u32,
    pub health_bar_data: HealthBarData,
    pub path_hash_to_self: u64,
    pub scene: u32,
    pub status_data: UiUnitStatusData,
    pub unk_0x740b49c: Option<Unk0x1739a4e6>,
    pub unk_0x7cae3080: Option<u32>,
    pub unk_0x863f1f6f: Option<u32>,
    pub unk_0x8a07caf5: Option<u32>,
    pub unk_0xae5a3d9b: Option<u32>,
    pub unk_0xe97a2996: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xeb997689 {
    pub definition: Unk0xfcb92181,
    pub name: u32,
    pub transform: Mat4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xec0f1b18 {}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xec733fe2 {
    pub default_visible: Option<bool>,
    pub name: u32,
    pub path_hash: u32,
    pub unk_0x8bff8cdf: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xeca2da9a {
    pub unk_0x8c54e274: Option<Vec<Unk0x2330a302>>,
    pub unk_0xc7b9113e: Option<bool>,
    pub unk_0xde888209: Option<Vec<Unk0x294650d7>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xecd03686 {
    pub active: u32,
    pub flipped: u32,
    pub inactive: u32,
    pub locked: u32,
    pub unk_0x2c41f9d3: String,
    pub unk_0x8d57d389: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xece8d41b {
    pub description_text: u32,
    pub first_reward_frame: u32,
    pub first_reward_group: u32,
    pub first_reward_icon: u32,
    pub progress_complete_icon: u32,
    pub progress_text: u32,
    pub reward_layout: u32,
    pub reward_region: u32,
    pub reward_text: u32,
    pub second_reward_frame: u32,
    pub second_reward_group: u32,
    pub second_reward_icon: u32,
    pub title_text: u32,
    pub unk_0x27688fd4: u32,
    pub unk_0x2da0899c: Option<String>,
    pub unk_0x85520fbb: Option<u32>,
    pub unk_0x892493d9: u32,
    pub unk_0x8c0aea87: u32,
    pub unk_0xbe7286ec: Option<String>,
    pub unk_0xedc62dad: u32,
    pub unk_0xf20dd357: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xed070692 {
    pub selected_terrain: IntTableSet,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xed124985 {
    pub minion_type: u8,
    pub override_behavior: Option<ConstantWaveBehavior>,
    pub unk_0xd851ffa3: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xed293bf2 {
    pub base_loadable: u32,
    pub cancel_button_definition: u32,
    pub close_button_definition: u32,
    pub confirm_button_definition: u32,
    pub content_scene: u32,
    pub kick_button: u32,
    pub path_hash_to_self: u64,
    pub player_icon: u32,
    pub player_name: u32,
    pub promote_button: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xee18a47b {
    pub unk_0x589a59c: u32,
    pub unk_0xb65bc23: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xee28fb8d {
    pub detailed_group: u32,
    pub detailed_hover_region: u32,
    pub detailed_stat_amount: u32,
    pub detailed_stat_name: u32,
    pub detailed_stat_tra: String,
    pub unk_0x1a01f930: u32,
    pub unk_0xab77f602: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xee39916f {
    pub emit_offset: Option<Vec3>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xee8fe512 {
    pub unk_0x3aefc768: f32,
    pub unk_0x74037860: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xef339ef9 {
    pub multiplier: FloatLiteralMaterialDriver,
    pub value: BuffCounterDynamicMaterialFloatDriver,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf07517b3 {
    pub unk_0x5307f5e1: u32,
    pub unk_0x6bfb316f: String,
    pub unk_0x7b12f519: Option<bool>,
    pub unk_0xb825b79c: Option<bool>,
    pub unk_0xe8de69a5: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf090d2e7 {
    pub unk_0x8ecb313b: Option<Vec2>,
    pub unk_0x9567c2a: Option<u8>,
    pub unk_0xa567dbd: Option<u8>,
    pub unk_0xbe0de52: Option<u8>,
    pub unk_0xce0dfe5: Option<u8>,
    pub unk_0xf00a15b2: Option<Vec2>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf0ae6ff1 {
    pub backdrop: u32,
    pub click_absorbing_region_element: u32,
    pub hr_bottom: u32,
    pub hr_bottom_sub_scene: u32,
    pub hr_top: u32,
    pub hr_top_sub_scene: u32,
    pub icon_element: u32,
    pub icon_overlay_element: u32,
    pub main_text_element: u32,
    pub post_script_left_element: u32,
    pub post_script_right_element: u32,
    pub post_script_title_element: u32,
    pub scene: u32,
    pub subtitle_left_element: u32,
    pub subtitle_right_element: u32,
    pub title_left_element: u32,
    pub title_right_element: u32,
    pub unk_0x7aef16a8: Option<u32>,
    pub unk_0xc62ec87b: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf0d64228 {
    pub unk_0xf248d989: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf1c0b81 {
    pub items: Vec<Unk0x99c5a706>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf1fd1323 {
    pub unk_0x45bfca37: u32,
    pub unk_0x53d20c62: u32,
    pub unk_0x8db26ec4: u32,
    pub unk_0xa8dd5e13: u32,
    pub unk_0xb63628: u32,
    pub unk_0xd98c2482: Unk0x51db35d3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf2485b58 {
    pub unk_0xfd36c650: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf2bc55fb {
    pub color: Option<[u8; 4]>,
    pub elements: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf2d18d28 {
    pub unk_0x608f5d1: Vec<u32>,
    pub unk_0x6cb6c4fd: f32,
    pub unk_0xc53c0790: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf2dd2d14 {
    pub bye_labels: Unk0xf3c319e2,
    pub title_defeat_tra: String,
    pub title_future_tra: String,
    pub title_next_tra: String,
    pub title_victory_tra: String,
    pub unk_0x10074827: String,
    pub unk_0x3fca802: String,
    pub unk_0x4c453e79: String,
    pub unk_0x8e3d338b: String,
    pub unk_0x9128c36f: String,
    pub unk_0xd0133f4a: String,
    pub unk_0xe130f1de: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf373182c {
    pub all_visibility_key: String,
    pub base_loadable: u32,
    pub checkbox_template: u32,
    pub layout: u32,
    pub minimap_visibility_key: String,
    pub path_hash_to_self: u64,
    pub scene_handle: u32,
    pub unk_0x1066c852: String,
    pub unk_0x27143566: String,
    pub unk_0x2905eed1: String,
    pub unk_0x4bccc6c8: String,
    pub unk_0x4f4267ad: String,
    pub unk_0x7bcda42a: String,
    pub unk_0x7e891dd7: String,
    pub unk_0x8757c675: String,
    pub unk_0xbc34076d: String,
    pub unk_0xe980a3f0: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf3c319e2 {
    pub title_tra: String,
    pub unk_0x3f8acbfd: String,
    pub unk_0x3fca802: String,
    pub unk_0x6c52029a: String,
    pub unk_0xbb21a9ed: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf3cbe7b2 {
    pub m_spell_calculation_key: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf3cf86a3 {
    pub background: u32,
    pub button: u32,
    pub button_text: u32,
    pub description: u32,
    pub icon: u32,
    pub scene: u32,
    pub title: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf43ad1ce {
    pub frame: u32,
    pub icon_shadow_t1: u32,
    pub icon_shadow_t2: u32,
    pub team1_meter: Unk0xb8a49c96,
    pub team2_meter: Unk0xb8a49c96,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf45e04e1 {
    pub rarity: Option<u8>,
    pub unk_0xe04df972: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf4737fbd {
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf4cc2ad8 {
    pub base_loadable: u32,
    pub notification_template: Unk0x9a4ba494,
    pub path_hash_to_self: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf5022dc7 {
    pub base_model: String,
    pub quality_setting: u32,
    pub version_threshold: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf68386bb {
    pub player_scene: u32,
    pub unk_0x2bb40018: u32,
    pub unk_0x59c1f847: f32,
    pub unk_0xccdd309: u32,
    pub unk_0xefd7430f: HashMap<u8, Unk0x4d80ee3d>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf6e1bec7 {
    pub function_name: u32,
    pub script: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf6f4bb5f {
    pub color: Option<[u8; 4]>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf72ada25 {
    pub base_loadable: u32,
    pub close_button_definition: u32,
    pub confirm_button_definition: u32,
    pub content_scene: u32,
    pub path_hash_to_self: u64,
    pub unk_0x26ebe4ed: UiHyperlink,
    pub unk_0x38eaaebf: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf775806c {
    pub character_record: String,
    pub skin: String,
    pub team: Option<u32>,
    pub unk_0x651de225: f32,
    pub unk_0xd1318f26: f32,
    pub unk_0xf908963: Vec3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf84a5b90 {
    pub button: u32,
    pub icons: Vec<u32>,
    pub unk_0x63a05834: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf85d936e {
    pub group: u32,
    pub icon: u32,
    pub unk_0x29c358a2: u32,
    pub unk_0x3d7eea1d: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf86c7aa6 {
    pub details_scene: u32,
    pub unk_0x10372616: u32,
    pub unk_0x53223ca7: u32,
    pub unk_0x897fb22: u32,
    pub unk_0x8a7f3b34: Option<HashMap<u8, u32>>,
    pub unk_0x8e8db3fa: Option<Vec<u32>>,
    pub unk_0xc455419a: HashMap<u8, u32>,
    pub unk_0xddeb31a1: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf8828af9 {
    pub placement_text: u32,
    pub progress_bar: u32,
    pub progress_text: u32,
    pub unk_0x13795538: u32,
    pub unk_0x14038de6: Unk0x692082f8,
    pub unk_0x1745c61c: u32,
    pub unk_0x44143e89: Unk0x692082f8,
    pub unk_0x54799742: Unk0x4b150334,
    pub unk_0x5e9039d1: Unk0x4b150334,
    pub unk_0x623a9941: Vec<u32>,
    pub unk_0x64a4b235: Unk0x4b150334,
    pub unk_0x701bc96f: u32,
    pub unk_0x8b23569d: Unk0x4b150334,
    pub unk_0x96e371d8: u32,
    pub unk_0x9aa49f71: Unk0x692082f8,
    pub unk_0xad770ba7: u32,
    pub unk_0xaea3ee73: u32,
    pub unk_0xd7ad7bfd: Vec<String>,
    pub unk_0xfa1f1ea0: u32,
    pub unk_0xfc61dac5: Unk0x692082f8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf8c18a6e {
    pub name_tra_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf96d9400 {
    pub button: u32,
    pub text: u32,
    pub tooltip_region: Option<u32>,
    pub unk_0xeb7c1529: Option<String>,
    pub unk_0xf906fd94: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xf9aa519e {
    pub unk_0x39a46ed1: u32,
    pub unk_0x59a36ff5: u32,
    pub unk_0x65c7b224: f32,
    pub unk_0xa7eaf050: u32,
    pub unk_0xc30c2f1f: u32,
    pub unk_0xd41fd074: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xfa33a427 {
    pub title: String,
    pub unk_0xbff2f361: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xfb16e4be {
    pub order_types: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xfb1989a3 {
    pub hover_vfx_system: u32,
    pub idle_vfx_system: u32,
    pub not_picked_vfx_system: u32,
    pub picked_vfx_system: u32,
    pub refresh_overlay_vfx_system: u32,
    pub refresh_vfx_system: u32,
    pub unk_0x34296273: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xfbbe5989 {
    pub unit_tags: ObjectTags,
    pub unk_0x998d15e9: bool,
    pub unk_0x9a6a9339: u32,
    pub unk_0xf43f2e26: Unk0x92024c11,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xfc331f53 {
    pub augment_layout: u32,
    pub augment_slot_data: AugmentSlot,
    pub unk_0x25c9d993: Option<Unk0x7aca1f4d>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xfc6af367 {
    pub hover_vfx_system: u32,
    pub idle_vfx_system: u32,
    pub not_picked_vfx_system: u32,
    pub picked_vfx_system: u32,
    pub refresh_overlay_vfx_system: u32,
    pub refresh_vfx_system: u32,
    pub unk_0x34296273: u32,
    pub unk_0x3c66a261: String,
    pub unk_0x56fabff4: String,
    pub unk_0x5be53b44: Option<String>,
    pub unk_0x8371223f: String,
    pub unk_0x941f723a: String,
    pub unk_0xb95f539d: String,
    pub unk_0xd908f898: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xfcb92181 {
    pub tags: Option<Vec<u32>>,
    pub team: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xfde6a2d7 {
    pub barracks_config: u32,
    pub team: Option<u32>,
    pub unk_0xdb6ea1a7: Option<u32>,
    pub unk_0xdbde2288: Vec<Unk0x82cab1b3>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xfe1f7ae3 {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xfe31ac4d {
    pub source: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xfe70e9c4 {
    pub unk_0x3ef62dce: u8,
    pub unk_0x4e748038: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xfeacedf2 {
    pub unk_0x7a721423: Option<Vec<Unk0x9eba3f83>>,
    pub unk_0x94eea539: Vec<EnumUnk0x394f5aaf>,
    pub value_driver: EnumDriver,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xff55db60 {
    pub unk_0x29f5a8d9: Vec<u32>,
    pub unk_0xbae18fd9: Vec<u32>,
    pub unk_0xd5f0627e: Vec<u32>,
    pub unk_0xfd2cf85: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Unk0xffc26938 {
    pub mutator: String,
    pub team: Option<u32>,
    pub unk_0xd9f00a6c: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdaterData {
    pub input: EnumParametricUpdater,
    pub m_output_type: u32,
    pub m_value_processor_data_list: Option<Vec<LinearTransformProcessorData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdaterResourceData {
    pub m_updater_data_list: Option<Vec<UpdaterData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeItemSelectionViewController {
    pub item_slots: Vec<UiItemSelectionSlotData>,
    pub loadable: u32,
    pub path_hash_to_self: u64,
    pub scene: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UseAutoattackCastTimeData {
    pub m_autoattack_cast_time_calculation: Option<GameCalculation>,
    pub m_use_cast_time_as_total_time: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UseableData {
    pub flags: Option<u32>,
    pub use_cooldown_spell_slot: Option<i32>,
    pub use_hero_spell_name: Option<String>,
    pub use_spell_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UvScaleBiasFromAnimationDynamicMaterialDriver {
    pub m_sub_mesh_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ValueColor {
    pub constant_value: Option<Vec4>,
    pub dynamics: Option<VfxAnimatedColorVariableData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ValueFloat {
    pub constant_value: Option<f32>,
    pub dynamics: Option<VfxAnimatedFloatVariableData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ValueVector2 {
    pub constant_value: Option<Vec2>,
    pub dynamics: Option<VfxAnimatedVector2fVariableData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ValueVector3 {
    pub constant_value: Option<Vec3>,
    pub dynamics: Option<VfxAnimatedVector3fVariableData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VectorGet {
    pub value: Option<Vec3>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VectorTableGet {
    pub table: Option<EnumVars>,
    pub var: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VectorTableSet {
    pub table: Option<EnumVars>,
    pub var: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VersionString {
    pub version_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxAlphaErosionDefinitionData {
    pub erosion_drive_curve: Option<ValueFloat>,
    pub erosion_drive_source: Option<u8>,
    pub erosion_feather_in: Option<f32>,
    pub erosion_feather_out: Option<f32>,
    pub erosion_map_address_mode: Option<u8>,
    pub erosion_map_channel_mixer: Option<ValueColor>,
    pub erosion_map_name: Option<String>,
    pub erosion_slice_width: Option<f32>,
    pub linger_erosion_drive_curve: Option<ValueFloat>,
    pub use_linger_erosion_drive_curve: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxAnimatedColorVariableData {
    pub probability_tables: Option<Vec<VfxProbabilityTableData>>,
    pub times: Option<Vec<f32>>,
    pub values: Option<Vec<Vec4>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxAnimatedFloatVariableData {
    pub probability_tables: Option<Vec<VfxProbabilityTableData>>,
    pub times: Vec<f32>,
    pub values: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxAnimatedVector2fVariableData {
    pub probability_tables: Option<Vec<VfxProbabilityTableData>>,
    pub times: Vec<f32>,
    pub values: Vec<Vec2>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxAnimatedVector3fVariableData {
    pub probability_tables: Option<Vec<VfxProbabilityTableData>>,
    pub times: Vec<f32>,
    pub values: Vec<Vec3>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxAssetRemap {
    pub new_asset: Option<String>,
    pub old_asset: Option<u32>,
    pub r#type: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxBeamDefinitionData {
    pub m_animated_color_with_distance: Option<ValueColor>,
    pub m_birth_tiling_size: Option<ValueVector3>,
    pub m_is_color_binded_with_distance: Option<bool>,
    pub m_local_space_source_offset: Option<Vec3>,
    pub m_local_space_target_offset: Option<Vec3>,
    pub m_mode: Option<u8>,
    pub m_segments: Option<i32>,
    pub m_trail_mode: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxChildIdentifier {
    pub effect: Option<u32>,
    pub effect_key: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxChildParticleSetDefinitionData {
    pub bone_to_spawn_at: Option<Vec<String>>,
    pub child_emit_on_death: Option<bool>,
    pub children_identifiers: Option<Vec<VfxChildIdentifier>>,
    pub children_probability: Option<ValueFloat>,
    pub parent_inheritance_definition: Option<VfxParentInheritanceParams>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxColorOverLifeMaterialDriver {
    pub colors: VfxAnimatedColorVariableData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxDistortionDefinitionData {
    pub distortion: Option<f32>,
    pub distortion_mode: Option<u8>,
    pub normal_map_texture: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxEmissionSurfaceData {
    pub unk_0x2808fffd: Option<Unk0xcd5a34f5>,
    pub unk_0xf8b81c77: Option<Unk0x671b7351>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxEmitterAudio {
    pub sound_on_create: Option<String>,
    pub sound_persistent: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxEmitterDefinitionData {
    pub acceleration: Option<ValueVector3>,
    pub alpha_erosion_definition: Option<VfxAlphaErosionDefinitionData>,
    pub alpha_ref: Option<u8>,
    pub audio: Option<VfxEmitterAudio>,
    pub bind_weight: Option<ValueFloat>,
    pub birth_acceleration: Option<ValueVector3>,
    pub birth_color: Option<ValueColor>,
    pub birth_drag: Option<ValueVector3>,
    pub birth_frame_rate: Option<ValueFloat>,
    pub birth_orbital_velocity: Option<ValueVector3>,
    pub birth_rotation0: Option<ValueVector3>,
    pub birth_rotational_acceleration: Option<ValueVector3>,
    pub birth_rotational_velocity0: Option<ValueVector3>,
    pub birth_scale0: Option<ValueVector3>,
    pub birth_uv_offset: Option<ValueVector2>,
    pub birth_uv_rotate_rate: Option<ValueFloat>,
    pub birth_uv_scroll_rate: Option<ValueVector2>,
    pub birth_velocity: Option<ValueVector3>,
    pub blend_mode: Option<u8>,
    pub censor_modulate_value: Option<Vec4>,
    pub chance_to_not_exist: Option<f32>,
    pub child_particle_set_definition: Option<VfxChildParticleSetDefinitionData>,
    pub color: Option<ValueColor>,
    pub color_look_up_offsets: Option<Vec2>,
    pub color_look_up_scales: Option<Vec2>,
    pub color_look_up_type_x: Option<u8>,
    pub color_look_up_type_y: Option<u8>,
    pub color_render_flags: Option<u8>,
    pub colorblind_visibility: Option<u8>,
    pub custom_material: Option<VfxMaterialDefinitionData>,
    pub depth_bias_factors: Option<Vec2>,
    pub direction_velocity_min_scale: Option<f32>,
    pub direction_velocity_scale: Option<f32>,
    pub disable_backface_cull: Option<bool>,
    pub disabled: Option<bool>,
    pub distortion_definition: Option<VfxDistortionDefinitionData>,
    pub does_cast_shadow: Option<bool>,
    pub does_lifetime_scale: Option<bool>,
    pub drag: Option<ValueVector3>,
    pub emission_mesh_name: Option<String>,
    pub emission_mesh_scale: Option<f32>,
    pub emission_surface_definition: Option<VfxEmissionSurfaceData>,
    pub emitter_linger: Option<f32>,
    pub emitter_name: Option<String>,
    pub emitter_position: Option<ValueVector3>,
    pub emitter_uv_scroll_rate: Option<Vec2>,
    pub falloff_texture: Option<String>,
    pub field_collection_definition: Option<VfxFieldCollectionDefinitionData>,
    pub filtering: Option<VfxEmitterFiltering>,
    pub flex_birth_rotational_velocity0: Option<FlexValueVector3>,
    pub flex_birth_uv_offset: Option<FlexValueVector2>,
    pub flex_birth_uv_scroll_rate: Option<FlexValueVector2>,
    pub flex_birth_velocity: Option<FlexValueVector3>,
    pub flex_instance_scale: Option<FlexTypeFloat>,
    pub flex_particle_lifetime: Option<FlexValueFloat>,
    pub flex_rate: Option<FlexValueFloat>,
    pub flex_scale_birth_scale: Option<FlexTypeFloat>,
    pub flex_shape_definition: Option<VfxFlexShapeDefinitionData>,
    pub frame_rate: Option<f32>,
    pub has_post_rotate_orientation: Option<bool>,
    pub has_variable_start_time: Option<bool>,
    pub importance: Option<u8>,
    pub is_direction_oriented: Option<bool>,
    pub is_emitter_space: Option<bool>,
    pub is_following_terrain: Option<bool>,
    pub is_ground_layer: Option<bool>,
    pub is_local_orientation: Option<bool>,
    pub is_random_start_frame: Option<bool>,
    pub is_rotation_enabled: Option<bool>,
    pub is_single_particle: Option<bool>,
    pub is_texture_pixelated: Option<bool>,
    pub is_uniform_scale: Option<bool>,
    pub legacy_simple: Option<VfxEmitterLegacySimple>,
    pub lifetime: Option<f32>,
    pub linger: Option<VfxLingerDefinitionData>,
    pub material_override_definitions: Option<Vec<VfxMaterialOverrideDefinitionData>>,
    pub maximum_rate_by_velocity: Option<f32>,
    pub mesh_render_flags: Option<u8>,
    pub misc_render_flags: Option<u8>,
    pub modulation_factor: Option<Vec4>,
    pub num_frames: Option<u16>,
    pub offset_life_scaling_symmetry_mode: Option<u8>,
    pub offset_lifetime_scaling: Option<Vec3>,
    pub palette_definition: Option<VfxPaletteDefinitionData>,
    pub particle_color_texture: Option<String>,
    pub particle_is_local_orientation: Option<bool>,
    pub particle_lifetime: Option<ValueFloat>,
    pub particle_linger: Option<f32>,
    pub particle_linger_type: Option<u8>,
    pub particle_uv_rotate_rate: Option<IntegratedValueFloat>,
    pub particle_uv_scroll_rate: Option<IntegratedValueVector2>,
    pub particles_share_random_value: Option<bool>,
    pub pass: Option<i16>,
    pub period: Option<f32>,
    pub post_rotate_orientation_axis: Option<Vec3>,
    pub primitive: Option<EnumVfxPrimitive>,
    pub rate: Option<ValueFloat>,
    pub rate_by_velocity_function: Option<ValueVector2>,
    pub reflection_definition: Option<VfxReflectionDefinitionData>,
    pub render_phase_override: Option<u8>,
    pub rotation0: Option<IntegratedValueVector3>,
    pub rotation_override: Option<Vec3>,
    pub scale0: Option<ValueVector3>,
    pub scale_override: Option<Vec3>,
    pub slice_technique_range: Option<f32>,
    pub soft_particle_params: Option<VfxSoftParticleDefinitionData>,
    pub sort_emitters_by_pos: Option<bool>,
    pub spawn_shape: Option<EnumVfxShape>,
    pub start_frame: Option<u16>,
    pub stencil_mode: Option<u8>,
    pub stencil_ref: Option<u8>,
    pub stencil_reference_id: Option<u32>,
    pub tex_address_mode_base: Option<u8>,
    pub tex_div: Option<Vec2>,
    pub texture: Option<String>,
    pub texture_flip_u: Option<bool>,
    pub texture_flip_v: Option<bool>,
    pub texture_mult: Option<VfxTextureMultDefinitionData>,
    pub time_active_during_period: Option<f32>,
    pub time_before_first_emission: Option<f32>,
    pub translation_override: Option<Vec3>,
    pub unk_0xcb13aff1: Option<f32>,
    pub unk_0xd1ee8634: Option<bool>,
    pub use_emission_mesh_normal_for_birth: Option<bool>,
    pub use_navmesh_mask: Option<bool>,
    pub uv_mode: Option<u8>,
    pub uv_parallax_scale: Option<f32>,
    pub uv_rotation: Option<ValueFloat>,
    pub uv_scale: Option<ValueVector2>,
    pub uv_scroll_clamp: Option<bool>,
    pub uv_transform_center: Option<Vec2>,
    pub velocity: Option<ValueVector3>,
    pub world_acceleration: Option<IntegratedValueVector3>,
    pub write_alpha_only: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxEmitterFiltering {
    pub censor_policy: Option<u8>,
    pub keywords_excluded: Option<Vec<String>>,
    pub keywords_required: Option<Vec<String>>,
    pub spectator_policy: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxEmitterLegacySimple {
    pub birth_rotation: Option<ValueFloat>,
    pub birth_rotational_velocity: Option<ValueFloat>,
    pub birth_scale: Option<ValueFloat>,
    pub has_fixed_orbit: Option<bool>,
    pub orientation: Option<u8>,
    pub particle_bind: Option<Vec2>,
    pub rotation: Option<ValueFloat>,
    pub scale: Option<ValueFloat>,
    pub scale_bias: Option<Vec2>,
    pub uv_scroll_rate: Option<Vec2>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxFieldAccelerationDefinitionData {
    pub acceleration: Option<ValueVector3>,
    pub is_local_space: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxFieldAttractionDefinitionData {
    pub acceleration: Option<ValueFloat>,
    pub position: Option<ValueVector3>,
    pub radius: Option<ValueFloat>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxFieldCollectionDefinitionData {
    pub field_acceleration_definitions: Option<Vec<VfxFieldAccelerationDefinitionData>>,
    pub field_attraction_definitions: Option<Vec<VfxFieldAttractionDefinitionData>>,
    pub field_drag_definitions: Option<Vec<VfxFieldDragDefinitionData>>,
    pub field_noise_definitions: Option<Vec<VfxFieldNoiseDefinitionData>>,
    pub field_orbital_definitions: Option<Vec<VfxFieldOrbitalDefinitionData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxFieldDragDefinitionData {
    pub position: Option<ValueVector3>,
    pub radius: Option<ValueFloat>,
    pub strength: Option<ValueFloat>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxFieldNoiseDefinitionData {
    pub axis_fraction: Option<Vec3>,
    pub frequency: Option<ValueFloat>,
    pub position: Option<ValueVector3>,
    pub radius: Option<ValueFloat>,
    pub velocity_delta: Option<ValueFloat>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxFieldOrbitalDefinitionData {
    pub direction: Option<ValueVector3>,
    pub is_local_space: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxFlexShapeDefinitionData {
    pub flex_birth_translation: Option<FlexValueVector3>,
    pub flex_scale_emit_offset: Option<FlexTypeFloat>,
    pub scale_birth_scale_by_bound_object_height: Option<f32>,
    pub scale_birth_scale_by_bound_object_radius: Option<f32>,
    pub scale_birth_scale_by_bound_object_size: Option<f32>,
    pub scale_birth_translation_by_bound_object_size: Option<f32>,
    pub scale_emit_offset_by_bound_object_height: Option<f32>,
    pub scale_emit_offset_by_bound_object_radius: Option<f32>,
    pub scale_emit_offset_by_bound_object_size: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxFloatOverLifeMaterialDriver {
    pub frequency: Option<u8>,
    pub graph: VfxAnimatedFloatVariableData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxLingerDefinitionData {
    pub keyed_linger_acceleration: Option<ValueVector3>,
    pub keyed_linger_drag: Option<ValueVector3>,
    pub keyed_linger_velocity: Option<ValueVector3>,
    pub linger_rotation: Option<ValueVector3>,
    pub linger_scale: Option<ValueVector3>,
    pub separate_linger_color: Option<ValueColor>,
    pub use_keyed_linger_acceleration: Option<bool>,
    pub use_keyed_linger_drag: Option<bool>,
    pub use_keyed_linger_velocity: Option<bool>,
    pub use_linger_rotation: Option<bool>,
    pub use_linger_scale: Option<bool>,
    pub use_separate_linger_color: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxMaterialDefinitionData {
    pub material: u32,
    pub material_drivers: Option<HashMap<String, EnumOverLifeMaterialDriver>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxMaterialOverrideDefinitionData {
    pub base_texture: Option<String>,
    pub gloss_texture: Option<String>,
    pub material: Option<u32>,
    pub override_blend_mode: Option<u32>,
    pub priority: Option<i32>,
    pub sub_mesh_name: Option<String>,
    pub transition_sample: Option<f32>,
    pub transition_source: Option<u32>,
    pub transition_texture: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxMeshDefinitionData {
    pub m_animation_name: Option<String>,
    pub m_animation_variants: Option<Vec<String>>,
    pub m_lock_mesh_to_attachment: Option<bool>,
    pub m_mesh_name: Option<String>,
    pub m_mesh_skeleton_name: Option<String>,
    pub m_simple_mesh_name: Option<String>,
    pub m_submeshes_to_draw: Option<Vec<u32>>,
    pub m_submeshes_to_draw_always: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxPaletteDefinitionData {
    pub palette_count: Option<i32>,
    pub palette_selector: Option<ValueVector3>,
    pub palette_texture: Option<String>,
    pub palette_texture_address_mode: Option<u8>,
    pub palette_u_animation_curve: Option<ValueFloat>,
    pub palette_v_animation_curve: Option<ValueFloat>,
    pub pallete_src_mix_color: Option<ValueColor>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxParentInheritanceParams {
    pub mode: Option<u8>,
    pub relative_offset: Option<ValueVector3>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxPrimitiveArbitraryTrail {
    pub m_trail: Option<VfxTrailDefinitionData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxPrimitiveAttachedMesh {
    pub align_pitch_to_camera: Option<bool>,
    pub align_yaw_to_camera: Option<bool>,
    pub m_mesh: Option<VfxMeshDefinitionData>,
    pub unk_0x6aec9e7a: Option<bool>,
    pub use_avatar_specific_submesh_mask: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxPrimitiveBeam {
    pub m_beam: Option<VfxBeamDefinitionData>,
    pub m_mesh: Option<VfxMeshDefinitionData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxPrimitiveCameraSegmentBeam {
    pub m_beam: Option<VfxBeamDefinitionData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxPrimitiveCameraSegmentSeriesBeam {
    pub name: String,
    pub vfx_mask: [u8; 4],
    pub vfx_system: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxPrimitiveCameraTrail {
    pub m_trail: Option<VfxTrailDefinitionData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxPrimitiveMesh {
    pub align_pitch_to_camera: Option<bool>,
    pub align_yaw_to_camera: Option<bool>,
    pub m_mesh: Option<VfxMeshDefinitionData>,
    pub unk_0x6aec9e7a: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxPrimitivePlanarProjection {
    pub m_projection: Option<VfxProjectionDefinitionData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxProbabilityTableData {
    pub key_times: Option<Vec<f32>>,
    pub key_values: Option<Vec<f32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxProjectionDefinitionData {
    pub color_modulate: Option<ValueColor>,
    pub m_fading: Option<f32>,
    pub m_y_range: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxReflectionDefinitionData {
    pub fresnel: Option<f32>,
    pub fresnel_color: Option<Vec4>,
    pub reflection_fresnel: Option<f32>,
    pub reflection_fresnel_color: Option<Vec4>,
    pub reflection_map_texture: Option<String>,
    pub reflection_opacity_direct: Option<f32>,
    pub reflection_opacity_glancing: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxShapeBox {
    pub flags: Option<u8>,
    pub size: Option<Vec3>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxShapeCylinder {
    pub flags: Option<u8>,
    pub height: Option<f32>,
    pub radius: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxShapeLegacy {
    pub emit_offset: Option<ValueVector3>,
    pub emit_rotation_angles: Option<Vec<ValueFloat>>,
    pub emit_rotation_axes: Option<Vec<Vec3>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxShapeSphere {
    pub flags: Option<u8>,
    pub radius: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxSoftParticleDefinitionData {
    pub begin_in: Option<f32>,
    pub begin_out: Option<f32>,
    pub delta_in: Option<f32>,
    pub delta_out: Option<f32>,
    pub unk_0x3bf176bc: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct VfxSystemDefinitionData {
    pub asset_remapping_table: Option<Vec<VfxAssetRemap>>,
    pub audio_parameter_flex_id: Option<i32>,
    pub audio_parameter_time_scaled_duration: Option<f32>,
    pub build_up_time: Option<f32>,
    pub clock_to_use: Option<u8>,
    pub complex_emitter_definition_data: Option<Vec<VfxEmitterDefinitionData>>,
    pub drawing_layer: Option<u8>,
    pub flags: Option<u16>,
    pub hud_anchor_position_from_world_projection: Option<bool>,
    pub hud_layer_dimension: Option<f32>,
    pub m_eye_candy: Option<bool>,
    pub m_is_pose_afterimage: Option<bool>,
    pub material_override_definitions: Option<Vec<VfxMaterialOverrideDefinitionData>>,
    pub override_scale_cap: Option<f32>,
    pub particle_name: String,
    pub particle_path: String,
    pub scale_dynamically_with_attached_bone: Option<bool>,
    pub self_illumination: Option<f32>,
    pub simple_emitter_definition_data: Option<Vec<VfxEmitterDefinitionData>>,
    pub sound_on_create_default: Option<String>,
    pub sound_persistent_default: Option<String>,
    pub transform: Option<Mat4>,
    pub unk_0x8b301739: Option<Unk0x75e34c40>,
    pub unk_0x9836cd87: Option<u8>,
    pub unk_0xf97b1289: Option<Unk0x7fb92f53>,
    pub visibility_radius: Option<f32>,
    pub voice_over_on_create_default: Option<String>,
    pub voice_over_persistent_default: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxTextureMultDefinitionData {
    pub birth_uv_offset_mult: Option<ValueVector2>,
    pub birth_uv_rotate_rate_mult: Option<ValueFloat>,
    pub birth_uv_scroll_rate_mult: Option<ValueVector2>,
    pub emitter_uv_scroll_rate_mult: Option<Vec2>,
    pub flex_birth_uv_scroll_rate_mult: Option<FlexValueVector2>,
    pub is_random_start_frame_mult: Option<bool>,
    pub particle_integrated_uv_rotate_mult: Option<IntegratedValueFloat>,
    pub particle_integrated_uv_scroll_mult: Option<IntegratedValueVector2>,
    pub tex_address_mode_mult: Option<u8>,
    pub tex_div_mult: Option<Vec2>,
    pub texture_mult: Option<String>,
    pub texture_mult_filp_u: Option<bool>,
    pub texture_mult_filp_v: Option<bool>,
    pub uv_rotation_mult: Option<ValueFloat>,
    pub uv_scale_mult: Option<ValueVector2>,
    pub uv_scroll_alpha_mult: Option<bool>,
    pub uv_scroll_clamp_mult: Option<bool>,
    pub uv_transform_center_mult: Option<Vec2>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VfxTrailDefinitionData {
    pub m_birth_tiling_size: Option<ValueVector3>,
    pub m_cutoff: Option<f32>,
    pub m_max_added_per_frame: Option<i32>,
    pub m_mode: Option<u8>,
    pub m_smoothing_mode: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ViewControllerFilterAnd {
    pub filters: Vec<Box<EnumViewControllerFilter>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ViewControllerFilterChampion {
    pub champion_skin_data_link: String,
    pub relationship: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ViewControllerFilterMap {
    pub map: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ViewControllerFilterMode {
    pub mode: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ViewControllerFilterNot {
    pub filter: Box<EnumViewControllerFilter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ViewControllerFilterOr {
    pub filters: Vec<Box<EnumViewControllerFilter>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ViewControllerList {
    pub filter: Option<EnumViewControllerFilter>,
    pub view_controllers: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct ViewControllerSet {
    pub client_state: String,
    pub lists_to_load: Vec<u32>,
    pub specified_game_modes: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Asset, TypePath)]
#[serde(rename_all = "camelCase")]
pub struct VoiceChatViewController {
    pub backdrop: u32,
    pub base_loadable: u32,
    pub connected_bg_region_handle: u32,
    pub error_text: u32,
    pub panel_scene_handle: u32,
    pub path_hash_to_self: u64,
    pub player_grid: u32,
    pub player_slot_data: VoiceChatViewPlayerSlotData,
    pub player_slot_region_handle: u32,
    pub self_slot: VoiceChatViewSelfSlot,
    pub unk_0x70f56833: Option<[u8; 4]>,
    pub unk_0xe38d9a41: [u8; 4],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VoiceChatViewPlayerSlotData {
    pub group: u32,
    pub halo: u32,
    pub mute_button: u32,
    pub name_text: u32,
    pub portrait: u32,
    pub volume_slider_bar: u32,
    pub volume_text: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VoiceChatViewSelfSlot {
    pub connection_button: u32,
    pub halo: u32,
    pub mic_volume_slider_bar: u32,
    pub mic_volume_text: u32,
    pub mute_button: u32,
    pub name_text: u32,
    pub portrait: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WallDetection {
    pub detection_range: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WallFollowMovement {
    pub m_counter_clockwise: Option<bool>,
    pub m_infer_direction_from_facing_if_needed: bool,
    pub m_speed: f32,
    pub m_start_bone_name: String,
    pub m_stop_halfway_around: bool,
    pub m_tracks_target: bool,
    pub m_use_ground_height_at_target: bool,
    pub m_wall_length: f32,
    pub m_wall_offset: f32,
    pub m_wall_search_radius: f32,
    pub use_point_smoothing: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WidthPerSecond {
    pub m_width_per_second: f32,
}
