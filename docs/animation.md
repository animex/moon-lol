//! Create and play an animation defined by code that operates on the [`Transform`] component.

use std::f32::consts::PI;

use bevy::{
    animation::{animated_field, AnimationTarget, AnimationTargetId},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 150.0,
            ..default()
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animations: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Light
    commands.spawn((
        PointLight {
            intensity: 500_000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 2.5, 0.0),
    ));

    // Let's use the `Name` component to target entities. We can use anything we
    // like, but names are convenient.
    let planet = Name::new("planet");
    let orbit_controller = Name::new("orbit_controller");
    let satellite = Name::new("satellite");

    // Creating the animation
    let mut animation = AnimationClip::default();
    // A curve can modify a single part of a transform: here, the translation.
    let planet_animation_target_id = AnimationTargetId::from_name(&planet);
    animation.add_curve_to_target(
        planet_animation_target_id,
        AnimatableCurve::new(
            animated_field!(Transform::translation),
            UnevenSampleAutoCurve::new([0.0, 1.0, 2.0, 3.0, 4.0].into_iter().zip([
                Vec3::new(1.0, 0.0, 1.0),
                Vec3::new(-1.0, 0.0, 1.0),
                Vec3::new(-1.0, 0.0, -1.0),
                Vec3::new(1.0, 0.0, -1.0),
                // in case seamless looping is wanted, the last keyframe should
                // be the same as the first one
                Vec3::new(1.0, 0.0, 1.0),
            ]))
            .expect("should be able to build translation curve because we pass in valid samples"),
        ),
    );
    // Or it can modify the rotation of the transform.
    // To find the entity to modify, the hierarchy will be traversed looking for
    // an entity with the right name at each level.
    let orbit_controller_animation_target_id =
        AnimationTargetId::from_names([planet.clone(), orbit_controller.clone()].iter());
    animation.add_curve_to_target(
        orbit_controller_animation_target_id,
        AnimatableCurve::new(
            animated_field!(Transform::rotation),
            UnevenSampleAutoCurve::new([0.0, 1.0, 2.0, 3.0, 4.0].into_iter().zip([
                Quat::IDENTITY,
                Quat::from_axis_angle(Vec3::Y, PI / 2.),
                Quat::from_axis_angle(Vec3::Y, PI / 2. * 2.),
                Quat::from_axis_angle(Vec3::Y, PI / 2. * 3.),
                Quat::IDENTITY,
            ]))
            .expect("Failed to build rotation curve"),
        ),
    );
    // If a curve in an animation is shorter than the other, it will not repeat
    // until all other curves are finished. In that case, another animation should
    // be created for each part that would have a different duration / period.
    let satellite_animation_target_id = AnimationTargetId::from_names(
        [planet.clone(), orbit_controller.clone(), satellite.clone()].iter(),
    );
    animation.add_curve_to_target(
        satellite_animation_target_id,
        AnimatableCurve::new(
            animated_field!(Transform::scale),
            UnevenSampleAutoCurve::new(
                [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0]
                    .into_iter()
                    .zip([
                        Vec3::splat(0.8),
                        Vec3::splat(1.2),
                        Vec3::splat(0.8),
                        Vec3::splat(1.2),
                        Vec3::splat(0.8),
                        Vec3::splat(1.2),
                        Vec3::splat(0.8),
                        Vec3::splat(1.2),
                        Vec3::splat(0.8),
                    ]),
            )
            .expect("Failed to build scale curve"),
        ),
    );
    // There can be more than one curve targeting the same entity path.
    animation.add_curve_to_target(
        AnimationTargetId::from_names(
            [planet.clone(), orbit_controller.clone(), satellite.clone()].iter(),
        ),
        AnimatableCurve::new(
            animated_field!(Transform::rotation),
            UnevenSampleAutoCurve::new([0.0, 1.0, 2.0, 3.0, 4.0].into_iter().zip([
                Quat::IDENTITY,
                Quat::from_axis_angle(Vec3::Y, PI / 2.),
                Quat::from_axis_angle(Vec3::Y, PI / 2. * 2.),
                Quat::from_axis_angle(Vec3::Y, PI / 2. * 3.),
                Quat::IDENTITY,
            ]))
            .expect("should be able to build translation curve because we pass in valid samples"),
        ),
    );

    // Create the animation graph
    let (graph, animation_index) = AnimationGraph::from_clip(animations.add(animation));

    // Create the animation player, and set it to repeat
    let mut player = AnimationPlayer::default();
    player.play(animation_index).repeat();

    // Create the scene that will be animated
    // First entity is the planet
    let planet_entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::default())),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            // Add the animation graph and player
            planet,
            AnimationGraphHandle(graphs.add(graph)),
            player,
        ))
        .id();
    commands
        .entity(planet_entity)
        .insert(AnimationTarget {
            id: planet_animation_target_id,
            player: planet_entity,
        })
        .with_children(|p| {
            // This entity is just used for animation, but doesn't display anything
            p.spawn((
                Transform::default(),
                Visibility::default(),
                orbit_controller,
                AnimationTarget {
                    id: orbit_controller_animation_target_id,
                    player: planet_entity,
                },
            ))
            .with_children(|p| {
                // The satellite, placed at a distance of the planet
                p.spawn((
                    Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
                    MeshMaterial3d(materials.add(Color::srgb(0.3, 0.9, 0.3))),
                    Transform::from_xyz(1.5, 0.0, 0.0),
                    AnimationTarget {
                        id: satellite_animation_target_id,
                        player: planet_entity,
                    },
                    satellite,
                ));
            });
        });
}


