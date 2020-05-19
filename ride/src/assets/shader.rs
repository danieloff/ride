use gl;

//use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::os::raw;

use std::ffi::CString;
use std::ptr;

use std::cell::RefCell;
//use std::any::Any;
use std::rc::Rc;
//use std::os::raw::c_char;

use std::fmt::Write;

use crate::core::{error, warning, debug};
use crate::asset::{AssetHandle, asset_hndl_ptr};
use crate::assets::texture::{Texture, texture_handle, texture_type};
//use engine::{Vec2, Vec3, Vec4, Mat3, Mat4, ToArr, gl_check_error};
use crate::engine::*;

use cgmath::*;
//use ::na::storage::Storage;

pub struct Shader(gl::types::GLuint);

impl Shader {
  fn to_gl(&self) -> gl::types::GLuint {
    self.0
  }
}

pub struct ShaderProgram(gl::types::GLuint);

impl ShaderProgram {
  fn to_gl(&self) -> gl::types::GLuint {
    self.0
  }
}

/*
/**
*** :: Shader ::
***
***   GLSL shader program.
***
**/

#ifndef shader_h
#define shader_h

#include "cengine.h"
#include "casset.h"

typedef GLuint shader;
typedef GLuint shader_program;

shader* vs_load_file(char* filename);
shader* fs_load_file(char* filename);
shader* gs_load_file(char* filename);
shader* tcs_load_file(char* filename);
shader* tes_load_file(char* filename);

void shader_delete(shader* s);
void shader_print_log(shader* s);
GLuint shader_handle(shader* s);

shader_program* shader_program_new();
void shader_program_delete(shader_program* p);

bool shader_program_has_shader(shader_program* p, shader* s);
void shader_program_attach_shader(shader_program* p, shader* s);
void shader_program_link(shader_program* p);

void shader_program_print_info(shader_program* p);
void shader_program_print_log(shader_program* p);

GLuint shader_program_handle(shader_program* p);
GLint shader_program_get_attribute(shader_program* p, char* name);

void shader_program_enable(shader_program* p);
void shader_program_disable(shader_program* p);

void shader_program_set_int(shader_program* p, char* name, int val);
void shader_program_set_float(shader_program* p, char* name, float val);
void shader_program_set_vec2(shader_program* p, char* name, vec2 val);
void shader_program_set_vec3(shader_program* p, char* name, vec3 val);
void shader_program_set_vec4(shader_program* p, char* name, vec4 val);
void shader_program_set_mat3(shader_program* p, char* name, mat3 val);
void shader_program_set_mat4(shader_program* p, char* name, mat4 val);
void shader_program_set_float_array(shader_program* p, char* name, float* vals, int count);
void shader_program_set_vec2_array(shader_program* p, char* name, vec2* vals, int count);
void shader_program_set_vec3_array(shader_program* p, char* name, vec3* vals, int count);
void shader_program_set_vec4_array(shader_program* p, char* name, vec4* vals, int count);
void shader_program_set_mat4_array(shader_program* p, char* name, mat4* vals, int count);
void shader_program_set_texture(shader_program* p, char* name, int index, asset_hndl t);
void shader_program_set_texture_id(shader_program* p, char* name, int index, GLint t);

void shader_program_enable_attribute(shader_program* p, char* name, int count, int stride, void* ptr);
void shader_program_enable_attribute_instance(shader_program* p, char* name, int count, int stride, void* ptr);
void shader_program_disable_attribute(shader_program* p, char* name);

void shader_program_enable_attribute_instance_matrix(shader_program* p, char* name, void* ptr);
void shader_program_disable_attribute_matrix(shader_program* p, char* name);

#endif


#include "assets/shader.h"
#include "assets/texture.h"

*/

