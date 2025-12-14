use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::Path;
use std::sync::Arc;

use binrw::{BinRead, Endian};
use league_file::{LeagueShaderChunk, LeagueShaderToc};
use league_utils::hash_shader_spec;

fn generate_define_combinations(base_defines: &[String]) -> Vec<Vec<String>> {
    let n = base_defines.len();
    let mut all_combinations = Vec::new();

    for i in 0..(1 << n) {
        let mut current_combination = Vec::new();
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                current_combination.push(base_defines[j].clone());
            }
        }
        all_combinations.push(current_combination);
    }

    all_combinations
}

fn main() -> std::io::Result<()> {
    let paths = vec![
        "assets/ASSETS/shaders/hlsl/skinnedmesh/particle_vs.vs.glsl",
        "assets/ASSETS/shaders/hlsl/skinnedmesh/particle_ps.ps.glsl",
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
        "assets/ASSETS/shaders/hlsl/environment/unlit_decal_vs.vs.glsl",
        "assets/ASSETS/shaders/hlsl/environment/unlit_decal_ps.ps.glsl",
    ];

    let out_put_dir = "assets/shaders_extract";

    fs::create_dir_all(out_put_dir)?;

    for path_str in paths {
        println!("> 处理文件: {}", path_str);

        let path = Path::new(path_str);
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let shader_category = path
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        let stem = file_name.strip_suffix(".glsl").unwrap_or(file_name);

        let (shader_type, original_filename_base) = if stem.ends_with(".ps") {
            ("ps", stem.strip_suffix(".ps").unwrap())
        } else if stem.ends_with(".vs") {
            ("vs", stem.strip_suffix(".vs").unwrap())
        } else {
            println!("! 无法确定着色器类型: {}", file_name);
            continue;
        };

        let shader_name = original_filename_base.replace("_ps", "").replace("_vs", "");

        let output_dir_str = format!("{out_put_dir}/{shader_category}/{shader_name}/{shader_type}");
        fs::create_dir_all(&output_dir_str)?;
        println!("  - 输出目录: {}", output_dir_str);

        let toc_file = File::open(path_str)?;
        let shader_toc = LeagueShaderToc::read(&mut BufReader::new(toc_file)).unwrap();

        let base_defines: Vec<String> = shader_toc
            .base_defines
            .iter()
            .map(|v| v.name.text.clone())
            .collect();

        println!(
            "  - 找到 {} 个基础宏定义: {:?}",
            base_defines.len(),
            base_defines
        );
        let combinations = generate_define_combinations(&base_defines);
        println!("  - 生成了 {} 种组合", combinations.len());

        let mut bundle_cache: HashMap<String, Arc<LeagueShaderChunk>> = HashMap::new();

        for combo in combinations {
            let defines_hash = hash_shader_spec(&combo);

            let Some(shader_index) = shader_toc
                .shader_hashes
                .iter()
                .position(|&h| h == defines_hash)
            else {
                continue;
            };

            let shader_id = shader_toc.shader_ids[shader_index];

            let shader_bundle_id = 100 * (shader_id / 100);
            let shader_index_in_bundle = (shader_id % 100) as usize;
            let shader_bundle_path = format!("{}_{}", path_str, shader_bundle_id);

            if !bundle_cache.contains_key(&shader_bundle_path) {
                if let Ok(bundle_file) = File::open(&shader_bundle_path) {
                    let mut reader = BufReader::new(bundle_file);
                    if let Ok(chunk_file) =
                        LeagueShaderChunk::read_options(&mut reader, Endian::Little, ())
                    {
                        bundle_cache.insert(shader_bundle_path.clone(), Arc::new(chunk_file));
                    } else {
                        println!("! 无法解析捆绑包: {}", shader_bundle_path);
                        bundle_cache.insert(
                            shader_bundle_path.clone(),
                            Arc::new(LeagueShaderChunk { files: vec![] }),
                        );
                    }
                } else {
                    println!("! 无法打开捆绑包: {}", shader_bundle_path);
                    continue;
                }
            }

            if let Some(chunk_file_arc) = bundle_cache.get(&shader_bundle_path) {
                if shader_index_in_bundle < chunk_file_arc.files.len() {
                    let content = &chunk_file_arc.files[shader_index_in_bundle].text;

                    let converted = content;

                    let name = if combo.is_empty() {
                        "BASE"
                    } else {
                        &combo.join("__").to_uppercase()
                    };

                    let output_filename = format!(
                        "{}.{}",
                        name,
                        if shader_type == "ps" { "frag" } else { "vert" }
                    );

                    let output_path = Path::new(&output_dir_str).join(&output_filename);

                    let mut output_file = File::create(&output_path)?;

                    output_file.write_all(converted.as_bytes())?;
                } else {
                    println!(
                        "! 索引越界: ID {} 在 {} 中索引为 {}, 但文件只有 {} 个着色器",
                        shader_id,
                        shader_bundle_path,
                        shader_index_in_bundle,
                        chunk_file_arc.files.len()
                    );
                }
            }
        }
    }

    Ok(())
}
