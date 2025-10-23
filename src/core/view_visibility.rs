use bevy::{prelude::*, render::view::VisibilitySystems};

#[derive(Default)]
pub struct PluginViewVisibility;

impl Plugin for PluginViewVisibility {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            force_visibility
                .in_set(VisibilitySystems::CheckVisibility)
                .before(VisibilitySystems::MarkNewlyHiddenEntitiesInvisible),
        );
    }
}

#[derive(Component)]
pub struct ViewVisibilityForce;

pub fn force_visibility(mut query: Query<&mut ViewVisibility, With<ViewVisibilityForce>>) {
    for mut view_visibility in &mut query {
        view_visibility.set();
    }
}