/*
static shader* load_shader_file(char* filename, GLenum type) {

  shader* new_shader = malloc(sizeof(shader));
  
  SDL_RWops* file = SDL_RWFromFile(filename, "r");
  if(file == NULL) {
    error("Cannot load file %s", filename);
  }
  
  long size = SDL_RWseek(file,0,SEEK_END);
  char* contents = malloc(size+1);
  contents[size] = '\0';
  
  SDL_RWseek(file, 0, SEEK_SET);
  SDL_RWread(file, contents, size, 1);
  SDL_RWclose(file);
  
  *new_shader = glCreateShader(type);
  
  glShaderSource(shader_handle(new_shader), 1, (const char**)&contents, NULL);
  glCompileShader(shader_handle(new_shader));
  
  free(contents);
  
  shader_print_log(new_shader);
  
  int compile_error = 0;
  glGetShaderiv(shader_handle(new_shader), GL_COMPILE_STATUS, &compile_error);
  if (compile_error == GL_FALSE) {
    error("Compiler Error on Shader %s.", filename);
  }
  
  SDL_GL_CheckError();
  
  return new_shader;
}
*/

fn load_shader_file(filename: &str, shader_type: gl::types::GLenum) -> Rc<RefCell<Shader>> {
  //need to find out why the shaders were pointers?

  let mut file = File::open(filename).unwrap();

  let mut buffer = Vec::new(); //String::new();

  let _ = file.read_to_end(&mut buffer);

  let contents = CString::new(buffer).unwrap();

  let new_shader = unsafe {

    let new_shader = Shader(gl::CreateShader(shader_type));
    let contents_ptr = contents.as_ptr(); //const char *
    let contents_vec_ptr = vec![contents_ptr];
    let contents_ptr_ptr = contents_vec_ptr.as_ptr();

    gl::ShaderSource(shader_handle(&new_shader), 1, contents_ptr_ptr, ptr::null());
    gl::CompileShader(shader_handle(&new_shader));

    shader_print_log(&new_shader);

    let mut compile_error: gl::types::GLint = 0;
    gl::GetShaderiv(
      shader_handle(&new_shader),
      gl::COMPILE_STATUS,
      &mut compile_error,
    ); //casts are stupid in rust. can cast int directly to pointer

    if compile_error == 0 {
      //questionable casting here too, why is zero an error??
      let mut error_full = String::new();
      let _ = write!(&mut error_full, "Compiler Error on Shader {}", filename);
      error(&error_full);
    }

    new_shader
  };

  return Rc::new(RefCell::new(new_shader));
}


/*

shader* vs_load_file(char* filename) {
  return load_shader_file(filename, GL_VERTEX_SHADER);
}

*/

pub fn vs_load_file(filename: &str) -> Rc<RefCell<Shader>> {
  return load_shader_file(filename, gl::VERTEX_SHADER);
}

/*

shader* fs_load_file(char* filename) {
  return load_shader_file(filename, GL_FRAGMENT_SHADER);
}
*/

pub fn fs_load_file(filename: &str) -> Rc<RefCell<Shader>> {
  return load_shader_file(filename, gl::FRAGMENT_SHADER);
}

/*

shader* gs_load_file(char* filename) {
  return load_shader_file(filename, GL_GEOMETRY_SHADER);
}

*/

pub fn gs_load_file(filename: &str) -> Rc<RefCell<Shader>> {
  return load_shader_file(filename, gl::GEOMETRY_SHADER);
}

/*

shader* tcs_load_file(char* filename) {
  return load_shader_file(filename, TESS_CONTROL_SHADER);
}

*/

pub fn tcs_load_file(filename: &str) -> Rc<RefCell<Shader>> {
  return load_shader_file(filename, gl::TESS_CONTROL_SHADER);
}

/*
shader* tes_load_file(char* filename) {
  return load_shader_file(filename, GL_TESS_EVALUATION_SHADER);
}
*/

pub fn tes_load_file(filename: &str) -> Rc<RefCell<Shader>> {
  return load_shader_file(filename, gl::TESS_EVALUATION_SHADER);
}

/*
shader_program* shader_program_new() {
  shader_program* program = malloc(sizeof(shader_program));  
  *program = glCreateProgram();
  return program;
}
*/

pub fn shader_program_new() -> Option<ShaderProgram> {
  let program = unsafe { gl::CreateProgram() };

  return Some(ShaderProgram(program));
}

