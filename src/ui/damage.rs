use bevy::color::palettes::css::{BLUE, RED, WHITE};
use bevy::prelude::*;

use crate::{DamageType, EventDamageCreate};

#[derive(Default)]
pub struct PluginUIDamage;

impl Plugin for PluginUIDamage {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_damage_numbers);
        app.add_observer(on_event_damage_create);
    }
}

/// Damage number component - used to display floating damage numbers
#[derive(Component)]
pub struct DamageNumber {
    /// Damage value
    pub damage: f32,
    /// Lifetime (seconds)
    pub lifetime: f32,
    /// Maximum lifetime
    pub max_lifetime: f32,
    /// Initial position
    pub start_position: Vec3,
    /// Vertical velocity
    pub velocity_y: f32,
    /// Gravity acceleration
    pub gravity: f32,
    /// Final font size
    pub final_scale: f32,
}

/// Listen for damage events and create damage numbers
fn on_event_damage_create(
    trigger: On<EventDamageCreate>,
    mut commands: Commands,
    global_transform: Query<&GlobalTransform>,
) {
    let target_entity = trigger.event_target();
    let damage_result = &trigger.damage_result;

    // Only display actual damage dealt
    if damage_result.final_damage <= 0.0 {
        return;
    }

    // Get target entity's position
    let Ok(target_transform) = global_transform.get(target_entity) else {
        return;
    };

    let world_position = target_transform.translation();

    // Create damage number UI
    commands.spawn((
        Text::new(format!("{:.0}", damage_result.final_damage)),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::Srgba(match trigger.damage_type {
            DamageType::Physical => RED,
            DamageType::Magic => BLUE,
            DamageType::True => WHITE,
        })),
        Node {
            position_type: PositionType::Absolute,
            ..default()
        },
        DamageNumber {
            damage: damage_result.final_damage,
            lifetime: 0.0,
            max_lifetime: 1.0, // 1 second lifetime
            start_position: world_position,
            velocity_y: 250.0, // Initial upward velocity
            gravity: -200.0,   // Gravity acceleration
            final_scale: 0.5,
        },
    ));
}

/// Update damage number animation effects
fn update_damage_numbers(
    mut commands: Commands,
    time: Res<Time>,
    camera_info: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut damage_numbers: Query<(
        Entity,
        &mut UiTransform,
        &mut DamageNumber,
        &mut Node,
        &mut TextColor,
    )>,
) {
    let (camera, camera_global_transform) = camera_info.into_inner();
    let delta_time = time.delta_secs();

    for (i, (entity, mut transform, mut damage_number, mut node, mut text_color)) in
        damage_numbers.iter_mut().enumerate()
    {
        // Update lifetime
        damage_number.lifetime += delta_time;

        // If exceeded lifetime, destroy entity
        if damage_number.lifetime >= damage_number.max_lifetime {
            commands.entity(entity).despawn();
            continue;
        }

        // Calculate animation progress (0.0 to 1.0)
        let progress = damage_number.lifetime / damage_number.max_lifetime;

        // Update vertical velocity (gravity effect)
        damage_number.velocity_y += damage_number.gravity * delta_time;

        // Calculate current position
        let current_y_offset = damage_number.velocity_y * damage_number.lifetime
            + 0.5 * damage_number.gravity * damage_number.lifetime * damage_number.lifetime;

        let current_world_pos =
            damage_number.start_position + Vec3::new(0.0, current_y_offset, 0.0);

        // Convert to screen coordinates
        if let Ok(viewport_position) =
            camera.world_to_viewport(camera_global_transform, current_world_pos)
        {
            node.left = Val::Px(viewport_position.x - 20.0); // Center offset
            node.top = Val::Px(viewport_position.y + i as f32 * 20.);
        }

        // Font size animation: from large to small
        let current_font_size = 1. - (1. - damage_number.final_scale) * progress;

        transform.scale = Vec2::splat(current_font_size);

        // Opacity animation: gradually fade out
        let alpha = 1.0 - progress;
        text_color.0 = text_color.0.with_alpha(alpha);
    }
}
