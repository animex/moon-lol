use bevy::prelude::*;
use moon_lol::core::{Animation, AnimationState, AnimationTimingSync, Attack, PluginCore, State};
use moon_lol::league::LeagueLoader;
use std::collections::HashMap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PluginCore)
        .add_systems(Startup, setup)
        .add_systems(Update, (test_attack_speed_changes, keyboard_input))
        .run();
}

#[derive(Component)]
struct TestEntity;

fn setup(mut commands: Commands) {
    // 创建测试实体，演示动画时间同步
    let entity = commands
        .spawn((
            TestEntity,
            // 基础攻击组件：攻击范围150，前摇0.3秒，总时间1.0秒
            Attack::new(150.0, 0.3, 1.0),
            // 动画时间同步组件 - 攻击动画将根据攻击速度自动调整
            AnimationTimingSync::for_attack(),
            // 动画组件（简化示例）
            Animation {
                hash_to_node: create_test_animations(),
            },
            // 动画状态
            AnimationState {
                current_hash: LeagueLoader::hash_bin("Idle1"),
                last_hash: 0,
            },
            // 实体状态
            State::Idle,
        ))
        .id();

    println!("创建测试实体: {:?}", entity);
    println!("按键说明:");
    println!("  1 - 切换到攻击状态 (触发动画时间同步)");
    println!("  2 - 增加攻击速度 (+50%)");
    println!("  3 - 减少攻击速度 (-25%)");
    println!("  4 - 重置攻击速度");
    println!("  I - 切换到待机状态");
}

fn create_test_animations() -> HashMap<u32, moon_lol::core::AnimationNode> {
    use moon_lol::core::AnimationNode;

    let mut animations = HashMap::new();

    // 添加基础动画节点（简化示例）
    animations.insert(
        LeagueLoader::hash_bin("Idle1"),
        AnimationNode::Clip {
            node_index: AnimationNodeIndex::new(0),
        },
    );

    animations.insert(
        LeagueLoader::hash_bin("Attack1"),
        AnimationNode::Clip {
            node_index: AnimationNodeIndex::new(1),
        },
    );

    animations.insert(
        LeagueLoader::hash_bin("Attack2"),
        AnimationNode::Clip {
            node_index: AnimationNodeIndex::new(2),
        },
    );

    animations
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Attack, &mut State), With<TestEntity>>,
) {
    let Ok((mut attack, mut state)) = query.get_single_mut() else {
        return;
    };

    if keys.just_pressed(KeyCode::Digit1) {
        *state = State::Attacking;
        println!("切换到攻击状态 - 动画时间将同步到攻击速度");
    }

    if keys.just_pressed(KeyCode::KeyI) {
        *state = State::Idle;
        println!("切换到待机状态");
    }

    if keys.just_pressed(KeyCode::Digit2) {
        attack.bonus_attack_speed += 0.5;
        println!(
            "增加攻击速度: +50% (总攻击速度: {:.2}, 攻击间隔: {:.2}秒)",
            attack.current_attack_speed(),
            attack.total_duration_secs()
        );
    }

    if keys.just_pressed(KeyCode::Digit3) {
        attack.bonus_attack_speed = (attack.bonus_attack_speed - 0.25).max(-0.9);
        println!(
            "减少攻击速度: -25% (总攻击速度: {:.2}, 攻击间隔: {:.2}秒)",
            attack.current_attack_speed(),
            attack.total_duration_secs()
        );
    }

    if keys.just_pressed(KeyCode::Digit4) {
        attack.bonus_attack_speed = 0.0;
        println!(
            "重置攻击速度 (总攻击速度: {:.2}, 攻击间隔: {:.2}秒)",
            attack.current_attack_speed(),
            attack.total_duration_secs()
        );
    }
}

fn test_attack_speed_changes(query: Query<(&Attack, &State), (With<TestEntity>, Changed<Attack>)>) {
    for (attack, state) in query.iter() {
        if matches!(state, State::Attacking) {
            let speed_multiplier = 1.0 / attack.total_duration_secs();
            println!(
                "攻击速度变化检测 - 动画播放速度将调整为: {:.2}x (攻击间隔: {:.2}秒)",
                speed_multiplier,
                attack.total_duration_secs()
            );
        }
    }
}
