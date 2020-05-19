
use std::fmt::Write;
use std::hash::Hash;
use std::hash::Hasher;
use std::f32;

use gl;
//use alga::linear::EuclideanSpace;
//use na::{Point4, Point3, Point2, Matrix4, Matrix3, Matrix2, Rotation2, Vector2};
use cgmath::*;
use ordered_float::OrderedFloat;


use crate::core::error;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;
//use std::ops::Mul;
//use cgmath::prelude::*;

/*

static const char* gl_error_string_invalid_enum = "Invalid Enum";
static const char* gl_error_string_invalid_value = "Invalid Value";
static const char* gl_error_string_invalid_operation = "Invalid Operation";
static const char* gl_error_string_out_of_memory = "Out of Memory";
static const char* gl_error_string_invalid_framebuffer_operation = "Invalid Framebuffer Operation";
static const char* gl_error_string_stack_overflow = "Stack Overflow";
static const char* gl_error_string_stack_underflow = "Stack Underflow";
static const char* gl_error_string_table_too_large = "Table Too Large";
static const char* gl_error_string_no_error = "No Error";
*/

pub struct GLExtValues;
impl GLExtValues {
  pub const GL_TEXTURE_MAX_ANISOTROPY_EXT: gl::types::GLenum = 0x84FE;
  pub const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: gl::types::GLenum = 0x84FF;
  /* Extension Constants - Found these from glew and Google */

  pub const GL_TABLE_TOO_LARGE: gl::types::GLenum = 0x8031;
  pub const GL_INVALID_FRAMEBUFFER_OPERATION: gl::types::GLenum = 0x0506;

  pub const GL_SHADING_LANGUAGE_VERSION: gl::types::GLenum = 0x8B8C;

  pub const GL_VERTEX_SHADER: gl::types::GLenum = 0x8B31;
  pub const GL_FRAGMENT_SHADER: gl::types::GLenum = 0x8B30;
  pub const GL_GEOMETRY_SHADER: gl::types::GLenum = 0x8DD9;
  pub const GL_COMPILE_STATUS: gl::types::GLenum = 0x8B81;
  pub const GL_LINK_STATUS: gl::types::GLenum = 0x8B82;

  //#undef GL_GEOMETRY_VERTICES_OUT
  //#undef GL_GEOMETRY_INPUT_TYPE
  //#undef GL_GEOMETRY_OUTPUT_TYPE
  pub const GL_GEOMETRY_VERTICES_OUT: gl::types::GLenum = 0x8DDA;
  pub const GL_GEOMETRY_INPUT_TYPE: gl::types::GLenum = 0x8DDB;
  pub const GL_GEOMETRY_OUTPUT_TYPE: gl::types::GLenum = 0x8DDC;
  pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES: gl::types::GLenum = 0x8DE0;
  pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: gl::types::GLenum = 0x8DE1;

  pub const GL_FRAMEBUFFER: gl::types::GLenum = 0x8D40;
  pub const GL_RENDERBUFFER: gl::types::GLenum = 0x8D41;
  pub const GL_READ_FRAMEBUFFER: gl::types::GLenum = 0x8CA8;
  pub const GL_DRAW_FRAMEBUFFER: gl::types::GLenum = 0x8CA9;
  pub const GL_ARRAY_BUFFER: gl::types::GLenum = 0x8892;
  pub const GL_ELEMENT_ARRAY_BUFFER: gl::types::GLenum = 0x8893;

  pub const GL_FRAMEBUFFER_COMPLETE: gl::types::GLenum = 0x8CD5;
  pub const GL_FRAMEBUFFER_UNDEFINED: gl::types::GLenum = 0x8219;
  pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: gl::types::GLenum = 0x8CD6;
  pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: gl::types::GLenum = 0x8CD7;
  pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: gl::types::GLenum = 0x8CDB;
  pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER: gl::types::GLenum = 0x8CDC;
  pub const GL_FRAMEBUFFER_UNSUPPORTED: gl::types::GLenum = 0x8CDD;
  pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: gl::types::GLenum = 0x8D56;
  pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: gl::types::GLenum = 0x8DA8;

  pub const GL_STATIC_DRAW: gl::types::GLenum = 0x88E4;
  pub const GL_DYNAMIC_COPY: gl::types::GLenum = 0x88EA;

  pub const GL_MAX_COLOR_ATTACHMENTS: gl::types::GLenum = 0x8CDF;
  pub const GL_COLOR_ATTACHMENT0: gl::types::GLenum = 0x8CE0;
  pub const GL_COLOR_ATTACHMENT1: gl::types::GLenum = 0x8CE1;
  pub const GL_COLOR_ATTACHMENT2: gl::types::GLenum = 0x8CE2;
  pub const GL_COLOR_ATTACHMENT3: gl::types::GLenum = 0x8CE3;
  pub const GL_COLOR_ATTACHMENT4: gl::types::GLenum = 0x8CE4;
  pub const GL_COLOR_ATTACHMENT5: gl::types::GLenum = 0x8CE5;
  pub const GL_COLOR_ATTACHMENT6: gl::types::GLenum = 0x8CE6;
  pub const GL_COLOR_ATTACHMENT7: gl::types::GLenum = 0x8CE7;
  pub const GL_COLOR_ATTACHMENT8: gl::types::GLenum = 0x8CE8;
  pub const GL_COLOR_ATTACHMENT9: gl::types::GLenum = 0x8CE9;
  pub const GL_COLOR_ATTACHMENT10: gl::types::GLenum = 0x8CEA;
  pub const GL_COLOR_ATTACHMENT11: gl::types::GLenum = 0x8CEB;
  pub const GL_COLOR_ATTACHMENT12: gl::types::GLenum = 0x8CEC;
  pub const GL_COLOR_ATTACHMENT13: gl::types::GLenum = 0x8CED;
  pub const GL_COLOR_ATTACHMENT14: gl::types::GLenum = 0x8CEE;
  pub const GL_COLOR_ATTACHMENT15: gl::types::GLenum = 0x8CEF;
  pub const GL_DEPTH_ATTACHMENT: gl::types::GLenum = 0x8D00;
  pub const GL_STENCIL_ATTACHMENT: gl::types::GLenum = 0x8D20;

  pub const GL_RGBA32F: gl::types::GLenum = 0x8814;
  pub const GL_RGBA16F: gl::types::GLenum = 0x881A;
  pub const GL_BGRA: gl::types::GLenum = 0x80E1;
  pub const GL_BGR: gl::types::GLenum = 0x80E0;
  pub const GL_COMPRESSED_RGBA_S3TC_DXT1: gl::types::GLenum = 0x83F1;
  pub const GL_COMPRESSED_RGBA_S3TC_DXT3: gl::types::GLenum = 0x83F2;
  pub const GL_COMPRESSED_RGBA_S3TC_DXT5: gl::types::GLenum = 0x83F3;

  pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: gl::types::GLenum = 0x8366;
  pub const GL_UNSIGNED_SHORT_5_6_5: gl::types::GLenum = 0x8363;
  pub const GL_DEPTH_COMPONENT24: gl::types::GLenum = 0x81A6;

  pub const GL_CLAMP_TO_EDGE: gl::types::GLenum = 0x812F;
  pub const GL_TEXTURE_WRAP_R: gl::types::GLenum = 0x8072;
  pub const GL_MIRRORED_REPEAT: gl::types::GLenum = 0x8370;
  pub const GL_TEXTURE_DEPTH: gl::types::GLenum = 0x8071;
  pub const GL_TEXTURE_MAX_ANISOTROPY: gl::types::GLenum = 0x84FE;
  pub const GL_MAX_TEXTURE_MAX_ANISOTROPY: gl::types::GLenum = 0x84FF;
  pub const GL_GENERATE_MIPMAP: gl::types::GLenum = 0x8191;
  pub const GL_TEXTURE_MAX_LEVEL: gl::types::GLenum = 0x813D;

  pub const GL_TEXTURE0: gl::types::GLenum = 0x84C0;
  pub const GL_TEXTURE_3D: gl::types::GLenum = 0x806F;
  pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: gl::types::GLenum = 0x884F;

  pub const GL_MULTISAMPLE: gl::types::GLenum = 0x809D;

  pub const GL_TESS_CONTROL_SHADER: gl::types::GLenum = 0x8E88;
  pub const GL_TESS_EVALUATION_SHADER: gl::types::GLenum = 0x8E87;
  pub const GL_PATCH_VERTICES: gl::types::GLenum = 0x8E72;
}

struct GLErrorStrings;

impl GLErrorStrings {
  fn gl_error_string_invalid_enum() -> String {
    "Invalid Enum".into()
  }
  fn gl_error_string_invalid_value() -> String {
    "Invalid Value".into()
  }
  fn gl_error_string_invalid_operation() -> String {
    "Invalid Operation".into()
  }
  fn gl_error_string_out_of_memory() -> String {
    "Out of Memory".into()
  }
  fn gl_error_string_invalid_framebuffer_operation() -> String {
    "Invalid Framebuffer Operation".into()
  }
  fn gl_error_string_stack_overflow() -> String {
    "Stack Overflow".into()
  }
  fn gl_error_string_stack_underflow() -> String {
    "Stack Underflow".into()
  }
  //fn gl_error_string_table_too_large()-> String { "Table Too Large".into() }
  fn gl_error_string_unknown_error() -> String {
    "Unknown Error".into()
  }
  //fn gl_error_string_no_error() -> String { "No Error".into() }
}



/* OpenGL error checking */
/*
#ifdef RELEASE
#define SDL_GL_CheckError()
#else
#define SDL_GL_CheckError() { GLenum __glerror = glGetError(); if (__glerror) { error("OpenGL Error: %s", SDL_GL_ErrorString(__glerror)); } }
#endif
*/