/*

GLuint shader_program_handle(shader_program* p) {
    
  if (p == NULL) {
    error("Cannot get handle for NULL shader program");
  }
  if (!glIsProgram(*p)) {
    error("Not a shader program");
  }
  return *p;
}
*/

//could these have been pointers for deletion? oh, type safety

pub fn shader_program_handle(p: &ShaderProgram) -> gl::types::GLuint {
  //let p_borrow = p.borrow();
  //let p_ref = p.as_ref().unwrap(); //assums p is something

  let p_ref = p;
  let result = unsafe { gl::IsProgram(p_ref.to_gl()) };

  if result as gl::types::GLboolean == gl::FALSE {
    error("Not a shader program");
  }

  return p_ref.to_gl();
}


/*
GLuint shader_handle(shader* s) {

  if (s == NULL) {
    error("Cannot get handle for NULL shader");
  }
  if (!glIsShader(*s)) {
    error("Not a shader");
  }
  return *s;
}
*/

pub fn shader_handle(s: &Shader) -> gl::types::GLuint {
  unsafe {
    if gl::IsShader(s.to_gl()) == 0 {
      error("Not a shader");
    }
  }

  return s.to_gl();
}


/*

void shader_program_attach_shader(shader_program* program, shader* shader) {
  
  if (shader_program_has_shader(program, shader)) {
    error("Shader already attached!");
  }
  
  glAttachShader(shader_program_handle(program), shader_handle(shader));
  
  shader_program_print_log(program);
  SDL_GL_CheckError();  
}*/

pub fn shader_program_attach_shader(program: &ShaderProgram, shader: &Shader) {
  if shader_program_has_shader(program, shader) {
    error("Shader already attached");
  }

  unsafe {
    gl::AttachShader(shader_program_handle(program), shader_handle(shader));
  }

  shader_program_print_log(program);

  gl_check_error();
}



/*

void shader_program_link(shader_program* program) {
  
  //GLint count = -1;
  //glGetIntegerv(GL_MAX_GEOMETRY_OUTPUT_VERTICES, &count);
  //glProgramParameteri(shader_program_handle(program), GL_GEOMETRY_VERTICES_OUT, count);
  
  
  glLinkProgram(shader_program_handle(program));
  
  shader_program_print_log(program);
  
  GLint is_linked = false;
  glGetProgramiv(shader_program_handle(program), GL_LINK_STATUS, &is_linked);
  if (!is_linked) {
    error("Error linking shader program!");
  }
  
  SDL_GL_CheckError();
}
*/

pub fn shader_program_link(program: &ShaderProgram) {
  unsafe {
    gl::LinkProgram(shader_program_handle(program));
  }

  shader_program_print_log(program);

  let mut is_linked: gl::types::GLint = 0; //false

  unsafe {
    gl::GetProgramiv(
      shader_program_handle(program),
      gl::LINK_STATUS,
      &mut is_linked,
    );
  }

  if is_linked == 0 {
    error("Error linking shader program!");
  }

  gl_check_error();
}

/*
bool shader_program_has_shader(shader_program* p, shader* s) {

  GLuint shaders[128];
  int num_shaders = 0;
  glGetAttachedShaders(shader_program_handle(p), 128, &num_shaders, shaders);

  for(int i = 0; i < num_shaders; i++) {
    if (shaders[i] == shader_handle(s)) return true;
  }
  
  return false;
}
*/

pub fn shader_program_has_shader(p: &ShaderProgram, s: &Shader) -> bool {
  let mut shaders: Vec<gl::types::GLuint> = vec![0; 128];
  let mut num_shaders = 0;

  unsafe {
    gl::GetAttachedShaders(
      shader_program_handle(p),
      shaders.len() as i32,
      &mut num_shaders,
      shaders.as_mut_ptr(),
    );
  }

  for i in 0..(num_shaders as usize) {
    if shaders[i] == shader_handle(s) {
      return true;
    }
  }

  return false;
}


