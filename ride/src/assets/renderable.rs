use crate::core::error;
//use engine::{Sphere, Mesh, Model, Vec4, Vec3, Vec2, Vertex
//            , mesh_bounding_sphere, vertex_new, mesh_new
//            , model_generate_normals, model_generate_texcoords_cylinder, model_generate_tangents
//            , gl_check_error};
use crate::engine::*;
use crate::asset::AssetHandle;
use crate::asset::{asset_hndl_new_load, asset_hndl_new};
use crate::assets::material::Material;

use std::rc::Rc;
use std::cell::RefCell;
use std::mem;
use std::os::raw;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::fmt::Write;
use std::collections::HashMap;
use std::path::PathBuf;

use gl;
use cgmath::*;
/*
/**
*** :: Renderable ::
***
***   A model or mesh loaded into vertex buffers
***   can be rigged or not depending on file type
***
***   Load using .bmf format for best performance.
***
**/

#ifndef renderable_h
#define renderable_h

#include "cengine.h"
#include "assets/material.h"

typedef struct {
  int bone_ids[3];
  float bone_weights[3];
} vertex_weight;
*/

/*
struct VertexWeight {
  bone_ids: [i32; 3],
  bone_weights: [f32; 3],
}
*/

/*
typedef struct {
  GLuint vertex_vbo;
  GLuint triangle_vbo;
  int num_verticies;
  int num_triangles;
  sphere bound;
} renderable_surface;
*/

pub struct RenderableSurface {
  pub vertex_vbo: gl::types::GLuint,
  pub triangle_vbo: gl::types::GLuint,
  pub vao: gl::types::GLuint,
  //pub vb_data: Vec<f32>,
  //pub tri_data: Vec<u32>,
  pub num_vertices: usize,
  pub num_triangles: usize,
  pub bound: Sphere,
}


/*
renderable_surface* renderable_surface_new(mesh* m);
renderable_surface* renderable_surface_new_rigged(mesh* m, vertex_weight* weights);
void renderable_surface_delete(renderable_surface* surface);

typedef struct {
  renderable_surface** surfaces;
  int num_surfaces;
  bool is_rigged;
  asset_hndl material;
} renderable;
*/

pub struct Renderable {
  //surfaces: Vec<Rc<RefCell<RenderableSurface>>>,
  pub surfaces: Vec<RenderableSurface>,
  pub is_rigged: bool,
  pub material: AssetHandle<Material>,
}



/*

renderable* renderable_new();
void renderable_delete(renderable* r);

void renderable_add_mesh(renderable* r, mesh* m);
void renderable_add_model(renderable* r, model* m);
void renderable_set_material(renderable* r, asset_hndl mat);

model* renderable_to_model(renderable* r);

renderable* bmf_load_file(char* filename);
renderable* obj_load_file(char* filename);
renderable* smd_load_file(char* filename);
renderable* ply_load_file(char* filename);

void bmf_save_file(renderable* r, char* filename);


#endif
*/

/*
#include "assets/renderable.h"

#include "data/vertex_hashtable.h"


void renderable_add_mesh(renderable* r, mesh* m) {
  
  renderable_surface* surface = renderable_surface_new(m);
  
  r->num_surfaces++;
  r->surfaces = realloc(r->surfaces, sizeof(renderable_surface*) *  r->num_surfaces);
  r->surfaces[r->num_surfaces-1] = surface;
  
}
*/

pub fn renderable_add_mesh(r: &mut Renderable, m: &Mesh) {
  let surface = renderable_surface_new(m);

  r.surfaces.push(surface);
}

/*

void renderable_add_model(renderable* r, model* m) {

  for(int i = 0; i < m->num_meshes; i++) {
    renderable_add_mesh(r, m->meshes[i]);
  }
  
}
*/

pub fn renderable_add_model(r: &mut Renderable, m: &Model) {
  for mesh in m.meshes.iter() {
    renderable_add_mesh(r, mesh);
  }
}

/*
fn renderable_add_model(r:&Renderable, m:&Model) {
    for mesh in &(m.meshes) {
        renderable_add_mesh(r, mesh);
    }
}
*/

/*
renderable* renderable_new() {
  
  renderable* r = malloc(sizeof(renderable));
  
  r->material = asset_hndl_new_load(P("$CORANGE/shaders/basic.mat"));
  r->num_surfaces = 0;
  r->surfaces = NULL;
  r->is_rigged = false;
  
  return r;

}
*/

pub fn renderable_new() -> Renderable {
  let ah = asset_hndl_new_load("$CORANGE/shaders/basic.mat");

  let r = Renderable {
    material: ah, //think about making this weak...
    surfaces: Vec::new(),
    is_rigged: false,
  };

  return r;
}

/*

void renderable_delete(renderable* r) {
  
  for(int i = 0; i < r->num_surfaces; i++) {
    renderable_surface_delete( r->surfaces[i] );
  }
  
  free(r);

}

*/

/*
pub fn renderable_delete(r:Rc<RefCell<Renderable>>) {
  while r.borrow().surfaces.len() > 0 {
    let s = r.borrow_mut().surfaces.pop().unwrap(); //borrow_mut can crash hard
    renderable_surface_delete(s);
  }
}
*/

/*
impl Drop for Renderable {
  fn drop(&mut self) {
    debug("Dropping Renderable");
  }
}
*/

/*

void renderable_set_material(renderable* r, asset_hndl mat) {
  r->material = mat;
}

model* renderable_to_model(renderable* r) {

  if (r->is_rigged) {
    error("Cannot convert rigged renderable to model");
  }  

  model* m = model_new();
  
  m->num_meshes = r->num_surfaces;
  m->meshes = realloc(m->meshes, sizeof(mesh*) * m->num_meshes);
  
  for(int i = 0; i < r->num_surfaces; i++) {
    
    renderable_surface* s = r->surfaces[i];
    
    float* vb_data = malloc(sizeof(float) * s->num_verticies * 18);
    uint32_t* ib_data = malloc(sizeof(uint32_t) * s->num_triangles * 3);
    
    glBindBuffer(GL_ARRAY_BUFFER, s->vertex_vbo);
    glGetBufferSubData(GL_ARRAY_BUFFER, 0, sizeof(float) * s->num_verticies * 18, vb_data);
    
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, s->triangle_vbo);
    glGetBufferSubData(GL_ELEMENT_ARRAY_BUFFER, 0, sizeof(uint32_t) * s->num_triangles * 3, ib_data);
    
    m->meshes[i] = mesh_new();
    
    mesh* me = m->meshes[i];
    
    me->num_verts = s->num_verticies;
    me->num_triangles = s->num_triangles;
    me->verticies = realloc(me->verticies, sizeof(vertex) * me->num_verts);
    me->triangles = realloc(me->triangles, sizeof(uint32_t) * me->num_triangles * 3);
    
    for(int j = 0; j < me->num_verts; j++) {
      me->verticies[j].position.x = vb_data[(j*18)+0];
      me->verticies[j].position.y = vb_data[(j*18)+1];
      me->verticies[j].position.z = vb_data[(j*18)+2];
      
      me->verticies[j].normal.x = vb_data[(j*18)+3];
      me->verticies[j].normal.y = vb_data[(j*18)+4];
      me->verticies[j].normal.z = vb_data[(j*18)+5];
      
      me->verticies[j].tangent.x = vb_data[(j*18)+6];
      me->verticies[j].tangent.y = vb_data[(j*18)+7];
      me->verticies[j].tangent.z = vb_data[(j*18)+8];
      
      me->verticies[j].binormal.x = vb_data[(j*18)+9];
      me->verticies[j].binormal.y = vb_data[(j*18)+10];
      me->verticies[j].binormal.z = vb_data[(j*18)+11];
      
      me->verticies[j].uvs.x = vb_data[(j*18)+12];
      me->verticies[j].uvs.y = vb_data[(j*18)+13];
      
      me->verticies[j].color.x = vb_data[(j*18)+14];
      me->verticies[j].color.y = vb_data[(j*18)+15];
      me->verticies[j].color.z = vb_data[(j*18)+16];
      me->verticies[j].color.w = vb_data[(j*18)+17];
    }
    
    for(int j = 0; j < me->num_triangles * 3; j++) {
      me->triangles[j] = ib_data[j];
    }
    
    free(vb_data);
    free(ib_data);
  
  }
  
  return m;
}
*/

