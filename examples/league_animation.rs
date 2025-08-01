use bevy::{pbr::MeshMaterial3d, prelude::*};
use binrw::BinRead;
use moon_lol::render::{AnimationFile, LeagueSkeleton};
use std::fs::File;
use std::io::BufReader;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let path = "assets/sruap_orderturret1_idle1.anm";
    println!("尝试解析文件: {}", path);

    let mut reader = BufReader::new(File::open(path).unwrap());
    let asset = AnimationFile::read(&mut reader).unwrap();

    // 使用 {:#?} 来进行格式化（pretty-print）输出
    match asset {
        AnimationFile::Compressed(modern) => {
            println!("解析成功，版本: Compressed");
            println!("{:#?}", modern.data.joint_hashes);
        }
        AnimationFile::Uncompressed(legacy) => {
            println!("解析成功，版本: Uncompressed");
            // println!("{:#?}", legacy.data);
            match legacy.data {
                moon_lol::render::UncompressedData::V3(uncompressed_data_v3) => {
                    println!("V3");
                }
                moon_lol::render::UncompressedData::V4(uncompressed_data_v4) => {
                    println!("V4");
                }
                moon_lol::render::UncompressedData::V5(uncompressed_data_v5) => {
                    println!("V5{:#?}", uncompressed_data_v5.joint_hashes);
                }
            }
        }
    }

    let animation_clip = AnimationClip::default();

    animation_clip.add_curve_to_target(target_id, curve);
    AnimatableCurve::new(property, curve)

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-250.0, 1000.0, 1000.0).looking_at(Vec3::new(0.0, 50.0, 0.0), Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, -0.8, -0.5, 0.0)),
    ));

    println!("正在解析骨骼文件: assets/turret.skl");
    let path = "assets/turret.skl";
    let skeleton = match File::open(path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            LeagueSkeleton::read(&mut reader).expect("解析 .skl 文件失败")
        }
        Err(e) => {
            eprintln!("错误: 无法打开文件 '{}'。", path);
            eprintln!("具体错误: {}", e);
            return;
        }
    };
    let joints = &skeleton.modern_data.joints;

    let sphere_mesh_handle = meshes.add(Sphere::new(3.0));
    let sphere_material_handle = materials.add(Color::srgb(1.0, 0.2, 0.2));
    let mut joint_entities = vec![None; joints.len()];

    for (i, joint) in joints.iter().enumerate() {
        let current_entity = commands
            .spawn((
                Mesh3d(sphere_mesh_handle.clone()),
                MeshMaterial3d(sphere_material_handle.clone()),
                Transform::from_matrix(joint.local_transform),
                Name::new(joint.name.clone()),
            ))
            .id();

        joint_entities[i] = Some(current_entity);

        if joint.parent_id >= 0 {
            if let Some(parent_entity) = joint_entities[joint.parent_id as usize] {
                commands.entity(parent_entity).add_child(current_entity);
            }
        }
    }
}