use crate::render::{BinQuat, BinVec3, LeagueLoader};
use bevy::math::{Quat, Vec3};
use binrw::io::{Read, Seek, SeekFrom};
use binrw::{binread, BinRead};
use binrw::{prelude::*, Endian};
use std::collections::HashMap;
use std::f32::consts::SQRT_2;
use std::fmt::Debug;

#[binread]
#[derive(Debug)]
#[br(little)]
pub struct UncompressedAnimationAsset {
    pub version: u32,

    #[br(args { version })]
    pub data: UncompressedData,
}

#[binread]
#[derive(Debug)]
#[br(little, import { version: u32 })]
pub enum UncompressedData {
    #[br(pre_assert(version == 3))]
    V3(#[br(parse_with = parse_uncompressed_data_v3)] UncompressedDataV3),
    #[br(pre_assert(version == 4))]
    V4(UncompressedDataV4),
    #[br(pre_assert(version == 5))]
    V5(UncompressedDataV5),
}

// ------------------- Version 5 -------------------
#[binread]
#[derive(Debug)]
#[br(little)]
pub struct UncompressedDataV5 {
    pub resource_size: u32,
    pub format_token: u32,
    pub version_again: u32, // Should be 5
    pub flags: u32,
    pub track_count: i32,
    pub frame_count: i32,
    pub frame_duration: f32,

    #[br(temp)]
    joint_name_hashes_offset: i32,
    #[br(temp)]
    asset_name_offset: i32,
    #[br(temp)]
    time_offset: i32,
    #[br(temp)]
    vector_palette_offset: i32,
    #[br(temp)]
    quat_palette_offset: i32,
    #[br(temp)]
    frames_offset: i32,

    #[br(
        seek_before = SeekFrom::Start(joint_name_hashes_offset as u64 + 12),
        count = (frames_offset - joint_name_hashes_offset) / 4
    )]
    pub joint_hashes: Vec<u32>,

    #[br(
        seek_before = SeekFrom::Start(vector_palette_offset as u64 + 12),
        count = (quat_palette_offset - vector_palette_offset) / 12
    )]
    pub vector_palette: Vec<BinVec3>,

    #[br(
        seek_before = SeekFrom::Start(quat_palette_offset as u64 + 12),
        count = (joint_name_hashes_offset - quat_palette_offset) / 6,
        map = |vals: Vec<[u8; 6]>| vals.into_iter().map(decompress_quat).map(|v| BinQuat(v)).collect()
    )]
    pub quat_palette: Vec<BinQuat>,

    #[br(
        seek_before = SeekFrom::Start(frames_offset as u64 + 12),
        count = track_count * frame_count
    )]
    pub frames: Vec<UncompressedFrame>,
}

