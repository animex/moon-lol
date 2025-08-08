use bevy::{prelude::*, window::CursorGrabMode};
use binrw::{binread, io::BufReader, BinRead};
use moon_lol::{combat::PluginCombat, entities::PluginEntities, render::PluginRender};
use std::fs::File;

// ====================================================================
// ↓↓↓ 将你提供的所有 binrw 结构体代码粘贴在这里 ↓↓↓
// (Header, Position, Cell3, Cell5, Cell7, AiMeshNGrid, etc.)
// ...
#[binread]
#[br(little)]
#[derive(Debug, Resource, Clone)] // <--- 为 AiMeshNGrid 添加 Resource 和 Clone
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[binread]
#[br(little)]
#[derive(Debug, Clone)]
struct Header {
    major_version: u8,
    minor_version: i16,

    #[br(if(major_version > 0))]
    min_grid_pos: Option<Position>,

    #[br(if(major_version > 0))]
    max_grid_pos: Option<Position>,

    #[br(if(major_version > 0))]
    cell_size: Option<f32>,

    #[br(if(major_version > 0))]
    x_cell_count: Option<u32>,

    #[br(if(major_version > 0))]
    y_cell_count: Option<u32>,
}

impl Header {
    fn get_x_cell_count(&self) -> u32 {
        self.x_cell_count.unwrap_or(294)
    }

    fn get_y_cell_count(&self) -> u32 {
        self.y_cell_count.unwrap_or(295)
    }
}

// 注意：为了在 Bevy 中使用，所有 Cell 和 Enum 都需要 Clone
#[binread]
#[br(little)]
#[derive(Debug, Clone)]
struct Cell3 {
    height: f32,
    unk1: u32,
    arrival: f32,
    open: u8,
    heuristic: f32,
    actors: u32,
    x: i16,
    z: i16,
    unk2: f32,
    unk3: f32,
    unk4: u32,
    session_id_related: u32,
    ref_value: f32,
    arrival_direction: i16,
    flags: i16,
    ref_nodes: [i16; 2],
    unk5: [u8; 3],
}

#[binread]
#[br(little)]
#[derive(Debug, Clone)]
struct Cell5 {
    height: f32,
    unk1: u32,
    arrival: f32,
    open: u8,
    heuristic: f32,
    actors: u32,
    x: i16,
    z: i16,
    unk2: f32,
    unk3: f32,
    unk4: u32,
    session_id_related: u32,
    ref_value: f32,
    arrival_direction: i16,
    flags: i16,
    ref_nodes: [i16; 2],
    unk5: [u8; 3],
}

#[binread]
#[br(little)]
#[derive(Debug, Clone)]
struct Cell7 {
    height: f32,
    unk1: u16,
    min_height: f32,
    unk2: u16,
    heuristic: f32,
    actors: u32,
    x: i16,
    z: i16,
    unk3: f32,
    unk4: f32,
    session_id_related: u32,
    ref_value: f32,
    arrival_direction: i16,
    flags: i16,
    ref_nodes: [i16; 2],
}

#[binread]
#[br(little)]
#[derive(Debug, Clone)]
#[br(import(version: u8))]
enum NavigationGridCell {
    #[br(pre_assert(version == 3))]
    Version3(Cell3),
    #[br(pre_assert(version == 5))]
    Version5(Cell5),
    #[br(pre_assert(version == 7))]
    Version7(Cell7),
}

#[binread]
#[br(little)]
#[derive(Debug, Clone)]
struct FloatMap {
    x_count: u32,
    y_count: u32,
    unk1: f32,
    unk2: f32,
    #[br(count = x_count * y_count)]
    unk3: Vec<f32>,
    #[br(count = 810899)]
    unk4: Vec<f32>,
    unk5: i16,
    unk6: i16,
}