/*

void shader_program_print_info(shader_program* p) {
  
  GLuint shaders[128];
  int num_shaders = 0;
  glGetAttachedShaders(shader_program_handle(p), 128, &num_shaders, shaders);
  
  debug("Program %i has %i shaders", shader_program_handle(p), num_shaders);
  for(int i = 0; i < num_shaders; i++) {
    debug("| Shader %i: %i", i, shaders[i]);
  }
  
}
*/

pub fn shader_program_print_info(p: &ShaderProgram) {
  let mut shaders: Vec<gl::types::GLuint> = vec![0; 128];
  let mut num_shaders: i32 = 0;
  unsafe {
    gl::GetAttachedShaders(
      shader_program_handle(p),
      shaders.len() as i32,
      &mut num_shaders,
      shaders.as_mut_ptr(),
    );
  }

  let mut debug_full = String::new();
  let _ = write!(
    &mut debug_full,
    "Program {} has {} shaders",
    shader_program_handle(p),
    num_shaders
  );
  debug(&debug_full);
  for i in 0..(num_shaders as usize) {
    let mut debug_full = String::new();
    let _ = write!(&mut debug_full, "| Shader {}: {}", i, shaders[i]);
    debug(&debug_full);
  }

}

/*
void shader_program_print_log(shader_program* program) {

  char log[2048];
  int i;
  glGetProgramInfoLog(shader_program_handle(program), 2048, &i, log);
  log[i] = '\0';
  
  if (strcmp(log, "") != 0) {
    debug("%s", log);
  }
  
}
*/

pub fn shader_program_print_log(program: &ShaderProgram) {
  let mut log: Vec<u8> = vec![0; 2048];
  let mut i = 0;

  unsafe {
    gl::GetProgramInfoLog(
      shader_program_handle(program),
      log.len() as i32,
      &mut i,
      log.as_mut_ptr() as *mut i8,
    );
  }

  log[i as usize] = 0;

  if log[0] != 0 {
    debug(&String::from_utf8(log).unwrap());
  }

}

/*
void shader_print_log(shader* shader) {

  char log[2048];
  int i;
  glGetShaderInfoLog(shader_handle(shader), 2048, &i, log);
  log[i] = '\0';
  
  if (strcmp(log, "") != 0) {
    debug("%s", log);
  }
  
}*/

fn shader_print_log(shader: &Shader) {
  let mut log: Vec<u8> = vec![0; 2048];
  let mut i: i32 = 0;

  unsafe {
    gl::GetShaderInfoLog(
      shader_handle(shader),
      2048,
      &mut i,
      log.as_mut_ptr() as *mut i8,
    ); //website says lie to c not to rust, so that is what I did...
  }

  log[i as usize] = 0;

  if log[0] != 0 {

    let strlog = String::from_utf8(log).unwrap();
    debug(&strlog);
  }
}

/*

void shader_program_delete(shader_program* program) {
  glDeleteProgram(shader_program_handle(program));
  free(program);
}

*/



impl Drop for ShaderProgram {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteProgram(shader_program_handle(self));
    }
  }
}

/*

void shader_delete(shader* shader) {
  glDeleteShader(shader_handle(shader));
  free(shader);
}

*/

pub fn shader_delete(shader: Rc<RefCell<Shader>>) {
  unsafe {
    gl::DeleteShader(shader_handle(&shader.borrow()));
  }
}

impl Drop for Shader {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteShader(shader_handle(self));
    }
  }
}

/*

GLint shader_program_get_attribute(shader_program* p, char* name) {
  GLint attr = glGetAttribLocation(shader_program_handle(p), name);
  if (attr == -1) {
    error("Shader has no attribute called '%s'", name);
    return -1;
  } else {
    return attr;
  }
}
*/

pub fn shader_program_get_attribute(p: &ShaderProgram, name: &str) -> gl::types::GLint {
  let attr = unsafe {
    let cname = CString::new(name).unwrap();
    gl::GetAttribLocation(shader_program_handle(p), cname.as_ptr())
  };

  if attr == -1 {
    let mut error_full = String::new();
    let _ = write!(&mut error_full, "Shader has no attribute called '{}'", name);
    return -1;
  }
  return attr;
}

/*

void shader_program_enable(shader_program* p) {
  glUseProgram(shader_program_handle(p));
}
*/

