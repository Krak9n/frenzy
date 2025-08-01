extern crate glfw;
extern crate glad_gl;

use glfw::{Action, Context, Key};
use glad_gl::gl;
use glad_gl::gl::*;
use std::os::raw::c_void;

static TITLE: &str = "fuck";
static WIDTH: u32 = 1500;
static HEIGHT: u32 = 800;

type Vertex = [f32; 6];

const VERTICES: [Vertex; 4] =
  [[-0.5, -0.5, 0.0, 1.0, 0.0, 0.0], // bottom left
  [0.5, -0.5, 0.0, 0.5, 0.5, 0.0], // bottom right
  [0.5, 0.5, 0.0, 0.0, 1.0, 0.0], // top right 
  [-0.5, 0.5, 0.0, 0.0, 0.0, 1.0]];  // top left

type Index = [u32; 3];

const INDICES: [Index; 2] =
  [[0, 1 , 2],
  [0, 3, 2]];

const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  layout (location = 1) in vec3 aColor;

  out vec3 color;

  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    color = aColor;
  }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 final_color;
  in vec3 color;

  void main() {
    final_color = vec4(color, 1.0);
  }
"#;

fn main() {
  let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

  let (mut window, events) = glfw.create_window(
      WIDTH, 
      HEIGHT, 
      TITLE, 
      glfw::WindowMode::Windowed)
      .expect("Failed to create GLFW window.");

  window.set_key_polling(true);
  window.make_current();

  gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

  unsafe {
    let mut vao = 0;
    
    // vao
    gl::GenVertexArrays(1, &mut vao);
    assert_ne!(vao, 0);
    // vbo
    let mut vbo = 0;
    gl::GenVertexArrays(1, &mut vbo);
    assert_ne!(vbo, 0);
    // ebo
    let mut ebo = 0;
    gl::GenVertexArrays(1, &mut ebo);
    assert_ne!(ebo, 0);

    // triangle attributes
    BindVertexArray(vao);
    gl::BindBuffer(ARRAY_BUFFER, vbo);
    gl::BufferData(
      ARRAY_BUFFER,
      size_of_val(&VERTICES) as isize,
      VERTICES.as_ptr().cast(),
      STATIC_DRAW);
    gl::BindBuffer(
      ELEMENT_ARRAY_BUFFER,
      ebo);
    gl::BufferData(
      ELEMENT_ARRAY_BUFFER,
      size_of_val(&INDICES) as isize,
      INDICES.as_ptr().cast(),
      STATIC_DRAW);

    VertexAttribPointer(
      0,
      3,
      FLOAT,
      FALSE,
      size_of::<Vertex>().try_into().unwrap(),
      0 as *const _,
      );
    EnableVertexAttribArray(0);
    gl::VertexAttribPointer(
      1, 
      3, 
      FLOAT,
      FALSE,
      size_of::<Vertex>().try_into().unwrap(),
      (size_of::<f32>() * 3) as *const c_void,
      );
    EnableVertexAttribArray(1);

    //BindBuffer(ARRAY_BUFFER, 0);
    //BindVertexArray(0);

    let vertex_shader = gl::CreateShader(VERTEX_SHADER);
    assert_ne!(vertex_shader, 0);
    gl::ShaderSource(
      vertex_shader,
      1,
      &(VERT_SHADER.as_bytes().as_ptr().cast()),
      &(VERT_SHADER.len().try_into().unwrap()),
    );
    gl::CompileShader(vertex_shader);
    let mut success = 0;
    gl::GetShaderiv(vertex_shader, COMPILE_STATUS, &mut success);
    if success == 0 {
      let mut v: Vec<u8> = Vec::with_capacity(1024);
      let mut log_len = 0_i32;
      gl::GetShaderInfoLog(
        vertex_shader,
        1024,
        &mut log_len,
        v.as_mut_ptr().cast(),
      );
      v.set_len(log_len.try_into().unwrap());
      panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
    }

    let fragment_shader = gl::CreateShader(FRAGMENT_SHADER);
    assert_ne!(fragment_shader, 0);
    gl::ShaderSource(
      fragment_shader,
      1,
      &(FRAG_SHADER.as_bytes().as_ptr().cast()),
      &(FRAG_SHADER.len().try_into().unwrap()),
    );
    gl::CompileShader(fragment_shader);
    let mut success = 0;
    gl::GetShaderiv(fragment_shader, COMPILE_STATUS, &mut success);
    if success == 0 {
      let mut v: Vec<u8> = Vec::with_capacity(1024);
      let mut log_len = 0_i32;
      gl::GetShaderInfoLog(
        fragment_shader,
        1024,
        &mut log_len,
        v.as_mut_ptr().cast(),
      );
      v.set_len(log_len.try_into().unwrap());
      panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
    }

    let shader_program = gl::CreateProgram();
    assert_ne!(shader_program, 0);
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);
    let mut success = 0;
    gl::GetProgramiv(shader_program, LINK_STATUS, &mut success);
    if success == 0 {
      let mut v: Vec<u8> = Vec::with_capacity(1024);
      let mut log_len = 0_i32;
      gl::GetProgramInfoLog(
        shader_program,
        1024,
        &mut log_len,
        v.as_mut_ptr().cast(),
      );
      v.set_len(log_len.try_into().unwrap());
      panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
    }
    DeleteShader(vertex_shader);
    DeleteShader(fragment_shader);

    UseProgram(shader_program);
    //BindVertexArray(vao);
  }

  while !window.should_close() {
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
        handle_window_event(&mut window, event);
    }

    unsafe {
      //gl::ClearColor(0.7, 0.9, 0.1, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);    
    
      DrawElements(TRIANGLES, 6, UNSIGNED_INT, std::ptr::null());
    }
    window.swap_buffers();
  }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
  match event {
    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
        window.set_should_close(true)
    }
    _ => {}
  }
}