#[binread]
#[br(little)]
#[derive(Debug, Resource, Clone)]
struct AiMeshNGrid {
    header: Header,
    #[br(count = header.get_x_cell_count() * header.get_y_cell_count())]
    #[br(args { inner: (header.major_version,) })]
    navigation_grid: Vec<NavigationGridCell>,
    #[br(if(header.major_version == 5))]
    #[br(count = header.get_x_cell_count() * header.get_y_cell_count())]
    version5_unk1: Option<Vec<u16>>,
    #[br(if(header.major_version == 5))]
    #[br(count = 528)]
    version5_unk2: Option<Vec<u8>>,
    #[br(if(header.major_version == 7))]
    #[br(count = header.get_x_cell_count() * header.get_y_cell_count())]
    version7_unk1: Option<Vec<u16>>,
    #[br(if(header.major_version == 7))]
    #[br(count = header.get_x_cell_count() * header.get_y_cell_count())]
    version7_unk2: Option<Vec<u16>>,
    #[br(if(header.major_version == 7))]
    #[br(count = header.get_x_cell_count() * header.get_y_cell_count())]
    version7_unk3: Option<Vec<u16>>,
    #[br(if(header.major_version == 7))]
    #[br(count = 1056)]
    version7_unk4: Option<Vec<u8>>,
    float_map: FloatMap,
}

const CELL_IS_BRUSH: i16 = 0x1;
const CELL_WALL: i16 = 0x2;
// ↑↑↑ 将你提供的所有 binrw 结构体代码粘贴在这里 ↑↑↑
// ====================================================================

fn main() {
    // 1. 在启动 Bevy 前，先加载和解析文件
    let file = File::open("assets/bloom.aimesh_ngrid").expect("找不到 aimesh_ngrid 文件！");
    let mut reader = BufReader::new(file);
    let aimesh = AiMeshNGrid::read(&mut reader).expect("解析 aimesh_ngrid 文件失败！");

    // 2. 启动 Bevy App
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PluginCombat)
        .add_plugins(PluginEntities)
        .add_plugins(PluginRender)
        // 3. 将解析出的 aimesh 数据作为 Resource 插入
        .insert_resource(aimesh)
        // 4. 添加我们的设置系统
        .add_systems(Startup, spawn_nav_grid)
        .run();
}

/// 读取 aimesh 资源并生成可视化网格
fn spawn_nav_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    aimesh: Res<AiMeshNGrid>,
) {
    // 从 Header 获取关键信息
    // 注意：这里的 .unwrap() 仅为示例，实际项目中应做更健壮的错误处理
    let min_pos = aimesh.header.min_grid_pos.as_ref().unwrap();
    let cell_size = aimesh.header.cell_size.unwrap();

    // 创建不同类型的材质
    let walkable_material = materials.add(Color::srgb(0.0, 1.0, 0.0));
    let wall_material = materials.add(Color::srgb(1.0, 0.0, 0.0));
    let brush_material = materials.add(Color::srgb(0.0, 0.0, 1.0));
    let unknown_material = materials.add(Color::srgb(1.0, 0.0, 1.0)); // 用于调试

    // 创建一个共享的方块网格，提高性能
    let cube_mesh = meshes.add(Sphere::new(cell_size / 2.0));

    println!(
        "开始生成导航网格，共 {} 个单元格...",
        aimesh.navigation_grid.len()
    );

    // 遍历所有导航单元格
    for nav_cell in aimesh.navigation_grid.iter() {
        let (x, z, height, flags) = match nav_cell {
            NavigationGridCell::Version3(c) => (c.x, c.z, c.height, c.flags),
            NavigationGridCell::Version5(c) => (c.x, c.z, c.height, c.flags),
            NavigationGridCell::Version7(c) => (c.x, c.z, c.height, c.flags),
        };

        // 跳过无效或未使用的单元格（通常 x/z 为 -1）
        if x < 0 || z < 0 {
            continue;
        }

        // --- 核心逻辑 ---
        // 1. 计算世界坐标
        let world_x = min_pos.x + (x as f32 * cell_size);
        let world_y = height;
        let world_z = -(min_pos.z + (z as f32 * cell_size));

        // 2. 根据 flags 选择材质
        let material = if (flags & CELL_WALL) != 0 {
            wall_material.clone()
        } else if (flags & CELL_IS_BRUSH) != 0 {
            brush_material.clone()
        } else {
            walkable_material.clone()
        };

        // 3. 生成实体
        commands.spawn((
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(material),
            Transform::from_xyz(world_x, world_y, world_z),
        ));
    }

    println!("导航网格生成完毕！");
}