/*
renderable_surface* renderable_surface_new(mesh* m) {

  renderable_surface* s = malloc(sizeof(renderable_surface));

  glGenBuffers(1, &s->vertex_vbo);
  glGenBuffers(1, &s->triangle_vbo);

  s->num_verticies = m->num_verts;
  s->num_triangles = m->num_triangles;
  s->bound = mesh_bounding_sphere(m);
  
  /* Position Normal Tangent Binormal Uvs Color      */
  /* 3        3      3       3        2   4     = 18 */
  float* vb_data = malloc(sizeof(float) * m->num_verts * 18);
  
  for(int i = 0; i < m->num_verts; i++) {
  
    vec3 pos = m->verticies[i].position;
    vec3 norm = m->verticies[i].normal;
    vec3 tang = m->verticies[i].tangent;
    vec3 bino = m->verticies[i].binormal;
    vec2 uvs = m->verticies[i].uvs;
    vec4 col = m->verticies[i].color;
    
    vb_data[(i*18)+0] = pos.x;
    vb_data[(i*18)+1] = pos.y;
    vb_data[(i*18)+2] = pos.z;
    
    vb_data[(i*18)+3] = norm.x;
    vb_data[(i*18)+4] = norm.y;
    vb_data[(i*18)+5] = norm.z;
    
    vb_data[(i*18)+6] = tang.x;
    vb_data[(i*18)+7] = tang.y;
    vb_data[(i*18)+8] = tang.z;
    
    vb_data[(i*18)+9] = bino.x;
    vb_data[(i*18)+10] = bino.y;
    vb_data[(i*18)+11] = bino.z;
    
    vb_data[(i*18)+12] = uvs.x;
    vb_data[(i*18)+13] = uvs.y;
    
    vb_data[(i*18)+14] = col.x;
    vb_data[(i*18)+15] = col.y;
    vb_data[(i*18)+16] = col.z;
    vb_data[(i*18)+17] = col.w;
  
  }
  
  glBindBuffer(GL_ARRAY_BUFFER, s->vertex_vbo);
  glBufferData(GL_ARRAY_BUFFER, sizeof(float) * s->num_verticies * 18, vb_data, GL_STATIC_DRAW);
  
  free(vb_data);
  
  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, s->triangle_vbo);
  glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(uint32_t) * s->num_triangles * 3, m->triangles, GL_STATIC_DRAW);
  
  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
  glBindBuffer(GL_ARRAY_BUFFER, 0);

  return s;
}
*/


pub fn renderable_surface_new(m: &Mesh) -> RenderableSurface {
  let mut s = RenderableSurface {
    vertex_vbo: 0,
    triangle_vbo: 0,
    vao: 0,
    //vb_data: Vec::new(),
    //tri_data: Vec::new(),
    num_vertices: 0,
    num_triangles: 0,
    bound: Sphere {
      center: Vec3::zero(),
      radius: 0.0,
    },
  };

  unsafe {
    gl::GenVertexArrays(1, &mut s.vao);
    gl::GenBuffers(1, &mut s.vertex_vbo);
    gl::GenBuffers(1, &mut s.triangle_vbo);
  }

  s.num_vertices = m.vertices.len();
  s.num_triangles = m.triangles.len() / 3;
  s.bound = mesh_bounding_sphere(m);

  /* Position Normal Tangent Binormal Uvs Color WeightIds WeightAmounts      */
  /* 3        3      3       3        2   4     3         3             = 24 */

  let mut vb_data: Vec<f32> = vec![0.0; m.vertices.len() * 18];

  for i in 0..m.vertices.len() {
    let pos = m.vertices[i].position;
    let norm = m.vertices[i].normal;
    let tang = m.vertices[i].tangent;
    let bino = m.vertices[i].binormal;
    let uvs = m.vertices[i].uvs;
    let col = m.vertices[i].color;

    vb_data[(i * 18) + 0] = pos.x;
    vb_data[(i * 18) + 1] = pos.y;
    vb_data[(i * 18) + 2] = pos.z;

    vb_data[(i * 18) + 3] = norm.x;
    vb_data[(i * 18) + 4] = norm.y;
    vb_data[(i * 18) + 5] = norm.z;

    vb_data[(i * 18) + 6] = tang.x;
    vb_data[(i * 18) + 7] = tang.y;
    vb_data[(i * 18) + 8] = tang.z;

    vb_data[(i * 18) + 9] = bino.x;
    vb_data[(i * 18) + 10] = bino.y;
    vb_data[(i * 18) + 11] = bino.z;

    vb_data[(i * 18) + 12] = uvs.x;
    vb_data[(i * 18) + 13] = uvs.y;

    vb_data[(i * 18) + 14] = col.x;
    vb_data[(i * 18) + 15] = col.y;
    vb_data[(i * 18) + 16] = col.z;
    vb_data[(i * 18) + 17] = col.w;
  }
  //s.vb_data = vb_data.clone();
  //s.tri_data = m.triangles.clone();
  unsafe {

    gl::BindVertexArray(s.vao);
    gl::BindBuffer(gl::ARRAY_BUFFER, s.vertex_vbo);
    gl::BufferData(
      gl::ARRAY_BUFFER,
      (mem::size_of::<f32>() * s.num_vertices * 18) as isize,
      vb_data.as_ptr() as *const raw::c_void,
      gl::STATIC_DRAW,
    );

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, s.triangle_vbo);
    gl::BufferData(
      gl::ELEMENT_ARRAY_BUFFER,
      (mem::size_of::<u32>() * s.num_triangles * 3) as isize,
      m.triangles.as_ptr() as *const raw::c_void,
      gl::STATIC_DRAW,
    );

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    //gl::BindVertexArray(0);

  }
  //gl_check_error();
  return s;
}