// ------------------- Version 4 -------------------
#[binread]
#[derive(Debug)]
#[br(little)]
pub struct UncompressedDataV4 {
    pub resource_size: u32,
    pub format_token: u32,
    pub version_again: u32, // Should be 4
    pub flags: u32,
    pub track_count: i32,
    pub frame_count: i32,
    pub frame_duration: f32,

    #[br(temp)]
    joint_name_hashes_offset: i32,
    #[br(temp)]
    asset_name_offset: i32,
    #[br(temp)]
    time_offset: i32,
    #[br(temp)]
    vector_palette_offset: i32,
    #[br(temp)]
    quat_palette_offset: i32,
    #[br(temp)]
    frames_offset: i32,

    #[br(
        seek_before = SeekFrom::Start(vector_palette_offset as u64 + 12),
        count = (quat_palette_offset - vector_palette_offset) / 12
    )]
    pub vector_palette: Vec<BinVec3>,

    #[br(
        seek_before = SeekFrom::Start(quat_palette_offset as u64 + 12),
        count = (frames_offset - quat_palette_offset) / 16
    )]
    pub quat_palette: Vec<BinQuat>,

    #[br(
        seek_before = SeekFrom::Start(frames_offset as u64 + 12),
        count = (track_count * frame_count) as usize,
        map = |frames: Vec<UncompressedFrameV4>| group_v4_frames(frames)
    )]
    pub joint_frames: HashMap<u32, Vec<UncompressedFrame>>,
}

#[binread]
#[derive(Debug)]
#[br(little)]
pub struct UncompressedFrameV4 {
    pub joint_hash: u32,
    pub frame: UncompressedFrame,
    pub padding: u16,
}

fn group_v4_frames(frames: Vec<UncompressedFrameV4>) -> HashMap<u32, Vec<UncompressedFrame>> {
    let mut map = HashMap::new();
    for frame_v4 in frames {
        map.entry(frame_v4.joint_hash)
            .or_insert_with(Vec::new)
            .push(frame_v4.frame);
    }
    map
}

// ------------------- Version 3 (Legacy) -------------------

// 用于V3解析的辅助结构体。它们使用宏来读取数据块。
#[binread]
#[derive(Debug)]
#[br(little, import { frame_count: i32 })]
struct RawTrackV3 {
    track_name_bytes: [u8; 32],
    _flags: u32,
    #[br(count = frame_count)]
    frames: Vec<RawFrameV3>,
}

#[binread]
#[derive(Debug)]
#[br(little)]
struct RawFrameV3 {
    rotation: BinQuat,
    translation: BinVec3,
}

/// V3 数据的最终结构。它本身不派生 BinRead。
#[derive(Debug)]
pub struct UncompressedDataV3 {
    pub skeleton_id: u32,
    pub track_count: i32,
    pub frame_count: i32,
    pub fps: i32,
    pub joint_frames: HashMap<u32, Vec<UncompressedFrame>>,
    pub vector_palette: Vec<BinVec3>,
    pub quat_palette: Vec<BinQuat>,
}

#[binread]
#[derive(Debug)]
#[br(little)]
struct RawDataV3 {
    skeleton_id: u32,
    track_count: i32,
    frame_count: i32,
    fps: i32,
    #[br(count = track_count, args { inner: RawTrackV3BinReadArgs { frame_count } })]
    tracks: Vec<RawTrackV3>,
}

