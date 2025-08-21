use std::{collections::HashMap, time::Duration};

use bevy::{prelude::*, reflect::ReflectRef};
use rand::{
    distr::{weighted::WeightedIndex, Distribution},
    rng,
};

use crate::{core::State, league::LeagueLoader};

#[derive(Component)]
pub struct Animation {
    pub hash_to_node: HashMap<u32, AnimationNode>,
}

impl Animation {
    pub fn play(&self, player: &mut AnimationPlayer, key: u32, weight: f32) {
        let Some(node) = self.hash_to_node.get(&key) else {
            return;
        };

        match node {
            AnimationNode::Clip { node_index } => {
                player.play(*node_index).set_weight(weight).repeat();
            }
            AnimationNode::ConditionFloat { conditions, .. } => {
                for condition in conditions {
                    self.play(player, condition.key, weight);
                }
            }
            AnimationNode::Selector { probably_nodes } => {
                let weights = probably_nodes.iter().map(|v| v.value).collect::<Vec<_>>();
                let dist = WeightedIndex::new(weights).unwrap();
                let index = dist.sample(&mut rng());

                self.play(player, probably_nodes[index].key, weight);
            }
        }
    }

    pub fn stop(&self, player: &mut AnimationPlayer, key: u32) {
        let Some(node) = self.hash_to_node.get(&key) else {
            return;
        };

        match node {
            AnimationNode::Clip { node_index } => {
                player.stop(*node_index);
            }
            AnimationNode::ConditionFloat { conditions, .. } => {
                for condition in conditions {
                    self.stop(player, condition.key);
                }
            }
            AnimationNode::Selector { probably_nodes } => {
                for node in probably_nodes {
                    self.stop(player, node.key);
                }
            }
        }
    }
}

#[derive(Component)]
pub struct AnimationState {
    pub current_hash: u32,
    pub last_hash: u32,
}

impl AnimationState {
    pub fn update(&mut self, hash: u32) {
        self.last_hash = self.current_hash;
        self.current_hash = hash;
    }
}

#[derive(Component)]
pub struct AnimationTransitionOut {
    pub hash: u32,
    pub weight: f32,
    pub duration: Duration,
    pub start_time: f32,
}

/// 动画时间同步组件 - 用于将动画时间与组件的时间属性同步
#[derive(Component)]
pub struct AnimationTimingSync {
    /// 目标组件名称
    pub component_name: String,
    /// 目标字段名称（返回时间长度的方法或字段）
    pub duration_field: String,
    /// 动画哈希值映射 - 指定哪些动画需要同步时间
    pub synced_animations: Vec<u32>,
}

/// 动画速度控制组件 - 存储当前动画的播放速度
#[derive(Component)]
pub struct AnimationSpeed {
    /// 当前播放速度倍率 (1.0 = 正常速度, 2.0 = 2倍速度)
    pub speed_multiplier: f32,
    /// 上一帧的速度倍率，用于检测变化
    pub last_speed_multiplier: f32,
}

#[derive(Clone)]
pub enum AnimationNode {
    Clip {
        node_index: AnimationNodeIndex,
    },
    ConditionFloat {
        component_name: String,
        field_name: String,
        conditions: Vec<AnimationNodeF32>,
    },
    Selector {
        probably_nodes: Vec<AnimationNodeF32>,
    },
}

#[derive(Clone)]
pub struct AnimationNodeF32 {
    pub key: u32,
    pub value: f32,
}

#[derive(Default)]
pub struct PluginAnimation;

impl Plugin for PluginAnimation {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_state_change);
        app.add_systems(Update, on_animation_state_change);
        app.add_systems(Update, update_transition_out);
        app.add_systems(Update, update_condition_animation);
        app.add_systems(Update, update_animation_timing_sync);
        app.add_systems(Update, apply_animation_speed);
    }
}