/*
renderable_surface* renderable_surface_new_rigged(mesh* m, vertex_weight* weights) {

  renderable_surface* s = malloc(sizeof(renderable_surface));

  glGenBuffers(1, &s->vertex_vbo);
  glGenBuffers(1, &s->triangle_vbo);

  s->num_verticies = m->num_verts;
  s->num_triangles = m->num_triangles;
  
  /* Position Normal Tangent Binormal Uvs Color WeightIds WeightAmounts      */
  /* 3        3      3       3        2   4     3         3             = 24 */
  float* vb_data = malloc(sizeof(float) * m->num_verts * 24);
  
  for(int i = 0; i < m->num_verts; i++) {
  
    vec3 pos = m->verticies[i].position;
    vec3 norm = m->verticies[i].normal;
    vec3 tang = m->verticies[i].tangent;
    vec3 bino = m->verticies[i].binormal;
    vec2 uvs = m->verticies[i].uvs;
    vec4 col = m->verticies[i].color;
    
    vb_data[(i*24)+0] = pos.x;
    vb_data[(i*24)+1] = pos.y;
    vb_data[(i*24)+2] = pos.z;
    
    vb_data[(i*24)+3] = norm.x;
    vb_data[(i*24)+4] = norm.y;
    vb_data[(i*24)+5] = norm.z;
    
    vb_data[(i*24)+6] = tang.x;
    vb_data[(i*24)+7] = tang.y;
    vb_data[(i*24)+8] = tang.z;
    
    vb_data[(i*24)+9] = bino.x;
    vb_data[(i*24)+10] = bino.y;
    vb_data[(i*24)+11] = bino.z;
    
    vb_data[(i*24)+12] = uvs.x;
    vb_data[(i*24)+13] = uvs.y;
    
    vb_data[(i*24)+14] = col.w;
    vb_data[(i*24)+15] = col.x;
    vb_data[(i*24)+16] = col.y;
    vb_data[(i*24)+17] = col.z;
  
    vb_data[(i*24)+18] = (float)weights[i].bone_ids[0];
    vb_data[(i*24)+19] = (float)weights[i].bone_ids[1];
    vb_data[(i*24)+20] = (float)weights[i].bone_ids[2];
    
    vb_data[(i*24)+21] = weights[i].bone_weights[0];
    vb_data[(i*24)+22] = weights[i].bone_weights[1];
    vb_data[(i*24)+23] = weights[i].bone_weights[2];
  
  }
  
  glBindBuffer(GL_ARRAY_BUFFER, s->vertex_vbo);
  glBufferData(GL_ARRAY_BUFFER, sizeof(float) * s->num_verticies * 24, vb_data, GL_STATIC_DRAW);
  
  free(vb_data);
  
  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, s->triangle_vbo);
  glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(uint32_t) * s->num_triangles * 3, m->triangles, GL_STATIC_DRAW);
  
  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
  glBindBuffer(GL_ARRAY_BUFFER, 0);
  
  SDL_GL_CheckError();
  
  return s;
}

void renderable_surface_delete(renderable_surface* s) {

  glDeleteBuffers(1, &s->vertex_vbo);
  glDeleteBuffers(1, &s->triangle_vbo);
  
  free(s);
  
}
*/

impl Drop for RenderableSurface {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteVertexArrays(1, &self.vao);
      gl::DeleteBuffers(1, &self.vertex_vbo);
      gl::DeleteBuffers(1, &self.triangle_vbo);
    }
  }
}

/*

static sphere renderable_surface_bounding_sphere(float* verts, int num_verts, int stride) {
  
  sphere s;
  
  s.center = vec3_zero();
  for(int i = 0; i < num_verts * stride; i += stride) {
    vec3 position = vec3_new(verts[i+0], verts[i+1], verts[i+2]);
    s.center = vec3_add(s.center, position);
  }
  s.center = vec3_div(s.center, num_verts);
  
  s.radius = 0;
  for(int i = 0; i < num_verts * stride; i += stride) {
    vec3 position = vec3_new(verts[i+0], verts[i+1], verts[i+2]);
    s.radius = max(s.radius, vec3_dist(s.center, position));
  }
  
  return s;
  
}

renderable* bmf_load_file(char* filename) {

  renderable* r = malloc(sizeof(renderable));
  
  SDL_RWops* file = SDL_RWFromFile(filename, "rb");
  
  if(file == NULL) {
    error("Could not load file %s", filename);
  }
  
  char magic[4];
  SDL_RWread(file, &magic, 3, 1);
  magic[3] = '\0';
  
  if (strcmp(magic, "BMF") != 0) {
    error("Badly formed bmf file '%s', missing magic number", filename);
  }  
  
  uint32_t version = 0;
  SDL_RWread(file, &version, sizeof(uint32_t), 1);
  
  if (version != 1) {
    error("Only version 1 of bmf format supported. Recieved file of version %i.", version);
  }
    
  SDL_RWread(file, &r->is_rigged, 1, 1);
    
  uint32_t mat_len;
  SDL_RWread(file, &mat_len, sizeof(uint32_t), 1);
    
  fpath material_path;
  SDL_RWread(file, material_path.ptr, mat_len, 1);
  material_path.ptr[mat_len] = '\0';
    
  r->material = asset_hndl_new_load(material_path);
  
  uint32_t num_surfaces;
  SDL_RWread(file, &num_surfaces, sizeof(uint32_t), 1);
  r->num_surfaces = num_surfaces;
  
  r->surfaces = malloc(sizeof(renderable_surface*) * r->num_surfaces);
  
  const int stride = r->is_rigged ? 24 : 18;
  
  for(int i = 0; i < r->num_surfaces; i++) {
    renderable_surface* s = malloc(sizeof(renderable_surface));
    
    uint32_t num_verticies;
    SDL_RWread(file, &num_verticies, sizeof(uint32_t), 1);
    s->num_verticies = num_verticies;
    
    float* vert_data = malloc(sizeof(float) * stride * s->num_verticies);
    SDL_RWread(file, vert_data, sizeof(float) * stride * s->num_verticies, 1);
    
    s->bound = renderable_surface_bounding_sphere(vert_data, s->num_verticies, stride);
    
    glGenBuffers(1, &s->vertex_vbo);
    glBindBuffer(GL_ARRAY_BUFFER, s->vertex_vbo);
    glBufferData(GL_ARRAY_BUFFER, sizeof(float) * s->num_verticies * stride, vert_data, GL_STATIC_DRAW);
    free(vert_data);
    
    uint32_t num_indicies;
    SDL_RWread(file, &num_indicies, sizeof(uint32_t), 1);
    s->num_triangles = num_indicies / 3;
    
    uint32_t* index_data = malloc(sizeof(uint32_t) * num_indicies);
    SDL_RWread(file, index_data, sizeof(uint32_t) * num_indicies, 1);
    glGenBuffers(1, &s->triangle_vbo);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, s->triangle_vbo);
    glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(uint32_t) * num_indicies, index_data, GL_STATIC_DRAW);
    free(index_data);
    
    r->surfaces[i] = s;
  }
  
  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
  glBindBuffer(GL_ARRAY_BUFFER, 0);
  
  SDL_RWclose(file);
  
  SDL_GL_CheckError();
  
  return r;
}

void bmf_save_file(renderable* r, char* filename) {
  
  SDL_RWops* file = SDL_RWFromFile(filename, "wb");
  
  uint32_t version = 1;
  
  SDL_RWwrite(file, "BMF", 3, 1);
  SDL_RWwrite(file, &version, sizeof(uint32_t), 1);
  SDL_RWwrite(file, &r->is_rigged, 1, 1);
  
  fpath matpath;
  SDL_PathRelative(matpath.ptr, r->material.path.ptr);
  SDL_PathForwardSlashes(matpath.ptr);
  
  uint32_t mat_len = strlen(matpath.ptr);
  SDL_RWwrite(file, &mat_len, sizeof(uint32_t), 1);
  SDL_RWwrite(file, matpath.ptr, mat_len, 1);
  
  uint32_t num_surfaces = r->num_surfaces;
  SDL_RWwrite(file, &num_surfaces, sizeof(uint32_t), 1);
  
  const uint32_t vertsize = r->is_rigged ? 24 : 18;
  
  for(int i = 0; i < r->num_surfaces; i++) {
    renderable_surface* s = r->surfaces[i];
    
    uint32_t num_verticies = s->num_verticies;
    SDL_RWwrite(file, &num_verticies, sizeof(uint32_t), 1);
    
    uint32_t vert_data_size = sizeof(float) * vertsize * num_verticies;
    float* vert_data = calloc(vert_data_size, 1);
    glBindBuffer(GL_ARRAY_BUFFER, s->vertex_vbo);
    glGetBufferSubData(GL_ARRAY_BUFFER, 0, vert_data_size, vert_data);
    SDL_RWwrite(file, vert_data, 1, vert_data_size);
    free(vert_data);
    
    uint32_t num_indicies = s->num_triangles * 3;
    SDL_RWwrite(file, &num_indicies, sizeof(uint32_t), 1);
    
    uint32_t index_data_size = sizeof(uint32_t) * num_indicies;
    uint32_t* index_data = calloc(index_data_size, 1);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, s->triangle_vbo);
    glGetBufferSubData(GL_ELEMENT_ARRAY_BUFFER, 0, index_data_size, index_data);
    SDL_RWwrite(file, index_data, 1, index_data_size);
    free(index_data);
    
  }
  
  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
  glBindBuffer(GL_ARRAY_BUFFER, 0);
  
  SDL_RWclose(file);
    
}

static int SDL_RWreadline(SDL_RWops* file, char* buffer, int buffersize) {
  
  char c;
  int status = 0;
  int i = 0;
  while(1) {
    
    status = SDL_RWread(file, &c, 1, 1);
    
    if (status == -1) return -1;
    if (i == buffersize-1) return -1;
    if (status == 0) break;
    
    buffer[i] = c;
    i++;
    
    if (c == '\n') {
      buffer[i] = '\0';
      return i;
    }
  }
  
  if(i > 0) {
    buffer[i] = '\0';
    return i;
  } else {
    return 0;
  }
  
}
*/

