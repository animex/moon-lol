use crate::render::{
    AnimationData, AnimationFile, LeagueLoader, LeagueSkeleton, LeagueSkinnedMesh,
    LeagueSkinnedMeshInternal,
};
use bevy::{
    animation::{animated_field, AnimationTarget, AnimationTargetId},
    asset::RenderAssetUsages,
    pbr::MeshMaterial3d,
    prelude::*,
    render::mesh::skinning::{SkinnedMesh, SkinnedMeshInverseBindposes},
};
use bevy::{image::Image, math::Mat4};
use binrw::io::NoSeek;
use binrw::BinRead;
use cdragon_prop::{
    BinEmbed, BinEntry, BinFloat, BinHash, BinLink, BinMap, BinMatrix, BinString, BinStruct, BinU32,
};
use std::io::BufReader;
use std::{collections::HashMap, fs::File};

#[derive(Debug, Clone)]
pub struct LeagueBinCharacterRecord {
    pub character_name: Option<String>,
    pub fallback_character_name: Option<String>,
    pub base_hp: Option<f32>,
    pub base_static_hp_regen: Option<f32>,
    pub health_bar_height: Option<f32>,
    pub base_damage: Option<f32>,
    pub base_armor: Option<f32>,
    pub base_spell_block: Option<f32>,
    pub base_move_speed: Option<f32>,
    pub attack_range: Option<f32>,
    pub attack_speed: Option<f32>,
    pub attack_speed_ratio: Option<f32>,
    pub attack_speed_per_level: Option<f32>,
    pub exp_given_on_death: Option<f32>,
    pub gold_given_on_death: Option<f32>,
    pub local_gold_given_on_death: Option<f32>,
    pub global_gold_given_on_death: Option<f32>,
    pub display_name: Option<String>,
    pub hit_fx_scale: Option<f32>,
    pub selection_height: Option<f32>,
    pub selection_radius: Option<f32>,
    pub pathfinding_collision_radius: Option<f32>,
    pub gameplay_collision_radius: Option<f32>,
    pub unit_tags: Option<String>,
    pub description: Option<String>,
}