fn on_state_change(mut query: Query<(&State, &mut AnimationState), Changed<State>>) {
    for (state, mut animation_state) in query.iter_mut() {
        match state {
            State::Idle => {
                animation_state.update(LeagueLoader::hash_bin("Idle1"));
            }
            State::Moving => {
                animation_state.update(LeagueLoader::hash_bin("Run"));
            }
            State::Attacking => {
                animation_state.update(LeagueLoader::hash_bin("Attack1"));
            }
        }
    }
}

fn on_animation_state_change(
    mut query: Query<
        (Entity, &mut AnimationPlayer, &Animation, &AnimationState),
        Changed<AnimationState>,
    >,
    q_transition_out: Query<&AnimationTransitionOut>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut player, animation, state) in query.iter_mut() {
        if state.current_hash == state.last_hash {
            continue;
        }

        if let Ok(transition_out) = q_transition_out.get(entity) {
            animation.stop(&mut player, transition_out.hash);
        }

        commands.entity(entity).insert(AnimationTransitionOut {
            hash: state.last_hash,
            weight: 1.0,
            duration: Duration::from_millis(200),
            start_time: time.elapsed_secs(),
        });

        animation.play(&mut player, state.current_hash, 1.0);
    }
}

fn update_transition_out(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut AnimationPlayer,
        &Animation,
        &AnimationTransitionOut,
    )>,
    time: Res<Time>,
) {
    for (entity, mut player, animation, transition_out) in query.iter_mut() {
        let now = time.elapsed_secs();

        let elapsed = now - transition_out.start_time;

        let duration = transition_out.duration.as_secs_f32();

        if elapsed > duration {
            animation.stop(&mut player, transition_out.hash);
            commands.entity(entity).remove::<AnimationTransitionOut>();
            continue;
        }

        let weight = transition_out.weight * (1.0 - (elapsed / duration));

        animation.play(&mut player, transition_out.hash, weight);
    }
}

fn update_condition_animation(world: &mut World) {
    let mut query = world.query::<(Entity, &Animation, &AnimationState)>();
    let mut player_query = world.query::<(&mut AnimationPlayer, &Animation)>();

    let play_list = query
        .iter(world)
        .filter_map(|(entity, animation, state)| {
            let Some(node) = animation
                .hash_to_node
                .get(&state.current_hash)
                .map(|v| v.clone())
            else {
                return None;
            };

            let AnimationNode::ConditionFloat {
                component_name,
                field_name,
                conditions,
                ..
            } = node
            else {
                return None;
            };

            let Some(value) =
                get_reflect_component_value(world, entity, &component_name, &field_name)
            else {
                return None;
            };

            if conditions.is_empty() {
                return None;
            }

            if value < conditions[0].value {
                return Some((entity, vec![(conditions[0].key, 1.0)]));
            }

            if let Some(condition) = conditions.last() {
                if value >= condition.value {
                    return Some((entity, vec![(condition.key, 1.0)]));
                }
            }

            let Some(window) = conditions
                .windows(2)
                .find(|w| value >= w[0].value && value < w[1].value)
            else {
                return None;
            };

            let lower = &window[0];
            let upper = &window[1];

            let range = upper.value - lower.value;

            let weight = if range > f32::EPSILON {
                ((value - lower.value) / range).clamp(0.0, 1.0)
            } else {
                0.0
            };

            return Some((entity, vec![(lower.key, 1.0 - weight), (upper.key, weight)]));
        })
        .collect::<Vec<_>>();

    for (entity, nodes) in play_list {
        let Ok((mut player, animation)) = player_query.get_mut(world, entity) else {
            continue;
        };

        for (key, weight) in nodes {
            animation.play(&mut player, key, weight);
        }
    }
}