pub fn shader_program_enable(p: &ShaderProgram) {
  unsafe {
    gl::UseProgram(shader_program_handle(p));
  }
}

/*

void shader_program_disable(shader_program* p) {
  glActiveTexture(GL_TEXTURE0);
  glUseProgram(0);
}

*/

pub fn shader_program_disable(_: &ShaderProgram) {
  unsafe {
    gl::ActiveTexture(gl::TEXTURE0);
    gl::UseProgram(0); //why does this use program zero only?? this is the disable...
  }
}

/*

void shader_program_set_int(shader_program* p, char* name, int val) {
  GLint location = glGetUniformLocation(shader_program_handle(p), name);
  if ( location == -1) {
    warning("Shader has no uniform called '%s'", name);
  } else {
    glUniform1i(location, val);
  }
}

*/

pub fn shader_program_set_int(p: &ShaderProgram, name: &str, val: i32) {
  unsafe {
    let cname = CString::new(name).unwrap();
    let location = gl::GetUniformLocation(shader_program_handle(p), cname.as_ptr());

    if location == -1 {
      let mut warning_full = String::new();
      let _ = write!(&mut warning_full, "Shader has no uniform called {}", name);
      warning(&warning_full);
    } else {
      gl::Uniform1i(location, val);
    }
  }
}

/*

void shader_program_set_float(shader_program* p, char* name, float val) {
  GLint location = glGetUniformLocation(shader_program_handle(p), name);
  if ( location == -1) {
    warning("Shader has no uniform called '%s'", name);
  } else {
    glUniform1f(location, val);
  }
}

*/

pub fn shader_program_set_float(p: &ShaderProgram, name: &str, val: f32) {
  unsafe {
    let cname = CString::new(name).unwrap();
    let location = gl::GetUniformLocation(shader_program_handle(p), cname.as_ptr());

    if location == -1 {
      let mut warning_full = String::new();
      let _ = write!(&mut warning_full, "Shader has no uniform called {}", name);
      warning(&warning_full);
    } else {
      gl::Uniform1f(location, val);
    }
  }
}

/*

void shader_program_set_vec2(shader_program* p, char* name, vec2 val) {
  GLint location = glGetUniformLocation(shader_program_handle(p), name);
  if ( location == -1) {
    warning("Shader has no uniform called '%s'", name);
  } else {
    glUniform2f(location, val.x, val.y);
  }
}

*/

pub fn shader_program_set_vec2(p: &ShaderProgram, name: &str, val: &Vec2) {
  unsafe {
    let cname = CString::new(name).unwrap();
    let location = gl::GetUniformLocation(shader_program_handle(p), cname.as_ptr());

    if location == -1 {
      let mut warning_full = String::new();
      let _ = write!(&mut warning_full, "Shader has no uniform called {}", name);
      warning(&warning_full);
    } else {
      gl::Uniform2f(location, val.x, val.y);
    }
  }
}

/*

void shader_program_set_vec3(shader_program* p, char* name, vec3 val) {
  GLint location = glGetUniformLocation(shader_program_handle(p), name);
  if ( location == -1) {
    warning("Shader has no uniform called '%s'", name);
  } else {
    glUniform3f(location, val.x, val.y, val.z);
  }
}

*/

pub fn shader_program_set_vec3(p: &ShaderProgram, name: &str, val: &Vec3) {
  unsafe {
    let cname = CString::new(name).unwrap();
    let location = gl::GetUniformLocation(shader_program_handle(p), cname.as_ptr());

    if location == -1 {
      let mut warning_full = String::new();
      let _ = write!(&mut warning_full, "Shader has no uniform called {}", name);
      warning(&warning_full);
    } else {
      gl::Uniform3f(location, val.x, val.y, val.z);
    }
  }
}

/*
void shader_program_set_vec4(shader_program* p, char* name, vec4 val) {
  GLint location = glGetUniformLocation(shader_program_handle(p), name);
  if ( location == -1) {
    warning("Shader has no uniform called '%s'", name);
  } else {
    glUniform4f(location, val.x, val.y, val.z, val.w);
  }
}
*/