/// 自定义解析函数，它使用辅助结构体来读取原始数据，
/// 然后将其转换为最终的 UncompressedDataV3 结构。
/// 这取代了手动的 `impl BinRead`。
fn parse_uncompressed_data_v3<R: Read + Seek>(
    reader: &mut R,
    _: Endian,
    _: (),
) -> BinResult<UncompressedDataV3> {
    // 定义一个临时结构体，使用 binrw 宏来读取原始数据块

    // 从流中读取原始数据结构
    let raw = RawDataV3::read(reader)?;

    // 现在，将原始数据转换为所需的 UncompressedDataV3 结构
    let track_count = raw.track_count;
    let frame_count = raw.frame_count;
    let mut joint_frames = HashMap::with_capacity(track_count as usize);
    let palette_size = (track_count * frame_count) as usize;
    let mut quat_palette = Vec::with_capacity(palette_size);
    // +1 是为人为添加的静态缩放向量
    let mut vector_palette = Vec::with_capacity(palette_size + 1);
    vector_palette.push(BinVec3(Vec3::ONE));

    for (i, raw_track) in raw.tracks.into_iter().enumerate() {
        let track_name = String::from_utf8_lossy(&raw_track.track_name_bytes)
            .trim_end_matches('\0')
            .to_string();

        // C# 代码使用 Elf.HashLower，我们在此模拟一个简单的小写哈希
        // 为了完美匹配，你需要实现确切的 Elf 哈希算法。
        let joint_hash = LeagueLoader::compute_binhash(&track_name);

        let mut frames_for_joint = Vec::with_capacity(frame_count as usize);

        for (j, raw_frame) in raw_track.frames.into_iter().enumerate() {
            let index = i * frame_count as usize + j;

            quat_palette.push(raw_frame.rotation);
            vector_palette.push(raw_frame.translation);

            frames_for_joint.push(UncompressedFrame {
                rotation_id: index as u16,
                // 旧版格式可能没有缩放，C# 默认使用索引为 0 的静态 Vector3.One
                scale_id: 0,
                translation_id: (index + 1) as u16,
            });
        }
        joint_frames.insert(joint_hash, frames_for_joint);
    }

    Ok(UncompressedDataV3 {
        skeleton_id: raw.skeleton_id,
        track_count,
        frame_count,
        fps: raw.fps,
        joint_frames,
        vector_palette,
        quat_palette,
    })
}

// ------------------- 通用未压缩结构 -------------------
#[derive(BinRead, Debug, Clone, Copy)]
#[br(little)]
pub struct UncompressedFrame {
    pub translation_id: u16,
    pub scale_id: u16,
    pub rotation_id: u16,
}

// **************************************************************************
// * 辅助函数和类型
// **************************************************************************

// 修正后的函数
fn decompress_quat(data: [u8; 6]) -> Quat {
    // 1. 将6字节数据（3个u16）合并成一个u64，与C#逻辑保持一致
    // C#的ReadOnlySpan<ushort>和位移操作表明数据是按小端（Little Endian）u16处理的
    let d0 = u16::from_le_bytes([data[0], data[1]]);
    let d1 = u16::from_le_bytes([data[2], data[3]]);
    let d2 = u16::from_le_bytes([data[4], data[5]]);

    // 使用 u64 来进行位操作，避免溢出
    let bits: u64 = (d0 as u64) | ((d1 as u64) << 16) | ((d2 as u64) << 32);

    // 2. 严格按照 C# 的位布局提取数据
    let max_index = (bits >> 45) & 0x03; // 2-bit index
    let v_a = (bits >> 30) & 0x7FFF; // 15-bit value
    let v_b = (bits >> 15) & 0x7FFF; // 15-bit value
    let v_c = bits & 0x7FFF; // 15-bit value

    const ONE_OVER_SQRT_2: f32 = 0.70710678118; // 1.0 / SQRT_2
                                                // 3. 应用与 C# 完全相同的反量化公式
                                                // C# 中使用 double 进行除法和常量计算，这里用 f32 可能会有微小精度差异
    let a = (v_a as f32 / 32767.0) * SQRT_2 - ONE_OVER_SQRT_2;
    let b = (v_b as f32 / 32767.0) * SQRT_2 - ONE_OVER_SQRT_2;
    let c = (v_c as f32 / 32767.0) * SQRT_2 - ONE_OVER_SQRT_2;

    // 4. 计算第四个分量，并加入安全检查
    let sum_sq = a * a + b * b + c * c;
    let d = (1.0 - sum_sq).max(0.0).sqrt();

    // 5. 根据索引重构四元数
    // 这里的 Quat::from_xyzw 是一个示例，请使用你实际的库函数
    let quat = match max_index {
        0 => Quat::from_xyzw(d, a, b, c), // x was omitted
        1 => Quat::from_xyzw(a, d, b, c), // y was omitted
        2 => Quat::from_xyzw(a, b, d, c), // z was omitted
        _ => Quat::from_xyzw(a, b, c, d), // w was omitted
    };

    // C# 版本没有显式调用 normalize，因为计算结果理论上就是单位化的。
    // 如果需要更高的精度保证，可以取消下面这行注释。
    // return BinQuat(quat.normalize());

    quat
}

