extern crate gl;
extern crate glfw;

use std::ffi::CString;
use std::fs;

use glfw::{Context, OpenGlProfileHint, WindowHint};

fn main() {
    println!("正在初始化GLFW以获取OpenGL上下文...");

    // 1. 初始化 GLFW
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("初始化GLFW失败");

    // 2. 请求 OpenGL 3.2 Core Profile (GLSL 150 对应 OpenGL 3.2)
    glfw.window_hint(WindowHint::ContextVersion(3, 2));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    // 3. 创建一个隐藏的窗口来承载上下文
    glfw.window_hint(WindowHint::Visible(false));
    let (mut window, _events) = glfw
        .create_window(100, 100, "Shader Compile Test", glfw::WindowMode::Windowed)
        .expect("创建GLFW窗口失败");

    // 4. 激活上下文并加载GL函数
    window.make_current();
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    println!("OpenGL上下文已创建。");

    // --- 修改的部分 ---

    // 5. 定义你的输入
    let shader_path = "src/my_shader.frag";

    // 6. 读取Shader源文件
    let shader_source_code = match fs::read_to_string(shader_path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("错误: 无法读取Shader文件 '{}': {}", shader_path, e);
            return;
        }
    };

    // 7. 编译Shader
    println!("正在编译 {}...", shader_path);
    //
    // *** 关键修改点 1: 更新函数调用 ***
    // 我们不再传入 version 和 macros
    //
    match compile_shader(
        gl::FRAGMENT_SHADER,
        &shader_source_code, // 只传入从文件读取的完整源码
    ) {
        Ok(shader_id) => {
            println!("\n===================================");
            println!("✅ Shader编译成功！");
            println!("===================================");
            // 别忘了清理
            unsafe {
                gl::DeleteShader(shader_id);
            }
        }
        Err(error_log) => {
            eprintln!("\n===================================");
            eprintln!("❌ Shader编译失败！");
            eprintln!("===================================");
            eprintln!("错误日志:\n{}", error_log);
        }
    }
}

//
// *** 关键修改点 2: 简化函数签名 ***
//
/// 编译一个Shader并返回其ID，或者返回错误日志
fn compile_shader(
    shader_type: gl::types::GLenum,
    source: &str, // 只接收一个完整的源码字符串
) -> Result<gl::types::GLuint, String> {
    unsafe {
        let shader = gl::CreateShader(shader_type);

        // 8. 准备C字符串 (只需要一个)
        let c_str_source = match CString::new(source.as_bytes()) {
            Ok(c) => c,
            Err(e) => return Err(format!("Shader源码中包含非法的NUL字符: {}", e)),
        };

        // 9. 组合Shader源 (数组只有一个元素)
        //
        // *** 关键修改点 3: sources 数组 ***
        //
        let sources = [c_str_source.as_ptr()];

        // 10. 设置Shader源码
        gl::ShaderSource(
            shader,
            sources.len() as i32, // 这里会传入 1
            sources.as_ptr(),     // 指向这个单元素数组
            std::ptr::null(),
        );

        // 11. 编译!
        gl::CompileShader(shader);

        // 12. 检查编译状态 (这部分代码不变)
        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success == gl::TRUE as gl::types::GLint {
            Ok(shader)
        } else {
            // 13. 获取错误日志 (这部分代码不变)
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
            // len包含null终止符，所以我们减1
            buffer.set_len((len as usize).saturating_sub(1));

            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut gl::types::GLchar,
            );

            // 编译失败，删除这个shader对象
            gl::DeleteShader(shader);

            Err(String::from_utf8(buffer)
                .unwrap_or_else(|e| format!("无法解码Shader错误日志: {}", e)))
        }
    }
}