pub fn gl_error_string(error: &gl::types::GLenum) -> String {
  match *error {
    gl::INVALID_ENUM => return GLErrorStrings::gl_error_string_invalid_enum(),
    gl::INVALID_VALUE => return GLErrorStrings::gl_error_string_invalid_value(),
    gl::INVALID_OPERATION => return GLErrorStrings::gl_error_string_invalid_operation(),
    gl::OUT_OF_MEMORY => return GLErrorStrings::gl_error_string_out_of_memory(),
    gl::INVALID_FRAMEBUFFER_OPERATION => {
      return GLErrorStrings::gl_error_string_invalid_framebuffer_operation()
    }
    gl::STACK_OVERFLOW => return GLErrorStrings::gl_error_string_stack_overflow(),
    gl::STACK_UNDERFLOW => return GLErrorStrings::gl_error_string_stack_underflow(),
    _ => return GLErrorStrings::gl_error_string_unknown_error(),
    //gl is missing this define
    //gl::TABLE_TOO_LARGE =>
    //  return GLErrorStrings::gl_error_string_table_too_large()
  }
  //return GLErrorStrings::gl_error_string_no_error();
}

pub fn gl_check_error() {
  let gl_error = unsafe { gl::GetError() };

  if gl_error != 0 {
    let mut error_full = String::new();
    let _ = write!(
      &mut error_full,
      "OpenGL error: {}",
      gl_error_string(&gl_error)
    );
    error(&error_full);
  }
}


//this doesn't work because I have to re-implement all the traits and so on. IE worthless
//pub struct Vec4(Point4<f32>); //<f32>);

pub type Vec4 = Vector4<f32>; //Point4<f32>;
pub type Vec3 = Vector3<f32>; //Point3<f32>;
pub type Vec2 = Vector2<f32>; //Point2<f32>;
pub type Dir2 = Vector2<f32>;
pub type Mat4 = Matrix4<f32>;
pub type Mat3 = Matrix3<f32>;
pub type Mat2 = Matrix2<f32>;
//pub trait Rot2: Rotation2<f32> {}

pub trait ToArr {
  type Output;
  fn to_arr(&self) -> Self::Output;
}

impl<T: BaseNum> ToArr for Matrix3<T> {
  type Output = [[T; 3]; 3];
  fn to_arr(&self) -> Self::Output {
    (*self).into()
  }
}
impl<T: BaseNum> ToArr for Matrix4<T> {
  type Output = [[T; 4]; 4];
  fn to_arr(&self) -> Self::Output {
    (*self).into()
  }
}

#[derive(Clone)]
pub struct Vertex {
  pub position: Vec3,
  pub normal: Vec3,
  pub tangent: Vec3,
  pub binormal: Vec3,
  pub color: Vec4,
  pub uvs: Vec2,
}

impl PartialEq for Vertex {
  fn eq(&self, other: &Vertex) -> bool {
    return vertex_equal(self, other);
  }
}

impl Eq for Vertex {}

impl Hash for Vertex {
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    //TODO I could keep the hash in the struct somewhere? or these ord floats? I hope the
    //slowdown isn't too big
    let v = [
      OrderedFloat(self.position.x),
      OrderedFloat(self.position.y),
      OrderedFloat(self.position.z),
      OrderedFloat(self.normal.x),
      OrderedFloat(self.normal.y),
      OrderedFloat(self.normal.z),
      OrderedFloat(self.uvs.x),
      OrderedFloat(self.uvs.y),
    ];
    v.hash(state);
  }
}


/*
typedef struct {
  int num_meshes;
  mesh** meshes;
} model;
*/
pub struct Model {
  pub meshes: Vec<Mesh>,
}

pub struct Mesh {
  pub vertices: Vec<Vertex>,
  pub triangles: Vec<u32>,
}

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
}

pub trait Vec3Additional {
  fn vec3_lerp(&self, other: &Vec3, amount: f32) -> Vec3;
}

pub trait Vec4Additional {
  fn vec4_one() -> Vec4;
  fn vec4_pow(&self, exp: f32) -> Vec4;
  fn vec4_min(&self, other: &Vec4) -> Vec4;
  fn vec4_max(&self, other: &Vec4) -> Vec4;
  fn vec4_abs(&self) -> Vec4;
  fn vec4_floor(&self) -> Vec4;
  fn vec4_fmod(&self, val: f32) -> Vec4;
  fn vec4_sqrt(&self) -> Vec4;
  fn vec4_saturate(&self) -> Vec4;
  fn vec4_binearest_interp(
    tl: &Vec4,
    tr: &Vec4,
    bl: &Vec4,
    br: &Vec4,
    x_amount: f32,
    y_amount: f32,
  ) -> Vec4;
  fn vec4_bilinear_interp(
    tl: &Vec4,
    tr: &Vec4,
    bl: &Vec4,
    br: &Vec4,
    x_amount: f32,
    y_amount: f32,
  ) -> Vec4;
}

pub fn saturate(x: f32) -> f32 {
  return x.max(0.0).min(1.0);
}

pub fn lerp(p1: f32, p2: f32, amount: f32) -> f32 {
  return (p2 * amount) + (p1 * (1.0 - amount));
}

pub fn binearest_interp(tl: f32, tr: f32, bl: f32, br: f32, x_amount: f32, y_amount: f32) -> f32 {
  let x_amount = if x_amount.round() != 0.0 { true } else { false };
  let y_amount = if y_amount.round() != 0.0 { true } else { false };

  if x_amount && !y_amount {
    return br;
  }
  if !x_amount && y_amount {
    return tl;
  }
  if !x_amount && !y_amount {
    return bl;
  }
  if x_amount && y_amount {
    return tr;
  }

  return 0.0;
}

pub fn bilinear_interp(tl: f32, tr: f32, bl: f32, br: f32, x_amount: f32, y_amount: f32) -> f32 {
  let left = lerp(tl, bl, y_amount);
  let right = lerp(tr, br, y_amount);

  return lerp(right, left, x_amount);
}

/*
void vec2_print(vec2 v) {
  printf("vec2(%4.2f,%4.2f)", v.x, v.y);
}
*/

pub fn vec2_print(v: &Vec2) {
  print!("vec2({},{})", v.x, v.y);
}

/*
void vec3_print(vec3 v) {
  printf("vec3(%4.2f,%4.2f,%4.2f)", v.x, v.y, v.z);
}
*/

pub fn vec3_print(v: &Vec3) {
  print!("vec3({},{},{})", v.x, v.y, v.z);
}

/*
void vec4_print(vec4 v) {
  printf("vec4(%4.2f, %4.2f, %4.2f, %4.2f)", v.x, v.y, v.z,  v.w);
}
*/
//TODO, look at how to format the lengths/precision in rust??
pub fn vec4_print(v: &Vec4) {
  print!("vec4({},{},{},{})", v.x, v.y, v.z, v.w)
}

impl Vec3Additional for Vec3 {
  fn vec3_lerp(&self, other: &Vec3, amount: f32) -> Vec3 {
    return Vec3::new(
      lerp(self.x, other.x, amount),
      lerp(self.y, other.y, amount),
      lerp(self.z, other.z, amount),
    );
  }
}

impl Vec4Additional for Vec4 {
  fn vec4_one() -> Vec4 {
    return Vec4::new(1.0, 1.0, 1.0, 1.0);
  }

  fn vec4_pow(&self, exp: f32) -> Vec4 {
    return Vec4::new(
      self.x.powf(exp),
      self.y.powf(exp),
      self.z.powf(exp),
      self.w.powf(exp),
    );
  }

  fn vec4_min(&self, other: &Vec4) -> Vec4 {
    return Vec4::new(
      self.x.min(other.x),
      self.y.min(other.y),
      self.z.min(other.z),
      self.w.min(other.w),
    );
  }

  fn vec4_max(&self, other: &Vec4) -> Vec4 {
    return Vec4::new(
      self.x.max(other.x),
      self.y.max(other.y),
      self.z.max(other.z),
      self.w.max(other.w),
    );
  }

  fn vec4_abs(&self) -> Vec4 {
    return Vec4::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs());
  }

  fn vec4_floor(&self) -> Vec4 {
    return Vec4::new(
      self.x.floor(),
      self.y.floor(),
      self.z.floor(),
      self.w.floor(),
    );
  }

  fn vec4_fmod(&self, val: f32) -> Vec4 {
    return Vec4::new(self.x % val, self.y % val, self.z % val, self.w % val);
  }

  fn vec4_sqrt(&self) -> Vec4 {
    return Vec4::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt(), self.w.sqrt());
  }

  fn vec4_saturate(&self) -> Vec4 {
    return Vec4::new(
      saturate(self.x),
      saturate(self.y),
      saturate(self.z),
      saturate(self.w),
    );
  }

  fn vec4_binearest_interp(
    tl: &Vec4,
    tr: &Vec4,
    bl: &Vec4,
    br: &Vec4,
    x_amount: f32,
    y_amount: f32,
  ) -> Vec4 {
    return Vec4::new(
      binearest_interp(tl.x, tr.x, bl.x, br.x, x_amount, y_amount),
      binearest_interp(tl.y, tr.y, bl.y, br.y, x_amount, y_amount),
      binearest_interp(tl.z, tr.z, bl.z, br.z, x_amount, y_amount),
      binearest_interp(tl.w, tr.w, bl.w, br.w, x_amount, y_amount),
    );
  }

  fn vec4_bilinear_interp(
    tl: &Vec4,
    tr: &Vec4,
    bl: &Vec4,
    br: &Vec4,
    x_amount: f32,
    y_amount: f32,
  ) -> Vec4 {
    return Vec4::new(
      bilinear_interp(tl.x, tr.x, bl.x, br.x, x_amount, y_amount),
      bilinear_interp(tl.y, tr.y, bl.y, br.y, x_amount, y_amount),
      bilinear_interp(tl.z, tr.z, bl.z, br.z, x_amount, y_amount),
      bilinear_interp(tl.w, tr.w, bl.w, br.w, x_amount, y_amount),
    );
  }
}

/*.h
/* Vertex */

typedef struct {
  vec3 position;
  vec3 normal;
  vec3 tangent;
  vec3 binormal;
  vec4 color;
  vec2 uvs;
} vertex;

vertex vertex_new();
bool vertex_equal(vertex v1, vertex v2);
void vertex_print(vertex v);

/* Mesh */