use crate::render::{BinQuat, BinVec3};
use bevy::math::{Mat4, Quat, Vec3, Vec4};
use binrw::io::{Read, Seek, SeekFrom};
use binrw::{binread, BinRead};
use binrw::{prelude::*, Endian};

// ===================================================================================
// 1. 常量和公共数据结构 (Public API)
// ===================================================================================

const FORMAT_TOKEN: u32 = 0x22FD4FC3; // FNV hash of the format token string

/// 顶层骨骼文件结构体。
/// 这是解析的入口点。
#[binread]
#[derive(Debug)]
#[br(little)]
pub struct LeagueSkeleton {
    #[br(temp)]
    _file_size: u32,

    #[br(temp)]
    format_token: u32,

    // 使用 args 传递 format_token，并用 map 将解析结果转换为最终的 SkeletonData
    #[br(args(format_token))]
    #[br(map = |kind: SkeletonDataKind| kind.into())]
    pub modern_data: SkeletonData,
}

/// 统一的骨骼数据，无论是从现代格式还是旧版格式解析而来。
#[derive(Debug)]
pub struct SkeletonData {
    pub flags: u16,
    pub name: String,
    pub asset_name: String,
    pub joints: Vec<Joint>,
    pub influences: Vec<i16>,
}

/// 最终在应用程序中使用的关节数据结构。
#[derive(Debug, Clone)]
pub struct Joint {
    pub name: String,
    pub flags: u16,
    pub id: i16,
    pub parent_id: i16,
    pub radius: f32,
    pub local_transform: Mat4,
    pub inverse_bind_transform: Mat4,
}

// ===================================================================================
// 2. 解析器实现 (Parser Implementation)
// ===================================================================================

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ 条件解析 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// 这是一个中间枚举，用于根据 `format_token` 进行条件解析。
#[binread]
#[derive(Debug)]
#[br(import(format_token: u32))]
enum SkeletonDataKind {
    /// 如果 format_token 匹配，则解析为现代骨骼
    #[br(pre_assert(format_token == FORMAT_TOKEN))]
    Modern(ModernSkeletonData),

    /// 否则，解析为旧版骨骼
    Legacy(LegacySkeletonData),
}