pub fn shader_program_set_vec4(p: &ShaderProgram, name: &str, val: &Vec4) {
  unsafe {
    let cname = CString::new(name).unwrap();
    let location = gl::GetUniformLocation(shader_program_handle(p), cname.as_ptr());

    if location == -1 {
      let mut warning_full = String::new();
      let _ = write!(&mut warning_full, "Shader has no uniform called {}", name);
      warning(&warning_full);
    } else {
      gl::Uniform4f(location, val.x, val.y, val.z, val.w);
    }
  }
}

/*
void shader_program_set_mat3(shader_program* p, char* name, mat3 val) {
  GLint location = glGetUniformLocation(shader_program_handle(p), name);
  if ( location == -1) {
    warning("Shader has no uniform called '%s'", name);
  } else {
    glUniformMatrix3fv(location, 1, GL_TRUE, (float*)&val);
  }
}
*/

pub fn shader_program_set_mat3(p: &ShaderProgram, name: &str, val: &Mat3) {
  unsafe {
    let cname = CString::new(name).unwrap();
    let location = gl::GetUniformLocation(shader_program_handle(p), cname.as_ptr());

    if location == -1 {
      let mut warning_full = String::new();
      let _ = write!(&mut warning_full, "Shader has no uniform called {}", name);
      warning(&warning_full);
    } else {
      //gl::UniformMatrix3fv(location, 1, gl::TRUE, val.transpose().as_ptr());
      gl::UniformMatrix3fv(location, 1, gl::FALSE, val.as_ptr()); //to_arr()[0][0]);
      //gl::UniformMatrix3fv(location, 1, gl::TRUE, val.as_ptr());//to_arr()[0][0]);
      //gl::UniformMatrix3fv(location, 1, gl::TRUE, &val.transpose().to_arr()[0][0]);
      //gl::UniformMatrix3fv(location, 1, gl::TRUE, val.transpose().as_slice().as_ptr());
    }
  }
}

/*
void shader_program_set_mat4(shader_program* p, char* name, mat4 val) {
  GLint location = glGetUniformLocation(shader_program_handle(p), name);
  if ( location == -1) {
    warning("Shader has no uniform called '%s'", name);
  } else {
    glUniformMatrix4fv(location, 1, GL_TRUE, (float*)&val);
  }
}
*/

pub fn shader_program_set_mat4(p: &ShaderProgram, name: &str, val: &Mat4) {
  unsafe {
    let cname = CString::new(name).unwrap();
    let location = gl::GetUniformLocation(shader_program_handle(p), cname.as_ptr());

    if location == -1 {
      let mut warning_full = String::new();
      let _ = write!(&mut warning_full, "Shader has no uniform called {}", name);
      warning(&warning_full);
    } else {
      //      println!("matrix {:?} ", val);
      gl::UniformMatrix4fv(location, 1, gl::FALSE, val.as_ptr());
      //gl::UniformMatrix4fv(location, 1, gl::TRUE, val.as_ptr());
    }
  }
}

/*

void shader_program_set_texture(shader_program* p, char* name, int index, asset_hndl t) {

  GLint location = glGetUniformLocation(shader_program_handle(p), name);
  if ( location == -1) {
    warning("Shader has no uniform called '%s'", name);
  } else {
    glActiveTexture(GL_TEXTURE0 + index);
    glBindTexture(texture_type(asset_hndl_ptr(&t)), texture_handle(asset_hndl_ptr(&t)));
    glUniform1i(location, index);
  }
  
}
*/

//TODO need to look into the mut &mut
pub fn shader_program_set_texture(
  p: &ShaderProgram,
  name: &str,
  index: i32,
  t: &mut AssetHandle<Texture>,
) {
  unsafe {
    let cname = CString::new(name).unwrap();
    let location = gl::GetUniformLocation(shader_program_handle(p), cname.as_ptr());

    if location == -1 {
      let mut warning_full = String::new();
      let _ = write!(&mut warning_full, "Shader has no uniform called '{}'", name);
      warning(&warning_full);
    } else {
      gl::ActiveTexture(gl::TEXTURE0 + (index as gl::types::GLuint)); //enum??
      if let Some(texture) = asset_hndl_ptr::<Texture>(t) {
        //let texture = texturecell.borrow();
        gl::BindTexture(
          texture_type(&texture.borrow()),
          texture_handle(&texture.borrow()),
        );
        gl::Uniform1i(location, index);
      } else {
        let mut error_full = String::new();
        let _ = write!(
          &mut error_full,
          "Could not find texture for input '{}'",
          t.path
        );
        error(&error_full);
      }
    }
  }
}