/*
renderable* obj_load_file(char* filename) {
    
  model* obj_model = malloc(sizeof(model));
  obj_model->num_meshes = 0;
  obj_model->meshes = malloc(sizeof(mesh*) * 0);
  
  mesh* active_mesh = NULL;
*/
pub fn obj_load_file(filename: &str) -> Rc<RefCell<Renderable>> {
  let mut obj_model = Model { meshes: Vec::new() };

  let mut has_active_mesh = false;
  //let active_mesh:Option<&Mesh> = None;

  /*
  vertex_list* vert_data = vertex_list_new();
  vertex_list* vert_list = vertex_list_new();
  int_list* tri_list = int_list_new();
  vertex_hashtable* vert_hashes = vertex_hashtable_new(4096);
  
  int num_pos, num_norm, num_tex;
  num_pos = num_norm = num_tex = 0;
  
  int vert_index = 0;
  
  bool has_normal_data = false;
  bool has_texcoord_data = false;
  
  SDL_RWops* file = SDL_RWFromFile(filename, "r");
  
  if(file == NULL) {
    error("Could not load file %s", filename);
  }
  
  char line[1024];
  while(SDL_RWreadline(file, line, 1024)) {
  */
  let mut vert_data: Vec<Vertex> = Vec::new();
  let mut vert_list: Vec<Vertex> = Vec::new();
  let mut tri_list: Vec<u32> = Vec::new(); //should this be unsigned?? TODO
  let mut vert_hashes: HashMap<Vertex, u32> = HashMap::new();


  let mut num_pos = 0;
  let mut num_norm = 0;
  let mut num_tex = 0;

  let mut has_normal_data = false;
  let mut has_texcoord_data = false;

  let file = File::open(filename);

  let file = match file {
    Ok(file) => file,
    Err(_) => {
      let mut full_error = String::new();
      let _ = write!(&mut full_error, "Could not load file '{}'", filename);
      error(&full_error);

      let null = Rc::new(RefCell::new(renderable_new())); //this would be much better with the error pattern of rust
      return null;
    }
  };

  let file = BufReader::new(&file);

  for lineresult in file.lines() {
    let line = lineresult.unwrap();

    if let (_, 1) = scan_fmt_count!(&line, "# {}", String) {
      //comment do nothing
    } else if let (_, 1) = scan_fmt_count!(&line, "mtllib {}", String) {
      // do nothing
    } else if let (_, 1) = scan_fmt_count!(&line, "o {}", String) {
      // do nothing
    } else if let (px, py, pz, 3) = scan_fmt_count!(&line, "v {} {} {}", f32, f32, f32) {
      while vert_data.len() <= num_pos {
        vert_data.push(vertex_new());
      }
      let vert = &mut vert_data[num_pos];
      vert.position = Vec3::new(px.unwrap(), py.unwrap(), pz.unwrap());
      num_pos = num_pos + 1;
    }
    /*
    char comment[512];
    char matlib[512];
    char object[512];
    char group[512];
    char material[512];
    float px, py, pz, tx, ty, nx, ny, nz;
    int smoothing_group;
    int pi1, ti1, ni1, pi2, ti2, ni2, pi3, ti3, ni3;
    
    if (sscanf(line, "# %512s", comment) == 1) {
      /* Comment, do nothing */
    }
    
    else if (sscanf(line, "mtllib %512s", matlib) == 1) {
      /* Do Nothing */
    }
    
    else if (sscanf(line, "o %512s", object) == 1) {
      /* Do Nothing */
    }
    
    else if (sscanf(line, "v %f %f %f", &px, &py, &pz) == 3) {
    
      while(vert_data->num_items <= num_pos) { vertex_list_push_back(vert_data, vertex_new()); }
      vertex vert = vertex_list_get(vert_data, num_pos);
      vert.position = vec3_new(px, py, pz);
      vertex_list_set(vert_data, num_pos, vert);
      num_pos++;
    }
    */
    else if let (tx, ty, 2) = scan_fmt_count!(&line, "vt {} {}", f32, f32) {
      while vert_data.len() <= num_tex {
        vert_data.push(vertex_new());
      }
      let vert = &mut vert_data[num_tex];
      vert.uvs = Vec2::new(tx.unwrap(), ty.unwrap());
      num_tex = num_tex + 1;
    }
    /*
    
    else if (sscanf(line, "vt %f %f", &tx, &ty) == 2) {
    
      while(vert_data->num_items <= num_tex) { vertex_list_push_back(vert_data, vertex_new()); }
      vertex vert = vertex_list_get(vert_data, num_tex);
      vert.uvs = vec2_new(tx, ty);
      vertex_list_set(vert_data, num_tex, vert);
      num_tex++;
    }
    
    else if (sscanf(line, "vn %f %f %f", &nx, &ny, &nz) == 3) {
    
      while(vert_data->num_items <= num_norm) { vertex_list_push_back(vert_data, vertex_new()); }
      vertex vert = vertex_list_get(vert_data, num_norm);
      vert.normal = vec3_new(nx, ny, nz);
      vertex_list_set(vert_data, num_norm, vert);
      num_norm++;
    }

    */
    else if let (nx, ny, nz, 3) = scan_fmt_count!(&line, "vn {} {} {}", f32, f32, f32) {
      while vert_data.len() <= num_norm {
        vert_data.push(vertex_new());
      }
      let vert = &mut vert_data[num_pos];
      vert.normal = Vec3::new(nx.unwrap(), ny.unwrap(), nz.unwrap());
      num_norm = num_norm + 1;
    }
    /*
    
    else if (sscanf(line, "g %512s", group) == 1) {
        
      if (active_mesh != NULL) {
      
        active_mesh->num_verts = vert_index;
        active_mesh->num_triangles = tri_list->num_items / 3;
        
        active_mesh->verticies = malloc(sizeof(vertex) * active_mesh->num_verts);
        for(int i = 0; i < active_mesh->num_verts; i++) {
          active_mesh->verticies[i] = vertex_list_get(vert_list, i);
        }
        
        active_mesh->triangles = malloc(sizeof(int) * active_mesh->num_triangles * 3);
        for(int i = 0; i < active_mesh->num_triangles * 3; i++) {
          active_mesh->triangles[i] = int_list_get(tri_list, i);
        }
      
        obj_model->num_meshes++;
        obj_model->meshes = realloc(obj_model->meshes, sizeof(mesh*) * obj_model->num_meshes);
        obj_model->meshes[obj_model->num_meshes-1] = active_mesh;
        
      }
      
      vert_index = 0;
      
      vertex_hashtable_delete(vert_hashes);
      vertex_list_delete(vert_list);
      int_list_delete(tri_list);
      
      vert_list = vertex_list_new();
      tri_list = int_list_new();
      vert_hashes = vertex_hashtable_new(4096);
      
      active_mesh = malloc(sizeof(mesh));
      
    }

    */
    else if let (_, 1) = scan_fmt_count!(&line, "g {}", String) {
      if has_active_mesh {
        let mut active_mesh = mesh_new();

        while vert_list.len() > 0 {
          active_mesh.vertices.push(vert_list.pop().unwrap()); //maybe there is a smarter way to do this operation
        }

        while tri_list.len() > 0 {
          active_mesh.triangles.push(tri_list.pop().unwrap() as u32);
        }

        obj_model.meshes.push(active_mesh);
      }

      vert_hashes.clear();
      vert_list.clear();
      tri_list.clear();
    }
    /*
    
    else if (sscanf(line, "usemtl %512s", material) == 1) {
      /* Do Nothing */
    }
    
    else if (sscanf(line, "s %i", &smoothing_group) == 1) {
      /* Smoothing group, do nothing */
    }

    */
    else if let (_, 1) = scan_fmt_count!(&line, "usemtl {}", String) {

    } else if let (_, 1) = scan_fmt_count!(&line, "s {}", i32) {

    }
    //else if let (pi1, ti1, ni1, pi2, ti2, ni2, pi3, ti3, ni3, 9) = scan_fmt_count!(&line, "f {}/{}/{} {}/{}/{} {}/{}/{}", i32, i32, i32, i32, i32, i32, i32, i32, i32) {
    else if let (_, _, _, _, _, _, _, _, _, 9) =
      scan_fmt_count!(
        &line,
        "f {}/{}/{} {}/{}/{} {}/{}/{}",
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32
      )
    {
      error("f 3 3 3 not implemented");

    }
    /*
    
    else if (sscanf(line, "f %i/%i/%i %i/%i/%i %i/%i/%i", &pi1, &ti1, &ni1, &pi2, &ti2, &ni2, &pi3, &ti3, &ni3) == 9) {
      
      if (active_mesh == NULL) {
        vert_index = 0;
        
        vertex_hashtable_delete(vert_hashes);
        vertex_list_delete(vert_list);
        int_list_delete(tri_list);
        
        vert_list = vertex_list_new();
        tri_list = int_list_new();
        vert_hashes = vertex_hashtable_new(4096);
        
        active_mesh = malloc(sizeof(mesh));
      }
      
      has_normal_data = true;
      has_texcoord_data = true;
      
      /* OBJ file indicies start from one, have to subtract one */
      pi1--; ti1--; ni1--; pi2--; ti2--; ni2--; pi3--; ti3--; ni3--;
      
      vertex v1, v2, v3;
      v1.position = vertex_list_get(vert_data, pi1).position;
      v1.uvs = vertex_list_get(vert_data, ti1).uvs;
      v1.normal = vertex_list_get(vert_data, ni1).normal;
      
      v2.position = vertex_list_get(vert_data, pi2).position;
      v2.uvs = vertex_list_get(vert_data, ti2).uvs;
      v2.normal = vertex_list_get(vert_data, ni2).normal;
      
      v3.position = vertex_list_get(vert_data, pi3).position;
      v3.uvs = vertex_list_get(vert_data, ti3).uvs;
      v3.normal = vertex_list_get(vert_data, ni3).normal;
      
      int v1_id = vertex_hashtable_get(vert_hashes, v1);
      if ( v1_id == -1 ) {
        vertex_hashtable_set(vert_hashes, v1, vert_index);
        vertex_list_push_back(vert_list, v1);
        int_list_push_back(tri_list, vert_index);
        vert_index++;
      } else {
        int_list_push_back(tri_list, v1_id);
      }
      
      int v2_id = vertex_hashtable_get(vert_hashes, v2);
      if ( v2_id == -1 ) {
        vertex_hashtable_set(vert_hashes, v2, vert_index);
        vertex_list_push_back(vert_list, v2);
        int_list_push_back(tri_list, vert_index);
        vert_index++;
      } else {
        int_list_push_back(tri_list, v2_id);
      }
      
      int v3_id = vertex_hashtable_get(vert_hashes, v3);
      if ( v3_id == -1 ) {
        vertex_hashtable_set(vert_hashes, v3, vert_index);
        vertex_list_push_back(vert_list, v3);
        int_list_push_back(tri_list, vert_index);
        vert_index++;
      } else {
        int_list_push_back(tri_list, v3_id);
      }
      
    }
    
    else if (sscanf(line, "f %i//%i %i//%i %i//%i", &pi1, &ni1, &pi2, &ni2, &pi3, &ni3) == 6) {
      
      if (active_mesh == NULL) {
        vert_index = 0;
        
        vertex_hashtable_delete(vert_hashes);
        vertex_list_delete(vert_list);
        int_list_delete(tri_list);
        
        vert_list = vertex_list_new();
        tri_list = int_list_new();
        vert_hashes = vertex_hashtable_new(4096);
        
        active_mesh = malloc(sizeof(mesh));
      }
      
      has_normal_data = true;
      has_texcoord_data = false;
      
      /* OBJ file indicies start from one, have to subtract one */
      pi1--; ni1--; pi2--; ni2--; pi3--; ni3--;
      
      vertex v1, v2, v3;
      v1.position = vertex_list_get(vert_data, pi1).position;
      v1.uvs = vec2_zero();
      v1.normal = vertex_list_get(vert_data, ni1).normal;
      
      v2.position = vertex_list_get(vert_data, pi2).position;
      v2.uvs = vec2_zero();
      v2.normal = vertex_list_get(vert_data, ni2).normal;
      
      v3.position = vertex_list_get(vert_data, pi3).position;
      v3.uvs = vec2_zero();
      v3.normal = vertex_list_get(vert_data, ni3).normal;
      
      int v1_id = vertex_hashtable_get(vert_hashes, v1);
      if ( v1_id == -1 ) {
        vertex_hashtable_set(vert_hashes, v1, vert_index);
        vertex_list_push_back(vert_list, v1);
        int_list_push_back(tri_list, vert_index);
        vert_index++;
      } else {
        int_list_push_back(tri_list, v1_id);
      }
      
      int v2_id = vertex_hashtable_get(vert_hashes, v2);
      if ( v2_id == -1 ) {
        vertex_hashtable_set(vert_hashes, v2, vert_index);
        vertex_list_push_back(vert_list, v2);
        int_list_push_back(tri_list, vert_index);
        vert_index++;
      } else {
        int_list_push_back(tri_list, v2_id);
      }
      
      int v3_id = vertex_hashtable_get(vert_hashes, v3);
      if ( v3_id == -1 ) {
        vertex_hashtable_set(vert_hashes, v3, vert_index);
        vertex_list_push_back(vert_list, v3);
        int_list_push_back(tri_list, vert_index);
        vert_index++;
      } else {
        int_list_push_back(tri_list, v3_id);
      }
      
    }
    
    else if (sscanf(line, "f %i/%i %i/%i %i/%i", &pi1, &ti1, &pi2, &ti2, &pi3, &ti3) == 6) {
      
      if (active_mesh == NULL) {
        vert_index = 0;
        
        vertex_hashtable_delete(vert_hashes);
        vertex_list_delete(vert_list);
        int_list_delete(tri_list);
        
        vert_list = vertex_list_new();
        tri_list = int_list_new();
        vert_hashes = vertex_hashtable_new(4096);
        
        active_mesh = malloc(sizeof(mesh));
      }
      
      has_normal_data = false;
      has_texcoord_data = true;
      
      /* OBJ file indicies start from one, have to subtract one */
      pi1--; ti1--; pi2--; ti2--; pi3--; ti3--;
      
      vertex v1, v2, v3;
      v1.position = vertex_list_get(vert_data, pi1).position;
      v1.uvs = vertex_list_get(vert_data, ti1).uvs;
      v1.normal = vec3_zero();
      
      v2.position = vertex_list_get(vert_data, pi2).position;
      v2.uvs = vertex_list_get(vert_data, ti2).uvs;
      v2.normal = vec3_zero();
      
      v3.position = vertex_list_get(vert_data, pi3).position;
      v3.uvs = vertex_list_get(vert_data, ti3).uvs;
      v3.normal = vec3_zero();
      
      int v1_id = vertex_hashtable_get(vert_hashes, v1);
      if ( v1_id == -1 ) {
        vertex_hashtable_set(vert_hashes, v1, vert_index);
        vertex_list_push_back(vert_list, v1);
        int_list_push_back(tri_list, vert_index);
        vert_index++;
      } else {
        int_list_push_back(tri_list, v1_id);
      }
      
      int v2_id = vertex_hashtable_get(vert_hashes, v2);
      if ( v2_id == -1 ) {
        vertex_hashtable_set(vert_hashes, v2, vert_index);
        vertex_list_push_back(vert_list, v2);
        int_list_push_back(tri_list, vert_index);
        vert_index++;
      } else {
        int_list_push_back(tri_list, v2_id);
      }
      
      int v3_id = vertex_hashtable_get(vert_hashes, v3);
      if ( v3_id == -1 ) {
        vertex_hashtable_set(vert_hashes, v3, vert_index);
        vertex_list_push_back(vert_list, v3);
        int_list_push_back(tri_list, vert_index);
        vert_index++;
      } else {
        int_list_push_back(tri_list, v3_id);
      }
      
    }
    */
    else if let (pi1, pi2, pi3, 3) = scan_fmt_count!(&line, "f {} {} {}", i32, i32, i32) {
      if !has_active_mesh {
        vert_list.clear();
        tri_list.clear();
        vert_hashes.clear();

        has_active_mesh = true;
      }

      has_normal_data = false;
      has_texcoord_data = false;

      let pi1 = pi1.unwrap();
      let pi2 = pi2.unwrap();
      let pi3 = pi3.unwrap();


      let pi1 = (pi1 - 1) as usize;
      let pi2 = (pi2 - 1) as usize;
      let pi3 = (pi3 - 1) as usize;

      let v1 = Vertex {
        position: vert_data[pi1].position,
        uvs: Vec2::zero(),
        normal: Vec3::zero(),
        tangent: Vec3::zero(),
        binormal: Vec3::zero(),
        color: Vec4::zero(),
      };
      let v2 = Vertex {
        position: vert_data[pi2].position,
        uvs: Vec2::zero(),
        normal: Vec3::zero(),
        tangent: Vec3::zero(),
        binormal: Vec3::zero(),
        color: Vec4::zero(),
      };
      let v3 = Vertex {
        position: vert_data[pi3].position,
        uvs: Vec2::zero(),
        normal: Vec3::zero(),
        tangent: Vec3::zero(),
        binormal: Vec3::zero(),
        color: Vec4::zero(),
      };

      {
        let v1_id = vert_hashes.entry(v1.clone()).or_insert_with(|| {
          let vert_index = vert_list.len();
          vert_list.push(v1);
          vert_index as u32
        });
        tri_list.push(*v1_id);
      }

      {
        let v2_id = vert_hashes.entry(v2.clone()).or_insert_with(|| {
          let vert_index = vert_list.len();
          vert_list.push(v2);
          vert_index as u32
        });
        tri_list.push(*v2_id);
      }

      {
        let v3_id = vert_hashes.entry(v3.clone()).or_insert_with(|| {
          let vert_index = vert_list.len();
          vert_list.push(v3);
          vert_index as u32
        });
        tri_list.push(*v3_id);
      }

    }

    /*
    else if (sscanf(line, "f %i %i %i", &pi1, &pi2, &pi3) == 3) {
      
      if (active_mesh == NULL) {
        vert_index = 0;
        
        vertex_hashtable_delete(vert_hashes);
        vertex_list_delete(vert_list);
        int_list_delete(tri_list);
        
        vert_list = vertex_list_new();
        tri_list = int_list_new();
        vert_hashes = vertex_hashtable_new(4096);
        
        active_mesh = malloc(sizeof(mesh));
      }
      
      has_normal_data = false;
      has_texcoord_data = false;
      
      /* OBJ file indicies start from one, have to subtract one */
      pi1--; pi2--; pi3--;
      
      vertex v1, v2, v3;
      v1.position = vertex_list_get(vert_data, pi1).position;
      v1.uvs = vec2_zero();
      v1.normal = vec3_zero();
      
      v2.position = vertex_list_get(vert_data, pi2).position;
      v2.uvs = vec2_zero();
      v2.normal = vec3_zero();
      
      v3.position = vertex_list_get(vert_data, pi3).position;
      v3.uvs = vec2_zero();
      v3.normal = vec3_zero();
      
      int v1_id = vertex_hashtable_get(vert_hashes, v1);
      if ( v1_id == -1 ) {
        vertex_hashtable_set(vert_hashes, v1, vert_index);
        vertex_list_push_back(vert_list, v1);
        int_list_push_back(tri_list, vert_index);
        vert_index++;
      } else {
        int_list_push_back(tri_list, v1_id);
      }
      
      int v2_id = vertex_hashtable_get(vert_hashes, v2);
      if ( v2_id == -1 ) {
        vertex_hashtable_set(vert_hashes, v2, vert_index);
        vertex_list_push_back(vert_list, v2);
        int_list_push_back(tri_list, vert_index);
        vert_index++;
      } else {
        int_list_push_back(tri_list, v2_id);
      }
      
      int v3_id = vertex_hashtable_get(vert_hashes, v3);
      if ( v3_id == -1 ) {
        vertex_hashtable_set(vert_hashes, v3, vert_index);
        vertex_list_push_back(vert_list, v3);
        int_list_push_back(tri_list, vert_index);
        vert_index++;
      } else {
        int_list_push_back(tri_list, v3_id);
      }
      
    }
    
  }
  */
  }

  if !has_active_mesh {
    let mut full_error = String::new();
    let _ = write!(
      &mut full_error,
      "Unable to load file '{}', it appears to be empty",
      filename
    );
    error(&full_error);
    //return early here
  }

  let mut active_mesh = mesh_new();

  active_mesh.vertices.clone_from(&vert_list);

  active_mesh.triangles.clone_from(&tri_list);

  obj_model.meshes.push(active_mesh);

  if !has_normal_data {
    model_generate_normals(&mut obj_model);
  }

  if !has_texcoord_data {
    model_generate_texcoords_cylinder(&mut obj_model);
  }

  model_generate_tangents(&mut obj_model);

  let mut renderable = renderable_new();

  renderable_add_model(&mut renderable, &obj_model);

  /*

  SDL_RWclose(file);
  
  if (active_mesh == NULL) {
    error("Unable to load file '%s', it appears to be empty.", filename);
  }
  
  active_mesh->num_verts = vert_index;
  active_mesh->num_triangles = tri_list->num_items / 3;
  
  active_mesh->verticies = malloc(sizeof(vertex) * active_mesh->num_verts);
  for(int i = 0; i < active_mesh->num_verts; i++) {
    active_mesh->verticies[i] = vertex_list_get(vert_list, i);
  }
  
  active_mesh->triangles = malloc(sizeof(int) * active_mesh->num_triangles * 3);
  for(int i = 0; i < active_mesh->num_triangles * 3; i++) {
    active_mesh->triangles[i] = int_list_get(tri_list, i);
  }
  
  obj_model->num_meshes++;
  obj_model->meshes = realloc(obj_model->meshes, sizeof(mesh*) * obj_model->num_meshes);
  obj_model->meshes[obj_model->num_meshes-1] = active_mesh;
  
  vertex_hashtable_delete(vert_hashes);
  vertex_list_delete(vert_data);
  vertex_list_delete(vert_list);
  int_list_delete(tri_list);
  
  if (!has_normal_data) {
    model_generate_normals(obj_model);
  }
  
  if (!has_texcoord_data) {
    model_generate_texcoords_cylinder(obj_model);
  }
  
  model_generate_tangents(obj_model);
  
  renderable* renderable = renderable_new();
  renderable_add_model(renderable, obj_model);
  model_delete(obj_model);
  */

  let mut mat_file = PathBuf::from(filename);
  let _ = mat_file.pop(); //.unwrap(); //TODO error handling
  let mut bmf_file = PathBuf::from(filename);
  let _ = bmf_file.pop(); //.parent().unwrap();
  let fileid = PathBuf::from(filename); //.file_stem(); //.clone().unwrap();
  let fileid = fileid.file_stem().unwrap();

  mat_file.push(fileid);
  let _ = mat_file.set_extension(".mat");

  if mat_file.exists() {
    renderable.material = asset_hndl_new(&mat_file.to_string_lossy());
  }

  bmf_file.push(fileid);
  let _ = bmf_file.set_extension(".bmf");

  //TODO bmf_save_file(renderable, bmf_file.to_string_lossy());

  return Rc::new(RefCell::new(renderable));


  /*
  fpath mat_file;
  fpath bmf_file;
  fpath fileid;
  
  SDL_PathFileLocation(mat_file.ptr, filename);
  SDL_PathFileLocation(bmf_file.ptr, filename);
  SDL_PathFileName(fileid.ptr, filename);
  
  strcat(mat_file.ptr, fileid.ptr);
  strcat(mat_file.ptr, ".mat");
  
  if (file_exists(mat_file)) {
    renderable->material = asset_hndl_new(mat_file);
  }
  
  strcat(bmf_file.ptr, fileid.ptr);
  strcat(bmf_file.ptr, ".bmf");
  bmf_save_file(renderable, bmf_file.ptr);
  
  return renderable;
}
*/
}

