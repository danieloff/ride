// Dan October-7-2017
/*
#![deny(non_camel_case_types)]
#![deny(unused_parens)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]
//#![warn(missing_docs)] // FIXME: should be denied.
#![deny(unused_results)]
#![allow(unused_unsafe)] // FIXME: should be denied
#![allow(missing_copy_implementations)]
//#![doc(html_root_url = "http://kiss3d.org/doc")]
*/
/*
extern crate libc;
extern crate gl;
extern crate num_traits as num;
extern crate nalgebra as na;
extern crate ncollide_procedural;
extern crate image;
extern crate freetype;
extern crate glfw;
extern crate backtrace;
*/

/*
mod error;
pub mod window;
pub mod scene;
pub mod camera;
pub mod light;
pub mod loader;
pub mod line_renderer;
pub mod point_renderer;
pub mod builtin;
pub mod post_processing;
pub mod resource;
pub mod text;
*/
use ctrlc;
//use libc;
//use libc::c_int;
use std::process::exit;
use backtrace::Backtrace;
use std::ops::AddAssign;

use std::fs::File;

use std::fmt::Write;

use crate::asset::*;
/*
use assets::material::{mat_load_file, material_delete};
use assets::renderable::{obj_load_file, renderable_delete};
use assets::shader::{vs_load_file, fs_load_file, shader_delete};
use assets::texture::{dds_load_file, texture_delete};
*/
use crate::assets::material::mat_load_file;
use crate::assets::renderable::obj_load_file;
use crate::assets::shader::{vs_load_file, fs_load_file};
use crate::assets::texture::dds_load_file;

use crate::entity::*;
use crate::entities::camera::camera_new; 

/*
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
*/

//I think I need a ride instance struct for global state?
//or multiple structs for individual (global) state?

struct RideState {
  logout: Option<File>,
}

impl RideState {
  fn write_log(&mut self, string: &str) {
    use std::io::Write;

    match self.logout {
      Some(ref mut file) => writeln!(file, "{}", string).unwrap(),
      None => {}
    }
  }
}

//is there an 'internal' level of public?
static mut G_RIDE_STATE: RideState = RideState { logout: None };

pub fn g_set_log_file(logout: File) {
  unsafe {
    G_RIDE_STATE.logout = Some(logout);
  };
}

pub fn g_ride_write_log(string: &str) {
  unsafe {
    G_RIDE_STATE.write_log(string);
  };
}

//this was a macro I think
pub fn error(string: &str) {
  let mut full: String = "[Error] ".into();
  full.add_assign(string);
  error_(&full);
}

//these too
/*
#define warning(MSG, ...) { \
  sprintf(warning_str, "[WARNING] (%s:%s:%i) ", __FILE__, __func__, __LINE__); \
  sprintf(warning_buf, MSG, ##__VA_ARGS__); strcat(warning_str, warning_buf); \
  warning_(warning_str); }
*/

pub fn warning(string: &str) {
  let mut full: String = "[WARNING] ".into(); //it would be nice to have the file/line info macro working
  full.add_assign(string);
  warning_(&full);
}

/*
#define debug(MSG, ...) { \
  sprintf(debug_str, "[DEBUG] (%s:%s:%i) ", __FILE__, __func__, __LINE__); \
  sprintf(debug_buf, MSG, ##__VA_ARGS__); strcat(debug_str, debug_buf); \
  debug_(debug_str); }
*/

pub fn debug(string: &str) {
  let mut full: String = "[DEBUG] ".into(); //it would be nice to have the file/line info macro working
  full.add_assign(string);
  debug_(&full);
}

fn error_(string: &str) {
  //this is supposed to call all the registered error functions
  ride_error(string);
}

fn warning_(string: &str) {
  ride_warning(string);
}

fn debug_(string: &str) {
  ride_debug(string);
}

fn ride_signal() {
  error("Program Terminated");
}
/*
fn ride_signal(sig: c_int) {
  //these would be caught by c land? panic seems to get the div by zero... Program Interrupted goes
  //through here

  match sig {
    libc::SIGABRT => error("Program Aborted"),
    libc::SIGFPE => error("Division By Zero"),
    libc::SIGILL => error("Illegal Instruction"),
    libc::SIGINT => error("Program Interrupted"),
    libc::SIGSEGV => error("Segmentation fault"),
    libc::SIGTERM => error("Program Terminated"),
    _ => {}
  }
}
*/
//static FILE* logout = NULL;

fn ride_error(string: &str) {
  eprintln!("{}", string);
  g_ride_write_log(string);

  //not sure how useful this backtrace is
  let bt = Backtrace::new();
  eprintln!("{:?}", bt);

  //panicked while panicking, sweet.
  //panic!();

  exit(::libc::EXIT_FAILURE);
}