impl From<BinEntry> for LeagueBinCharacterRecord {
    fn from(value: BinEntry) -> Self {
        let char_name_hash = LeagueLoader::hash_bin("mCharacterName");
        let fallback_char_name_hash = LeagueLoader::hash_bin("mFallbackCharacterName");
        let base_hp_hash = LeagueLoader::hash_bin("baseHP");
        let base_static_hp_regen_hash = LeagueLoader::hash_bin("baseStaticHPRegen");
        let health_bar_height_hash = LeagueLoader::hash_bin("healthBarHeight");
        let base_damage_hash = LeagueLoader::hash_bin("baseDamage");
        let base_armor_hash = LeagueLoader::hash_bin("baseArmor");
        let base_spell_block_hash = LeagueLoader::hash_bin("baseSpellBlock");
        let base_move_speed_hash = LeagueLoader::hash_bin("baseMoveSpeed");
        let attack_range_hash = LeagueLoader::hash_bin("attackRange");
        let attack_speed_hash = LeagueLoader::hash_bin("attackSpeed");
        let attack_speed_ratio_hash = LeagueLoader::hash_bin("attackSpeedRatio");
        let attack_speed_per_level_hash = LeagueLoader::hash_bin("attackSpeedPerLevel");
        let exp_given_on_death_hash = LeagueLoader::hash_bin("expGivenOnDeath");
        let gold_given_on_death_hash = LeagueLoader::hash_bin("goldGivenOnDeath");
        let local_gold_given_on_death_hash = LeagueLoader::hash_bin("localGoldGivenOnDeath");
        let global_gold_given_on_death_hash = LeagueLoader::hash_bin("globalGoldGivenOnDeath");
        let display_name_hash = LeagueLoader::hash_bin("name");
        let hit_fx_scale_hash = LeagueLoader::hash_bin("hitFxScale");
        let selection_height_hash = LeagueLoader::hash_bin("selectionHeight");
        let selection_radius_hash = LeagueLoader::hash_bin("selectionRadius");
        let pathfinding_collision_radius_hash =
            LeagueLoader::hash_bin("pathfindingCollisionRadius");
        let gameplay_collision_radius_hash =
            LeagueLoader::hash_bin("overrideGameplayCollisionRadius");
        let unit_tags_hash = LeagueLoader::hash_bin("unitTagsString");
        let description_hash = LeagueLoader::hash_bin("description");

        let character_name = value
            .getv::<BinString>(char_name_hash.into())
            .map(|s| s.0.clone());
        let fallback_character_name = value
            .getv::<BinString>(fallback_char_name_hash.into())
            .map(|s| s.0.clone());
        let base_hp = value.getv::<BinFloat>(base_hp_hash.into()).map(|f| f.0);
        let base_static_hp_regen = value
            .getv::<BinFloat>(base_static_hp_regen_hash.into())
            .map(|f| f.0);
        let health_bar_height = value
            .getv::<BinFloat>(health_bar_height_hash.into())
            .map(|f| f.0);
        let base_damage = value.getv::<BinFloat>(base_damage_hash.into()).map(|f| f.0);
        let base_armor = value.getv::<BinFloat>(base_armor_hash.into()).map(|f| f.0);
        let base_spell_block = value
            .getv::<BinFloat>(base_spell_block_hash.into())
            .map(|f| f.0);
        let base_move_speed = value
            .getv::<BinFloat>(base_move_speed_hash.into())
            .map(|f| f.0);
        let attack_range = value
            .getv::<BinFloat>(attack_range_hash.into())
            .map(|f| f.0);
        let attack_speed = value
            .getv::<BinFloat>(attack_speed_hash.into())
            .map(|f| f.0);
        let attack_speed_ratio = value
            .getv::<BinFloat>(attack_speed_ratio_hash.into())
            .map(|f| f.0);
        let attack_speed_per_level = value
            .getv::<BinFloat>(attack_speed_per_level_hash.into())
            .map(|f| f.0);
        let exp_given_on_death = value
            .getv::<BinFloat>(exp_given_on_death_hash.into())
            .map(|f| f.0);
        let gold_given_on_death = value
            .getv::<BinFloat>(gold_given_on_death_hash.into())
            .map(|f| f.0);
        let local_gold_given_on_death = value
            .getv::<BinFloat>(local_gold_given_on_death_hash.into())
            .map(|f| f.0);
        let global_gold_given_on_death = value
            .getv::<BinFloat>(global_gold_given_on_death_hash.into())
            .map(|f| f.0);
        let display_name = value
            .getv::<BinString>(display_name_hash.into())
            .map(|s| s.0.clone());
        let hit_fx_scale = value
            .getv::<BinFloat>(hit_fx_scale_hash.into())
            .map(|f| f.0);
        let selection_height = value
            .getv::<BinFloat>(selection_height_hash.into())
            .map(|f| f.0);
        let selection_radius = value
            .getv::<BinFloat>(selection_radius_hash.into())
            .map(|f| f.0);
        let pathfinding_collision_radius = value
            .getv::<BinFloat>(pathfinding_collision_radius_hash.into())
            .map(|f| f.0);
        let gameplay_collision_radius = value
            .getv::<BinFloat>(gameplay_collision_radius_hash.into())
            .map(|f| f.0);
        let unit_tags = value
            .getv::<BinString>(unit_tags_hash.into())
            .map(|s| s.0.clone());
        let description = value
            .getv::<BinString>(description_hash.into())
            .map(|s| s.0.clone());

        LeagueBinCharacterRecord {
            character_name,
            fallback_character_name,
            base_hp,
            base_static_hp_regen,
            health_bar_height,
            base_damage,
            base_armor,
            base_spell_block,
            base_move_speed,
            attack_range,
            attack_speed,
            attack_speed_ratio,
            attack_speed_per_level,
            exp_given_on_death,
            gold_given_on_death,
            local_gold_given_on_death,
            global_gold_given_on_death,
            display_name,
            hit_fx_scale,
            selection_height,
            selection_radius,
            pathfinding_collision_radius,
            gameplay_collision_radius,
            unit_tags,
            description,
        }
    }
}

