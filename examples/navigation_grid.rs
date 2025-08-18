use std::time::Instant;

use bevy::asset::RenderAssetUsages;
use bevy::color::palettes;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::{
    settings::{Backends, RenderCreation, WgpuSettings},
    RenderPlugin,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use moon_lol::core::{
    find_grid_path_with_result, on_click_map, post_process_path, simplify_path, AStarResult,
    CommandMovementFollowPath, CommandNavigationTo, ConfigMap, Map, MovementState, PluginCore,
    PluginNavigaton,
};
use moon_lol::entities::PluginEntities;
use moon_lol::league::VisionPathingFlags;
use moon_lol::logging::PluginLogging;
use moon_lol::system_debug;

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
            PluginCore.build().disable::<PluginNavigaton>(),
            PluginEntities,
        ))
        .init_resource::<FlagFilters>()
        .add_systems(Startup, setup)
        .add_systems(EguiPrimaryContextPass, ui_system)
        .add_systems(Update, update_grid_visibility)
        .add_systems(Update, on_key_space)
        // override navigation plugin
        .init_resource::<AStarVisualization>()
        .add_systems(Update, draw_move_path)
        .add_systems(Update, update_astar_visualization)
        .add_observer(command_navigation_to)
        // sample height
        .add_systems(Startup, setup_sample_height_textured)
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
        Vec2::splat(navigation_grid.cell_size / 2.0 - 5.0),
    ));

    let red_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.0, 0.0),
        unlit: true,
        depth_bias: 100.0,
        ..default()
    });

    let green_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 1.0, 0.0),
        unlit: true,
        depth_bias: 100.0,
        ..default()
    });

    for (x, row) in navigation_grid.cells.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            let position = navigation_grid.get_cell_center_position_by_xy((x, y));
            commands
                .spawn((
                    Mesh3d(mesh.clone()),
                    // MeshMaterial3d(materials.add(StandardMaterial {
                    //     base_color: Color::srgb(position.y / 100.0, 0.0, 0.0),
                    //     unlit: true,
                    //     ..default()
                    // })),
                    MeshMaterial3d(
                        if cell.vision_pathing_flags.contains(VisionPathingFlags::Wall) {
                            red_material.clone()
                        } else {
                            green_material.clone()
                        },
                    ),
                    Transform::from_translation(position),
                    GridCell {
                        flags: cell.vision_pathing_flags.bits() as u32,
                    },
                    Visibility::Visible,
                ))
                .observe(on_click_map);
        }
    }
}

