# 动画时间同步系统

## 概述

动画时间同步系统提供了一种优雅的方式来根据组件的时间属性动态调整动画播放速度。这个系统特别适用于攻击动画需要根据攻击速度调整，以及其他需要时间同步的动画场景。

## 核心组件

### `AnimationTimingSync`

用于配置哪些动画需要与特定组件的时间属性同步。

### `AnimationSpeed`

存储和管理动画的播放速度倍率。

## 使用方法

### 1. 攻击动画同步

为实体添加攻击动画时间同步：

```rust
use crate::core::{Attack, AnimationTimingSync, Animation, AnimationState};

// 在生成实体时添加动画时间同步组件
commands.entity(entity)
    .insert(Attack::new(150.0, 0.3, 1.0)) // 攻击范围150，前摇0.3秒，总时间1秒
    .insert(AnimationTimingSync::for_attack()) // 使用预设的攻击动画同步配置
    .insert(Animation { /* ... */ })
    .insert(AnimationState { /* ... */ });
```

### 2. 自定义动画同步

为其他组件创建自定义动画同步：

```rust
// 例如：技能冷却动画同步
let skill_timing_sync = AnimationTimingSync::new(
    "Skill".to_string(),           // 组件名称
    "cooldown_duration".to_string(), // 时间字段名称
    vec!["SkillCast", "SkillChannel"] // 需要同步的动画
);

commands.entity(entity)
    .insert(skill_timing_sync);
```

### 3. 动态添加动画

可以动态添加需要同步的动画：

```rust
let timing_sync = AnimationTimingSync::for_attack()
    .with_animation("SpecialAttack")
    .with_animation("ComboAttack");
```

## 工作原理

1. **时间检测**: `update_animation_timing_sync` 系统监控具有 `AnimationTimingSync` 组件的实体
2. **速度计算**: 当实体播放同步动画时，系统通过反射获取目标组件的时间属性
3. **速度应用**: 计算出的速度倍率通过 `AnimationSpeed` 组件存储，并由 `apply_animation_speed` 系统应用到 `AnimationPlayer`

## 速度计算公式

```
speed_multiplier = 1.0 / duration
```

- 如果攻击总时间为 0.5 秒，动画播放速度为 2.0 倍
- 如果攻击总时间为 2.0 秒，动画播放速度为 0.5 倍

## 预设配置

### `AnimationTimingSync::for_attack()`

预配置的攻击动画同步，包含以下动画：

- Attack1
- Attack2
- Attack3
- Crit

## 扩展性

这个系统设计为通用的，可以轻松扩展到其他需要时间同步的场景：

- 移动动画与移动速度同步
- 技能动画与技能持续时间同步
- 生产动画与生产时间同步
- 任何需要时间匹配的动画场景

## 性能考虑

- 系统只在动画状态改变时进行计算
- 使用反射获取组件值，但仅在必要时执行
- 速度变化检测避免不必要的 AnimationPlayer 更新