#[derive(Debug)]
pub struct LeagueBinMaybeCharacterMapRecord {
    pub transform: Mat4,
    pub name: u32,
    pub definition: CharacterMapRecordDefinition,
}

impl From<&BinStruct> for LeagueBinMaybeCharacterMapRecord {
    fn from(value: &BinStruct) -> Self {
        let transform = value
            .getv::<BinMatrix>(LeagueLoader::hash_bin("transform").into())
            .unwrap();

        let name = value
            .getv::<BinHash>(LeagueLoader::hash_bin("name").into())
            .unwrap();

        let definition = value
            .getv::<BinStruct>(LeagueLoader::hash_bin("definition").into())
            .unwrap();

        Self {
            transform: Mat4::from_cols_array_2d(&transform.0),
            name: name.0.hash,
            definition: definition.into(),
        }
    }
}

#[derive(Debug)]
pub struct CharacterMapRecordDefinition {
    pub team: Option<u32>,
    pub character_record: String,
    pub skin: String,
}

impl From<&BinStruct> for CharacterMapRecordDefinition {
    fn from(value: &BinStruct) -> Self {
        let team = value
            .getv::<BinU32>(LeagueLoader::hash_bin("Team").into())
            .map(|v| v.0);

        let character_record = value
            .getv::<BinString>(LeagueLoader::hash_bin("CharacterRecord").into())
            .unwrap();

        let skin = value
            .getv::<BinString>(LeagueLoader::hash_bin("Skin").into())
            .unwrap();

        Self {
            team,
            character_record: character_record.0.clone(),
            skin: skin.0.clone(),
        }
    }
}

pub struct SkinCharacterDataProperties {
    pub skin_animation_properties: SkinAnimationProperties,
    pub skin_mesh_properties: SkinMeshDataProperties,
}

impl From<&BinEntry> for SkinCharacterDataProperties {
    fn from(value: &BinEntry) -> Self {
        let skin_animation_properties = value
            .getv::<BinEmbed>(LeagueLoader::hash_bin("skinAnimationProperties").into())
            .unwrap();

        let skin_mesh_properties = value
            .getv::<BinEmbed>(LeagueLoader::hash_bin("skinMeshProperties").into())
            .unwrap();

        Self {
            skin_animation_properties: skin_animation_properties.into(),
            skin_mesh_properties: skin_mesh_properties.into(),
        }
    }
}

pub struct SkinMeshDataProperties {
    pub skeleton: String,
    pub simple_skin: String,
    pub texture: String,
}

impl From<&BinEmbed> for SkinMeshDataProperties {
    fn from(value: &BinEmbed) -> Self {
        let skeleton = value
            .getv::<BinString>(LeagueLoader::hash_bin("skeleton").into())
            .map(|v| v.0.clone())
            .unwrap();

        let simple_skin = value
            .getv::<BinString>(LeagueLoader::hash_bin("simpleSkin").into())
            .unwrap();

        let texture = value
            .getv::<BinString>(LeagueLoader::hash_bin("texture").into())
            .unwrap();

        Self {
            skeleton,
            simple_skin: simple_skin.0.clone(),
            texture: texture.0.clone(),
        }
    }
}

pub struct SkinAnimationProperties {
    pub animation_graph_data: u32,
}

impl From<&BinEmbed> for SkinAnimationProperties {
    fn from(value: &BinEmbed) -> Self {
        let animation_graph_data = value
            .getv::<BinLink>(LeagueLoader::hash_bin("animationGraphData").into())
            .map(|v| v.0.hash)
            .unwrap();

        Self {
            animation_graph_data,
        }
    }
}

