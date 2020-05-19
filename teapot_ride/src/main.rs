//Daniel Off 10/22/2019 rough port from corange example on github

use cgmath::*;

use glfw::{Context, WindowEvent};
use std::sync::mpsc::Receiver;
use std::os::raw;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

use ride::core::*;
use ride::engine::*;
use ride::asset::*;
use ride::assets::renderable::*;
use ride::assets::shader::*;
use ride::assets::material::*;
use ride::entity::*;
use ride::entities::camera::*;

struct WindowStuff {
  window: glfw::Window,
  glfw: glfw::Glfw,
  events: Receiver<(f64, WindowEvent)>,
}


fn init(title: &str, width: u32, height: u32) -> WindowStuff {
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

  glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
  glfw.window_hint(glfw::WindowHint::OpenGlProfile(
    glfw::OpenGlProfileHint::Core,
  ));

  glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

  let create_window_result = glfw
    .create_window(width, height, title, glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window.");

  let mut window: glfw::Window = create_window_result.0;
  let events: Receiver<(f64, WindowEvent)> = create_window_result.1;
  //let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
  //    .expect("Failed to create GLFW window.");

  window.set_key_polling(true);
  window.set_cursor_pos_polling(true);
  window.set_mouse_button_polling(true);
  window.set_scroll_polling(true);
  window.make_current();
  glfw.set_swap_interval(glfw::SwapInterval::Sync(1)); //enable vsync

  // the supplied function must be of the type:
  // `&fn(symbol: &str) -> Option<extern "C" fn()>`
  // `window` is a glfw::Window
  gl::load_with(|s| window.get_proc_address(s) as *const _);

  // Define the viewport dimensions
  let (width, height) = window.get_framebuffer_size(); //get_size(); //different from window size?, yes, on retina the window size is returned as smaller!!

  unsafe {
    gl::Viewport(0, 0, width, height);
  }

  window.set_sticky_mouse_buttons(true);

  WindowStuff {
    window: window,
    glfw: glfw,
    events: events,
  }
}

fn main() {
  //  #ifdef _WIN32
  //    FILE* ctt = fopen("CON", "w" );
  //    FILE* fout = freopen( "CON", "w", stdout );
  //    FILE* ferr = freopen( "CON", "w", stderr );
  //  #endif

  let WindowStuff {
    mut window,
    mut glfw,
    events,
  } = init("Teapot", 1280, 720);

  ride_init("./assets_core");

  //setup the camera
  {
    let cam = entity_new::<Camera>("camera", "cameratype").unwrap();
    let mut cam = cam.borrow_mut();
    cam.position = Vec3::new(5.0, 5.0, 5.0);
    cam.target = Vec3::new(0.0, 0.0, 0.0);
  }

  //get some objects to play with
  let mut teapot_material_hndl = asset_hndl_new_load("./assets/teapot.mat");
  let mut teapot_object_hndl = asset_hndl_new_load::<Renderable>("./assets/teapot.obj");
  let mut mousepos = {
    let (x, y) = window.get_cursor_pos();
    Vec2::new(x as f32, y as f32)
  };

  //construct surfaces, linking shader to buffer format
  unsafe {
    let teapot_object = asset_hndl_ptr(&mut teapot_object_hndl).unwrap();
    let teapot_material = asset_hndl_ptr(&mut teapot_material_hndl).unwrap();
    let mut teapot_material = teapot_material.borrow_mut();
    let shader = material_first_program(&mut teapot_material).unwrap();

    for s in teapot_object.borrow_mut().surfaces.iter_mut() {
      //gl::GenVertexArrays(1, &mut s.vao);
      gl::BindVertexArray(s.vao);
      gl::BindBuffer(gl::ARRAY_BUFFER, s.vertex_vbo);
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, s.triangle_vbo);

      let size_of_float = 4;
      shader_program_enable_attribute(shader, "vPosition", 3, 18, 0 as *const raw::c_void);
      shader_program_enable_attribute(
        shader,
        "vNormal",
        3,
        18,
        (size_of_float * 3) as *const raw::c_void,
      );

      gl::BindVertexArray(0);
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); //Perhaps I must STOP the binding of the VAO before unbinding the buffers!!->
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
  }

  while !window.should_close() {
    //ride::frame_begin();
    let cam = entity_get::<Camera>("camera").unwrap();
    {
      let mut cam = cam.borrow_mut();
      glfw.poll_events();
      for (_, event) in glfw::flush_messages(&events) {
        handle_window_event(&mut window, event, &mut cam, &mut mousepos);
      }
    }

    //ride::ui_update();

    unsafe {
      gl::ClearColor(0.25, 0.25, 0.25, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
      gl::Enable(gl::DEPTH_TEST);
    }

    {
      let teapot_material = asset_hndl_ptr(&mut teapot_material_hndl).unwrap();
      let mut teapot_material = teapot_material.borrow_mut(); //take advantage of the lexical scoping??
      let shader = material_first_program(&mut teapot_material).unwrap();
      let cam = cam.borrow();
      let (width, height) = window.get_size();
      //let ratio: f32 = (width as f32) / (height as f32);
      let ratio: f32 = (height as f32) / (width as f32);
      let mut cube_sea_texture = asset_hndl_new_load("$CORANGE/water/cube_sea.dds");
      let teapot_object = asset_hndl_ptr(&mut teapot_object_hndl).unwrap();
      let teapot_object = teapot_object.borrow();

      shader_program_enable(shader);
      shader_program_set_mat4(shader, "world", &Mat4::identity());
      shader_program_set_mat4(shader, "view", &camera_view_matrix(&cam));
      shader_program_set_mat4(shader, "proj", &camera_proj_matrix(&cam, ratio));
      shader_program_set_texture(shader, "cube_beach", 0, &mut cube_sea_texture);
      shader_program_set_vec3(shader, "camera_direction", &camera_direction(&cam));

      for surface in teapot_object.surfaces.iter() {
        unsafe {
          gl::BindVertexArray(surface.vao);
          gl::DrawElements(
            gl::TRIANGLES,
            (surface.num_triangles * 3) as gl::types::GLint,
            gl::UNSIGNED_INT,
            0 as *const raw::c_void,
          );
          gl::BindVertexArray(0);
        }
      }

      shader_program_disable(shader);
    }

    unsafe {
      gl::Disable(gl::DEPTH_TEST);
    }
    /*ride::ui_render();

      ride::graphics_swap();

      ride::frame_end();*/

    window.swap_buffers();
  }

  ride_finish();
}

fn camera_control_orbit(
  window: &mut glfw::Window,
  c: &mut Camera,
  mousemove: &Vec2,
  scrollmove: &Vec2,
) {
  let translation = c.target;
  c.position = c.position.sub(translation);
  c.target = c.target.sub(translation);

  let state = window.get_mouse_button(glfw::MouseButtonLeft);

  if state == glfw::Action::Press {
    //println!("in press");
    let a1 = mousemove.x * 0.005;  // 0.43;
    let a2 = mousemove.y * -0.005; //-1.0;
    c.position = mat3_rotation_y(a1).rotate_vector(c.position);
    let axis = c.position
      .sub(c.target)
      .cross(Vec3::new(0.0, 1.0, 0.0))
      .normalize();
    c.position = mat3_rotation_angle_axis(a2, axis).rotate_vector(c.position);
  }

  if scrollmove.y != 0.0 {
    let delta = c.position.normalize().mul(-scrollmove.y);
    c.position = c.position.add(delta);
  }

  c.position = c.position.add(translation);
  c.target = c.target.add(translation);
}

fn handle_window_event(
  window: &mut glfw::Window,
  event: glfw::WindowEvent,
  c: &mut Camera,
  mousepos: &mut Vec2,
) {
  let startmouse = *mousepos;
  let mut startscroll = Vec2::zero();
  match event {
    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
      window.set_should_close(true) //TODO this should exit full screen on OSX!! NOT close
    }
    glfw::WindowEvent::Scroll(x, y) => {
      startscroll.x = startscroll.x + x as f32;
      startscroll.y = startscroll.y + y as f32;
    }
    //glfw::WindowEvent::CursorPos(x, y) => {
    //  deltamouse = startmouse.sub(&Vec2::new(x as f32, y as f32));
    //  println!("moved mouse");
    //}
    _ => {}
  }

  let curpos = {
    let (x, y) = window.get_cursor_pos(); //TODO, use callbacks probably
    Vec2::new(x as f32, y as f32)
  };

  let deltamouse = startmouse.sub(&curpos);

  //println!("deltamouse {} ", deltamouse);
  camera_control_orbit(window, c, &deltamouse, &startscroll);
  *mousepos = curpos;
}