typedef struct {
  int num_verts;
  int num_triangles;
  vertex* verticies;
  uint32_t* triangles;
} mesh;

mesh* mesh_new();
void mesh_delete(mesh* m);

void mesh_generate_normals(mesh* m);
void mesh_generate_tangents(mesh* m);
void mesh_generate_orthagonal_tangents(mesh* m);
void mesh_generate_texcoords_cylinder(mesh* m);

void mesh_print(mesh* m);
float mesh_surface_area(mesh* m);

void mesh_transform(mesh* m, mat4 transform);
void mesh_translate(mesh* m, vec3 translation);
void mesh_scale(mesh* m, float scale);

sphere mesh_bounding_sphere(mesh* m);

*/

/*.cpp
vertex vertex_new() {
  vertex v;
  memset(&v, 0, sizeof(vertex));
  return v;
}
*/

pub fn vertex_new() -> Vertex {
  return Vertex {
    position: Vec3::zero(),
    normal: Vec3::zero(),
    tangent: Vec3::zero(),
    binormal: Vec3::zero(),
    color: Vec4::zero(),
    uvs: Vec2::zero(),
  };
}

/*

bool vertex_equal(vertex v1, vertex v2) {
  
  if(!vec3_equ(v1.position, v2.position)) { return false; }
  if(!vec3_equ(v1.normal, v2.normal)) { return false; }
  if(!vec2_equ(v1.uvs, v2.uvs)) { return false; }
  
  return true;  
}

*/

pub fn vertex_equal(v1: &Vertex, v2: &Vertex) -> bool {
  return v1.position == v2.position && v1.normal == v2.normal && v1.uvs == v2.uvs;
}

/*

void vertex_print(vertex v) {

  printf("V(Position: "); vec3_print(v.position);
  printf(", Normal: "); vec3_print(v.normal);
  printf(", Tangent: "); vec3_print(v.tangent);
  printf(", Binormal: "); vec3_print(v.binormal);
  printf(", Color: "); vec4_print(v.color);
  printf(", Uvs: "); vec2_print(v.uvs);
  printf(")");
  
}
*/

pub fn vertex_print(v: &Vertex) {
  print!("V(Position: ");
  vec3_print(&v.position);
  print!(", Normal: ");
  vec3_print(&v.normal);
  print!(", Tangent: ");
  vec3_print(&v.tangent);
  print!(", Binormal: ");
  vec3_print(&v.binormal);
  print!(", Color: ");
  vec4_print(&v.color);
  print!(", Uvs: ");
  vec2_print(&v.uvs);
  print!(")");
}

/*
void mesh_print(mesh* m) {
  printf("Num Verts: %i\n", m->num_verts);
  printf("Num Tris: %i\n", m->num_triangles);
  for(int i=0; i < m->num_verts; i++) {
    vertex_print(m->verticies[i]); printf("\n");
  }
  printf("Triangle Indicies:");
  for(int i=0; i < m->num_triangles * 3; i++) {
    printf("%i ", m->triangles[i]);
  }
  printf("\n");
}
*/

pub fn mesh_print(m: &Mesh) {
  println!("Num Verts: {}", m.vertices.len());
  println!("Num Tris: {}", m.triangles.len());
  for ref vert in m.vertices.iter() {
    vertex_print(vert);
    println!();
  }
  print!("Triangle Indicies:");
  for ref i in m.triangles.iter() {
    print!("{} ", i);
  }
  println!();
}

/*
mesh* mesh_new() {
  
  mesh* m = malloc(sizeof(mesh));
  m->num_verts = 0;
  m->num_triangles = 0;
  m->verticies = malloc(sizeof(vertex) * m->num_verts);
  m->triangles = malloc(sizeof(int) * m->num_triangles * 3);
  
  return m;
  
}
*/

pub fn mesh_new() -> Mesh {
  return Mesh {
    vertices: Vec::new(),
    triangles: Vec::new(),
  };
}

/*

void mesh_delete(mesh* m) {
  free(m->verticies);
  free(m->triangles);
  free(m);
}

*/

pub fn mesh_delete(_: Mesh) {}

/*

void mesh_generate_tangents(mesh* m) {
  
  /* Clear all tangents to 0,0,0 */
  for(int i = 0; i < m->num_verts; i++) {
    m->verticies[i].tangent = vec3_zero();
    m->verticies[i].binormal = vec3_zero();
  }
  
  /* Loop over faces, calculate tangent and append to verticies of that face */
  int i = 0;
  while( i < m->num_triangles * 3) {
    
    int t_i1 = m->triangles[i];
    int t_i2 = m->triangles[i+1];
    int t_i3 = m->triangles[i+2];
    
    vertex v1 = m->verticies[t_i1];
    vertex v2 = m->verticies[t_i2];
    vertex v3 = m->verticies[t_i3];
    
    vec3 face_tangent = triangle_tangent(v1, v2, v3);
    vec3 face_binormal = triangle_binormal(v1, v2, v3);
    
    v1.tangent = vec3_add(face_tangent, v1.tangent);
    v2.tangent = vec3_add(face_tangent, v2.tangent);
    v3.tangent = vec3_add(face_tangent, v3.tangent);
    
    v1.binormal = vec3_add(face_binormal, v1.binormal);
    v2.binormal = vec3_add(face_binormal, v2.binormal);
    v3.binormal = vec3_add(face_binormal, v3.binormal);
    
    m->verticies[t_i1] = v1;
    m->verticies[t_i2] = v2;
    m->verticies[t_i3] = v3;
    
    i = i + 3;
  }
  
  /* normalize all tangents */
  for(int i = 0; i < m->num_verts; i++) {
    m->verticies[i].tangent = vec3_normalize( m->verticies[i].tangent );
    m->verticies[i].binormal = vec3_normalize( m->verticies[i].binormal );
  }
  
}
*/

pub fn mesh_generate_tangents(m: &mut Mesh) {
  //Clear all tangents to 0,0,0
  for ref mut vertex in m.vertices.iter_mut() {
    vertex.tangent = Vec3::zero();
    vertex.binormal = Vec3::zero();
  }

  /* Loop over faces, calculate tangent and append to verticies of that face */
  //loop over faces, calculate normals and append to verticies of that face
  for i in 0..(m.triangles.len() / 3) {
    //should I add num_triangles back in?
    let t_i1 = m.triangles[i * 3];
    let t_i2 = m.triangles[i * 3 + 1];
    let t_i3 = m.triangles[i * 3 + 2];

    let (face_tangent, face_binormal) = {
      let v1 = &m.vertices[t_i1 as usize]; //cannot take a mut ref to these things because it thinks the whole array is mut refd
      let v2 = &m.vertices[t_i2 as usize];
      let v3 = &m.vertices[t_i3 as usize];
      let face_tangent = triangle_tangent(&v1, &v2, &v3);
      let face_binormal = triangle_binormal(&v1, &v2, &v3);
      (face_tangent, face_binormal)
    };

    m.vertices[t_i1 as usize].tangent = m.vertices[t_i1 as usize].tangent.add(face_tangent);
    m.vertices[t_i2 as usize].tangent = m.vertices[t_i2 as usize].tangent.add(face_tangent);
    m.vertices[t_i3 as usize].tangent = m.vertices[t_i3 as usize].tangent.add(face_tangent);


    m.vertices[t_i1 as usize].binormal = m.vertices[t_i1 as usize].binormal.add(face_binormal);
    m.vertices[t_i2 as usize].binormal = m.vertices[t_i2 as usize].binormal.add(face_binormal);
    m.vertices[t_i3 as usize].binormal = m.vertices[t_i3 as usize].binormal.add(face_binormal);
  }

  // normalize all tangents
  for vertex in m.vertices.iter_mut() {
    vertex.tangent = vertex.tangent.normalize();
    vertex.binormal = vertex.binormal.normalize();
  }
}

/*
void mesh_generate_normals(mesh* m) {
  
  /* Clear all normals to 0,0,0 */
  for(int i = 0; i < m->num_verts; i++) {
    m->verticies[i].normal = vec3_zero();
  }
  
  /* Loop over faces, calculate normals and append to verticies of that face */
  int i = 0;
  while( i < m->num_triangles * 3) {
    
    int t_i1 = m->triangles[i];
    int t_i2 = m->triangles[i+1];
    int t_i3 = m->triangles[i+2];
    
    vertex v1 = m->verticies[t_i1];
    vertex v2 = m->verticies[t_i2];
    vertex v3 = m->verticies[t_i3];
    
    vec3 face_normal = triangle_normal(v1, v2, v3);
    
    v1.normal = vec3_add(face_normal, v1.normal);
    v2.normal = vec3_add(face_normal, v2.normal);
    v3.normal = vec3_add(face_normal, v3.normal);
    
    m->verticies[t_i1] = v1;
    m->verticies[t_i2] = v2;
    m->verticies[t_i3] = v3;
    
    i = i + 3;
  }
  
  /* normalize all normals */
  for(int i = 0; i < m->num_verts; i++) {
    m->verticies[i].normal = vec3_normalize( m->verticies[i].normal );
  }
  
}
*/

pub fn mesh_generate_normals(m: &mut Mesh) {

  //clear out all normals
  for ref mut vertex in m.vertices.iter_mut() {
    vertex.normal = Vec3::zero();
  }

  //loop over faces, calculate normals and append to verticies of that face
  for i in 0..(m.triangles.len() / 3) {
    //should I add num_triangles back in?
    let t_i1 = m.triangles[i * 3];
    let t_i2 = m.triangles[i * 3 + 1];
    let t_i3 = m.triangles[i * 3 + 2];

    let face_normal = {
      let v1 = &m.vertices[t_i1 as usize]; //cannot take a mut ref to these things because it thinks the whole array is mut refd
      let v2 = &m.vertices[t_i2 as usize];
      let v3 = &m.vertices[t_i3 as usize];
      triangle_normal(&v1, &v2, &v3)
    };

    m.vertices[t_i1 as usize].normal = m.vertices[t_i1 as usize].normal.add(face_normal);
    m.vertices[t_i2 as usize].normal = m.vertices[t_i2 as usize].normal.add(face_normal);
    m.vertices[t_i3 as usize].normal = m.vertices[t_i3 as usize].normal.add(face_normal);
  }

  //normalize all normals
  for vertex in m.vertices.iter_mut() {
    vertex.normal = vertex.normal.normalize();
  }
}