pub fn load_character(
    loader: &LeagueLoader,
    character_map_record: &BinStruct,
) -> (
    LeagueBinMaybeCharacterMapRecord,
    LeagueBinCharacterRecord,
    Image,
    LeagueSkinnedMesh,
    LeagueSkeleton,
    Option<AnimationData>,
) {
    let character_map_record = LeagueBinMaybeCharacterMapRecord::from(character_map_record);

    let character_record = loader
        .character_map
        .get(&LeagueLoader::hash_bin(
            &character_map_record.definition.character_record,
        ))
        .unwrap()
        .clone();

    let skin_path = format!("data/{}.bin", character_map_record.definition.skin);

    let skin_bin = loader.get_prop_bin_by_path(&skin_path).unwrap();

    let skin_mesh_properties = skin_bin
        .entries
        .iter()
        .find(|v| v.ctype.hash == LeagueLoader::hash_bin("SkinCharacterDataProperties"))
        .unwrap();

    let skin_character_data_properties = SkinCharacterDataProperties::from(skin_mesh_properties);

    let texture = loader
        .get_image_by_texture_path(&skin_character_data_properties.skin_mesh_properties.texture)
        .unwrap();

    let mut reader = loader
        .get_wad_entry_no_seek_reader_by_path(
            &skin_character_data_properties
                .skin_mesh_properties
                .simple_skin,
        )
        .unwrap();

    let skinned_mesh =
        LeagueSkinnedMesh::from(LeagueSkinnedMeshInternal::read(&mut reader).unwrap());

    let flat_map: HashMap<_, _> = skin_bin
        .linked_files
        .iter()
        .map(|v| loader.get_prop_bin_by_path(v).unwrap())
        .flat_map(|v| v.entries)
        .map(|v| (v.path.hash, v))
        .collect();

    let league_skeleton = loader
        .get_wad_entry_reader_by_path(&skin_character_data_properties.skin_mesh_properties.skeleton)
        .map(|mut v| LeagueSkeleton::read(&mut v).unwrap())
        .unwrap();

    let gr_da: AnimationGraphData = flat_map
        .get(
            &skin_character_data_properties
                .skin_animation_properties
                .animation_graph_data,
        )
        .unwrap()
        .into();

    let idle_path = gr_da
        .mClipDataMap
        .get(&0x35f43992)
        .iter()
        .filter_map(|v| match v {
            AnimationClipData::AtomicClipData {
                mAnimationResourceData,
            } => Some(mAnimationResourceData.mAnimationFilePath.clone()),
            AnimationClipData::Unknown => None,
        })
        .collect::<Vec<_>>();
    let idle_path = idle_path.first();

    println!("{:?}", idle_path);

    let animation_data = idle_path.map(|v| {
        loader
            .get_wad_entry_reader_by_path(v)
            .map(|mut v| AnimationFile::read(&mut v).unwrap().into())
            .unwrap()
    });

    return (
        character_map_record,
        character_record,
        texture,
        skinned_mesh,
        league_skeleton,
        animation_data,
    );
}

#[derive(Debug)]
pub struct AnimationGraphData {
    mClipDataMap: HashMap<u32, AnimationClipData>,
}

impl From<&BinEntry> for AnimationGraphData {
    fn from(value: &BinEntry) -> Self {
        let map = value
            .getv::<BinMap>(LeagueLoader::hash_bin("mClipDataMap").into())
            .unwrap()
            .downcast::<BinHash, BinStruct>()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.0.hash, v.into()))
            .collect();

        // Self { mClipDataMap:  }
        Self { mClipDataMap: map }
    }
}

#[derive(Debug)]
pub enum AnimationClipData {
    AtomicClipData {
        mAnimationResourceData: AnimationResourceData,
    },
    Unknown,
}

impl From<&BinStruct> for AnimationClipData {
    fn from(value: &BinStruct) -> Self {
        let hash = LeagueLoader::hash_bin("AtomicClipData");

        if value.ctype.hash == hash {
            AnimationClipData::AtomicClipData {
                mAnimationResourceData: value
                    .getv::<BinEmbed>(LeagueLoader::hash_bin("mAnimationResourceData").into())
                    .unwrap()
                    .into(),
            }
        } else {
            AnimationClipData::Unknown
        }
    }
}

#[derive(Debug)]
pub struct AnimationResourceData {
    mAnimationFilePath: String,
}

impl From<&BinEmbed> for AnimationResourceData {
    fn from(value: &BinEmbed) -> Self {
        let mAnimationFilePath = value
            .getv::<BinString>(LeagueLoader::hash_bin("mAnimationFilePath").into())
            .map(|v| v.0.clone())
            .unwrap();

        AnimationResourceData { mAnimationFilePath }
    }
}
