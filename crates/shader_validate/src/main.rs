extern crate gl;
extern crate glfw;

use std::ffi::CString;
use std::fs;

use glfw::{Context, OpenGlProfileHint, WindowHint};

fn main() {
    println!("Initializing GLFW to obtain OpenGL context...");

    // 1. Initialize GLFW
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to initialize GLFW");

    // 2. Request OpenGL 3.2 Core Profile (GLSL 150 corresponds to OpenGL 3.2)
    glfw.window_hint(WindowHint::ContextVersion(3, 2));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    // 3. Create a hidden window to host the context
    glfw.window_hint(WindowHint::Visible(false));
    let (mut window, _events) = glfw
        .create_window(100, 100, "Shader Compile Test", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    // 4. Activate context and load GL functions
    window.make_current();
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    println!("OpenGL context created.");

    // --- Modified section ---

    // 5. Define your input
    let shader_path = "src/my_shader.frag";

    // 6. Read Shader source file
    let shader_source_code = match fs::read_to_string(shader_path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: Unable to read Shader file '{}': {}", shader_path, e);
            return;
        }
    };

    // 7. Compile Shader
    println!("Compiling {}...", shader_path);
    //
    // *** Key modification 1: Update function call ***
    // We no longer pass in version and macros
    //
    match compile_shader(
        gl::FRAGMENT_SHADER,
        &shader_source_code, // Only pass in the complete source code read from file
    ) {
        Ok(shader_id) => {
            println!("\n===================================");
            println!("Shader compilation successful!");
            println!("===================================");
            // Don't forget to clean up
            unsafe {
                gl::DeleteShader(shader_id);
            }
        }
        Err(error_log) => {
            eprintln!("\n===================================");
            eprintln!("Shader compilation failed!");
            eprintln!("===================================");
            eprintln!("Error log:\n{}", error_log);
        }
    }
}

//
// *** Key modification 2: Simplify function signature ***
//
/// Compile a Shader and return its ID, or return the error log
fn compile_shader(
    shader_type: gl::types::GLenum,
    source: &str, // Only accepts a complete source code string
) -> Result<gl::types::GLuint, String> {
    unsafe {
        let shader = gl::CreateShader(shader_type);

        // 8. Prepare C string (only need one)
        let c_str_source = match CString::new(source.as_bytes()) {
            Ok(c) => c,
            Err(e) => return Err(format!("Shader source contains illegal NUL character: {}", e)),
        };

        // 9. Combine Shader sources (array has only one element)
        //
        // *** Key modification 3: sources array ***
        //
        let sources = [c_str_source.as_ptr()];

        // 10. Set Shader source code
        gl::ShaderSource(
            shader,
            sources.len() as i32, // This will pass in 1
            sources.as_ptr(),     // Points to this single-element array
            std::ptr::null(),
        );

        // 11. Compile!
        gl::CompileShader(shader);

        // 12. Check compilation status (this part of code remains unchanged)
        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success == gl::TRUE as gl::types::GLint {
            Ok(shader)
        } else {
            // 13. Get error log (this part of code remains unchanged)
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
            // len includes null terminator, so we subtract 1
            buffer.set_len((len as usize).saturating_sub(1));

            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut gl::types::GLchar,
            );

            // Compilation failed, delete this shader object
            gl::DeleteShader(shader);

            Err(String::from_utf8(buffer)
                .unwrap_or_else(|e| format!("Unable to decode Shader error log: {}", e)))
        }
    }
}