/*
void mesh_generate_orthagonal_tangents(mesh* m) {
  
  /* Clear all tangents to 0,0,0 */
  for(int i = 0; i < m->num_verts; i++) {
    m->verticies[i].tangent = vec3_zero();
    m->verticies[i].binormal = vec3_zero();
  }
  
  /* Loop over faces, calculate tangent and append to verticies of that face */
  int i = 0;
  while( i < m->num_triangles * 3) {
    
    int t_i1 = m->triangles[i];
    int t_i2 = m->triangles[i+1];
    int t_i3 = m->triangles[i+2];
    
    vertex v1 = m->verticies[t_i1];
    vertex v2 = m->verticies[t_i2];
    vertex v3 = m->verticies[t_i3];
    
    vec3 face_normal = triangle_normal(v1, v2, v3);    
    vec3 face_binormal_temp = triangle_binormal(v1, v2, v3);
    
    vec3 face_tangent = vec3_normalize( vec3_cross(face_binormal_temp, face_normal) );
    vec3 face_binormal = vec3_normalize( vec3_cross(face_tangent, face_normal) );
    
    v1.tangent = vec3_add(face_tangent, v1.tangent);
    v2.tangent = vec3_add(face_tangent, v2.tangent);
    v3.tangent = vec3_add(face_tangent, v3.tangent);
    
    v1.binormal = vec3_add(face_binormal, v1.binormal);
    v2.binormal = vec3_add(face_binormal, v2.binormal);
    v3.binormal = vec3_add(face_binormal, v3.binormal);
    
    m->verticies[t_i1] = v1;
    m->verticies[t_i2] = v2;
    m->verticies[t_i3] = v3;
    
    i = i + 3;
  }
  
  /* normalize all tangents */
  for(int i = 0; i < m->num_verts; i++) {
    m->verticies[i].tangent = vec3_normalize( m->verticies[i].tangent );
    m->verticies[i].binormal = vec3_normalize( m->verticies[i].binormal );
  }
}
*/

/*
void mesh_generate_texcoords_cylinder(mesh* m) {
  
	vec2 unwrap_vector = vec2_new(1, 0);
	
	float max_height = -99999999;
	float min_height = 99999999;
	
	for(int i = 0; i < m->num_verts; i++) {
		float v = m->verticies[i].position.y;
		max_height = max(max_height, v);
		min_height = min(min_height, v);
		
		vec2 proj_position = vec2_new(m->verticies[i].position.x, m->verticies[i].position.z);
		vec2 from_center = vec2_normalize(proj_position);
		float u = (vec2_dot(from_center, unwrap_vector) + 1) / 8;
		
		m->verticies[i].uvs = vec2_new(u, v);
	}
	
	float scale = (max_height - min_height);
	
	for(int i = 0; i < m->num_verts; i++) {
		m->verticies[i].uvs = vec2_new(m->verticies[i].uvs.x, m->verticies[i].uvs.y / scale);
	}
  
}

*/

pub fn mesh_generate_texcoords_cylinder(mesh: &mut Mesh) {
  //TODO should exit early here for no vertices
  let unwrap_vector = Vec2::new(1.0, 0.0);

  let mut max_height = f32::MIN;
  let mut min_height = f32::MAX;

  for vertex in mesh.vertices.iter_mut() {
    let v = vertex.position.y;
    max_height = max_height.max(v);
    min_height = min_height.min(v);

    let proj_position = Vec2::new(vertex.position.x, vertex.position.z);
    let from_center = proj_position.normalize();
    let u = (from_center.dot(unwrap_vector) + 1.0) / 8.0;

    vertex.uvs = Vec2::new(u, v);
  }

  let scale = max_height - min_height;

  for vertex in mesh.vertices.iter_mut() {
    vertex.uvs.y = vertex.uvs.y / scale;
  }
}

/*

float mesh_surface_area(mesh* m) {
  
  float total = 0.0;
  
  int i = 0;
  while( i < m->num_triangles * 3) {
  
    int t_i1 = m->triangles[i];
    int t_i2 = m->triangles[i+1];
    int t_i3 = m->triangles[i+2];

    vertex v1 = m->verticies[t_i1];
    vertex v2 = m->verticies[t_i2];
    vertex v3 = m->verticies[t_i3];
    
    total += triangle_area(v1, v2, v3);
    
    i = i + 3;
  }
  
  return total;
  
}

void mesh_translate(mesh* m, vec3 translation) {

  int i = 0;
  while(i < m->num_triangles * 3) {
  
    int t_i1 = m->triangles[i];
    int t_i2 = m->triangles[i+1];
    int t_i3 = m->triangles[i+2];
    
    m->verticies[t_i1].position = vec3_add(m->verticies[t_i1].position, translation);
    m->verticies[t_i2].position = vec3_add(m->verticies[t_i2].position, translation);
    m->verticies[t_i3].position = vec3_add(m->verticies[t_i3].position, translation);
    
    i = i + 3;
  }

}

void mesh_scale(mesh* m, float scale) {

  int i = 0;
  while(i < m->num_triangles * 3) {
  
    int t_i1 = m->triangles[i];
    int t_i2 = m->triangles[i+1];
    int t_i3 = m->triangles[i+2];
    
    m->verticies[t_i1].position = vec3_mul(m->verticies[t_i1].position, scale);
    m->verticies[t_i2].position = vec3_mul(m->verticies[t_i2].position, scale);
    m->verticies[t_i3].position = vec3_mul(m->verticies[t_i3].position, scale);
    
    i = i + 3;
  }

}

void mesh_transform(mesh* m, mat4 transform) {

  int i = 0;
  while(i < m->num_triangles * 3) {
  
    int t_i1 = m->triangles[i];
    int t_i2 = m->triangles[i+1];
    int t_i3 = m->triangles[i+2];
    
    m->verticies[t_i1].position = mat4_mul_vec3(transform, m->verticies[t_i1].position);
    m->verticies[t_i2].position = mat4_mul_vec3(transform, m->verticies[t_i2].position);
    m->verticies[t_i3].position = mat4_mul_vec3(transform, m->verticies[t_i3].position);
    
    i = i + 3;
  }

}
*/

/*
sphere mesh_bounding_sphere(mesh* m) {
  
  sphere s = sphere_new(vec3_zero(), 0);
  
  for (int i = 0; i < m->num_verts; i++) {
    s.center = vec3_add(s.center, m->verticies[i].position);
  }
  s.center = vec3_div(s.center, m->num_verts);
  
  for (int i = 0; i < m->num_verts; i++) {
    s.radius = max(s.radius, vec3_dist(s.center, m->verticies[i].position));
  }
  
  return s;
}

*/

pub fn mesh_bounding_sphere(m: &Mesh) -> Sphere {
  let mut s = Sphere {
    center: Vec3::zero(),
    radius: 0.0,
  };

  for i in 0..m.vertices.len() {
    s.center = s.center.add(m.vertices[i].position);
  }

  s.center = s.center.div(m.vertices.len() as f32);

  for i in 0..m.vertices.len() {
    s.radius = s.radius.max(s.center.distance(m.vertices[i].position));
  }

  return s;
}

//model after mesh
/*
void model_print(model* m) {
  for(int i=0; i<m->num_meshes; i++) {
    mesh_print( m->meshes[i] );
  }
}
*/

pub fn model_print(m: &Model) {
  for ref mesh in m.meshes.iter() {
    mesh_print(mesh);
  }
}

/*

model* model_new() {
  model* m = malloc(sizeof(model));
  m->num_meshes = 0;
  m->meshes = malloc(sizeof(mesh*) * m->num_meshes);
  return m;
}
*/

pub fn model_new() -> Model {
  return Model { meshes: Vec::new() };
}

/*
void model_delete(model* m) {
  for(int i=0; i<m->num_meshes; i++) {
    mesh_delete( m->meshes[i] );
  }
  free(m);
}
*/

pub fn model_delete(mut m: Model) {
  while m.meshes.len() > 0 {
    mesh_delete(m.meshes.pop().unwrap());
  }
}

/*
void model_generate_normals(model* m) {

  for(int i = 0; i < m->num_meshes; i++) {
    mesh_generate_normals( m->meshes[i] );
  }
  
}
*/

pub fn model_generate_normals(m: &mut Model) {
  for ref mut mesh in m.meshes.iter_mut() {
    mesh_generate_normals(mesh);
  }
}

/*
void model_generate_tangents(model* m) {

  for(int i = 0; i < m->num_meshes; i++) {
    mesh_generate_tangents( m->meshes[i] );
  }

}
*/

pub fn model_generate_tangents(m: &mut Model) {
  for ref mut mesh in m.meshes.iter_mut() {
    mesh_generate_tangents(mesh);
  }
}

/*

void model_generate_orthagonal_tangents(model* m) {
  for(int i = 0; i < m->num_meshes; i++) {
    mesh_generate_orthagonal_tangents( m->meshes[i] );
  }
}

void model_generate_texcoords_cylinder(model* m) {
  for(int i = 0; i < m->num_meshes; i++) {
    mesh_generate_texcoords_cylinder( m->meshes[i] );
  }
}

*/

pub fn model_generate_texcoords_cylinder(m: &mut Model) {
  for ref mut mesh in m.meshes.iter_mut() {
    mesh_generate_texcoords_cylinder(mesh);
  }
}

/*

float model_surface_area(model* m) {
  float total = 0.0f;
  
  for(int i = 0; i < m->num_meshes; i++) {
    total += mesh_surface_area( m->meshes[i] );
  }
  
  return total;
}

void model_translate(model* m, vec3 translation) {
  for(int i = 0; i < m->num_meshes; i++) {
    mesh_translate(m->meshes[i], translation);
  }
}

void model_scale(model* m, float scale) {
  for(int i = 0; i < m->num_meshes; i++) {
    mesh_scale(m->meshes[i], scale);
  }
}

void model_transform(model* m, mat4 transform) {
  for(int i = 0; i < m->num_meshes; i++) {
    mesh_transform(m->meshes[i], transform);
  }
}
*/