/*
void shader_program_set_texture_id(shader_program* p, char* name, int index, GLint t) {

  GLint location = glGetUniformLocation(shader_program_handle(p), name);
  if ( location == -1) {
    warning("Shader has no uniform called '%s'", name);
  } else {
    glActiveTexture(GL_TEXTURE0 + index);
    glBindTexture(GL_TEXTURE_2D, t);
    glUniform1i(location, index);
  }

}
*/

pub fn shader_program_set_texture_id(
  p: &ShaderProgram,
  name: &str,
  index: i32,
  t: gl::types::GLuint,
) {
  unsafe {
    let cname = CString::new(name).unwrap();
    let location = gl::GetUniformLocation(shader_program_handle(p), cname.as_ptr());

    if location == -1 {
      let mut warning_full = String::new();
      let _ = write!(&mut warning_full, "Shader has no uniform called '{}'", name);
      warning(&warning_full);
    } else {
      gl::ActiveTexture(gl::TEXTURE0 + (index as gl::types::GLuint));
      gl::BindTexture(gl::TEXTURE_2D, t);
      gl::Uniform1i(location, index);
    }
  }
}

/*

void shader_program_set_float_array(shader_program* p, char* name, float* vals, int count) {
  GLint location = glGetUniformLocation(shader_program_handle(p), name);
  if ( location == -1) {
    warning("Shader has no uniform called '%s'", name);
  } else {
    glUniform1fv(location, count, vals);
  }
}

void shader_program_set_vec2_array(shader_program* p, char* name, vec2* vals, int count) {
  GLint location = glGetUniformLocation(shader_program_handle(p), name);
  if ( location == -1) {
    warning("Shader has no uniform called '%s'", name);
  } else {
    glUniform2fv(location, count, (float*)vals);
  }
}

void shader_program_set_vec3_array(shader_program* p, char* name, vec3* vals, int count) {
  GLint location = glGetUniformLocation(shader_program_handle(p), name);
  if ( location == -1) {
    warning("Shader has no uniform called '%s'", name);
  } else {
    glUniform3fv(location, count, (float*)vals);
  }
}

void shader_program_set_vec4_array(shader_program* p, char* name, vec4* vals, int count) {
  GLint location = glGetUniformLocation(shader_program_handle(p), name);
  if ( location == -1) {
    warning("Shader has no uniform called '%s'", name);
  } else {
    glUniform4fv(location, count, (float*)vals);
  }
}

void shader_program_set_mat4_array(shader_program* p, char* name, mat4* vals, int count) {
  GLint location = glGetUniformLocation(shader_program_handle(p), name);
  if ( location == -1) {
    warning("Shader has no uniform called '%s'", name);
  } else {
    glUniformMatrix4fv(location, count, GL_TRUE, (float*)vals);
  }
}
*/

/*
void shader_program_enable_attribute(shader_program* p, char* name, int count, int stride, void* ptr) {
  GLint attr = glGetAttribLocation(shader_program_handle(p), name);
  if (attr == -1) {
    warning("Shader has no attribute called '%s'", name);
  } else {
    glEnableVertexAttribArray(attr);  
    glVertexAttribPointer(attr, count, GL_FLOAT, GL_FALSE, sizeof(float) * stride, ptr);
  }
}
*/