/*

static void renderable_add_mesh_rigged(renderable* r, mesh* m, vertex_weight* weights) {
  
  renderable_surface* surface = renderable_surface_new_rigged(m, weights);
  
  r->num_surfaces++;
  r->surfaces = realloc(r->surfaces, sizeof(renderable_surface*) *  r->num_surfaces);
  r->surfaces[r->num_surfaces-1] = surface;
  
}

enum {
  STATE_LOAD_EMPTY     = 0,
  STATE_LOAD_TRIANGLES = 1,
};

renderable* smd_load_file(char* filename) {
  
  int state = STATE_LOAD_EMPTY;
  char state_material[1024];
  
  vertex_hashtable* hashes = vertex_hashtable_new(1024);
  vertex_list* vert_list = vertex_list_new();
  int_list* tri_list = int_list_new();
  
  int allocated_weights = 1024;
  vertex_weight* weights = malloc(sizeof(vertex_weight) * allocated_weights);
  
  int vert_index = 0;
  
  SDL_RWops* file = SDL_RWFromFile(filename, "r");
  
  if(file == NULL) {
    error("Could not load file %s", filename);
  }
  
  renderable* r = renderable_new();
  r->is_rigged = true;
  
  char line[1024];
  while(SDL_RWreadline(file, line, 1024)) {
    
    if (strstr(line, "end")) {
      state = STATE_LOAD_EMPTY;
    }
    
    if (state == STATE_LOAD_EMPTY) {
      
      int version;
      if (sscanf(line, "version %i", &version) > 0) {
        if (version != 1) {
          error("Can't load SMD file %s. Don't know how to load version %i\n", filename, version);
        }
      }
      
      if (strstr(line, "triangles")) {
        state = STATE_LOAD_TRIANGLES;
      }
    }
    
    if (state == STATE_LOAD_TRIANGLES) {
      
      int id, l1_id, l2_id, l3_id;
      int num_links = 0;
      float x, y, z, nx, ny, nz, u, v, l1_amount, l2_amount, l3_amount;
      if (sscanf(line, "%i %f %f %f %f %f %f %f %f %i %i %f %i %f %i %f", 
          &id, &x, &y, &z, &nx, &ny, &nz, &u, &v, &num_links, 
          &l1_id, &l1_amount, &l2_id, &l2_amount, &l3_id, &l3_amount) > 9) {
        
        if (num_links > 3) {
          //warning("Loading file '%s'. More than 3 bones rigged to vertex (%i). Ignoring other bones", filename, num_links);
          num_links = 3;
        }
        
        if (num_links == 0) {
          warning("Loading file '%s'. Vertex has no direct bone links", filename);
          num_links = 1;
          l1_id = id;
        }
        
        vertex vert;
        /* Swap y and z axis */
        vert.position = vec3_new(x, z, y);
        vert.normal = vec3_new(nx, nz, ny);
        vert.uvs = vec2_new(u, v);
        vert.color = vec4_one();
        vert.tangent = vec3_zero();
        vert.binormal = vec3_zero();
        
        int vert_pos = vertex_hashtable_get(hashes, vert);
        
        /* Not already in hashtable */
        if (vert_pos == -1) {
          vertex_hashtable_set(hashes, vert, vert_index);
          vert_pos = vert_index;
          vertex_list_push_back(vert_list, vert);
          
          vertex_weight vw;
          switch (num_links) {
            case 3:
              vw.bone_ids[0] = l1_id;
              vw.bone_ids[1] = l2_id;
              vw.bone_ids[2] = l3_id;
              vw.bone_weights[0] = l1_amount;
              vw.bone_weights[1] = l2_amount;
              vw.bone_weights[2] = l3_amount;
            break;
            case 2:
              vw.bone_ids[0] = l1_id;
              vw.bone_ids[1] = l2_id;
              vw.bone_ids[2] = 0;
              vw.bone_weights[0] = l1_amount;
              vw.bone_weights[1] = l2_amount;
              vw.bone_weights[2] = 0;
            break;
            case 1:
              vw.bone_ids[0] = l1_id;
              vw.bone_ids[1] = 0;
              vw.bone_ids[2] = 0;
              vw.bone_weights[0] = 1;
              vw.bone_weights[1] = 0;
              vw.bone_weights[2] = 0;
            break;
            default:
              warning("Loading file %s. Unrigged vertex!", filename);
              vw.bone_ids[0] = 0;
              vw.bone_ids[1] = 0;
              vw.bone_ids[2] = 0;
              vw.bone_weights[0] = 1;
              vw.bone_weights[1] = 0;
              vw.bone_weights[2] = 0;
            break;
          }
          
          float total = vw.bone_weights[0] + vw.bone_weights[1] + vw.bone_weights[2];
          
          if (total != 0) {
            vw.bone_weights[0] /= total;
            vw.bone_weights[1] /= total;
            vw.bone_weights[2] /= total;
          }
          
          while(vert_pos >= allocated_weights) {
            allocated_weights = allocated_weights * 2;
            weights = realloc(weights, sizeof(vertex_weight) * allocated_weights);
          }

          weights[vert_pos] = vw;
          
          vert_index++;
        }
        
        int_list_push_back(tri_list, vert_pos);
        
      } else {
        
        if (vert_index == 0) {
          strcpy(state_material, line);
        }
        
        else if (strcmp(state_material, line)) {
          
          strcpy(state_material, line);
          
          mesh* m = malloc(sizeof(mesh));
          m->num_verts = vert_list->num_items;
          m->num_triangles = tri_list->num_items / 3;
          
          m->verticies = malloc(sizeof(vertex) * m->num_verts);
          m->triangles = malloc(sizeof(int) * m->num_triangles * 3);
          
          for(int i = 0; i < m->num_verts; i++) {
            m->verticies[i] = vertex_list_get(vert_list, i);
          }
          
          for(int i = 0; i < m->num_triangles * 3; i+=3) {
            m->triangles[i+0] = int_list_get(tri_list, i+2);
            m->triangles[i+1] = int_list_get(tri_list, i+1);
            m->triangles[i+2] = int_list_get(tri_list, i+0);
          }
          
          mesh_generate_tangents(m);
          renderable_add_mesh_rigged(r, m, weights);
          mesh_delete(m);
          
          vertex_hashtable_delete(hashes);
          vertex_list_delete(vert_list);
          int_list_delete(tri_list);
          
          vert_index = 0;
          hashes = vertex_hashtable_new(1024);
          vert_list = vertex_list_new();
          tri_list = int_list_new();
          
          allocated_weights = 1024;
          weights = realloc(weights, sizeof(vertex_weight) * 1024);
          
        }
        
      }
      
    }
    
  }
  
  SDL_RWclose(file);
  
  mesh* m = malloc(sizeof(mesh));
  m->num_verts = vert_list->num_items;
  m->num_triangles = tri_list->num_items / 3;
  m->verticies = malloc(sizeof(vertex) * m->num_verts);
  m->triangles = malloc(sizeof(int) * m->num_triangles * 3);
  
  for(int i = 0; i < m->num_verts; i++) {
    m->verticies[i] = vertex_list_get(vert_list, i);
  }
  
  for(int i = 0; i < m->num_triangles * 3; i+=3) {
    m->triangles[i+0] = int_list_get(tri_list, i+2);
    m->triangles[i+1] = int_list_get(tri_list, i+1);
    m->triangles[i+2] = int_list_get(tri_list, i+0);
  }
  
  mesh_generate_tangents(m);  
  renderable_add_mesh_rigged(r, m, weights);
  mesh_delete(m);
  
  vertex_hashtable_delete(hashes);
  vertex_list_delete(vert_list);
  int_list_delete(tri_list);
  free(weights);

  fpath mat_file;
  fpath bmf_file;
  fpath fileid;
  
  SDL_PathFileLocation(mat_file.ptr, filename);
  SDL_PathFileLocation(bmf_file.ptr, filename);
  SDL_PathFileName(fileid.ptr, filename);
  
  strcat(mat_file.ptr, fileid.ptr);
  strcat(mat_file.ptr, ".mat");
  
  if (file_exists(mat_file)) {
    r->material = asset_hndl_new(mat_file);
  }
  
  strcat(bmf_file.ptr, fileid.ptr);
  strcat(bmf_file.ptr, ".bmf");
  bmf_save_file(r, bmf_file.ptr);
  
  return r;
}

