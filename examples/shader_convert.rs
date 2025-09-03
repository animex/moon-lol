use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// 使用 Rust 将 GLSL 150 着色器转换为 GLSL 450 核心规范
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 输入的 GLSL 150 文件路径
    #[arg(value_name = "INPUT_FILE")]
    input_file: PathBuf,

    /// 输出的 GLSL 450 文件路径
    #[arg(value_name = "OUTPUT_FILE")]
    output_file: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let source_code = fs::read_to_string(&args.input_file)
        .map_err(|e| format!("无法读取输入文件 '{}': {}", args.input_file.display(), e))?;

    let converted_code = convert_glsl(&source_code);

    fs::write(&args.output_file, converted_code)
        .map_err(|e| format!("无法写入输出文件 '{}': {}", args.output_file.display(), e))?;

    println!(
        "✅ 成功将 '{}' 转换为 '{}'",
        args.input_file.display(),
        args.output_file.display()
    );
    println!("🔔 请检查输出文件，并根据你的引擎需求手动调整 set/binding。");

    Ok(())
}

fn convert_glsl(source_code: &str) -> String {
    let re_in = Regex::new(r"in\s+(\w+)\s+(\w+);").unwrap();
    let re_out = Regex::new(r"out\s+(\w+)\s+(\w+);").unwrap();
    let re_sampler = Regex::new(r"uniform\s+sampler2D\s+(\w+);").unwrap();

    // --- 主要变更点 ---
    // 1. 使用 (?s) 标志来允许多行匹配 (dot matches newline)
    let re_uniform_struct_def = Regex::new(r"(?s)struct\s+(\w+)\s*\{([^}]+)\};").unwrap();
    let re_uniform_struct_var = Regex::new(r"uniform\s+(\w+)\s+(\w+);").unwrap();

    let mut output_header = Vec::new();
    let mut body_lines = Vec::new();
    let mut uniforms_to_ubo = Vec::new();
    let mut samplers = HashMap::new();

    // 创建一个可变副本用于预处理
    let mut processed_source = source_code.to_string();

    let mut location_in_counter = 0;
    let mut location_out_counter = 0;
    let mut sampler_binding_counter = 0;
    let ubo_set = 0;
    let sampler_set = 1;

    output_header.push("#version 450 core".to_string());

    // --- 2. 预处理阶段：在逐行扫描前处理多行 struct ---
    let mut uniform_struct_instance_name = String::new();
    if let Some(struct_caps) = re_uniform_struct_def.captures(source_code) {
        let struct_name = &struct_caps[1];
        let struct_body = &struct_caps[2].trim();

        // 找到使用该 struct 的 uniform 实例
        if let Some(var_caps) = re_uniform_struct_var.captures(source_code) {
            if &var_caps[1] == struct_name {
                uniform_struct_instance_name = var_caps[2].to_string(); // e.g., _UniformsVertex

                // 填充 UBO 成员
                for member in struct_body.split(';').filter(|s| !s.trim().is_empty()) {
                    uniforms_to_ubo.push(format!("    {};", member.trim()));
                }

                // 3. 从源码中移除已处理的定义，避免在后续循环中重复处理
                processed_source = re_uniform_struct_def
                    .replace(&processed_source, "")
                    .to_string();
                processed_source = re_uniform_struct_var
                    .replace(&processed_source, "")
                    .to_string();
            }
        }
    }

    // --- 第一遍：在预处理过的源码上进行逐行识别和分类 ---
    for line in processed_source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("#version") {
            continue;
        }

        if let Some(caps) = re_in.captures(trimmed) {
            output_header.push(format!(
                "layout(location = {}) in {} {};",
                location_in_counter, &caps[1], &caps[2]
            ));
            location_in_counter += 1;
        } else if let Some(caps) = re_out.captures(trimmed) {
            output_header.push(format!(
                "layout(location = {}) out {} {};",
                location_out_counter, &caps[1], &caps[2]
            ));
            location_out_counter += 1;
        } else if let Some(caps) = re_sampler.captures(trimmed) {
            let sampler_name = &caps[1];
            output_header.push(format!(
                "layout(set = {}, binding = {}) uniform texture2D {};",
                sampler_set, sampler_binding_counter, sampler_name
            ));
            sampler_binding_counter += 1;
            output_header.push(format!(
                "layout(set = {}, binding = {}) uniform sampler {}_sampler;",
                sampler_set, sampler_binding_counter, sampler_name
            ));
            sampler_binding_counter += 1;

            samplers.insert(
                sampler_name.to_string(),
                format!("{}_sampler", sampler_name),
            );
        } else {
            // 其他所有行先视为 body
            body_lines.push(line.to_string());
        }
    }

    // --- 第二遍：构建 UBO 和替换代码 ---
    if !uniforms_to_ubo.is_empty() {
        let mut ubo_block = vec!["".to_string()];
        ubo_block.push(format!(
            "layout(set = {}, binding = 0, std140) uniform Uniforms {{",
            ubo_set
        ));
        ubo_block.extend(uniforms_to_ubo);
        ubo_block.push("};".to_string());
        output_header.append(&mut ubo_block);
    }

    let mut final_body = body_lines.join("\n");

    if !uniform_struct_instance_name.is_empty() {
        let access_pattern =
            Regex::new(&format!(r"{}\.(\w+)", uniform_struct_instance_name)).unwrap();
        final_body = access_pattern.replace_all(&final_body, "$1").to_string();
    }

    for (tex_name, sampler_name) in &samplers {
        let pattern = Regex::new(&format!(r"texture\(\s*{}\s*,\s*(.*?)\)", tex_name)).unwrap();
        let replacement = format!("texture(sampler2D({}, {}), $1)", tex_name, sampler_name);
        final_body = pattern.replace_all(&final_body, replacement).to_string();
    }

    format!("{}\n\n{}", output_header.join("\n"), final_body)
}
