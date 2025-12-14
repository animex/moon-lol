use binrw::binread;
use binrw::prelude::*;

use crate::SizedStringU32;

#[binread]
#[derive(Debug)]
#[br(little)]
pub struct LeagueShaderToc {
    pub magic: SizedStringU32,

    pub shader_count: u32,
    pub base_define_count: u32,
    pub bundled_shader_count: u32,
    // 0 = vs 1 = ps
    pub shader_type: u32,
    pub base_defines_section_magic: SizedStringU32,

    #[br(count = base_define_count)]
    pub base_defines: Vec<ShaderMacroDefinition>,
    pub shaders_section_magic: SizedStringU32,

    #[br(count = shader_count)]
    pub shader_hashes: Vec<u64>,

    #[br(count = shader_count)]
    pub shader_ids: Vec<u32>,
}

#[binread]
#[derive(Debug)]
#[br(little)]
pub struct ShaderMacroDefinition {
    pub name: SizedStringU32,
    pub value: SizedStringU32,
}

#[derive(Debug, Clone)]
pub struct LeagueShaderChunk {
    pub files: Vec<SizedStringU32>,
}

impl BinRead for LeagueShaderChunk {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let mut files = Vec::new();
        while let Ok(shader_file) = SizedStringU32::read_options(reader, endian, ()) {
            files.push(shader_file);
        }
        Ok(LeagueShaderChunk { files })
    }
}