/// 实现 From trait，将中间解析结果 `SkeletonDataKind` 转换为最终的公共类型 `SkeletonData`。
/// 这样可以统一不同文件格式的数据。
impl From<SkeletonDataKind> for SkeletonData {
    fn from(kind: SkeletonDataKind) -> Self {
        match kind {
            SkeletonDataKind::Modern(modern) => {
                // 将 RigResourceJoint 转换为应用程序使用的 Joint
                let joints = modern
                    .joints
                    .into_iter()
                    .map(|j| Joint {
                        name: j.name,
                        flags: j.flags,
                        id: j.id,
                        parent_id: j.parent_id,
                        radius: j.radius,
                        local_transform: j.local_transform,
                        inverse_bind_transform: j.inverse_bind_transform,
                    })
                    .collect();

                SkeletonData {
                    flags: modern.flags,
                    name: modern.name,
                    asset_name: modern.asset_name,
                    joints,
                    influences: modern.influences,
                }
            }
            SkeletonDataKind::Legacy(legacy) => SkeletonData {
                flags: 0,
                name: String::new(), // 旧版格式没有骨骼名称和资产名称
                asset_name: String::new(),
                joints: legacy.joints,
                influences: legacy.influences,
            },
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~ 现代格式解析 (Modern Format) ~~~~~~~~~~~~~~~~~~~~~~~~~

#[binread]
#[derive(Debug)]
#[br(little)]
struct ModernSkeletonData {
    version: u32,
    #[br(assert(version == 0, "Invalid skeleton version: {}", version))]
    pub flags: u16,
    joint_count: u16,
    influences_count: u32,

    // Offsets
    joints_offset: i32,
    _joint_indices_offset: i32,
    influences_offset: i32,
    name_offset: i32,
    asset_name_offset: i32,
    _bone_names_offset: i32,

    #[br(count = 5)]
    _reserved: Vec<i32>,

    // Data blocks parsed from offsets
    #[br(
        seek_before = SeekFrom::Start(name_offset as u64),
        if(name_offset > 0),
        parse_with = read_null_terminated_string,
        restore_position
    )]
    pub name: String,

    #[br(
        seek_before = SeekFrom::Start(asset_name_offset as u64),
        if(asset_name_offset > 0),
        parse_with = read_null_terminated_string,
        restore_position
    )]
    pub asset_name: String,

    #[br(
        seek_before = SeekFrom::Start(joints_offset as u64),
        count = joint_count,
        if(joints_offset > 0)
    )]
    pub joints: Vec<RigResourceJoint>,

    #[br(
        seek_before = SeekFrom::Start(influences_offset as u64),
        count = influences_count,
        if(influences_offset > 0)
    )]
    pub influences: Vec<i16>,
}

#[binread]
#[derive(Debug)]
#[br(little)]
struct RigResourceJoint {
    pub flags: u16,
    pub id: i16,
    pub parent_id: i16,
    #[br(temp)]
    _padding: u16,
    #[br(temp)]
    _name_hash: u32,
    pub radius: f32,

    // Transform components
    local_translation: BinVec3,
    local_scale: BinVec3,
    local_rotation: BinQuat,
    inverse_bind_translation: BinVec3,
    inverse_bind_scale: BinVec3,
    inverse_bind_rotation: BinQuat,

    // 使用自定义的 RelativeString 类型来处理相对偏移
    #[br(map = |rs: RelativeString| rs.0)]
    pub name: String,

    // Calculated fields
    #[br(calc = create_transform_matrix(local_scale.0, local_rotation.0, local_translation.0))]
    pub local_transform: Mat4,

    #[br(calc = create_transform_matrix(inverse_bind_scale.0, inverse_bind_rotation.0, inverse_bind_translation.0))]
    pub inverse_bind_transform: Mat4,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~ 旧版格式解析 (Legacy Format) ~~~~~~~~~~~~~~~~~~~~~~~~~~

#[binread]
#[derive(Debug)]
#[br(little)]
struct LegacySkeletonData {
    #[br(seek_before = SeekFrom::Start(0), magic = b"r3d2sklt")]
    _magic: (),

    version: u32,
    #[br(assert(version == 1 || version == 2, "Invalid legacy skeleton version: {}", version))]
    _skeleton_id: u32,
    joint_count: u32,

    #[br(count = joint_count)]
    legacy_joints: Vec<RigResourceLegacyJoint>,

    #[br(
        if(version == 2),
        parse_with = parse_legacy_influences
    )]
    influences_v2: Vec<i16>,

    // 使用 calc 在解析后计算最终的 joints 和 influences
    #[br(calc = Self::calculate_joints(&legacy_joints))]
    pub joints: Vec<Joint>,

    #[br(calc = Self::calculate_influences(version, joint_count, &influences_v2))]
    pub influences: Vec<i16>,
}

