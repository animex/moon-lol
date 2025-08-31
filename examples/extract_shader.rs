use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

use binrw::BinRead;
use league_file::LeagueShader;
use league_utils::hash_wad;

fn main() {
    let paths = vec![
        "assets/ASSETS/shaders/hlsl/particlesystem/distortion_mesh_ps.ps.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/distortion_mesh_vs.vs.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/distortion_ps.ps.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/distortion_vs.vs.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/mesh_ps_slice.ps.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/mesh_ps.ps.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/mesh_vs.vs.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/quad_ps_fixedalphauv.ps.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/quad_ps_slice.ps.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/quad_ps.ps.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/quad_screenspaceuv.ps.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/quad_screenspaceuv.vs.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/quad_vs_fixedalphauv.vs.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/quad_vs.vs.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/shadow_mesh_ps.ps.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/shadow_mesh_vs.vs.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/shadow_quad_ps.ps.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/shadow_quad_vs.vs.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/simple_projected_ps.ps.glsl",
        "assets/ASSETS/shaders/hlsl/particlesystem/simple_projected_vs.vs.glsl",
    ];

    for path in paths {
        println!("尝试读取文件: {}", path);

        let file = File::open(path).unwrap();

        let shader_toc = LeagueShader::read(&mut BufReader::new(file)).unwrap();

        println!(
            "{:?}",
            shader_toc
                .base_defines
                .iter()
                .map(|v| v.name.text.clone())
                .collect::<Vec<String>>()
        );

        let Some(shader_index) = shader_toc
            .shader_hashes
            .iter()
            .position(|&v| v == hash_wad(""))
        else {
            continue;
        };

        // var shaderId = shaderObject.ShaderIds[shaderIndex];
        // var shaderBundleId = 100 * (shaderId / 100);
        // var shaderIndexInBundle = shaderId % 100;
        // var shaderBundlePath = $"{shaderObjectPath}_{shaderBundleId}";

        let shader_id = shader_toc.shader_ids[shader_index];
        let shader_bundle_id = 100 * (shader_id / 100);
        let shader_index_in_bundle = shader_id % 100;
        let shader_bundle_path = format!("{}_{}", path, shader_bundle_id);

        let file = File::open(&shader_bundle_path).unwrap();

        let mut reader = BufReader::new(file);

        for i in 0..shader_index_in_bundle {
            let mut buf = [0; 4];
            let size = reader.read(&mut buf).unwrap();
            let shader_size = u32::from_le_bytes(buf);

            println!("shader_size: {}", shader_size);
            let mut bytes = vec![0; shader_size as usize];
            reader.read(&mut bytes).unwrap();
        }

        let mut buf = [0; 4];
        let size = reader.read(&mut buf).unwrap();
        let shader_size = u32::from_le_bytes(buf);

        println!("final shader_size: {}", shader_size);

        let mut bytes = vec![0; shader_size as usize];
        reader.read(&mut bytes).unwrap();

        let shader_bytecode_file = File::create(format!("{}.glbc", &shader_bundle_path)).unwrap();
        let mut writer = BufWriter::new(shader_bytecode_file);
        writer.write_all(&bytes).unwrap();

        // println!(
        //     "{:?}",
        //     shader_toc
        //         .base_defines
        //         .iter()
        //         .map(|v| v.name.text.clone())
        //         .collect::<Vec<String>>()
        // );

        // println!("{:?}", shader_bundle_path);
    }
}