renderable* ply_load_file(char* filename) {
  
  SDL_RWops* file = SDL_RWFromFile(filename, "r");
  
  if(file == NULL) {
    error("Could not load file %s", filename);
  }
  
  renderable* r = renderable_new();
  
  mesh* curr_mesh = NULL;
  int vert_id = 0;
  int tri_id = 0;
  
  char line[1024];
  while(SDL_RWreadline(file, line, 1024)) {
        
    if (strstr(line, "ply")) {
      
      if (curr_mesh != NULL) {
        mesh_generate_tangents(curr_mesh);
        renderable_add_mesh(r, curr_mesh);
        mesh_delete(curr_mesh);
      }
    
      curr_mesh = mesh_new();
      vert_id = 0;
      tri_id = 0;
    }
    
    int vert_count = 0;
    if (sscanf(line, "element vertex %i", &vert_count)) {
      curr_mesh->num_verts = vert_count;
      curr_mesh->verticies = malloc(sizeof(vertex) * vert_count);
    }
    
    int tri_count = 0;
    if (sscanf(line, "element face %i", &tri_count)) {
      curr_mesh->num_triangles = tri_count;
      curr_mesh->triangles = malloc(sizeof(uint32_t) * tri_count * 3);
    }
    
    vertex v;
    int r, g, b, a;
    if (sscanf(line, "%f %f %f %f %f %f %f %f %i %i %i %i",
      &v.position.x, &v.position.y, &v.position.z,
      &v.normal.x, &v.normal.y, &v.normal.z,
      &v.uvs.x, &v.uvs.y, &r, &g, &b, &a) == 12) {
      
      v.color = vec4_new(
        (float)r / 255.0, (float)g / 255.0, 
        (float)b / 255.0, (float)a / 255.0);
      
      curr_mesh->verticies[vert_id] = v; vert_id++;
    }
    
    int i0, i1, i2;
    if (sscanf(line, "3 %i %i %i", &i0, &i1, &i2) == 3) {
      curr_mesh->triangles[tri_id] = i0; tri_id++;
      curr_mesh->triangles[tri_id] = i1; tri_id++;
      curr_mesh->triangles[tri_id] = i2; tri_id++;
    }
    
  }
  
  SDL_RWclose(file);
  
  if (curr_mesh != NULL) {
    mesh_generate_tangents(curr_mesh);
    renderable_add_mesh(r, curr_mesh);
    mesh_delete(curr_mesh);
  }
  
  fpath mat_file;
  fpath bmf_file;
  fpath fileid;
  
  SDL_PathFileLocation(mat_file.ptr, filename);
  SDL_PathFileLocation(bmf_file.ptr, filename);
  SDL_PathFileName(fileid.ptr, filename);
  
  strcat(mat_file.ptr, fileid.ptr);
  strcat(mat_file.ptr, ".mat");
  
  if (file_exists(mat_file)) {
    r->material = asset_hndl_new(mat_file);
  }
  
  strcat(bmf_file.ptr, fileid.ptr);
  strcat(bmf_file.ptr, ".bmf");
  bmf_save_file(r, bmf_file.ptr);
  
  return r;
  
}

*/