fn setup_sample_height_textured(
    mut commands: Commands,
    configs: Res<ConfigMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>, // 新增：用于创建贴图的资源
) {
    let navigation_grid = &configs.navigation_grid;

    // 1. 计算高度图的尺寸和颜色范围
    let width = navigation_grid.height_x_len;
    let height = navigation_grid.height_samples.len() / width;

    if width == 0 || height == 0 {
        return; // 避免除零错误
    }

    let max_sample_height = navigation_grid
        .height_samples
        .iter()
        .copied()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(1.0);
    let min_sample_height = navigation_grid
        .height_samples
        .iter()
        .copied()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0);

    let height_range = max_sample_height - min_sample_height;
    // 如果所有高度都一样，防止除零
    let height_range = if height_range == 0.0 {
        1.0
    } else {
        height_range
    };

    // 2. 创建图像数据 (像素字节)
    // 我们将为每个高度样本创建一个像素。格式为 RGBA8。
    let mut image_data = Vec::with_capacity(width * height * 4);
    for h_val in &navigation_grid.height_samples {
        let normalized_height = (h_val - min_sample_height) / height_range;

        // 将 0.0-1.0 的浮点数颜色转换为 0-255 的 u8 字节
        let r = (normalized_height * 255.0) as u8;
        let g = 0;
        let b = 0;
        let a = 255; // Alpha通道，255表示完全不透明

        image_data.extend_from_slice(&[r, g, b, a]);
    }

    // 3. 从原始数据创建 Bevy Image 资源
    let mut image = Image::new(
        Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        image_data,
        // 使用 sRGB 格式以获得正确的颜色显示
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );

    // 使用最近邻采样，避免像素模糊，让网格看起来更清晰
    image.sampler = ImageSampler::nearest();

    let image_handle = images.add(image);

    // 4. 创建一个大的平面网格来承载贴图
    let cell_size = 2.0;
    let plane_width = width as f32 * cell_size;
    let plane_z_height = height as f32 * cell_size;

    let plane_mesh = meshes.add(Plane3d::new(
        Vec3::Y, // 法线朝上
        Vec2::new(plane_width, plane_z_height) / 2.0,
    ));

    // 5. 创建使用该贴图的材质
    let plane_material = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        unlit: true, // 不受光照影响，以显示原始颜色
        ..default()
    });

    // 6. 生成单个实体
    // 将平面的中心点对齐到网格的中心，以匹配原来的坐标系
    let transform = Transform::from_xyz(
        (plane_width - cell_size) / 2.0,
        0.0,
        -((plane_z_height - cell_size) / 2.0),
    );

    commands.spawn((
        Mesh3d(plane_mesh),
        MeshMaterial3d(plane_material),
        transform,
    ));
    // commands.spawn(PbrBundle {
    //     mesh: plane_mesh,
    //     material: plane_material,
    //     transform,
    //     ..default()
    // });

    // 注意：原来的 .observe(on_click_map) 逻辑需要改变，见下方说明
}

fn on_key_space(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_map: Query<&mut Visibility, With<Map>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut visibility in q_map.iter_mut() {
            visibility.toggle_visible_hidden();
        }
    }
}

fn draw_move_path(
    config_map: Res<ConfigMap>,
    mut gizmos: Gizmos,
    q_movement_path: Query<&MovementState>,
) {
    for movement_path in q_movement_path.iter() {
        // 绘制路径线段
        for path_point in movement_path.path.windows(2) {
            gizmos.line(
                config_map
                    .navigation_grid
                    .get_world_position_by_position(&path_point[0])
                    + vec3(0.0, 20.0, 0.0),
                config_map
                    .navigation_grid
                    .get_world_position_by_position(&path_point[1])
                    + vec3(0.0, 20.0, 0.0),
                Color::Srgba(palettes::tailwind::BLUE_500),
            );
        }
    }
}

