use bevy::prelude::*;
use bevy::render::{
    settings::{Backends, RenderCreation, WgpuSettings},
    RenderPlugin,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use moon_lol::core::{ConfigMap, PluginCamera, PluginResource};
use moon_lol::logging::PluginLogging;

fn main() {
    App::new()
        .add_plugins((
            PluginLogging,
            DefaultPlugins
                .build()
                .disable::<bevy::log::LogPlugin>()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Navigation Grid with Flag Controls".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }),
                    ..default()
                }),
            EguiPlugin::default(),
            PluginCamera,
            PluginResource,
        ))
        .init_resource::<FlagFilters>()
        .add_systems(Startup, setup)
        .add_systems(EguiPrimaryContextPass, ui_system)
        .add_systems(Update, update_grid_visibility)
        .run();
}

// Common navigation grid flags
const COMMON_FLAGS: &[u32] = &[
    0 << 0,
    1 << 0,
    1 << 1,
    1 << 2,
    1 << 3,
    1 << 4,
    1 << 5,
    1 << 6,
    1 << 7,
    1 << 8,
    1 << 9,
    1 << 10,
    1 << 11,
    1 << 12,
    1 << 13,
    1 << 14,
    1 << 15,
    1 << 16,
    1 << 17,
];

#[derive(Resource, Default)]
struct FlagFilters {
    enabled_flags: std::collections::HashSet<u32>,
    show_all: bool,
}

#[derive(Component)]
struct GridCell {
    flags: u32,
}

fn setup(
    mut commands: Commands,
    configs: Res<ConfigMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut flag_filters: ResMut<FlagFilters>,
) {
    let navigation_grid = &configs.navigation_grid;

    // Initialize to show all grid points
    flag_filters.show_all = true;

    let mesh = meshes.add(Plane3d::new(
        vec3(0.0, 1.0, 0.0),
        Vec2::splat(navigation_grid.cell_size / 2.0),
    ));
    let color = Color::srgb(1.0, 0.0, 0.0);
    let material = materials.add(color);

    for (x, row) in navigation_grid.cells.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            commands.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(navigation_grid.get_cell_pos(x, y)),
                GridCell {
                    flags: cell.vision_pathing_flags.bits() as u32,
                },
                Visibility::Visible,
            ));
        }
    }
}

fn ui_system(mut contexts: EguiContexts, mut flag_filters: ResMut<FlagFilters>) {
    egui::Window::new("Grid Point Filter")
        .default_width(300.0)
        .show(contexts.ctx_mut().unwrap(), |ui| {
            ui.heading("Display Control");

            ui.checkbox(&mut flag_filters.show_all, "Show All Grid Points");

            if !flag_filters.show_all {
                ui.separator();
                ui.heading("Filter by Flag");

                for &flag in COMMON_FLAGS {
                    let mut enabled = flag_filters.enabled_flags.contains(&flag);
                    if ui
                        .checkbox(&mut enabled, format!("Flag {}", flag))
                        .changed()
                    {
                        if enabled {
                            flag_filters.enabled_flags.insert(flag);
                        } else {
                            flag_filters.enabled_flags.remove(&flag);
                        }
                    }
                }

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Select All").clicked() {
                        for &flag in COMMON_FLAGS {
                            flag_filters.enabled_flags.insert(flag);
                        }
                    }

                    if ui.button("Deselect All").clicked() {
                        flag_filters.enabled_flags.clear();
                    }
                });
            }

            ui.separator();
            ui.label(format!(
                "Currently displayed flags: {:?}",
                if flag_filters.show_all {
                    "All".to_string()
                } else {
                    format!("{:?}", flag_filters.enabled_flags)
                }
            ));
        });
}

fn update_grid_visibility(
    flag_filters: Res<FlagFilters>,
    mut query: Query<(&GridCell, &mut Visibility)>,
) {
    if !flag_filters.is_changed() {
        return;
    }

    for (grid_cell, mut visibility) in query.iter_mut() {
        if flag_filters.show_all {
            *visibility = Visibility::Visible;
        } else {
            *visibility = if flag_filters
                .enabled_flags
                .iter()
                .all(|&flag| (grid_cell.flags & flag) != 0)
            {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}