pub fn ride_warning(string: &str) {
  println!("{}", string);
  g_ride_write_log(string);
}

pub fn ride_debug(string: &str) {
  println!("{}", string);
  g_ride_write_log(string);
}

//struct Fish;

//fn loadme(path:&str) -> Rc<RefCell<Any>> {
//    return Rc::new(RefCell::new(0))
//}

//fn deleteme(a:Rc<RefCell<Any>>) {
//}

#[allow(unused_variables)]
pub fn ride_init(core_assets_path: &str) {

  // Attach signal handlers
  let _ = ctrlc::set_handler(ride_signal);
/*  unsafe {
    let _ = libc::signal(libc::SIGABRT, ride_signal as libc::sighandler_t);
    let _ = libc::signal(libc::SIGFPE, ride_signal as libc::sighandler_t);
    let _ = libc::signal(libc::SIGILL, ride_signal as libc::sighandler_t);
    let _ = libc::signal(libc::SIGINT, ride_signal as libc::sighandler_t);
    let _ = libc::signal(libc::SIGSEGV, ride_signal as libc::sighandler_t);
    let _ = libc::signal(libc::SIGTERM, ride_signal as libc::sighandler_t);
  }
*/
  /*
       logout = fopen("output.log", "w");

       at_error(corange_error);
       at_warning(corange_warning);
       at_debug(corange_debug);
    */

  let logout = File::create("output.log").unwrap();

  g_set_log_file(logout);

  //g_ride_write_log("Hello World");


  /*
    // Starting Corange
    debug("Starting Ride...");

    // Asset Manager 
    debug("Creating Asset Manager...");
    debug("Core Assets At '%s' ...", core_assets_path);

    asset_init();
    asset_add_path_variable(P("$CORANGE"), P(core_assets_path));

    asset_handler(renderable, "bmf", bmf_load_file, renderable_delete);
    asset_handler(renderable, "obj", obj_load_file, renderable_delete);
    asset_handler(renderable, "smd", smd_load_file, renderable_delete);
    asset_handler(renderable, "ply", ply_load_file, renderable_delete);
    asset_handler(skeleton, "skl", skl_load_file, skeleton_delete);
    asset_handler(animation, "ani", ani_load_file, animation_delete);
    asset_handler(cmesh, "col", col_load_file, cmesh_delete);
    asset_handler(terrain, "raw", raw_load_file, terrain_delete);

    asset_handler(texture, "bmp", bmp_load_file, texture_delete);
    asset_handler(texture, "tga", tga_load_file, texture_delete);
    asset_handler(texture, "dds", dds_load_file, texture_delete);
    asset_handler(texture, "lut", lut_load_file, texture_delete);
    asset_handler(texture, "acv", acv_load_file, texture_delete);

    asset_handler(shader, "vs" , vs_load_file, shader_delete);
    asset_handler(shader, "fs" , fs_load_file, shader_delete);
    asset_handler(shader, "gs" , gs_load_file, shader_delete);
    asset_handler(shader, "tcs" , tcs_load_file, shader_delete);
    asset_handler(shader, "tes" , tes_load_file, shader_delete);

    asset_handler(config, "cfg", cfg_load_file, config_delete);
    asset_handler(lang, "lang", lang_load_file, lang_delete);
    asset_handler(font, "fnt", font_load_file, font_delete);

    asset_handler(material, "mat", mat_load_file, material_delete);
    asset_handler(effect, "effect" , effect_load_file, effect_delete);

    asset_handler(sound, "wav", wav_load_file, sound_delete);
    asset_handler(music, "ogg", ogg_load_file, music_delete);
    asset_handler(music, "mp3", mp3_load_file, music_delete);
    */

  debug("Starting Ride...");

  // Asset Manager
  debug("Creating Asset Manager...");
  let mut debug_full: String = String::new();
  let _ = write!(&mut debug_full, "Core assets at '{}' ...", core_assets_path);
  debug(&debug_full);

  asset_init();
  asset_add_path_variable("$CORANGE", core_assets_path);

  asset_handler("obj", obj_load_file); //, renderable_delete);
  asset_handler("mat", mat_load_file); //, material_delete);

  asset_handler("vs", vs_load_file); //, shader_delete);
  asset_handler("fs", fs_load_file); //, shader_delete);


  asset_handler("dds", dds_load_file); //, texture_delete);


  /*
    asset_handler!(Renderable, "bmf", bmf_load_file, renderable_delete);
    asset_handler!(Renderable, "obj", obj_load_file, renderable_delete);
    asset_handler!(Renderable, "smd", smd_load_file, renderable_delete);
    asset_handler!(Renderable, "ply", ply_load_file, renderable_delete);
    asset_handler!(Skeleton, "skl", skl_load_file, skeleton_delete);
    asset_handler!(Animation, "ani", ani_load_file, animation_delete);
    asset_handler!(CMesh, "col", col_load_file, cmesh_delete);
    asset_handler!(Terrain, "raw", raw_load_file, terrain_delete);

    asset_handler!(Texture, "bmp", bmp_load_file, texture_delete);
    asset_handler!(Texture, "tga", tga_load_file, texture_delete);
    asset_handler!(Texture, "dds", dds_load_file, texture_delete);
    asset_handler!(Texture, "lut", lut_load_file, texture_delete);
    asset_handler!(Texture, "acv", acv_load_file, texture_delete);

    asset_handler!(Shader, "vs" , vs_load_file, shader_delete);
    asset_handler!(Shader, "fs" , fs_load_file, shader_delete);
    asset_handler!(Shader, "gs" , gs_load_file, shader_delete);
    asset_handler!(Shader, "tcs" , tcs_load_file, shader_delete);
    asset_handler!(Shader, "tes" , tes_load_file, shader_delete);

    asset_handler!(Config, "cfg", cfg_load_file, config_delete);
    asset_handler!(Lang, "lang", lang_load_file, lang_delete);
    asset_handler!(Font, "fnt", font_load_file, font_delete);

    asset_handler!(Material, "mat", mat_load_file, material_delete);
    asset_handler!(Effect, "effect" , effect_load_file, effect_delete);

    asset_handler!(Sound, "wav", wav_load_file, sound_delete);
    asset_handler!(Music, "ogg", ogg_load_file, music_delete);
    asset_handler!(Music, "mp3", mp3_load_file, music_delete);

*/

  /*
   */
  // Entity Manager
  debug("Creating Entity Manager...");

  entity_init();
  /*
    entity_handler(static_object, static_object_new, static_object_delete);
    entity_handler(animated_object, animated_object_new, animated_object_delete);
    entity_handler(physics_object, physics_object_new, physics_object_delete);
    entity_handler(instance_object, instance_object_new, instance_object_delete);
*/
  entity_handler(camera_new);
  /*
    entity_handler(light, light_new, light_delete);
    entity_handler(landscape, landscape_new, landscape_delete);
    entity_handler(particles, particles_new, particles_delete);

    // UI Manager 
    debug("Creating UI Manager...");

    ui_init();

    ui_handler(ui_rectangle, ui_rectangle_new, ui_rectangle_delete, ui_rectangle_event, ui_rectangle_update, ui_rectangle_render);
    ui_handler(ui_text, ui_text_new, ui_text_delete, ui_text_event, ui_text_update, ui_text_render);
    ui_handler(ui_spinner, ui_spinner_new, ui_spinner_delete, ui_spinner_event, ui_spinner_update, ui_spinner_render);
    ui_handler(ui_button, ui_button_new, ui_button_delete, ui_button_event, ui_button_update, ui_button_render);
    ui_handler(ui_textbox, ui_textbox_new, ui_textbox_delete, ui_textbox_event, ui_textbox_update, ui_textbox_render);
    ui_handler(ui_browser, ui_browser_new, ui_browser_delete, ui_browser_event, ui_browser_update, ui_browser_render);
    ui_handler(ui_toast, ui_toast_new, ui_toast_delete, ui_toast_event, ui_toast_update, ui_toast_render);
    ui_handler(ui_dialog, ui_dialog_new, ui_dialog_delete, ui_dialog_event, ui_dialog_update, ui_dialog_render);
    ui_handler(ui_listbox, ui_listbox_new, ui_listbox_delete, ui_listbox_event, ui_listbox_update, ui_listbox_render);
    ui_handler(ui_option, ui_option_new, ui_option_delete, ui_option_event, ui_option_update, ui_option_render);
    ui_handler(ui_slider, ui_slider_new, ui_slider_delete, ui_slider_event, ui_slider_update, ui_slider_render);

    // Graphics Manager 
    debug("Creating Graphics Manager...");
    graphics_init();

    // Audio Manager
    debug("Creating Audio Manager...");
    audio_init();

    // Joystick Manager
    debug("Creating Joystick Manager...");
    joystick_init();

    // Network Manager 
    debug("Creating Network Manager...");
    net_init();

    debug("Finished!");
    */
}

pub fn ride_finish() {
  entity_finish();
  asset_finish();
  /*
  ui_finish();
  entity_finish();
  asset_finish();
  
  net_finish();
  joystick_finish();
  audio_finish();
  graphics_finish();
  
  SDL_Quit();

  if (logout) { fclose(logout); }
  */
}
