use gl;
use glfw::Context;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 640;
const WINDOW_TITLE: &str = "GLFW Triangle";

const VERT_SHADER: &str = "#version 330 core
    layout (location = 0) in vec3 position;
     
    void main()
    {
        gl_Position = vec4(position, 1.0);
    }";

const FRAG_SHADER: &str = "#version 330 core
    out vec4 Color;
    void main()
    {
        Color = vec4(0.9, 0.2, 0.6, 1.0);
    }";

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    let (mut window, events) = glfw
        .create_window(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            WINDOW_TITLE,
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");
    let (screen_width, screen_height) = window.get_framebuffer_size();

    window.make_current();
    // Set window to receive events
    window.set_key_polling(true);
    // Load GL Lib
    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    // Set Background Color
    unsafe {
        gl::Viewport(0, 0, screen_width, screen_height);
        gl_clear_color(Color {
            r: 0.12,
            g: 0.12,
            b: 0.12,
            a: 1.0,
        });
    }

    // HANDLE VERTEX SHADER (Set coordinates)
    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    unsafe {
        gl::ShaderSource(
            vertex_shader,
            1,
            &VERT_SHADER.as_bytes().as_ptr().cast(),
            &VERT_SHADER.len().try_into().unwrap(),
        );
        gl::CompileShader(vertex_shader);

        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log_len = 0_i32;
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            gl::GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!(
                "Vertex Shared Compile Error: {}",
                String::from_utf8_lossy(&v)
            );
        }
    }

    // HANDLE FRAGMENT SHADER (Calculates the color output of the pixels)
    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    unsafe {
        gl::ShaderSource(
            fragment_shader,
            1,
            &FRAG_SHADER.as_bytes().as_ptr().cast(),
            &FRAG_SHADER.len().try_into().unwrap(),
        );
        gl::CompileShader(fragment_shader);

        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!(
                "Fragment Shader Compile Error: {}",
                String::from_utf8_lossy(&v)
            );
        }
    }

    // SHADER PROGRAM CREATION
    let shader_program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }

        gl::DetachShader(shader_program, vertex_shader);
        gl::DetachShader(shader_program, fragment_shader);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(vertex_shader);
    }

    // Triangle Coords (X, Y, Z)
    #[rustfmt::skip]
    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, 
        0.5, -0.5, 0.0, 
        0.0, 0.5, 0.0
    ];

    let mut vao = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao) };

    let mut vbo = 0;
    unsafe { gl::GenBuffers(1, &mut vbo) };

    unsafe {
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as i32,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    println!("OpenGL version: {}", gl_get_string(gl::VERSION));
    println!(
        "GLSL version: {}",
        gl_get_string(gl::SHADING_LANGUAGE_VERSION)
    );

    while !window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            glfw_handle_event(&mut window, event);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        unsafe {
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
        }

        window.swap_buffers();
    }
}

pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

pub fn gl_clear_color(c: Color) {
    unsafe { gl::ClearColor(c.r, c.g, c.b, c.a) }
}

pub fn gl_get_string<'a>(name: gl::types::GLenum) -> &'a str {
    let v = unsafe { gl::GetString(name) };
    let v: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(v as *const i8) };
    v.to_str().unwrap()
}

pub fn glfw_handle_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    use glfw::WindowEvent as Event;
    use glfw::{Action, Key};

    println!("{event:?}");
    match event {
        Event::Close => window.set_should_close(true),
        Event::Key(Key::Q, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
