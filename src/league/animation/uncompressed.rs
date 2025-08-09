use crate::league::{decompress_quat, BinQuat, BinVec3, LeagueLoader};
use bevy::math::{Quat, Vec3};
use binrw::io::{Read, Seek, SeekFrom};
use binrw::{binread, BinRead};
use binrw::{prelude::*, Endian};
use std::collections::HashMap;
use std::fmt::Debug;

#[binread]
#[derive(Debug, Clone)]
#[br(little)]
pub struct UncompressedAnimationAsset {
    pub version: u32,

    #[br(args { version })]
    pub data: UncompressedData,
}

#[binread]
#[derive(Debug, Clone)]
#[br(little, import { version: u32 })]
pub enum UncompressedData {
    #[br(pre_assert(version == 3))]
    V3(#[br(parse_with = parse_uncompressed_data_v3)] UncompressedDataV3),
    #[br(pre_assert(version == 4))]
    V4(UncompressedDataV4),
    #[br(pre_assert(version == 5))]
    V5(UncompressedDataV5),
}

#[binread]
#[derive(Debug, Clone)]
#[br(little)]
pub struct UncompressedDataV5 {
    pub resource_size: u32,
    pub format_token: u32,
    pub version_again: u32,
    pub flags: u32,
    pub track_count: i32,
    pub frame_count: i32,
    pub frame_duration: f32,

    #[br(temp)]
    joint_name_hashes_offset: i32,
    #[br(temp)]
    _asset_name_offset: i32,
    #[br(temp)]
    _time_offset: i32,
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
        map = |vals: Vec<[u16; 3]>| vals.iter().map(decompress_quat).collect()
    )]
    pub quat_palette: Vec<Quat>,

    #[br(
        seek_before = SeekFrom::Start(frames_offset as u64 + 12),
        count = track_count * frame_count
    )]
    pub frames: Vec<UncompressedFrame>,
}

#[binread]
#[derive(Debug, Clone)]
#[br(little)]
pub struct UncompressedDataV4 {
    pub resource_size: u32,
    pub format_token: u32,
    pub version_again: u32,
    pub flags: u32,
    pub track_count: i32,
    pub frame_count: i32,
    pub frame_duration: f32,

    #[br(temp)]
    _joint_name_hashes_offset: i32,
    #[br(temp)]
    _asset_name_offset: i32,
    #[br(temp)]
    _time_offset: i32,
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
#[derive(Debug, Clone)]
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

#[binread]
#[derive(Debug, Clone)]
#[br(little, import { frame_count: i32 })]
struct RawTrackV3 {
    track_name_bytes: [u8; 32],
    _flags: u32,
    #[br(count = frame_count)]
    frames: Vec<RawFrameV3>,
}

#[binread]
#[derive(Debug, Clone)]
#[br(little)]
struct RawFrameV3 {
    rotation: BinQuat,
    translation: BinVec3,
}

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
#[br(little)]
struct RawDataV3 {
    skeleton_id: u32,
    track_count: i32,
    frame_count: i32,
    fps: i32,
    #[br(count = track_count, args { inner: RawTrackV3BinReadArgs { frame_count } })]
    tracks: Vec<RawTrackV3>,
}

fn parse_uncompressed_data_v3<R: Read + Seek>(
    reader: &mut R,
    _: Endian,
    _: (),
) -> BinResult<UncompressedDataV3> {
    let raw = RawDataV3::read(reader)?;

    let track_count = raw.track_count;
    let frame_count = raw.frame_count;
    let mut joint_frames = HashMap::with_capacity(track_count as usize);
    let palette_size = (track_count * frame_count) as usize;
    let mut quat_palette = Vec::with_capacity(palette_size);

    let mut vector_palette = Vec::with_capacity(palette_size + 1);
    vector_palette.push(BinVec3(Vec3::ONE));

    for (i, raw_track) in raw.tracks.into_iter().enumerate() {
        let track_name = String::from_utf8_lossy(&raw_track.track_name_bytes)
            .trim_end_matches('\0')
            .to_string();

        let joint_hash = LeagueLoader::hash_joint(&track_name);

        let mut frames_for_joint = Vec::with_capacity(frame_count as usize);

        for (j, raw_frame) in raw_track.frames.into_iter().enumerate() {
            let index = i * frame_count as usize + j;

            quat_palette.push(raw_frame.rotation);
            vector_palette.push(raw_frame.translation);

            frames_for_joint.push(UncompressedFrame {
                rotation_id: index as u16,

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

#[derive(BinRead, Debug, Clone, Copy)]
#[br(little)]
pub struct UncompressedFrame {
    pub translation_id: u16,
    pub scale_id: u16,
    pub rotation_id: u16,
}