//triangle after model

/*
vec3 triangle_tangent(vertex vert1, vertex vert2, vertex vert3) {
  
  vec3 pos1 = vert1.position;
  vec3 pos2 = vert2.position;
  vec3 pos3 = vert3.position;
  
  vec2 uv1 = vert1.uvs;
  vec2 uv2 = vert2.uvs;
  vec2 uv3 = vert3.uvs;
  
  /* Get component vectors */
  float x1 = pos2.x - pos1.x;
  float x2 = pos3.x - pos1.x;
  
  float y1 = pos2.y - pos1.y;
  float y2 = pos3.y - pos1.y;

  float z1 = pos2.z - pos1.z;
  float z2 = pos3.z - pos1.z;
  
  /* Generate uv space vectors */
  float s1 = uv2.x - uv1.x;
  float s2 = uv3.x - uv1.x;

  float t1 = uv2.y - uv1.y;
  float t2 = uv3.y - uv1.y;
  
  float r = 1.0f / ((s1 * t2) - (s2 * t1));
  
  vec3 tdir = vec3_new(
    (s1 * x2 - s2 * x1) * r, 
    (s1 * y2 - s2 * y1) * r,
    (s1 * z2 - s2 * z1) * r);
  
  return vec3_normalize(tdir);

}
*/

pub fn triangle_tangent(vert1: &Vertex, vert2: &Vertex, vert3: &Vertex) -> Vec3 {
  let pos1 = vert1.position;
  let pos2 = vert2.position;
  let pos3 = vert3.position;

  let uv1 = vert1.uvs;
  let uv2 = vert2.uvs;
  let uv3 = vert3.uvs;

  let x1 = pos2.x - pos1.x;
  let x2 = pos3.x - pos1.x;

  let y1 = pos2.y - pos1.y;
  let y2 = pos3.y - pos1.y;

  let z1 = pos2.z - pos1.z;
  let z2 = pos3.z - pos1.z;

  let s1 = uv2.x - uv1.x;
  let s2 = uv3.x - uv1.x;

  let t1 = uv2.y - uv1.y;
  let t2 = uv3.y - uv1.y;

  let r = 1.0 / ((s1 * t2) - (s2 * t1));

  let mut tdir = Vec3::new(
    (s1 * x2 - s2 * x1) * r,
    (s1 * y2 - s2 * y1) * r,
    (s1 * z2 - s2 * z1) * r,
  );
  tdir = tdir.normalize();
  return tdir;
}

/*

vec3 triangle_binormal(vertex vert1, vertex vert2, vertex vert3) {
  
  vec3 pos1 = vert1.position;
  vec3 pos2 = vert2.position;
  vec3 pos3 = vert3.position;
  
  vec2 uv1 = vert1.uvs;
  vec2 uv2 = vert2.uvs;
  vec2 uv3 = vert3.uvs;
  
  /* Get component Vectors */
  float x1 = pos2.x - pos1.x;
  float x2 = pos3.x - pos1.x;
  
  float y1 = pos2.y - pos1.y;
  float y2 = pos3.y - pos1.y;

  float z1 = pos2.z - pos1.z;
  float z2 = pos3.z - pos1.z;
  
  /* Generate uv space vectors */
  float s1 = uv2.x - uv1.x;
  float s2 = uv3.x - uv1.x;

  float t1 = uv2.y - uv1.y;
  float t2 = uv3.y - uv1.y;
  
  float r = 1.0f / ((s1 * t2) - (s2 * t1));
  
  vec3 sdir = vec3_new(
          (t2 * x1 - t1 * x2) * r, 
          (t2 * y1 - t1 * y2) * r,
          (t2 * z1 - t1 * z2) * r
          );
  
  return vec3_normalize(sdir);

}
*/

pub fn triangle_binormal(vert1: &Vertex, vert2: &Vertex, vert3: &Vertex) -> Vec3 {
  let pos1 = vert1.position;
  let pos2 = vert2.position;
  let pos3 = vert3.position;

  let uv1 = vert1.uvs;
  let uv2 = vert2.uvs;
  let uv3 = vert3.uvs;

  let x1 = pos2.x - pos1.x;
  let x2 = pos3.x - pos1.x;

  let y1 = pos2.y - pos1.y;
  let y2 = pos3.y - pos1.y;

  let z1 = pos2.z - pos1.z;
  let z2 = pos3.z - pos1.z;

  let s1 = uv2.x - uv1.x;
  let s2 = uv3.x - uv1.x;

  let t1 = uv2.y - uv1.y;
  let t2 = uv3.y - uv1.y;

  let r = 1.0 / ((s1 * t2) - (s2 * t1));

  let mut sdir = Vec3::new(
    (t2 * x1 - t1 * x2) * r,
    (t2 * y1 - t1 * y2) * r,
    (t2 * z1 - t1 * z2) * r,
  );
  sdir = sdir.normalize();
  return sdir;
}

/*
vec3 triangle_normal(vertex v1, vertex v2, vertex v3) {
  vec3 edge1 = vec3_sub(v2.position, v1.position);
  vec3 edge2 = vec3_sub(v3.position, v1.position);
  vec3 normal = vec3_cross(edge1, edge2);
  
  return vec3_normalize(normal);
}
*/