impl LegacySkeletonData {
    /// 从旧版关节数据计算出符合现代格式的关节列表（包含局部变换矩阵）。
    fn calculate_joints(legacy_joints: &[RigResourceLegacyJoint]) -> Vec<Joint> {
        // 验证关节必须按层级顺序排列
        for (i, joint) in legacy_joints.iter().enumerate() {
            if i as i32 <= joint.parent_id {
                // 在生产代码中，应返回一个 Result 而不是 panic
                panic!("Joints must be ordered hierarchically");
            }
        }

        // 计算每个关节的局部变换矩阵
        let local_transforms: Vec<Mat4> = legacy_joints
            .iter()
            .map(|joint| {
                if joint.parent_id == -1 {
                    joint.global_transform
                } else {
                    let parent_global = legacy_joints[joint.parent_id as usize].global_transform;
                    // local_transform = global_transform * inverse(parent_global_transform)
                    joint.global_transform * parent_global.inverse()
                }
            })
            .collect();

        // 转换为统一的 Joint 格式
        legacy_joints
            .iter()
            .enumerate()
            .map(|(i, legacy_joint)| Joint {
                name: legacy_joint.name.clone(),
                flags: 0,
                id: i as i16,
                parent_id: legacy_joint.parent_id as i16,
                radius: legacy_joint.radius,
                local_transform: local_transforms[i],
                inverse_bind_transform: legacy_joint.global_transform.inverse(),
            })
            .collect()
    }

    /// 根据版本计算 influences 列表。
    fn calculate_influences(version: u32, joint_count: u32, influences_v2: &[i16]) -> Vec<i16> {
        if version == 2 {
            influences_v2.to_vec()
        } else {
            // Version 1
            (0..joint_count as i16).collect()
        }
    }
}

#[binread]
#[derive(Debug)]
#[br(little)]
struct RigResourceLegacyJoint {
    #[br(count = 32, map = |bytes: Vec<u8>| String::from_utf8_lossy(&bytes).trim_end_matches('\0').to_string())]
    pub name: String,
    pub parent_id: i32,
    pub radius: f32,
    #[br(map = |data: LegacyTransformData| data.into())]
    pub global_transform: Mat4,
}

#[binread]
#[derive(Debug)]
#[br(little)]
struct LegacyTransformData {
    col0: [f32; 4],
    col1: [f32; 4],
    col2: [f32; 4],
}

impl From<LegacyTransformData> for Mat4 {
    fn from(data: LegacyTransformData) -> Self {
        Mat4::from_cols(
            data.col0.into(),
            data.col1.into(),
            data.col2.into(),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }
}

/// 解析旧版格式（版本2）中的 influences 列表。
fn parse_legacy_influences<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
    _: (),
) -> BinResult<Vec<i16>> {
    let count = u32::read_options(reader, endian, ())?;
    let mut influences = Vec::with_capacity(count as usize);
    for _ in 0..count {
        influences.push(u32::read_options(reader, endian, ())? as i16);
    }
    Ok(influences)
}

// ===================================================================================
// 3. 辅助类型和函数 (Helpers)
// ===================================================================================

/// 一个自定义类型，用于封装“读取相对偏移 -> 跳转 -> 读取字符串 -> 恢复”的逻辑。
struct RelativeString(pub String);

impl BinRead for RelativeString {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let base_pos = reader.stream_position()?;
        let offset = i32::read_options(reader, endian, ())?;
        let after_offset_pos = reader.stream_position()?;

        let string_pos = (base_pos as i64 + offset as i64) as u64;

        reader.seek(SeekFrom::Start(string_pos))?;
        let value = read_null_terminated_string(reader, endian, ())?;
        reader.seek(SeekFrom::Start(after_offset_pos))?;

        Ok(RelativeString(value))
    }
}

/// 从字节流中读取一个以 null (`\0`) 结尾的字符串。
fn read_null_terminated_string<R: Read>(reader: &mut R, _: Endian, _: ()) -> BinResult<String> {
    let mut bytes = Vec::new();
    loop {
        let mut byte = [0u8; 1];
        reader.read_exact(&mut byte)?;
        if byte[0] == 0 {
            break;
        }
        bytes.push(byte[0]);
    }
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

/// 根据缩放、旋转和平移分量创建一个 4x4 变换矩阵。
fn create_transform_matrix(scale: Vec3, rotation: Quat, translation: Vec3) -> Mat4 {
    Mat4::from_scale_rotation_translation(scale, rotation, translation)
}



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