fn ui_system(
    mut contexts: EguiContexts,
    mut flag_filters: ResMut<FlagFilters>,
    mut astar_vis: ResMut<AStarVisualization>,
) {
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

            ui.separator();
            ui.heading("A* Visualization");

            ui.checkbox(
                &mut astar_vis.show_visited,
                "Show A* Visited Cells (Yellow)",
            );

            ui.checkbox(&mut astar_vis.show_path, "Show A* Path Cells (White)");

            ui.separator();

            ui.label(format!(
                "Last search visited {} cells",
                astar_vis.visited_cells.len()
            ));

            ui.label(format!("Path length: {} cells", astar_vis.path_cells.len()));

            ui.horizontal(|ui| {
                if ui.button("Show Both").clicked() {
                    astar_vis.show_visited = true;
                    astar_vis.show_path = true;
                }

                if ui.button("Hide All").clicked() {
                    astar_vis.show_visited = false;
                    astar_vis.show_path = false;
                }
            });
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

// A* 可视化相关组件和资源
#[derive(Component)]
struct AStarCell;

#[derive(Component)]
struct AStarPathCell;

#[derive(Resource, Default)]
pub struct AStarVisualization {
    pub visited_cells: Vec<(usize, usize)>,
    pub path_cells: Vec<(usize, usize)>,
    pub show_visited: bool,
    pub show_path: bool,
}

fn command_navigation_to(
    trigger: Trigger<CommandNavigationTo>,
    mut commands: Commands,
    configs: Res<ConfigMap>,
    mut q_transform: Query<&Transform>,
    mut astar_vis: ResMut<AStarVisualization>,
) {
    let entity = trigger.target();
    let destination = trigger.event().0;

    // 获取当前位置
    if let Ok(transform) = q_transform.get_mut(entity) {
        let start_pos = transform.translation;
        let end_pos = Vec3::new(destination.x, start_pos.y, destination.y);

        let start = Instant::now();
        // 使用A*算法规划路径，对于单点移动，创建长度为1的路径
        if let Some(result) = find_path_with_visualization(&configs, start_pos, end_pos) {
            let duration = start.elapsed();

            system_debug!(
                "command_movement_move_to",
                "Path found in {:.6}ms",
                duration.as_millis()
            );

            // 更新 A* 可视化数据
            astar_vis.visited_cells = result.visited_cells;
            astar_vis.path_cells = result.path.clone();

            if result.path.is_empty() {
                return;
            }

            let simplified_path = simplify_path(&result.path);

            let world_path = post_process_path(
                &configs.navigation_grid,
                &simplified_path,
                &start_pos,
                &end_pos,
            );

            commands.trigger_targets(CommandMovementFollowPath(world_path), entity);
        }
    }
}

/// 带可视化的寻路函数
fn find_path_with_visualization(
    configs: &ConfigMap,
    start: Vec3,
    end: Vec3,
) -> Option<AStarResult> {
    let grid = &configs.navigation_grid;

    // 使用带结果的A*算法
    if let Some(astar_result) = find_grid_path_with_result(grid, start, end) {
        // 注意：这里不对路径进行简化，保持原始的A*网格路径
        // 简化路径只在最终转换为世界坐标时使用
        Some(astar_result)
    } else {
        None
    }
}

/// 更新 A* 可视化系统
fn update_astar_visualization(
    mut commands: Commands,
    configs: Res<ConfigMap>,
    astar_vis: Res<AStarVisualization>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    visited_query: Query<Entity, With<AStarCell>>,
    path_query: Query<Entity, With<AStarPathCell>>,
) {
    if !astar_vis.is_changed() {
        return;
    }

    // 删除旧的 A* 可视化单元格
    for entity in visited_query.iter() {
        commands.entity(entity).despawn();
    }

    for entity in path_query.iter() {
        commands.entity(entity).despawn();
    }

    let mesh = meshes.add(Plane3d::new(
        vec3(0.0, 1.0, 0.0),
        Vec2::splat(configs.navigation_grid.cell_size / 2.0 - 3.0),
    ));

    // 如果需要显示访问的单元格（黄色）
    if astar_vis.show_visited {
        let yellow_material = materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 0.0), // 黄色
            unlit: true,
            depth_bias: 50.0,
            ..default()
        });

        // 创建访问过的单元格
        for &(x, y) in &astar_vis.visited_cells {
            commands.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(yellow_material.clone()),
                Transform::from_translation(
                    configs
                        .navigation_grid
                        .get_cell_center_position_by_xy((x, y))
                        + Vec3::new(0.0, 10.0, 0.0),
                ),
                AStarCell,
                Visibility::Visible,
            ));
        }
    }

    // 如果需要显示路径单元格（白色）
    if astar_vis.show_path {
        let white_material = materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 1.0), // 白色
            unlit: true,
            depth_bias: 60.0, // 比黄色稍高一点，确保显示在上面
            ..default()
        });

        // 创建路径单元格
        for &(x, y) in &astar_vis.path_cells {
            commands.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(white_material.clone()),
                Transform::from_translation(
                    configs
                        .navigation_grid
                        .get_cell_center_position_by_xy((x, y))
                        + Vec3::new(0.0, 15.0, 0.0), // 比黄色更高
                ),
                AStarPathCell,
                Visibility::Visible,
            ));
        }
    }
}