pub fn triangle_normal(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec3 {
  let edge1 = v2.position.sub(v1.position);
  let edge2 = v3.position.sub(v1.position);
  let normal = edge1.cross(edge2).normalize();

  return Vec3::new(normal.x, normal.y, normal.z);
}

/*
float triangle_area(vertex v1, vertex v2, vertex v3) {
  
  vec3 ab = vec3_sub(v1.position, v2.position);
  vec3 ac = vec3_sub(v1.position, v3.position);
  
  float area = 0.5 * vec3_length(vec3_cross(ab, ac));
  
  return area;
}

vec3 triangle_random_position(vertex v1, vertex v2, vertex v3) {
  
  float r1 = (float)rand() / (float)RAND_MAX;
  float r2 = (float)rand() / (float)RAND_MAX;
  
  if(r1 + r2 >= 1) {
    r1 = 1 - r1;
    r2 = 1 - r2;
  }
  
  vec3 ab = vec3_sub(v1.position, v2.position);
  vec3 ac = vec3_sub(v1.position, v3.position);
  
  vec3 a = v1.position;
  a = vec3_sub(a, vec3_mul(ab , r1) );
  a = vec3_sub(a, vec3_mul(ac , r2) );
  
  return a;
  
}

vertex triangle_random_position_interpolation(vertex v1, vertex v2, vertex v3) {

  float r1 = (float)rand() / (float)RAND_MAX;
  float r2 = (float)rand() / (float)RAND_MAX;
  
  if(r1 + r2 >= 1) {
    r1 = 1 - r1;
    r2 = 1 - r2;
  }
  
  vertex v;
  
  vec3 v_pos, v_norm, v_tang, v_binorm;
  vec4 v_col;
  vec2 v_uv;
  
  v_pos = v1.position;
  v_pos = vec3_sub(v_pos, vec3_mul(vec3_sub(v1.position, v2.position) , r1) );
  v_pos = vec3_sub(v_pos, vec3_mul(vec3_sub(v1.position, v3.position) , r2) );
  
  v_norm = v1.normal;
  v_norm = vec3_sub(v_norm, vec3_mul(vec3_sub(v1.normal, v2.normal) , r1) );
  v_norm = vec3_sub(v_norm, vec3_mul(vec3_sub(v1.normal, v3.normal) , r2) );
  
  v_tang = v1.tangent;
  v_tang = vec3_sub(v_tang, vec3_mul(vec3_sub(v1.tangent, v2.tangent) , r1) );
  v_tang = vec3_sub(v_tang, vec3_mul(vec3_sub(v1.tangent, v3.tangent) , r2) );
  
  v_binorm = v1.binormal;
  v_binorm = vec3_sub(v_binorm, vec3_mul(vec3_sub(v1.binormal, v2.binormal) , r1) );
  v_binorm = vec3_sub(v_binorm, vec3_mul(vec3_sub(v1.binormal, v3.binormal) , r2) );
  
  v_col = v1.color;
  v_col = vec4_sub(v_col, vec4_mul(vec4_sub(v1.color, v2.color) , r1) );
  v_col = vec4_sub(v_col, vec4_mul(vec4_sub(v1.color, v3.color)  , r2) );
  
  v_uv = v1.uvs;
  v_uv = vec2_sub(v_uv, vec2_mul(vec2_sub(v1.uvs, v2.uvs) , r1) );
  v_uv = vec2_sub(v_uv, vec2_mul(vec2_sub(v1.uvs, v3.uvs)  , r2) );
  
  v.position = v_pos;
  v.normal = v_norm;
  v.tangent = v_tang;
  v.binormal = v_binorm;
  v.color = v_col;
  v.uvs = v_uv;
  
  return v;
}


float triangle_difference_u(vertex v1, vertex v2, vertex v3) {
  float max = v1.uvs.x;
  max = v2.uvs.x > max ? v2.uvs.x : max;
  max = v3.uvs.x > max ? v3.uvs.x : max;
  
  float min = v1.uvs.x;
  min = v2.uvs.x < min ? v2.uvs.x : min;
  min = v3.uvs.x < min ? v3.uvs.x : min;
  
  return max - min;
}

float triangle_difference_v(vertex v1, vertex v2, vertex v3) {
  float max = v1.uvs.y;
  max = v2.uvs.x > max ? v2.uvs.y : max;
  max = v3.uvs.x > max ? v3.uvs.y : max;
  
  float min = v1.uvs.y;
  min = v2.uvs.y < min ? v2.uvs.y : min;
  min = v3.uvs.y < min ? v3.uvs.y : min;
  
  return max - min;
}
*/

/* Matrix 3x3 */
/*
mat3 mat3_rotation_y(float a)
{

  mat3 m = mat3_id();

  m.xx = cos(a);
  m.xz = sin(a);
  m.zx = -sin(a);
  m.zz = cos(a);

  return m;
}
*/

pub fn mat3_rotation_y(a: f32) -> Basis3<f32> {
  let m: Basis3<f32> = Rotation3::from_angle_y(Rad(a)); //let mut m = Mat3::identity();
  //let axis = Vec3::new(0.0, 1.0, 0.0).normalize();
  //let m:Basis3<f32> = Rotation3::from_axis_angle(axis, Deg(a));
  /*m[0][0] = a.cos();
  m[0][2] = a.sin();
  m[2][0] = -a.sin();
  m[2][2] = a.cos();
*/
  return m;
}

/*
mat3 mat3_rotation_angle_axis(float a, vec3 v)
{

  mat3 m;

  float c = cos(a);
  float s = sin(a);
  float nc = 1 - c;

  m.xx = v.x * v.x * nc + c;
  m.xy = v.x * v.y * nc - v.z * s;
  m.xz = v.x * v.z * nc + v.y * s;

  m.yx = v.y * v.x * nc + v.z * s;
  m.yy = v.y * v.y * nc + c;
  m.yz = v.y * v.z * nc - v.x * s;

  m.zx = v.z * v.x * nc - v.y * s;
  m.zy = v.z * v.y * nc + v.x * s;
  m.zz = v.z * v.z * nc + c;

  return m;
}
*/

pub fn mat3_rotation_angle_axis(a: f32, v: Vec3) -> Basis3<f32> {
  //let mut m = Mat3::identity(); //TODO new??
  let m: Basis3<f32> = Rotation3::from_axis_angle(v, Rad(a));
  /*
  let c = a.cos();
  let s = a.sin();
  let nc = 1.0 - c;

  m[0][0] = v.x * v.x * nc + c;
  m[0][1] = v.x * v.y * nc - v.z * s;
  m[0][2] = v.x * v.z * nc + v.y * s;

  m[1][0] = v.y * v.x * nc + v.z * s;
  m[1][1] = v.y * v.y * nc + c;
  m[1][2] = v.y * v.z * nc - v.x * s;

  m[2][0] = v.z * v.x * nc - v.y * s;
  m[2][1] = v.z * v.y * nc + v.x * s;
  m[2][2] = v.z * v.z * nc + c;
*/
  return m;
}



/*
/* Matrix 4x4 */

mat4 mat4_zero() {
  mat4 mat;
  
  mat.xx = 0.0f;
  mat.xy = 0.0f;
  mat.xz = 0.0f;
  mat.xw = 0.0f;
  
  mat.yx = 0.0f;
  mat.yy = 0.0f;
  mat.yz = 0.0f;
  mat.yw = 0.0f;
  
  mat.zx = 0.0f;
  mat.zy = 0.0f;
  mat.zz = 0.0f;
  mat.zw = 0.0f;
  
  mat.wx = 0.0f;
  mat.wy = 0.0f;
  mat.wz = 0.0f;
  mat.ww = 0.0f;
  
  return mat;
}

mat4 mat4_id(){
  
  mat4 mat = mat4_zero();
  
  mat.xx = 1.0f;
  mat.yy = 1.0f;
  mat.zz = 1.0f;
  mat.ww = 1.0f;
  
  
  return mat;
}

float mat4_at(mat4 m, int x, int y) {
  float* arr = (float*)(&m);
  return arr[x + (y*4)];  
}

mat4 mat4_set(mat4 m, int x, int y, float v) {
  
  float* arr = (float*)(&m);
  arr[x + (y*4)] = v;
  
  return m;
}

mat4 mat4_new(float xx, float xy, float xz, float xw,
              float yx, float yy, float yz, float yw,
              float zx, float zy, float zz, float zw,
              float wx, float wy, float wz, float ww) {
         
  mat4 mat;
  
  mat.xx = xx;
  mat.xy = xy;
  mat.xz = xz;
  mat.xw = xw;
  
  mat.yx = yx;
  mat.yy = yy;
  mat.yz = yz;
  mat.yw = yw;
  
  mat.zx = zx;
  mat.zy = zy;
  mat.zz = zz;
  mat.zw = zw;
  
  mat.wx = wx;
  mat.wy = wy;
  mat.wz = wz;
  mat.ww = ww;
  
  return mat;
}

mat4 mat4_transpose(mat4 m) {
  mat4 mat;
  
  mat.xx = m.xx;
  mat.xy = m.yx;
  mat.xz = m.zx;
  mat.xw = m.wx;
  
  mat.yx = m.xy;
  mat.yy = m.yy;
  mat.yz = m.zy;
  mat.yw = m.wy;
  
  mat.zx = m.xz;
  mat.zy = m.yz;
  mat.zz = m.zz;
  mat.zw = m.wz;
  
  mat.wx = m.xw;
  mat.wy = m.yw;
  mat.wz = m.zw;
  mat.ww = m.ww;
  
  return mat;
}

mat4 mat3_to_mat4(mat3 m) {

  mat4 mat;
  
  mat.xx = m.xx;
  mat.xy = m.xy;
  mat.xz = m.xz;
  mat.xw = 0.0f;
  
  mat.yx = m.yx;
  mat.yy = m.yy;
  mat.yz = m.yz;
  mat.yw = 0.0f;
  
  mat.zx = m.zx;
  mat.zy = m.zy;
  mat.zz = m.zz;
  mat.zw = 0.0f;
  
  mat.wx = 0.0f;
  mat.wy = 0.0f;
  mat.wz = 0.0f;
  mat.ww = 1.0f;
  
  return mat;
}

mat4 mat4_mul_mat4(mat4 m1, mat4 m2) {

  mat4 mat;

  mat.xx = (m1.xx * m2.xx) + (m1.xy * m2.yx) + (m1.xz * m2.zx) + (m1.xw * m2.wx);
  mat.xy = (m1.xx * m2.xy) + (m1.xy * m2.yy) + (m1.xz * m2.zy) + (m1.xw * m2.wy);
  mat.xz = (m1.xx * m2.xz) + (m1.xy * m2.yz) + (m1.xz * m2.zz) + (m1.xw * m2.wz);
  mat.xw = (m1.xx * m2.xw) + (m1.xy * m2.yw) + (m1.xz * m2.zw) + (m1.xw * m2.ww);
  
  mat.yx = (m1.yx * m2.xx) + (m1.yy * m2.yx) + (m1.yz * m2.zx) + (m1.yw * m2.wx);
  mat.yy = (m1.yx * m2.xy) + (m1.yy * m2.yy) + (m1.yz * m2.zy) + (m1.yw * m2.wy);
  mat.yz = (m1.yx * m2.xz) + (m1.yy * m2.yz) + (m1.yz * m2.zz) + (m1.yw * m2.wz);
  mat.yw = (m1.yx * m2.xw) + (m1.yy * m2.yw) + (m1.yz * m2.zw) + (m1.yw * m2.ww);
 
  mat.zx = (m1.zx * m2.xx) + (m1.zy * m2.yx) + (m1.zz * m2.zx) + (m1.zw * m2.wx);
  mat.zy = (m1.zx * m2.xy) + (m1.zy * m2.yy) + (m1.zz * m2.zy) + (m1.zw * m2.wy);
  mat.zz = (m1.zx * m2.xz) + (m1.zy * m2.yz) + (m1.zz * m2.zz) + (m1.zw * m2.wz);
  mat.zw = (m1.zx * m2.xw) + (m1.zy * m2.yw) + (m1.zz * m2.zw) + (m1.zw * m2.ww);
  
  mat.wx = (m1.wx * m2.xx) + (m1.wy * m2.yx) + (m1.wz * m2.zx) + (m1.ww * m2.wx);
  mat.wy = (m1.wx * m2.xy) + (m1.wy * m2.yy) + (m1.wz * m2.zy) + (m1.ww * m2.wy);
  mat.wz = (m1.wx * m2.xz) + (m1.wy * m2.yz) + (m1.wz * m2.zz) + (m1.ww * m2.wz);
  mat.ww = (m1.wx * m2.xw) + (m1.wy * m2.yw) + (m1.wz * m2.zw) + (m1.ww * m2.ww);
  
  return mat;
  
}

vec4 mat4_mul_vec4(mat4 m, vec4 v) {
  
  vec4 vec;
  
  vec.x = (m.xx * v.x) + (m.xy * v.y) + (m.xz * v.z) + (m.xw * v.w);
  vec.y = (m.yx * v.x) + (m.yy * v.y) + (m.yz * v.z) + (m.yw * v.w);
  vec.z = (m.zx * v.x) + (m.zy * v.y) + (m.zz * v.z) + (m.zw * v.w);
  vec.w = (m.wx * v.x) + (m.wy * v.y) + (m.wz * v.z) + (m.ww * v.w);
  
  return vec;
}

vec3 mat4_mul_vec3(mat4 m, vec3 v) {
  
  vec4 v_homo = vec4_new(v.x, v.y, v.z, 1);
  v_homo = mat4_mul_vec4(m, v_homo);
  
  v_homo = vec4_div(v_homo, v_homo.w);
  
  return vec3_new(v_homo.x, v_homo.y, v_homo.z);
}

mat3 mat4_to_mat3(mat4 m) {

  mat3 mat;
  
  mat.xx = m.xx;
  mat.xy = m.xy;
  mat.xz = m.xz;
  
  mat.yx = m.yx;
  mat.yy = m.yy;
  mat.yz = m.yz;
  
  mat.zx = m.zx;
  mat.zy = m.zy;
  mat.zz = m.zz;
  
  return mat;
  
}

quat mat4_to_quat(mat4 m) {

  float tr = m.xx + m.yy + m.zz;

  if (tr > 0.0f) {
    
    float s = sqrtf( tr + 1.0f );
    
    float w = s / 2.0f;
    float x = ( mat4_at(m, 1, 2) - mat4_at(m, 2, 1) ) * (0.5f / s);
    float y = ( mat4_at(m, 2, 0) - mat4_at(m, 0, 2) ) * (0.5f / s);
    float z = ( mat4_at(m, 0, 1) - mat4_at(m, 1, 0) ) * (0.5f / s);
    return quat_new(x, y, z, w);
    
  } else {
    
    int nxt[3] = {1, 2, 0};
    float q[4];
    int  i, j, k;
    
    i = 0;
    if ( mat4_at(m, 1, 1) > mat4_at(m, 0, 0) ) {	i = 1;	}
    if ( mat4_at(m, 2, 2) > mat4_at(m, i, i) ) {	i = 2;	}
    j = nxt[i];
    k = nxt[j];

    float s = sqrtf( (mat4_at(m, i, i) - (mat4_at(m, j, j) + mat4_at(m, k, k))) + 1.0f );

    q[i] = s * 0.5f;

    if ( s != 0.0f )	{	s = 0.5f / s;	}

    q[3] = ( mat4_at(m, j, k) - mat4_at(m, k, j) ) * s;
    q[j] = ( mat4_at(m, i, j) + mat4_at(m, j, i) ) * s;
    q[k] = ( mat4_at(m, i, k) + mat4_at(m, k, i) ) * s;

    return quat_new(q[0], q[1], q[2], q[3]);
  }

}

quat_dual mat4_to_quat_dual(mat4 m) {
  quat rotation = mat4_to_quat(m);
  vec3 translation = mat4_mul_vec3(m, vec3_zero());
  return quat_dual_transform(rotation, translation);
}

float mat4_det(mat4 m) {
  
  float cofact_xx =  mat3_det(mat3_new(m.yy, m.yz, m.yw, m.zy, m.zz, m.zw, m.wy, m.wz, m.ww));
  float cofact_xy = -mat3_det(mat3_new(m.yx, m.yz, m.yw, m.zx, m.zz, m.zw, m.wx, m.wz, m.ww));
  float cofact_xz =  mat3_det(mat3_new(m.yx, m.yy, m.yw, m.zx, m.zy, m.zw, m.wx, m.wy, m.ww));
  float cofact_xw = -mat3_det(mat3_new(m.yx, m.yy, m.yz, m.zx, m.zy, m.zz, m.wx, m.wy, m.wz));
  
  return (cofact_xx * m.xx) + (cofact_xy * m.xy) + (cofact_xz * m.xz) + (cofact_xw * m.xw);
}

mat4 mat4_inverse(mat4 m) {
    
  float det = mat4_det(m);
  float fac = 1.0 / det;
  
  mat4 ret;
  ret.xx = fac *  mat3_det(mat3_new(m.yy, m.yz, m.yw, m.zy, m.zz, m.zw, m.wy, m.wz, m.ww));
  ret.xy = fac * -mat3_det(mat3_new(m.yx, m.yz, m.yw, m.zx, m.zz, m.zw, m.wx, m.wz, m.ww));
  ret.xz = fac *  mat3_det(mat3_new(m.yx, m.yy, m.yw, m.zx, m.zy, m.zw, m.wx, m.wy, m.ww));
  ret.xw = fac * -mat3_det(mat3_new(m.yx, m.yy, m.yz, m.zx, m.zy, m.zz, m.wx, m.wy, m.wz));
  
  ret.yx = fac * -mat3_det(mat3_new(m.xy, m.xz, m.xw, m.zy, m.zz, m.zw, m.wy, m.wz, m.ww));
  ret.yy = fac *  mat3_det(mat3_new(m.xx, m.xz, m.xw, m.zx, m.zz, m.zw, m.wx, m.wz, m.ww));
  ret.yz = fac * -mat3_det(mat3_new(m.xx, m.xy, m.xw, m.zx, m.zy, m.zw, m.wx, m.wy, m.ww));
  ret.yw = fac *  mat3_det(mat3_new(m.xx, m.xy, m.xz, m.zx, m.zy, m.zz, m.wx, m.wy, m.wz));
  
  ret.zx = fac *  mat3_det(mat3_new(m.xy, m.xz, m.xw, m.yy, m.yz, m.yw, m.wy, m.wz, m.ww));
  ret.zy = fac * -mat3_det(mat3_new(m.xx, m.xz, m.xw, m.yx, m.yz, m.yw, m.wx, m.wz, m.ww));
  ret.zz = fac *  mat3_det(mat3_new(m.xx, m.xy, m.xw, m.yx, m.yy, m.yw, m.wx, m.wy, m.ww));
  ret.zw = fac * -mat3_det(mat3_new(m.xx, m.xy, m.xz, m.yx, m.yy, m.yz, m.wx, m.wy, m.wz));
  
  ret.wx = fac * -mat3_det(mat3_new(m.xy, m.xz, m.xw, m.yy, m.yz, m.yw, m.zy, m.zz, m.zw));
  ret.wy = fac *  mat3_det(mat3_new(m.xx, m.xz, m.xw, m.yx, m.yz, m.yw, m.zx, m.zz, m.zw));
  ret.wz = fac * -mat3_det(mat3_new(m.xx, m.xy, m.xw, m.yx, m.yy, m.yw, m.zx, m.zy, m.zw));
  ret.ww = fac *  mat3_det(mat3_new(m.xx, m.xy, m.xz, m.yx, m.yy, m.yz, m.zx, m.zy, m.zz));
  
  ret = mat4_transpose(ret);
  
  return ret;
}

void mat4_to_array(mat4 m, float* out) {
  
  out[0] = m.xx;
  out[1] = m.yx;
  out[2] = m.zx;
  out[3] = m.wx;

  out[4] = m.xy;
  out[5] = m.yy;
  out[6] = m.zy;
  out[7] = m.wy;
  
  out[8] = m.xz;
  out[9] = m.yz;
  out[10] = m.zz;
  out[11] = m.wz;
  
  out[12] = m.xw;
  out[13] = m.yw;
  out[14] = m.zw;
  out[15] = m.ww;
  
}

void mat4_to_array_trans(mat4 m, float* out) {
  
  out[0] = m.xx;
  out[1] = m.xy;
  out[2] = m.xz;
  out[3] = m.xw;

  out[4] = m.yx;
  out[5] = m.yy;
  out[6] = m.yz;
  out[7] = m.yw;
  
  out[8] = m.zx;
  out[9] = m.zy;
  out[10] = m.zz;
  out[11] = m.zw;
  
  out[12] = m.wx;
  out[13] = m.wy;
  out[14] = m.wz;
  out[15] = m.ww;
  
}

void mat4_print(mat4 m) {

  printf("|%4.2f, %4.2f, %4.2f, %4.2f|\n", m.xx, m.xy, m.xz, m.xw);
  printf("|%4.2f, %4.2f, %4.2f, %4.2f|\n", m.yx, m.yy, m.yz, m.yw);
  printf("|%4.2f, %4.2f, %4.2f, %4.2f|\n", m.zx, m.zy, m.zz, m.zw);
  printf("|%4.2f, %4.2f, %4.2f, %4.2f|\n", m.wx, m.wy, m.wz, m.ww);
  
}
*/

/*
mat4 mat4_view_look_at(vec3 position, vec3 target, vec3 up) {
  
  vec3 zaxis = vec3_normalize( vec3_sub(target, position) );
  vec3 xaxis = vec3_normalize( vec3_cross(up, zaxis) );
  vec3 yaxis = vec3_cross(zaxis, xaxis);

  mat4 view_matrix = mat4_id();
  view_matrix.xx = xaxis.x;
  view_matrix.xy = xaxis.y;
  view_matrix.xz = xaxis.z;
  
  view_matrix.yx = yaxis.x;
  view_matrix.yy = yaxis.y;
  view_matrix.yz = yaxis.z;
  
  view_matrix.zx = -zaxis.x;
  view_matrix.zy = -zaxis.y;
  view_matrix.zz = -zaxis.z;
  
  view_matrix = mat4_mul_mat4(view_matrix, mat4_translation(vec3_neg(position)) );
  
  return view_matrix;
}
*/

pub fn mat4_view_look_at(position: Vec3, target: Vec3, up: Vec3) -> Mat4 {
  let view_matrix = Mat4::look_at(Point3::from_vec(position), Point3::from_vec(target), up);
  return view_matrix;
  /*
  let zaxis = target.sub(position).normalize();
  let xaxis = up.cross(zaxis).normalize();
  let yaxis = zaxis.cross(xaxis);

  let mut view_matrix = Mat4::identity();
  view_matrix[0][0] = xaxis.x;
  view_matrix[0][1] = xaxis.y;
  view_matrix[0][2] = xaxis.z;

  view_matrix[1][0] = yaxis.x;
  view_matrix[1][1] = yaxis.y;
  view_matrix[1][2] = yaxis.z;

  view_matrix[2][0] = -zaxis.x;
  view_matrix[2][1] = -zaxis.y;
  view_matrix[2][2] = -zaxis.z;

  let mut negpos = position.clone();
  negpos = negpos.mul(-1.0);
  let translate_matrix = mat4_translation(&negpos);
  //view_matrix = view_matrix.mul(translate_matrix);
  view_matrix = translate_matrix.mul(view_matrix); //.mul(translate_matrix);
  //println!("matrix \n {}", view_matrix);
  //println!("matrix {:?}", view_matrix.to_arr());
  //println!("matrix \n {:?}", view_matrix.transpose().as_slice());
  //println!("matrix \n {:?}", view_matrix.as_slice());
  return view_matrix;
*/
}

/*

mat4 mat4_perspective(float fov, float near_clip, float far_clip, float ratio) {
  
  float right, left, bottom, top;
  
  right = -(near_clip * tanf(fov));
  left = -right;
  
  top = ratio * near_clip * tanf(fov);
  bottom = -top;
  
  mat4 proj_matrix = mat4_zero();
  proj_matrix.xx = (2.0 * near_clip) / (right - left);
  proj_matrix.yy = (2.0 * near_clip) / (top - bottom);
  proj_matrix.xz = (right + left) / (right - left);
  proj_matrix.yz = (top + bottom) / (top - bottom);
  proj_matrix.zz = (-far_clip - near_clip) / (far_clip - near_clip);
  proj_matrix.wz = -1.0;
  proj_matrix.zw = ( -(2.0 * near_clip) * far_clip) / (far_clip - near_clip);
  
  return proj_matrix;
}

*/

pub fn mat4_perspective(fov: f32, near_clip: f32, far_clip: f32, ratio: f32) -> Mat4 {
  //  let two:f32 = 2.0;
  let fovx = Rad(fov);
  let angle = fovx; // two;
  let xmax = near_clip * Rad::tan(angle);
  let ymax = xmax * ratio;

  let p = Perspective {
    left: -xmax,
    right: xmax,
    bottom: -ymax,
    top: ymax,
    near: near_clip,
    far: far_clip,
  };
  return p.into();

  //  let ratio = 1.0 / ratio;

  // let proj_matrix = perspective(Rad(fov), ratio, near_clip, far_clip);
  //return proj_matrix;
/*  let right = -near_clip * fov.tan();
  let left = -right;

  let top = ratio * near_clip * fov.tan();
  let bottom = -top;

  //let mut proj_matrix = Mat4::identity();
// proj_matrix; //.zero();
  let mut proj_matrix = Mat4::zero();

  proj_matrix[0][0] = (2.0 * near_clip) / (right - left);
  proj_matrix[1][1] = (2.0 * near_clip) / (top - bottom);
  proj_matrix[0][2] = (right + left) / (right - left);
  proj_matrix[1][2] = (top + bottom) / (top - bottom);
  proj_matrix[2][2] = (-far_clip - near_clip) / (far_clip - near_clip);
  proj_matrix[3][2] = -1.0;
  proj_matrix[2][3] = ( -(2.0 * near_clip) * far_clip) / (far_clip - near_clip);

  //println!("proj matrix {}", proj_matrix);
  proj_matrix = proj_matrix.transpose();
  return proj_matrix;
*/
}

/*

mat4 mat4_orthographic(float left, float right, float bottom, float top, float clip_near, float clip_far) {

  mat4 m = mat4_id();
  
  m.xx = 2 / (right - left);
  m.yy = 2 / (top - bottom);
  m.zz = 1 / (clip_near - clip_far);
  
  m.xw = -1 - 2 * left / (right - left);
  m.yw =  1 + 2 * top  / (bottom - top);
  m.zw = clip_near / (clip_near - clip_far);
  
  return m;

}
*/

/*
mat4 mat4_translation(vec3 v) {

  mat4 m = mat4_id();
  m.xw = v.x;
  m.yw = v.y;
  m.zw = v.z;

  return m;
  
}
*/

pub fn mat4_translation(v: &Vec3) -> Mat4 {
  let mut m = Mat4::identity();
  m[0][3] = v.x;
  m[1][3] = v.y;
  m[2][3] = v.z;

  return m;
}

/*

mat4 mat4_scale(vec3 v) {

  mat4 m = mat4_id();
  m.xx = v.x;
  m.yy = v.y;
  m.zz = v.z;

  return m;
}

mat4 mat4_rotation_x(float a) {

  mat4 m = mat4_id();
  
  m.yy = cos(a);
  m.yz = -sin(a);
  m.zy = sin(a);
  m.zz = cos(a);
  
  return m;
  
}

mat4 mat4_rotation_y(float a) {

  mat4 m = mat4_id();
  
  m.xx = cos(a);
  m.xz = sin(a);
  m.zx = -sin(a);
  m.zz = cos(a);

  return m;
  
}

mat4 mat4_rotation_z(float a) {

  mat4 m = mat4_id();
  
  m.xx = cos(a);
  m.xy = -sin(a);
  m.yx = sin(a);
  m.yy = cos(a);

  return m;
  
}

mat4 mat4_rotation_axis_angle(vec3 v, float angle) {

  mat4 m = mat4_id();

  float c = cos(angle);
  float s = sin(angle);
  float nc = 1 - c;
  
  m.xx = v.x * v.x * nc + c;
  m.xy = v.x * v.y * nc - v.z * s;
  m.xz = v.x * v.z * nc + v.y * s;
  
  m.yx = v.y * v.x * nc + v.z * s;
  m.yy = v.y * v.y * nc + c;
  m.yz = v.y * v.z * nc - v.x * s;
  
  m.zx = v.z * v.x * nc - v.y * s;
  m.zy = v.z * v.y * nc + v.x * s;
  m.zz = v.z * v.z * nc + c;
  
  return m;

}

mat4 mat4_rotation_euler(float x, float y, float z) {

  mat4 m = mat4_zero();

  float cosx = cos(x);
  float cosy = cos(y);
  float cosz = cos(z);
  float sinx = sin(x);
  float siny = sin(y);
  float sinz = sin(z);

  m.xx = cosy * cosz;
  m.yx = -cosx * sinz + sinx * siny * cosz;
  m.zx = sinx * sinz + cosx * siny * cosz;

  m.xy = cosy * sinz;
  m.yy = cosx * cosz + sinx * siny * sinz;
  m.zy = -sinx * cosz + cosx * siny * sinz;

  m.xz = -siny;
  m.yz = sinx * cosy;
  m.zz = cosx * cosy;

  m.ww = 1;
  
  return m;
}

mat4 mat4_rotation_quat(vec4 q) {

  float x2 = q.x + q.x; 
  float y2 = q.y + q.y; 
  float z2 = q.z + q.z;
  float xx = q.x * x2;  
  float yy = q.y * y2;  
  float wx = q.w * x2;  
  float xy = q.x * y2;   
  float yz = q.y * z2;   
  float wy = q.w * y2;
  float xz = q.x * z2;
  float zz = q.z * z2;  
  float wz = q.w * z2;  
  
  return mat4_new(
    1.0f - ( yy + zz ),	xy - wz, xz + wy,	0.0f,
    xy + wz, 1.0f - ( xx + zz ), yz - wx, 0.0f,
    xz - wy, yz + wx, 1.0f - ( xx + yy ), 0.0f,
    0.0f,	0.0f, 0.0f,	1.0f);
    
}

mat4 mat4_rotation_quat_dual(quat_dual q) {
  
  float rx = q.real.x, ry = q.real.y, rz = q.real.z, rw = q.real.w;
  float tx = q.dual.x, ty = q.dual.y, tz = q.dual.z, tw = q.dual.w;

  mat4 m = mat4_id();
  m.xx = rw*rw + rx*rx - ry*ry - rz*rz;              
  m.xy = 2.f*(rx*ry - rw*rz);                        
  m.xz = 2*(rx*rz + rw*ry);
  m.yx = 2*(rx*ry + rw*rz);                                  
  m.yy = rw*rw - rx*rx + ry*ry - rz*rz;      
  m.yz = 2*(ry*rz - rw*rx);
  m.zx = 2*(rx*rz - rw*ry);                                  
  m.zy = 2*(ry*rz + rw*rx);                          
  m.zz = rw*rw - rx*rx - ry*ry + rz*rz;

  m.xw = -2*tw*rx + 2*rw*tx - 2*ty*rz + 2*ry*tz;
  m.yw = -2*tw*ry + 2*tx*rz - 2*rx*tz + 2*rw*ty;
  m.zw = -2*tw*rz + 2*rx*ty + 2*rw*tz - 2*tx*ry;

  return m;
}

mat4 mat4_world(vec3 position, vec3 scale, quat rotation) {
  
  mat4 pos_m, sca_m, rot_m, result;
  
  pos_m = mat4_translation(position);
  rot_m = mat4_rotation_quat(rotation);
  sca_m = mat4_scale(scale);
  
  result = mat4_id();
  result = mat4_mul_mat4( result, pos_m );
  result = mat4_mul_mat4( result, rot_m );
  result = mat4_mul_mat4( result, sca_m );
  
  return result;
  
}

mat4 mat4_lerp(mat4 m1, mat4 m2, float amount) {
  mat4 m;
  
  m.xx = lerp(m1.xx, m2.xx, amount);
  m.xy = lerp(m1.xy, m2.xy, amount);
  m.xz = lerp(m1.xz, m2.xz, amount);
  m.xw = lerp(m1.xw, m2.xw, amount);
  
  m.yx = lerp(m1.yx, m2.yx, amount);
  m.yy = lerp(m1.yy, m2.yy, amount);
  m.yz = lerp(m1.yz, m2.yz, amount);
  m.yw = lerp(m1.yw, m2.yw, amount);
  
  m.zx = lerp(m1.zx, m2.zx, amount);
  m.zy = lerp(m1.zy, m2.zy, amount);
  m.zz = lerp(m1.zz, m2.zz, amount);
  m.zw = lerp(m1.zw, m2.zw, amount);
  
  m.wx = lerp(m1.wx, m2.wx, amount);
  m.wy = lerp(m1.wy, m2.wy, amount);
  m.wz = lerp(m1.wz, m2.wz, amount);
  m.ww = lerp(m1.ww, m2.ww, amount);
  
  return m;
}

mat4 mat4_smoothstep(mat4 m1, mat4 m2, float amount) {
  mat4 m;
  
  m.xx = smoothstep(m1.xx, m2.xx, amount);
  m.xy = smoothstep(m1.xy, m2.xy, amount);
  m.xz = smoothstep(m1.xz, m2.xz, amount);
  m.xw = smoothstep(m1.xw, m2.xw, amount);
  
  m.yx = smoothstep(m1.yx, m2.yx, amount);
  m.yy = smoothstep(m1.yy, m2.yy, amount);
  m.yz = smoothstep(m1.yz, m2.yz, amount);
  m.yw = smoothstep(m1.yw, m2.yw, amount);
  
  m.zx = smoothstep(m1.zx, m2.zx, amount);
  m.zy = smoothstep(m1.zy, m2.zy, amount);
  m.zz = smoothstep(m1.zz, m2.zz, amount);
  m.zw = smoothstep(m1.zw, m2.zw, amount);
  
  m.wx = smoothstep(m1.wx, m2.wx, amount);
  m.wy = smoothstep(m1.wy, m2.wy, amount);
  m.wz = smoothstep(m1.wz, m2.wz, amount);
  m.ww = smoothstep(m1.ww, m2.ww, amount);
  
  return m;
}
*/
