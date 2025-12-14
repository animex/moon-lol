use std::fs::{read, write};

use league_to_lol::{convert_frag, convert_vert};

fn main() {
    let convert_list = [
        (
            "assets/shader_convert.vert",
            "assets/shader_convert.converted.vert",
        ),
        // 添加其他文件映射...
    ];

    for (source, dest) in convert_list {
        println!("Converting {} -> {}", source, dest);
        let data = read(source).expect("Failed to read source file");
        let code = String::from_utf8_lossy(&data);

        let result = if source.ends_with("vert") {
            convert_vert(&code)
        } else {
            convert_frag(&code)
        };

        write(dest, result.as_bytes()).expect("Failed to write dest file");
    }
    println!("Conversion complete!");
}