pub fn shader_program_enable_attribute(
  p: &ShaderProgram,
  name: &str,
  count: i32,
  stride: i32,
  ptr: *const raw::c_void,
) {
  let cname = CString::new(name).unwrap();

  let attr = unsafe { gl::GetAttribLocation(shader_program_handle(p), cname.as_ptr()) };
  if attr == -1 {
    let mut warning_full = String::new();
    let _ = write!(
      &mut warning_full,
      "Shader has no attribute called '{}'",
      name
    );
    warning(&warning_full);
  } else {
    let size_of_float = 4;
    unsafe {
      gl::VertexAttribPointer(
        attr as gl::types::GLuint,
        count,
        gl::FLOAT,
        gl::FALSE,
        size_of_float * stride,
        ptr,
      );
      gl::EnableVertexAttribArray(attr as gl::types::GLuint);
      //println!("enabled vertex attr {}", attr);
      //gl_check_error();
    }
  }
}

/*

void shader_program_enable_attribute_instance(shader_program* p, char* name, int count, int stride, void* ptr) {
  GLint attr = glGetAttribLocation(shader_program_handle(p), name);
  if (attr == -1) {
    warning("Shader has no attribute called '%s'", name);
  } else {
    glEnableVertexAttribArray(attr);  
    glVertexAttribPointer(attr, count, GL_FLOAT, GL_FALSE, sizeof(float) * stride, ptr);
    glVertexAttribDivisor(attr, 1);
  }
}

void shader_program_enable_attribute_instance_matrix(shader_program* p, char* name, void* ptr) {
  GLint attr = glGetAttribLocation(shader_program_handle(p), name);
  if (attr == -1) {
    warning("Shader has no attribute called '%s'", name);
  } else {
    glEnableVertexAttribArray(attr+0);  
    glEnableVertexAttribArray(attr+1);  
    glEnableVertexAttribArray(attr+2);  
    glEnableVertexAttribArray(attr+3);
    glVertexAttribPointer(attr+0, 4, GL_FLOAT, GL_FALSE, sizeof(float) * 4 * 4, ptr);
    glVertexAttribPointer(attr+1, 4, GL_FLOAT, GL_FALSE, sizeof(float) * 4 * 4, ptr + sizeof(float) * 4);
    glVertexAttribPointer(attr+2, 4, GL_FLOAT, GL_FALSE, sizeof(float) * 4 * 4, ptr + sizeof(float) * 8);
    glVertexAttribPointer(attr+3, 4, GL_FLOAT, GL_FALSE, sizeof(float) * 4 * 4, ptr + sizeof(float) * 12);
    glVertexAttribDivisor(attr+0, 1);
    glVertexAttribDivisor(attr+1, 1);
    glVertexAttribDivisor(attr+2, 1);
    glVertexAttribDivisor(attr+3, 1);
  }
}

*/

/*
void shader_program_disable_attribute(shader_program* p, char* name) {
  GLint attr = glGetAttribLocation(shader_program_handle(p), name);
  if (attr == -1) {
    warning("Shader has no attribute called '%s'", name);
  } else {
    glDisableVertexAttribArray(attr);  
  }
}
*/

pub fn shader_program_disable_attribute(p: &ShaderProgram, name: &str) {
  let cname = CString::new(name).unwrap();
  let attr = unsafe { gl::GetAttribLocation(shader_program_handle(p), cname.as_ptr()) };
  if attr == -1 {
    let mut warning_full = String::new();
    let _ = write!(
      &mut warning_full,
      "Shader has no attribute called '{}'",
      name
    );
    warning(&warning_full);
  } else {
    unsafe {
      gl::DisableVertexAttribArray(attr as gl::types::GLuint);
    }
  }
}

/*

void shader_program_disable_attribute_matrix(shader_program* p, char* name) {
  GLint attr = glGetAttribLocation(shader_program_handle(p), name);
  if (attr == -1) {
    warning("Shader has no attribute called '%s'", name);
  } else {
    glVertexAttribDivisor(attr+0, 0);
    glVertexAttribDivisor(attr+1, 0);
    glVertexAttribDivisor(attr+2, 0);
    glVertexAttribDivisor(attr+3, 0);
    glDisableVertexAttribArray(attr+0);  
    glDisableVertexAttribArray(attr+1);  
    glDisableVertexAttribArray(attr+2);  
    glDisableVertexAttribArray(attr+3);
  }
}

*/
