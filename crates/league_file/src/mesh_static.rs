use std::io::{Read, Seek};

use bevy::prelude::*;
use binrw::{binread, BinRead, BinResult, Endian};
use bitflags::bitflags;
use league_utils::get_padded_string_128; // 假设这个函数存在于您的工具库中
use league_utils::BoundingBox; // 假设这个结构体存在于您的工具库中

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct StaticMeshFlags: u32 {
        const HasVcp = 1;
        const HasLocalOriginLocatorAndPivot = 2;
    }
}

impl BinRead for StaticMeshFlags {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let val = u32::read_options(reader, endian, ())?;
        Ok(Self::from_bits_truncate(val))
    }
}

/// 辅助函数，用于从 64 字节的缓冲区中读取字符串
/// C# 的 `ReadPaddedString(64)`
fn get_padded_string_64(bytes: [u8; 64]) -> String {
    let null_pos = bytes.iter().position(|&c| c == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..null_pos]).to_string()
}

/// 代表文件中面（face）颜色的结构
#[binread]
#[derive(Debug, Clone, Copy)]
#[br(little)]
struct FaceColors {
    color0: [u8; 3],
    color1: [u8; 3],
    color2: [u8; 3],
}

/// 这是一个新的结构，精确匹配 C# 代码从文件中读取单个“面”数据的布局
#[binread]
#[derive(Debug, Clone)]
#[br(little)]
struct StaticMeshFaceDisk {
    // C# 将 3 个 u32 读取为索引，然后转换为 u16
    indices: [u32; 3],
    // C# 读取一个 64 字节的填充字符串作为材质名称
    #[br(map = get_padded_string_64)]
    material: String,
    // C# 先读取所有顶点的 UV.X，然后再读取所有顶点的 UV.Y
    uv_x: [f32; 3], // [UV0.X, UV1.X, UV2.X]
    uv_y: [f32; 3], // [UV0.Y, UV1.Y, UV2.Y]
}

/// 最终在内存中表示“面”数据的结构，已根据 C# 代码修正
#[derive(Debug, Clone)]
pub struct StaticMeshFace {
    pub material: String,
    pub indices: [u16; 3],
    pub uvs: [[f32; 2]; 3],
    pub color0: [u8; 3],
    pub color1: [u8; 3],
    pub color2: [u8; 3],
}

#[binread]
#[derive(Debug)]
#[br(little)]
#[br(magic = b"r3d2Mesh")]
pub struct LeagueMeshStatic {
    pub major: u16,
    pub minor: u16,
    // C# 检查: if (major is not (2 or 3) && minor is not 1) throw
    // 这意味着通过条件是: (major == 2 || major == 3) || minor == 1
    #[br(assert((major == 2 || major == 3) || minor == 1, "无效的文件版本: {}.{}", major, minor))]
    _version_check: (),

    #[br(map = get_padded_string_128)]
    pub name: String,

    #[br(temp)]
    vertex_count: i32,
    #[br(temp)]
    face_count: i32,

    #[br(temp)]
    flags: StaticMeshFlags,

    pub bounding_box: BoundingBox,

    #[br(temp)]
    #[br(if(major >= 3 && minor >= 2))]
    has_vertex_colors_raw: Option<u32>,
    #[br(calc = has_vertex_colors_raw.unwrap_or(0) == 1)]
    pub has_vertex_colors: bool,

    #[br(count = vertex_count as usize)]
    pub vertices: Vec<[f32; 3]>,

    // C# 读取 BgraU8 (4 bytes)
    #[br(if(has_vertex_colors))]
    #[br(count = vertex_count as usize)]
    pub vertex_colors: Option<Vec<[u8; 4]>>,

    #[br(map = |v: [f32; 3]| Vec3::from_array(v))]
    pub central_point: Vec3,

    // 使用新的中间结构来正确读取面数据
    #[br(count = face_count as usize)]
    #[br(temp)]
    faces_on_disk: Vec<StaticMeshFaceDisk>,

    // C# 读取 RgbU8 (3 bytes) * 3
    #[br(if(flags.contains(StaticMeshFlags::HasVcp)))]
    #[br(count = face_count as usize)]
    #[br(temp)]
    face_colors_data: Option<Vec<FaceColors>>,

    #[br(calc = Self::build_faces(faces_on_disk, face_colors_data))]
    pub faces: Vec<StaticMeshFace>,
}

impl LeagueMeshStatic {
    /// 修正后的函数，用于将从磁盘读取的原始数据转换为最终的面数据结构
    fn build_faces(
        disk_data: Vec<StaticMeshFaceDisk>,
        colors_data: Option<Vec<FaceColors>>,
    ) -> Vec<StaticMeshFace> {
        // C# 中的 Color.One 对应于 RGB(255, 255, 255)
        let default_color = [255, 255, 255];

        let faces_iter = disk_data.into_iter().map(|disk_face| {
            // 从特殊的 X/Y 分离布局中重组 UV 坐标
            let uvs = [
                [disk_face.uv_x[0], disk_face.uv_y[0]], // UV0
                [disk_face.uv_x[1], disk_face.uv_y[1]], // UV1
                [disk_face.uv_x[2], disk_face.uv_y[2]], // UV2
            ];

            // C# 将 u32 的索引转换为 u16
            let indices = [
                disk_face.indices[0] as u16,
                disk_face.indices[1] as u16,
                disk_face.indices[2] as u16,
            ];

            // 创建一个带有默认颜色的临时面
            StaticMeshFace {
                material: disk_face.material,
                indices,
                uvs,
                color0: default_color,
                color1: default_color,
                color2: default_color,
            }
        });

        if let Some(colors) = colors_data {
            // 如果存在面颜色数据，则将其与面数据合并
            faces_iter
                .zip(colors.into_iter())
                .map(|(mut face, color_data)| {
                    face.color0 = color_data.color0;
                    face.color1 = color_data.color1;
                    face.color2 = color_data.color2;
                    face
                })
                .collect()
        } else {
            // 否则，直接使用带有默认颜色的面数据
            faces_iter.collect()
        }
    }
}