fn get_reflect_component_value(
    world: &World,
    entity: Entity,
    component_name: &str,
    field_name: &str,
) -> Option<f32> {
    let registry = world.resource::<AppTypeRegistry>().read();
    let Some(type_registration) = registry.get_with_short_type_path(component_name) else {
        return None;
    };
    let Some(reflect_component) = type_registration.data::<ReflectComponent>() else {
        return None;
    };
    let Ok(entity_ref) = world.get_entity(entity) else {
        return None;
    };
    let Some(component) = reflect_component.reflect(entity_ref) else {
        return None;
    };
    let ReflectRef::Struct(struct_ref) = component.reflect_ref() else {
        return None;
    };
    let Some(value) = struct_ref.get_field::<f32>(field_name) else {
        return None;
    };
    Some(*value)
}
/// 更新动画时间同步 - 根据组件的时间属性调整动画播放速度
fn update_animation_timing_sync(world: &mut World) {
    let mut query = world.query::<(Entity, &AnimationTimingSync, &AnimationState)>();

    let sync_updates = query
        .iter(world)
        .filter_map(|(entity, timing_sync, animation_state)| {
            // 检查当前动画是否需要同步
            if !timing_sync
                .synced_animations
                .contains(&animation_state.current_hash)
            {
                return None;
            }

            // 获取目标组件的时间长度
            let duration = get_reflect_component_value(
                world,
                entity,
                &timing_sync.component_name,
                &timing_sync.duration_field,
            )?;

            // 计算速度倍率 - 假设基础动画时长为1秒，根据实际时长调整
            // 如果攻击时间为0.5秒，则动画应该播放2倍速度
            let speed_multiplier = if duration > 0.0 { 1.0 / duration } else { 1.0 };

            Some((entity, speed_multiplier))
        })
        .collect::<Vec<_>>();

    let mut commands = world.commands();
    for (entity, speed_multiplier) in sync_updates {
        // 更新或插入 AnimationSpeed 组件
        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.insert(AnimationSpeed {
                speed_multiplier,
                last_speed_multiplier: 0.0, // 强制更新
            });
        }
    }
}

/// 应用动画速度 - 将计算出的速度应用到 AnimationPlayer
fn apply_animation_speed(
    mut query: Query<(&mut AnimationPlayer, &mut AnimationSpeed), Changed<AnimationSpeed>>,
) {
    for (mut player, mut animation_speed) in query.iter_mut() {
        // 检查速度是否发生变化
        if (animation_speed.speed_multiplier - animation_speed.last_speed_multiplier).abs()
            < f32::EPSILON
        {
            continue;
        }

        // 应用新的播放速度到所有活跃的动画
        for (_, active_animation) in player.playing_animations_mut() {
            active_animation.set_speed(animation_speed.speed_multiplier);
        }

        // 更新上一帧的速度
        animation_speed.last_speed_multiplier = animation_speed.speed_multiplier;
    }
}

impl AnimationTimingSync {
    /// 创建攻击动画时间同步配置
    pub fn for_attack() -> Self {
        Self {
            component_name: "Attack".to_string(),
            duration_field: "total_duration_secs".to_string(),
            synced_animations: vec![
                LeagueLoader::hash_bin("Attack1"),
                LeagueLoader::hash_bin("Attack2"),
                LeagueLoader::hash_bin("Attack3"),
                LeagueLoader::hash_bin("Crit"),
            ],
        }
    }

    /// 创建自定义动画时间同步配置
    pub fn new(component_name: String, duration_field: String, animations: Vec<&str>) -> Self {
        Self {
            component_name,
            duration_field,
            synced_animations: animations
                .iter()
                .map(|name| LeagueLoader::hash_bin(name))
                .collect(),
        }
    }

    /// 添加需要同步的动画
    pub fn with_animation(mut self, animation_name: &str) -> Self {
        self.synced_animations
            .push(LeagueLoader::hash_bin(animation_name));
        self
    }
}

impl Default for AnimationSpeed {
    fn default() -> Self {
        Self {
            speed_multiplier: 1.0,
            last_speed_multiplier: 1.0,
        }
    }
}
