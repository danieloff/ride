use std::fs::File;
use std::fmt::Write;
use std::io::prelude::*;
use std::os::raw;
use std::rc::Rc;
use std::cell::RefCell;

use byteorder::{LittleEndian as LE, ReadBytesExt};

use std::io;
use std::io::BufReader;

use gl;
use crate::core::error;
use crate::engine::{GLExtValues, gl_check_error};


/*
/**
*** :: Texture ::
***
***   OpenGL stored texture
***
***   Supports the loading of LUT files
***   as 3d textures for color correction
***
**/

#ifndef texture_h
#define texture_h

#include "cengine.h"
#include "assets/image.h"
*/

/*
typedef struct {
  GLuint handle;
  GLenum type;
} texture;
*/

pub struct Texture {
  handle: gl::types::GLuint,
  ttype: gl::types::GLenum,
}

/*
texture* texture_new();
texture* texture_new_handle(GLuint h);
void texture_delete(texture* t);

GLuint texture_handle(texture* t);
GLenum texture_type(texture* t);

void texture_set_image(texture* t, image* i);
image* texture_get_image(texture* t);

void texture_generate_mipmaps(texture* t);
void texture_set_filtering_nearest(texture* t);
void texture_set_filtering_linear(texture* t);
void texture_set_filtering_anisotropic(texture* t);

texture* bmp_load_file( char* filename );
texture* tga_load_file( char* filename );
texture* dds_load_file( char* filename );
texture* lut_load_file( char* filename );
texture* acv_load_file( char* filename );

void texture_write_to_file(texture* t, char* filename);
void texture3d_write_to_file(texture* t, char* filename);

#endif

#include "assets/texture.h"

#include "data/spline.h"
*/
/*
texture* texture_new() {
  
  texture* t = malloc(sizeof(texture));
  glGenTextures(1, &t->handle);
  t->type = GL_TEXTURE_2D;
  
  return t;
}
*/

pub fn texture_new() -> Texture {
  let mut t = Texture {
    handle: 0,
    ttype: gl::TEXTURE_2D,
  };
  unsafe {
    gl::GenTextures(1, &mut t.handle);
    //println!("texture handle is {}", t.handle)
  }
  return t;
}

/*
void texture_delete(texture* t) {
  glDeleteTextures(1, &t->handle);
  free(t);
}
*/

pub fn texture_delete(t: Rc<RefCell<Texture>>) {
  unsafe {
    gl::DeleteTextures(1, &t.borrow().handle);
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteTextures(1, &self.handle);
    }
  }
}

/*
GLuint texture_handle(texture* t) {
  return t->handle;
}
*/
///add a from asset trait?
//impl<'a> From<Weak<RefCell<Any>>> for &'a Texture {
/*impl Texture {
  pub fn from_asset_handle<'a>(any:&'a Rc<Any>) -> Rc<RefCell<Texture>> { //I think this lifetime is probably wrong.. If I don't destroy the weak it allows it??
    //let strong = any.upgrade().unwrap();

    //let borrow_mut = any.borrow();

    let texture = any.clone().downcast::<RefCell<Texture>>().unwrap();

    return texture;
  }

}*/


pub fn texture_handle(t: &Texture) -> gl::types::GLuint {
  return t.handle;
}

/*
GLenum texture_type(texture* t) {
  return t->type;
}
*/

pub fn texture_type(t: &Texture) -> gl::types::GLenum {
  return t.ttype;
}

/*

void texture_set_image(texture* t, image* i) {
  glBindTexture(GL_TEXTURE_2D, texture_handle(t));
  glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, i->width, i->height, 0, GL_RGBA, GL_UNSIGNED_BYTE, i->data );
}

texture* tga_load_file( char* filename ) {
  
  image* i = image_tga_load_file(filename);
  
  texture* t = texture_new();
  glBindTexture(GL_TEXTURE_2D, texture_handle(t));
  glTexParameteri(GL_TEXTURE_2D, GL_GENERATE_MIPMAP, GL_FALSE);
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAX_LEVEL, 0);
  
  texture_set_image(t, i);
  texture_set_filtering_anisotropic(t);
  
  image_delete(i);
  
  return t;
}

texture* bmp_load_file( char* filename ) {

  image* i = image_bmp_load_file(filename);
  
  texture* t = texture_new();
  glBindTexture(GL_TEXTURE_2D, texture_handle(t));
  glTexParameteri( GL_TEXTURE_2D, GL_GENERATE_MIPMAP, GL_FALSE );
  glTexParameteri( GL_TEXTURE_2D, GL_TEXTURE_MAX_LEVEL, 0 );
  
  texture_set_image(t, i);
  texture_set_filtering_anisotropic(t);
  
  image_delete(i);
  
  return t;
}

image* texture_get_image(texture* t) {
  
  int width = 0;
  int height = 0;
  int format = 0;
  
  glBindTexture(GL_TEXTURE_2D, texture_handle(t));
  glGetTexLevelParameteriv(GL_TEXTURE_2D, 0, GL_TEXTURE_WIDTH, &width);
  glGetTexLevelParameteriv(GL_TEXTURE_2D, 0, GL_TEXTURE_HEIGHT, &height);
  glGetTexLevelParameteriv(GL_TEXTURE_2D, 0, GL_TEXTURE_INTERNAL_FORMAT, &format);
  
  if ((width == 0) || (height == 0)) {
    error("Texture has zero size width/height: (%i, %i)", width, height);
  }
  
  unsigned char* data = malloc(width * height * 4);
  float* data_flt;
  unsigned int* data_int;
  
  switch (format) {
    
    case GL_RGBA:
    case GL_COMPRESSED_RGBA_S3TC_DXT1_EXT:
      glGetTexImage(GL_TEXTURE_2D, 0, GL_RGBA, GL_UNSIGNED_BYTE, data);
    break;
  
    case GL_ALPHA16:
    
      data_flt = malloc(sizeof(float) * width * height);
      glGetTexImage(GL_TEXTURE_2D, 0, GL_ALPHA, GL_FLOAT, data_flt);
        
      for(int x = 0; x < width; x++)
      for(int y = 0; y < height; y++) {
        data[(y*4*width) + (x*4) + 0] = pow(data_flt[(y*width) + x], 256.0) * 255;
        data[(y*4*width) + (x*4) + 1] = pow(data_flt[(y*width) + x], 256.0) * 255;
        data[(y*4*width) + (x*4) + 2] = pow(data_flt[(y*width) + x], 256.0) * 255;
        data[(y*4*width) + (x*4) + 3] = pow(data_flt[(y*width) + x], 256.0) * 255;
      }
        
      free(data_flt);
    
    break;
    
    case GL_RGBA32F:
    case GL_RGBA16F:
    
      data_flt = malloc(4 * sizeof(float) * width * height);
      glGetTexImage(GL_TEXTURE_2D, 0, GL_RGBA, GL_FLOAT, data_flt);
      
      for(int x = 0; x < width; x++)
      for(int y = 0; y < height; y++) {
        data[(y*4*width) + (x*4) + 0] = clamp(data_flt[(y*4*width) + (x*4) + 0] * 127 + 127, 0, 255);
        data[(y*4*width) + (x*4) + 1] = clamp(data_flt[(y*4*width) + (x*4) + 1] * 127 + 127, 0, 255);
        data[(y*4*width) + (x*4) + 2] = clamp(data_flt[(y*4*width) + (x*4) + 2] * 127 + 127, 0, 255);
        data[(y*4*width) + (x*4) + 3] = clamp(data_flt[(y*4*width) + (x*4) + 3] * 127 + 127, 0, 255);
      }
      
      free(data_flt);
    
    break;
    
    case GL_DEPTH_COMPONENT:
    case GL_DEPTH_COMPONENT24:
    
      data_int = malloc(sizeof(unsigned int) * width * height);
      glGetTexImage(GL_TEXTURE_2D, 0, GL_DEPTH_COMPONENT, GL_UNSIGNED_INT, data_int);
      
      for(int x = 0; x < width; x++)
      for(int y = 0; y < height; y++) {
        data[(y*4*width) + (x*4) + 0] = data_int[(y*width) + x];
        data[(y*4*width) + (x*4) + 1] = data_int[(y*width) + x];
        data[(y*4*width) + (x*4) + 2] = data_int[(y*width) + x];
        data[(y*4*width) + (x*4) + 3] = data_int[(y*width) + x];
      }
      
      free(data_int);
    
    break;
    
    default:
      error("Can't convert that particular texture format %i to an image.", format);
      
  }
  
  SDL_GL_CheckError();
  
  image* i = image_new(width, height, data);
  
  free(data);
  
  return i;
}

void texture_generate_mipmaps(texture* t) {

  glBindTexture(t->type, texture_handle(t));
  glGenerateMipmap(t->type);
  
}

void texture_set_filtering_nearest(texture* t) {
  
  glBindTexture(t->type, texture_handle(t));
  glTexParameteri(t->type, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
  glTexParameteri(t->type, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
  glTexParameteri(t->type, GL_TEXTURE_MAX_ANISOTROPY_EXT, 0);
  
}

void texture_set_filtering_linear(texture* t) {

  glBindTexture(t->type, texture_handle(t));
  glTexParameteri(t->type, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
  glTexParameteri(t->type, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
  glTexParameteri(t->type, GL_TEXTURE_MAX_ANISOTROPY_EXT, 0);
  
}
*/
/*
void texture_set_filtering_anisotropic(texture* t) {

  float max = 0;
  glGetFloatv(GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT, &max);

  glBindTexture(t->type, texture_handle(t));
  glTexParameteri(t->type, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR);
  glTexParameteri(t->type, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
  glTexParameteri(t->type, GL_TEXTURE_MAX_ANISOTROPY_EXT, max);

}*/

pub fn texture_set_filtering_anisotropic(t: &Texture) {
  let mut max: f32 = 0.0;
  unsafe {
    gl::GetFloatv(GLExtValues::GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT, &mut max);

    gl::BindTexture(t.ttype, texture_handle(t));
    gl::TexParameteri(
      t.ttype,
      gl::TEXTURE_MIN_FILTER,
      gl::LINEAR_MIPMAP_LINEAR as gl::types::GLint,
    );
    gl::TexParameteri(
      t.ttype,
      gl::TEXTURE_MAG_FILTER,
      gl::LINEAR as gl::types::GLint,
    );
    gl::TexParameterf(
      t.ttype,
      GLExtValues::GL_TEXTURE_MAX_ANISOTROPY_EXT,
      max as gl::types::GLfloat,
    ); //this is a float??
  }
}

/*


void texture_write_to_file(texture* t, char* filename){
  
  image* i = texture_get_image(t);
  image_write_to_file(i, filename);
  image_delete(i);
  
}

texture* lut_load_file( char* filename ) {
  
  SDL_RWops* file = SDL_RWFromFile(filename, "r");
  if(file == NULL) {
    error("Cannot load file %s", filename);
  }
  
  long size = SDL_RWseek(file,0,SEEK_END);
  unsigned char* contents = malloc(size+1);
  contents[size] = '\0';
  SDL_RWseek(file, 0, SEEK_SET);
  SDL_RWread(file, contents, size, 1);
  
  SDL_RWclose(file);
  
  int head = sizeof("CORANGE-LUT")-1;
  int lut_size = (unsigned char)contents[head] | (unsigned char)contents[head + 1];
  
  int offset = head + 3;
  
  texture* t = texture_new();
  t->type = GL_TEXTURE_3D;
  
  glBindTexture(t->type, texture_handle(t));
  glTexImage3D(t->type, 0, GL_RGB, lut_size, lut_size, lut_size, 0, GL_RGB, GL_UNSIGNED_BYTE, contents + offset);
  glTexParameteri(t->type, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
  glTexParameteri(t->type, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
  glTexParameteri(t->type, GL_TEXTURE_WRAP_S, GL_MIRRORED_REPEAT);
  glTexParameteri(t->type, GL_TEXTURE_WRAP_T, GL_MIRRORED_REPEAT);
  glTexParameteri(t->type, GL_TEXTURE_WRAP_R, GL_MIRRORED_REPEAT);
  
  free(contents);
  
  return t;
  
}

void texture3d_write_to_file(texture* t, char* filename) {
  
  int t_width; 
  int t_height;
  int t_depth;
  
  glBindTexture(t->type, texture_handle(t));
  glGetTexLevelParameteriv(t->type, 0, GL_TEXTURE_WIDTH, &t_width);
  glGetTexLevelParameteriv(t->type, 0, GL_TEXTURE_HEIGHT, &t_height);
  glGetTexLevelParameteriv(t->type, 0, GL_TEXTURE_DEPTH, &t_depth);
  
  int width = t_width;
  int height = t_height * t_depth;
  
  unsigned char* data = malloc(width * height * 4);
  
  glGetTexImage(t->type, 0, GL_RGBA, GL_UNSIGNED_BYTE, data);
  
  int xa= width % 256;
  int xb= (width-xa)/256;

  int ya= height % 256;
  int yb= (height-ya)/256;
  unsigned char header[18]={0,0,2,0,0,0,0,0,0,0,0,0,(char)xa,(char)xb,(char)ya,(char)yb,32,0};
  
  SDL_RWops* file = SDL_RWFromFile(filename, "wb");
  SDL_RWwrite(file, header, sizeof(header), 1);
  SDL_RWwrite(file, data, width * height * 4, 1 );
  SDL_RWclose(file);
  
  free(data);
}

/* DDS file stuff */

/* little-endian, of course */
*/
/*
#define DDS_MAGIC 0x20534444

/* DDS_header.dw_flags */
#define DDSD_CAPS                   0x00000001 
#define DDSD_HEIGHT                 0x00000002 
#define DDSD_WIDTH                  0x00000004 
#define DDSD_PITCH                  0x00000008 
#define DDSD_PIXELFORMAT            0x00001000 
#define DDSD_MIPMAPCOUNT            0x00020000 
#define DDSD_LINEARSIZE             0x00080000 
#define DDSD_DEPTH                  0x00800000 

/* DDS_header.s_pixel_format.dw_flags */
#define DDPF_ALPHAPIXELS            0x00000001 
#define DDPF_FOURCC                 0x00000004 
#define DDPF_INDEXED                0x00000020 
#define DDPF_RGB                    0x00000040 

/* DDS_header.s_caps.dw_caps_1 */
#define DDSCAPS_COMPLEX             0x00000008 
#define DDSCAPS_TEXTURE             0x00001000 
#define DDSCAPS_MIPMAP              0x00400000 

/* DDS_header.s_caps.dw_caps_2 */
#define DDSCAPS2_CUBEMAP            0x00000200 
#define DDSCAPS2_CUBEMAP_POSITIVEX  0x00000400 
#define DDSCAPS2_CUBEMAP_NEGATIVEX  0x00000800 
#define DDSCAPS2_CUBEMAP_POSITIVEY  0x00001000 
#define DDSCAPS2_CUBEMAP_NEGATIVEY  0x00002000 
#define DDSCAPS2_CUBEMAP_POSITIVEZ  0x00004000 
#define DDSCAPS2_CUBEMAP_NEGATIVEZ  0x00008000 
#define DDSCAPS2_VOLUME             0x00200000 

#define D3DFMT_DXT1     0x31545844    /* DXT1 compression texture format */
#define D3DFMT_DXT2     0x32545844    /* DXT2 compression texture format */
#define D3DFMT_DXT3     0x33545844    /* DXT3 compression texture format */
#define D3DFMT_DXT4     0x34545844    /* DXT4 compression texture format */
#define D3DFMT_DXT5     0x35545844    /* DXT5 compression texture format */
*/
const DDS_MAGIC: u32 = 0x20534444;

const DDSD_CAPS: u32 = 0x00000001;
//const DDSD_HEIGHT:u32          =        0x00000002;
//const DDSD_WIDTH:u32           =        0x00000004;
//const DDSD_PITCH:u32           =        0x00000008;
const DDSD_PIXELFORMAT: u32 = 0x00001000;
const DDSD_MIPMAPCOUNT: u32 = 0x00020000;
//const DDSD_LINEARSIZE:u32      =        0x00080000;
//const DDSD_DEPTH:u32           =        0x00800000;

/* DDS_header.s_pixel_format.dw_flags */
const DDPF_ALPHAPIXELS: u32 = 0x00000001;
const DDPF_FOURCC: u32 = 0x00000004;
const DDPF_INDEXED: u32 = 0x00000020;
const DDPF_RGB: u32 = 0x00000040;

/* DDS_header.s_caps.dw_caps_1 */
//const DDSCAPS_COMPLEX:u32      =        0x00000008;
//const DDSCAPS_TEXTURE:u32      =        0x00001000;
//const DDSCAPS_MIPMAP:u32       =        0x00400000;

/* DDS_header.s_caps.dw_caps_2 */
const DDSCAPS2_CUBEMAP: u32 = 0x00000200;
//const DDSCAPS2_CUBEMAP_POSITIVEX:u32  = 0x00000400;
//const DDSCAPS2_CUBEMAP_NEGATIVEX:u32  = 0x00000800;
//const DDSCAPS2_CUBEMAP_POSITIVEY:u32  = 0x00001000;
//const DDSCAPS2_CUBEMAP_NEGATIVEY:u32  = 0x00002000;
//const DDSCAPS2_CUBEMAP_POSITIVEZ:u32  = 0x00004000;
//const DDSCAPS2_CUBEMAP_NEGATIVEZ:u32  = 0x00008000;
//const DDSCAPS2_VOLUME:u32             = 0x00200000;

const D3DFMT_DXT1: u32 = 0x31545844; /* DXT1 compression texture format */
//const D3DFMT_DXT2:u32     = 0x32545844;    /* DXT2 compression texture format */
const D3DFMT_DXT3: u32 = 0x33545844; /* DXT3 compression texture format */
//const D3DFMT_DXT4:u32     = 0x34545844;    /* DXT4 compression texture format */
const D3DFMT_DXT5: u32 = 0x35545844; /* DXT5 compression texture format */

/*
#define PF_IS_DXT1(pf) \
  ((pf.dw_flags & DDPF_FOURCC) && \
   (pf.dw_four_cc == D3DFMT_DXT1))
*/

fn pf_is_dxt1(pf: &PixelFormat) -> bool {
  return ((pf.dw_flags & DDPF_FOURCC) != 0) && (pf.dw_four_cc == D3DFMT_DXT1);
}

/*
#define PF_IS_DXT3(pf) \
  ((pf.dw_flags & DDPF_FOURCC) && \
   (pf.dw_four_cc == D3DFMT_DXT3))
   */

fn pf_is_dxt3(pf: &PixelFormat) -> bool {
  return ((pf.dw_flags & DDPF_FOURCC) != 0) && (pf.dw_four_cc == D3DFMT_DXT3);
}

/*

#define PF_IS_DXT5(pf) \
  ((pf.dw_flags & DDPF_FOURCC) && \
   (pf.dw_four_cc == D3DFMT_DXT5))
*/

fn pf_is_dxt5(pf: &PixelFormat) -> bool {
  return ((pf.dw_flags & DDPF_FOURCC) != 0) && (pf.dw_four_cc == D3DFMT_DXT5);
}

/*
#define PF_IS_BGRA8(pf) \
  ((pf.dw_flags & DDPF_RGB) && \
   (pf.dw_flags & DDPF_ALPHAPIXELS) && \
   (pf.dw_rgb_bit_count == 32) && \
   (pf.dw_r_bit_mask == 0xff0000) && \
   (pf.dw_g_bit_mask == 0xff00) && \
   (pf.dw_b_bit_mask == 0xff) && \
   (pf.dw_alpha_bit_mask == 0xff000000U))
*/

fn pf_is_bgra8(pf: &PixelFormat) -> bool {
  return ((pf.dw_flags & DDPF_RGB) != 0) && ((pf.dw_flags & DDPF_ALPHAPIXELS) != 0) &&
    (pf.dw_rgb_bit_count == 32) && (pf.dw_r_bit_mask == 0xff0000) &&
    (pf.dw_g_bit_mask == 0xff00) &&
    (pf.dw_b_bit_mask == 0xff) && (pf.dw_alpha_bit_mask == 0xff000000);
}

/*
#define PF_IS_BGR8(pf) \
  ((pf.dw_flags & DDPF_RGB) && \
  !(pf.dw_flags & DDPF_ALPHAPIXELS) && \
   (pf.dw_rgb_bit_count == 24) && \
   (pf.dw_r_bit_mask == 0xff0000) && \
   (pf.dw_g_bit_mask == 0xff00) && \
   (pf.dw_b_bit_mask == 0xff))
   */

fn pf_is_bgr8(pf: &PixelFormat) -> bool {
  return ((pf.dw_flags & DDPF_RGB) != 0) && ((pf.dw_flags & DDPF_ALPHAPIXELS) == 0) &&
    (pf.dw_rgb_bit_count == 24) && (pf.dw_r_bit_mask == 0xff0000) &&
    (pf.dw_g_bit_mask == 0xff00) && (pf.dw_b_bit_mask == 0xff);
}
/*

#define PF_IS_BGR5A1(pf) \
  ((pf.dw_flags & DDPF_RGB) && \
   (pf.dw_flags & DDPF_ALPHAPIXELS) && \
   (pf.dw_rgb_bit_count == 16) && \
   (pf.dw_r_bit_mask == 0x00007c00) && \
   (pf.dw_g_bit_mask == 0x000003e0) && \
   (pf.dw_b_bit_mask == 0x0000001f) && \
   (pf.dw_alpha_bit_mask == 0x00008000))
*/

fn pf_is_bgr5a1(pf: &PixelFormat) -> bool {
  return ((pf.dw_flags & DDPF_RGB) != 0) && //TODO why is rgb in a bgr check?
    ((pf.dw_flags & DDPF_ALPHAPIXELS) != 0) && (pf.dw_rgb_bit_count == 16) &&
    (pf.dw_r_bit_mask == 0x00007c00) && (pf.dw_g_bit_mask == 0x000003e0) &&
    (pf.dw_b_bit_mask == 0x0000001f) && (pf.dw_alpha_bit_mask == 0x00008000);
}

/*
#define PF_IS_BGR565(pf) \
  ((pf.dw_flags & DDPF_RGB) && \
  !(pf.dw_flags & DDPF_ALPHAPIXELS) && \
   (pf.dw_rgb_bit_count == 16) && \
   (pf.dw_r_bit_mask == 0x0000f800) && \
   (pf.dw_g_bit_mask == 0x000007e0) && \
   (pf.dw_b_bit_mask == 0x0000001f))
   */

fn pf_is_bgr565(pf: &PixelFormat) -> bool {
  return ((pf.dw_flags & DDPF_RGB) != 0) && ((pf.dw_flags & DDPF_ALPHAPIXELS) == 0) &&
    (pf.dw_rgb_bit_count == 16) && (pf.dw_r_bit_mask == 0x0000f800) &&
    (pf.dw_g_bit_mask == 0x000007e0) && (pf.dw_b_bit_mask == 0x0000001f);
}

/*

#define PF_IS_INDEX8(pf) \
  ((pf.dw_flags & DDPF_INDEXED) && \
   (pf.dw_rgb_bit_count == 8))

*/

fn pf_is_index8(pf: &PixelFormat) -> bool {
  return ((pf.dw_flags & DDPF_INDEXED) != 0) && (pf.dw_rgb_bit_count == 8);
}

/*
typedef struct {
  unsigned int    dw_magic;
  unsigned int    dw_size;
  unsigned int    dw_flags;
  unsigned int    dw_height;
  unsigned int    dw_width;
  unsigned int    dw_pitch_or_linear_size;
  unsigned int    dw_depth;
  unsigned int    dw_mip_map_count;
  unsigned int    dw_reserved1[ 11 ];

  /* DDPIXELFORMAT */
  struct {
    unsigned int    dw_size;
    unsigned int    dw_flags;
    unsigned int    dw_four_cc;
    unsigned int    dw_rgb_bit_count;
    unsigned int    dw_r_bit_mask;
    unsigned int    dw_g_bit_mask;
    unsigned int    dw_b_bit_mask;
    unsigned int    dw_alpha_bit_mask;
  } s_pixel_format;

  /* DDCAPS2 */
  struct {
    unsigned int    dw_caps_1;
    unsigned int    dw_caps_2;
    unsigned int    dw_ddsx;
    unsigned int    dw_reserved;
  } s_caps;
  
  unsigned int dw_reserved2;
} DDS_header ;
*/
#[allow(dead_code)]
struct PixelFormat {
  dw_size: u32,
  dw_flags: u32,
  dw_four_cc: u32,
  dw_rgb_bit_count: u32,
  dw_r_bit_mask: u32,
  dw_g_bit_mask: u32,
  dw_b_bit_mask: u32,
  dw_alpha_bit_mask: u32,
}

#[allow(dead_code)]
struct SCaps {
  dw_caps_1: u32,
  dw_caps_2: u32,
  dw_ddsx: u32,
  dw_reserved: u32,
}

#[allow(dead_code)]
struct DDSHeader {
  dw_magic: u32,
  dw_size: u32,
  dw_flags: u32,
  dw_height: u32,
  dw_width: u32,
  dw_pitch_or_linear_size: u32,
  dw_depth: u32,
  dw_mip_map_count: u32,
  dw_reserved1: [u32; 11],
  s_pixel_format: PixelFormat,
  s_caps: SCaps,
  dw_reserved2: u32,
}

/*
typedef struct {

  bool compressed;
  bool swap;
  bool palette;
  int div_size;
  int block_bytes;
  GLenum internal_format;
  GLenum external_format;
  GLenum type;
  
} DdsLoadInfo;
*/

struct DdsLoadInfo {
  compressed: bool,
  swap: bool,
  palette: bool,
  div_size: u32,
  block_bytes: u32,
  internal_format: gl::types::GLenum,
  external_format: gl::types::GLenum,
  image_type: gl::types::GLenum,
}

/*


static bool is_power_of_two(unsigned int x) {
  while (((x % 2) == 0) && x > 1) { x /= 2; }
  return (x == 1);
}
*/

fn is_power_of_two(x: u32) -> bool {
  let mut x = x;
  while (x % 2) == 0 && x > 1 {
    x = x / 2;
  }
  return x == 1;
}

fn read_pixel_format<R: Read>(reader: &mut R) -> io::Result<PixelFormat> {
  let dw_size: u32 = r#try!(reader.read_u32::<LE>());
  let dw_flags: u32 = r#try!(reader.read_u32::<LE>());
  let dw_four_cc: u32 = r#try!(reader.read_u32::<LE>());
  let dw_rgb_bit_count: u32 = r#try!(reader.read_u32::<LE>());
  let dw_r_bit_mask: u32 = r#try!(reader.read_u32::<LE>());
  let dw_g_bit_mask: u32 = r#try!(reader.read_u32::<LE>());
  let dw_b_bit_mask: u32 = r#try!(reader.read_u32::<LE>());
  let dw_alpha_bit_mask: u32 = r#try!(reader.read_u32::<LE>());

  return Ok(PixelFormat {
    dw_size: dw_size,
    dw_flags: dw_flags,
    dw_four_cc: dw_four_cc,
    dw_rgb_bit_count: dw_rgb_bit_count,
    dw_r_bit_mask: dw_r_bit_mask,
    dw_g_bit_mask: dw_g_bit_mask,
    dw_b_bit_mask: dw_b_bit_mask,
    dw_alpha_bit_mask: dw_alpha_bit_mask,
  });
}

fn read_scaps<R: Read>(reader: &mut R) -> io::Result<SCaps> {
  let dw_caps_1: u32 = r#try!(reader.read_u32::<LE>());
  let dw_caps_2: u32 = r#try!(reader.read_u32::<LE>());
  let dw_ddsx: u32 = r#try!(reader.read_u32::<LE>());
  let dw_reserved: u32 = r#try!(reader.read_u32::<LE>());

  return Ok(SCaps {
    dw_caps_1: dw_caps_1,
    dw_caps_2: dw_caps_2,
    dw_ddsx: dw_ddsx,
    dw_reserved: dw_reserved,
  });
}

fn read_dds_header<R: Read>(reader: &mut R) -> io::Result<DDSHeader> {
  let dw_magic: u32 = r#try!(reader.read_u32::<LE>());
  let dw_size: u32 = r#try!(reader.read_u32::<LE>());
  let dw_flags: u32 = r#try!(reader.read_u32::<LE>());
  let dw_height: u32 = r#try!(reader.read_u32::<LE>());
  let dw_width: u32 = r#try!(reader.read_u32::<LE>());
  let dw_pitch_or_linear_size: u32 = r#try!(reader.read_u32::<LE>());
  let dw_depth: u32 = r#try!(reader.read_u32::<LE>());
  let dw_mip_map_count: u32 = r#try!(reader.read_u32::<LE>());
  let mut dw_reserved1: [u32; 11] = [0; 11];

  //try!(reader.read(&mut [0; 80])); (reads bytes)
  for i in dw_reserved1.iter_mut() {
    *i = r#try!(reader.read_u32::<LE>());
  }
  let s_pixel_format = r#try!(read_pixel_format(reader));
  let s_caps = r#try!(read_scaps(reader));
  let dw_reserved2: u32 = r#try!(reader.read_u32::<LE>());

  return Ok(DDSHeader {
    dw_magic: dw_magic,
    dw_size: dw_size,
    dw_flags: dw_flags,
    dw_height: dw_height,
    dw_width: dw_width,
    dw_pitch_or_linear_size: dw_pitch_or_linear_size,
    dw_depth: dw_depth,
    dw_mip_map_count: dw_mip_map_count,
    dw_reserved1: dw_reserved1,
    s_pixel_format: s_pixel_format,
    s_caps: s_caps,
    dw_reserved2: dw_reserved2,
  });
}

pub fn dds_load_file(filename: &str) -> Rc<RefCell<Texture>> {
  let load_info_dxt1 = DdsLoadInfo {
    compressed: true,
    swap: false,
    palette: false,
    div_size: 4,
    block_bytes: 8,
    internal_format: GLExtValues::GL_COMPRESSED_RGBA_S3TC_DXT1,
    external_format: gl::RGBA,
    image_type: gl::UNSIGNED_BYTE,
  };
  let load_info_dxt3 = DdsLoadInfo {
    compressed: true,
    swap: false,
    palette: false,
    div_size: 4,
    block_bytes: 16,
    internal_format: GLExtValues::GL_COMPRESSED_RGBA_S3TC_DXT3,
    external_format: gl::RGBA,
    image_type: gl::UNSIGNED_BYTE,
  };
  let load_info_dxt5 = DdsLoadInfo {
    compressed: true,
    swap: false,
    palette: false,
    div_size: 4,
    block_bytes: 16,
    internal_format: GLExtValues::GL_COMPRESSED_RGBA_S3TC_DXT5,
    external_format: gl::RGBA,
    image_type: gl::UNSIGNED_BYTE,
  };
  let load_info_bgra8 = DdsLoadInfo {
    compressed: false,
    swap: false,
    palette: false,
    div_size: 1,
    block_bytes: 4,
    internal_format: gl::RGBA8,
    external_format: gl::BGRA,
    image_type: gl::UNSIGNED_BYTE,
  };
  let load_info_bgr8 = DdsLoadInfo {
    compressed: false,
    swap: false,
    palette: false,
    div_size: 1,
    block_bytes: 3,
    internal_format: gl::RGB8,
    external_format: gl::BGR,
    image_type: gl::UNSIGNED_BYTE,
  };
  let load_info_bgr5_a1 = DdsLoadInfo {
    compressed: false,
    swap: true,
    palette: false,
    div_size: 1,
    block_bytes: 2,
    internal_format: gl::RGB5_A1,
    external_format: gl::BGRA,
    image_type: gl::UNSIGNED_SHORT_1_5_5_5_REV,
  };
  let load_info_bgr565 = DdsLoadInfo {
    compressed: false,
    swap: true,
    palette: false,
    div_size: 1,
    block_bytes: 2,
    internal_format: gl::RGB5,
    external_format: gl::RGB,
    image_type: gl::UNSIGNED_SHORT_5_6_5,
  };
  let load_info_index8 = DdsLoadInfo {
    compressed: false,
    swap: false,
    palette: true,
    div_size: 1,
    block_bytes: 1,
    internal_format: gl::RGB8,
    external_format: gl::BGRA,
    image_type: gl::UNSIGNED_BYTE,
  };

  let file = File::open(filename).ok();

  if file.is_none() {
    let mut error_full = String::new();
    let _ = write!(&mut error_full, "Cannot load file '{}'", filename);
    error(&error_full);
  }

  let file = file.unwrap();

  let mut file = BufReader::new(file);

  let hdr = read_dds_header(&mut file);

  if hdr.is_err() {
    let mut error_full = String::new();
    let _ = write!(
      &mut error_full,
      "Cannot read header from file '{}'",
      filename
    );
    error(&error_full);
  }

  let hdr = hdr.ok().unwrap();

  if hdr.dw_magic != DDS_MAGIC || hdr.dw_size != 124 || (hdr.dw_flags & DDSD_PIXELFORMAT) == 0 ||
    (hdr.dw_flags & DDSD_CAPS) == 0
  {
    let mut error_full = String::new();
    let _ = write!(
      &mut error_full,
      "Cannot load file '{}': Does not appear to be a .dds file.",
      filename
    );
    error(&error_full);
  }

  let x = hdr.dw_width;
  let y = hdr.dw_height;
  let mip_map_num = if hdr.dw_flags & DDSD_MIPMAPCOUNT != 0 {
    hdr.dw_mip_map_count
  } else {
    1
  };

  if !is_power_of_two(x) {
    let mut error_full = String::new();
    let _ = write!(
      &mut error_full,
      "Texture {} width is {} pixels which is not a power of two!",
      filename,
      x
    );
    error(&error_full);
  }
  if !is_power_of_two(y) {
    let mut error_full = String::new();
    let _ = write!(
      &mut error_full,
      "Texture {} height is {} pixels which is not a power of two!",
      filename,
      y
    );
    error(&error_full);
  }

  let mut li = &load_info_dxt1;

  if pf_is_dxt1(&hdr.s_pixel_format) {
    li = &load_info_dxt1;
  } else if pf_is_dxt3(&hdr.s_pixel_format) {
    li = &load_info_dxt3;
  } else if pf_is_dxt5(&hdr.s_pixel_format) {
    li = &load_info_dxt5;
  } else if pf_is_bgra8(&hdr.s_pixel_format) {
    li = &load_info_bgra8;
  } else if pf_is_bgr8(&hdr.s_pixel_format) {
    li = &load_info_bgr8;
  } else if pf_is_bgr5a1(&hdr.s_pixel_format) {
    li = &load_info_bgr5_a1;
  } else if pf_is_bgr565(&hdr.s_pixel_format) {
    li = &load_info_bgr565;
  } else if pf_is_index8(&hdr.s_pixel_format) {
    li = &load_info_index8;
  } else {
    let mut error_full = String::new();
    let _ = write!(
      &mut error_full,
      "Cannot Load File '{}': Unknown DDS File format type.",
      filename
    );
    error(&error_full);
  }

  /*
  texture* t = texture_new();
  
  if (hdr.s_caps.dw_caps_2 & DDSCAPS2_CUBEMAP) {
    t->type = GL_TEXTURE_CUBE_MAP;
    glBindTexture(GL_TEXTURE_CUBE_MAP, texture_handle(t));
    glTexParameteri(GL_TEXTURE_CUBE_MAP, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_CUBE_MAP, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_CUBE_MAP, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_CUBE_MAP, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_CUBE_MAP, GL_TEXTURE_WRAP_R, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_CUBE_MAP, GL_GENERATE_MIPMAP, GL_FALSE);
    glTexParameteri(GL_TEXTURE_CUBE_MAP, GL_TEXTURE_BASE_LEVEL, 0);
    glTexParameteri(GL_TEXTURE_CUBE_MAP, GL_TEXTURE_MAX_LEVEL, mip_map_num-1);
  } else {
    t->type = GL_TEXTURE_2D;
    glBindTexture(GL_TEXTURE_2D, texture_handle(t));
    glTexParameteri(GL_TEXTURE_2D, GL_GENERATE_MIPMAP, GL_FALSE);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_BASE_LEVEL, 0);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAX_LEVEL, mip_map_num-1);
    texture_set_filtering_anisotropic(t);
  }
  */

  let mut t = texture_new();

  if (hdr.s_caps.dw_caps_2 & DDSCAPS2_CUBEMAP) != 0 {
    t.ttype = gl::TEXTURE_CUBE_MAP;
    unsafe {
      gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_handle(&t));
      gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_MAG_FILTER,
        gl::LINEAR as gl::types::GLint,
      );
      gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR as gl::types::GLint,
      );
      gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_WRAP_S,
        gl::CLAMP_TO_EDGE as gl::types::GLint,
      );
      gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_WRAP_T,
        gl::CLAMP_TO_EDGE as gl::types::GLint,
      );
      gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_WRAP_R,
        gl::CLAMP_TO_EDGE as gl::types::GLint,
      );
      //gl::TexParameteri(gl::TEXTURE_CUBE_MAP, GLExtValues::GL_GENERATE_MIPMAP, gl::FALSE as gl::types::GLint);
      gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_BASE_LEVEL, 0);
      gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_MAX_LEVEL,
        mip_map_num as gl::types::GLint - 1,
      );

      gl_check_error();
    }
  } else {
    t.ttype = gl::TEXTURE_2D;
    unsafe {
      gl::BindTexture(gl::TEXTURE_2D, texture_handle(&t));
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
      gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MAX_LEVEL,
        mip_map_num as gl::types::GLint - 1,
      );
    }
    gl_check_error();
    texture_set_filtering_anisotropic(&t);
    gl_check_error();
  }
  /*
for (int i = 0; i < (t->type == GL_TEXTURE_CUBE_MAP ? 6 : 1); i++) {
  
    GLenum target = t->type;
    
    if (t->type == GL_TEXTURE_CUBE_MAP) {
      target = GL_TEXTURE_CUBE_MAP_POSITIVE_X + i;
    }
        
    int x = hdr.dw_width;
    int y = hdr.dw_height;
    int mip_map_num = (hdr.dw_flags & DDSD_MIPMAPCOUNT) ? hdr.dw_mip_map_count : 1;
    
    if ( li->compressed ) {
      
      size_t size = max(li->div_size, x) / li->div_size * max(li->div_size, y) / li->div_size * li->block_bytes;
      char* data = malloc(size);
      
      for(int ix = 0; ix < mip_map_num; ix++) {
      
        SDL_RWread(f, data, 1, size);
        glCompressedTexImage2D(target, ix, li->internal_format, x, y, 0, size, data);
        
        x = (x+1)>>1;
        y = (y+1)>>1;
        
        size = max(li->div_size, x) / li->div_size * max(li->div_size, y) / li->div_size * li->block_bytes;
      }
      
      free(data);
      */
  for i in 0..
    if t.ttype == gl::TEXTURE_CUBE_MAP {
      6
    } else {
      1
    }
  {
    let mut target = t.ttype;

    if t.ttype == gl::TEXTURE_CUBE_MAP {
      target = gl::TEXTURE_CUBE_MAP_POSITIVE_X + i;
    }

    let mut x = hdr.dw_width;
    let mut y = hdr.dw_height;
    let mip_map_num = if (hdr.dw_flags & DDSD_MIPMAPCOUNT) != 0 {
      hdr.dw_mip_map_count
    } else {
      1
    };

    if li.compressed {
      let mut size = li.div_size.max(x) / li.div_size * li.div_size.max(y) / li.div_size *
        li.block_bytes;
      let mut data: Vec<u8> = vec![0; size as usize];

      for ix in 0..mip_map_num {
        let read = file.read_exact(&mut data[0..(size as usize)]); //NOTE: DO *NOT* CALL READ, in RUST this is READ_EXACT to fill the buffer
        if read.ok().is_none() {
          println!("Read nothing for file {}", filename);
        }
        unsafe {
          gl::CompressedTexImage2D(
            target,
            ix as gl::types::GLint,
            li.internal_format,
            x as gl::types::GLsizei,
            y as gl::types::GLsizei,
            0,
            size as gl::types::GLsizei,
            data.as_ptr() as *const raw::c_void,
          );
          gl_check_error();
        }
        x = (x + 1) >> 1;
        y = (y + 1) >> 1;
        size = li.div_size.max(x) / li.div_size * li.div_size.max(y) / li.div_size * li.block_bytes;
      }
    } else if li.palette {
      let mut size = hdr.dw_pitch_or_linear_size * y;
      let mut data: Vec<u8> = vec![0; size as usize];
      let mut palette: [i32; 256] = [0; 256];
      let mut unpacked: Vec<i32> = vec![0; size as usize];

      for x in palette.iter_mut() {
        *x = file.read_i32::<LE>().ok().unwrap(); //probably very slow to do it this way TODO?
      }

      for ix in 0..mip_map_num {
        let _ = file.read_exact(&mut data[0..(size as usize)]);

        for zz in 0..size {
          unpacked[zz as usize] = palette[data[zz as usize] as usize];
        }

        unsafe {
          gl::PixelStorei(gl::UNPACK_ROW_LENGTH, y as gl::types::GLint);
          gl::TexImage2D(
            target,
            ix as gl::types::GLint,
            li.internal_format as gl::types::GLint,
            x as gl::types::GLsizei,
            y as gl::types::GLsizei,
            0,
            li.external_format,
            li.image_type,
            unpacked.as_ptr() as *const raw::c_void,
          );
        }

        x = (x + 1) >> 1;
        y = (y + 1) >> 1;

        size = x * y * li.block_bytes;
      }
    } else {
      if li.swap {
        unsafe {
          gl::PixelStorei(gl::UNPACK_SWAP_BYTES, gl::TRUE as gl::types::GLint);
        }
      }

      let mut size = x * y * li.block_bytes;
      let mut data: Vec<u8> = vec![0; size as usize];

      for ix in 0..mip_map_num {
        let _ = file.read_exact(&mut data[0..(size as usize)]); //size changes, have to be carefull!!
        unsafe {
          gl::PixelStorei(gl::UNPACK_ROW_LENGTH, y as gl::types::GLint);
          gl::TexImage2D(
            target,
            ix as gl::types::GLint,
            li.internal_format as gl::types::GLint,
            x as gl::types::GLsizei,
            y as gl::types::GLsizei,
            0,
            li.external_format,
            li.image_type,
            data.as_ptr() as *const raw::c_void,
          );
        }
        x = (x + 1) >> 1;
        y = (y + 1) >> 1;
        size = x * y * li.block_bytes;
      }

      if li.swap {
        unsafe {
          gl::PixelStorei(gl::UNPACK_SWAP_BYTES, gl::FALSE as gl::types::GLint);
        }
      }
    }
  }
  gl_check_error();

  return Rc::new(RefCell::new(t));
}

/*
texture* dds_load_file( char* filename ) {
  
  DdsLoadInfo load_info_dxt1 =   { true,  false, false, 4, 8,  GL_COMPRESSED_RGBA_S3TC_DXT1 };
  DdsLoadInfo load_info_dxt3 =   { true,  false, false, 4, 16, GL_COMPRESSED_RGBA_S3TC_DXT3 };
  DdsLoadInfo load_info_dxt5 =   { true,  false, false, 4, 16, GL_COMPRESSED_RGBA_S3TC_DXT5 };
  DdsLoadInfo load_info_bgra8 =  { false, false, false, 1, 4,  GL_RGBA8,   GL_BGRA, GL_UNSIGNED_BYTE };
  DdsLoadInfo load_info_bgr8 =   { false, false, false, 1, 3,  GL_RGB8,    GL_BGR,  GL_UNSIGNED_BYTE };
  DdsLoadInfo load_info_bgr5_a1 = { false, true,  false, 1, 2,  GL_RGB5_A1, GL_BGRA, GL_UNSIGNED_SHORT_1_5_5_5_REV };
  DdsLoadInfo load_info_bgr565 = { false, true,  false, 1, 2,  GL_RGB5,    GL_RGB,  GL_UNSIGNED_SHORT_5_6_5 };
  DdsLoadInfo load_info_index8 = { false, false, true,  1, 1,  GL_RGB8,    GL_BGRA, GL_UNSIGNED_BYTE };
  
  SDL_RWops* f = SDL_RWFromFile(filename, "rb");
  
  if (f == NULL) {
    error("Cannot load file %s", filename);
  }
  
  DDS_header hdr;
  SDL_RWread(f, &hdr, 1, sizeof(DDS_header));
  
  if( hdr.dw_magic != DDS_MAGIC || hdr.dw_size != 124 ||
    !(hdr.dw_flags & DDSD_PIXELFORMAT) || !(hdr.dw_flags & DDSD_CAPS) ) {
    error("Cannot Load File %s: Does not appear to be a .dds file.\n", filename);
  }

  int x = hdr.dw_width;
  int y = hdr.dw_height;
  int mip_map_num = (hdr.dw_flags & DDSD_MIPMAPCOUNT) ? hdr.dw_mip_map_count : 1;
  
  if (!is_power_of_two(x)) { error("Texture %s with is %i pixels which is not a power of two!", filename, x); }
  if (!is_power_of_two(y)) { error("Texture %s height is %i pixels which is not a power of two!", filename, y); }
  
  DdsLoadInfo* li = &load_info_dxt1;
  if      (PF_IS_DXT1(hdr.s_pixel_format  )) { li = &load_info_dxt1;   }
  else if (PF_IS_DXT3(hdr.s_pixel_format  )) { li = &load_info_dxt3;   }
  else if (PF_IS_DXT5(hdr.s_pixel_format  )) { li = &load_info_dxt5;   } 
  else if (PF_IS_BGRA8(hdr.s_pixel_format )) { li = &load_info_bgra8;  }
  else if (PF_IS_BGR8(hdr.s_pixel_format  )) { li = &load_info_bgr8;   }
  else if (PF_IS_BGR5A1(hdr.s_pixel_format)) { li = &load_info_bgr5_a1; }
  else if (PF_IS_BGR565(hdr.s_pixel_format)) { li = &load_info_bgr565; } 
  else if (PF_IS_INDEX8(hdr.s_pixel_format)) { li = &load_info_index8; }
  else { error("Cannot Load File %s: Unknown DDS File format type.", filename); }
  
  texture* t = texture_new();
  
  if (hdr.s_caps.dw_caps_2 & DDSCAPS2_CUBEMAP) {
    t->type = GL_TEXTURE_CUBE_MAP;
    glBindTexture(GL_TEXTURE_CUBE_MAP, texture_handle(t));
    glTexParameteri(GL_TEXTURE_CUBE_MAP, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_CUBE_MAP, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_CUBE_MAP, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_CUBE_MAP, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_CUBE_MAP, GL_TEXTURE_WRAP_R, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_CUBE_MAP, GL_GENERATE_MIPMAP, GL_FALSE);
    glTexParameteri(GL_TEXTURE_CUBE_MAP, GL_TEXTURE_BASE_LEVEL, 0);
    glTexParameteri(GL_TEXTURE_CUBE_MAP, GL_TEXTURE_MAX_LEVEL, mip_map_num-1);
  } else {
    t->type = GL_TEXTURE_2D;
    glBindTexture(GL_TEXTURE_2D, texture_handle(t));
    glTexParameteri(GL_TEXTURE_2D, GL_GENERATE_MIPMAP, GL_FALSE);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_BASE_LEVEL, 0);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAX_LEVEL, mip_map_num-1);
    texture_set_filtering_anisotropic(t);
  }
   
  for (int i = 0; i < (t->type == GL_TEXTURE_CUBE_MAP ? 6 : 1); i++) {
  
    GLenum target = t->type;
    
    if (t->type == GL_TEXTURE_CUBE_MAP) {
      target = GL_TEXTURE_CUBE_MAP_POSITIVE_X + i;
    }
        
    int x = hdr.dw_width;
    int y = hdr.dw_height;
    int mip_map_num = (hdr.dw_flags & DDSD_MIPMAPCOUNT) ? hdr.dw_mip_map_count : 1;
    
    if ( li->compressed ) {
      
      size_t size = max(li->div_size, x) / li->div_size * max(li->div_size, y) / li->div_size * li->block_bytes;
      char* data = malloc(size);
      
      for(int ix = 0; ix < mip_map_num; ix++) {
      
        SDL_RWread(f, data, 1, size);
        glCompressedTexImage2D(target, ix, li->internal_format, x, y, 0, size, data);
        
        x = (x+1)>>1;
        y = (y+1)>>1;
        
        size = max(li->div_size, x) / li->div_size * max(li->div_size, y) / li->div_size * li->block_bytes;
      }
      
      free(data);
      
    } else if ( li->palette ) {
      
      size_t size = hdr.dw_pitch_or_linear_size * y;
      char* data = malloc(size);
      int palette[256];
      int* unpacked = malloc(size * sizeof(int));
      
      SDL_RWread(f, palette, 4, 256);
      for(int ix = 0; ix < mip_map_num; ix++) {
      
        SDL_RWread(f, data, 1, size);
        
        for( int zz = 0; zz < size; ++zz ) {
          unpacked[ zz ] = palette[ (short)data[ zz ] ];
        }
        
        glPixelStorei(GL_UNPACK_ROW_LENGTH, y);
        glTexImage2D(target, ix, li->internal_format, x, y, 0, li->external_format, li->type, unpacked);
        
        x = (x+1)>>1;
        y = (y+1)>>1;
        
        size = x * y * li->block_bytes;
      }
      
      free(data);
      free(unpacked);
      
    } else {
    
      if (li->swap) { glPixelStorei(GL_UNPACK_SWAP_BYTES, GL_TRUE); }
      
      size_t size = x * y * li->block_bytes;
      char* data = malloc(size);
      
      for (int ix = 0; ix < mip_map_num; ix++) {
      
        SDL_RWread(f, data, 1, size);
        glPixelStorei(GL_UNPACK_ROW_LENGTH, y);
        glTexImage2D(target, ix, li->internal_format, x, y, 0, li->external_format, li->type, data);
        
        x = (x+1)>>1;
        y = (y+1)>>1;
        size = x * y * li->block_bytes;
      }
      
      free(data);
      
      if (li->swap) { glPixelStorei(GL_UNPACK_SWAP_BYTES, GL_FALSE); }
      
    }
    
  }
  
  SDL_RWclose(f);
  
  SDL_GL_CheckError();
  
  return t;
  
}
*/

/*

texture* acv_load_file( char* filename ) {

  color_curves* cc = color_curves_load(filename);
  
  uint32_t lut_size = 64;
  
  unsigned char* lut_data = malloc(sizeof(char) * 3 * lut_size * lut_size * lut_size);
  
  int r, g, b;
  for(r = 0; r < lut_size; r++)
  for(g = 0; g < lut_size; g++)
  for(b = 0; b < lut_size; b++) {
    
    int i = (3 * r) + (3 * lut_size * g) + (3 * lut_size * lut_size * b);
    
    float red   = (float)r / lut_size;
    float green = (float)g / lut_size;
    float blue  = (float)b / lut_size;
    
    red = spline_get_x(cc->r_spline, red);
    green = spline_get_x(cc->g_spline, green);
    blue = spline_get_x(cc->b_spline, blue);
    
    red = spline_get_x(cc->rgb_spline, red);
    green = spline_get_x(cc->rgb_spline, green);
    blue = spline_get_x(cc->rgb_spline, blue);
    
    lut_data[i+0] = (unsigned char) (red   * 255);
    lut_data[i+1] = (unsigned char) (green * 255);
    lut_data[i+2] = (unsigned char) (blue  * 255);
  
  }
  
  color_curves_delete(cc);

  texture* t = texture_new();
  t->type = GL_TEXTURE_3D;
  
  glBindTexture(t->type, texture_handle(t));
  glTexImage3D(t->type, 0, GL_RGB, lut_size, lut_size, lut_size, 0, GL_RGB, GL_UNSIGNED_BYTE, lut_data);
  glTexParameteri(t->type, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
  glTexParameteri(t->type, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
  glTexParameteri(t->type, GL_TEXTURE_WRAP_S, GL_MIRRORED_REPEAT);
  glTexParameteri(t->type, GL_TEXTURE_WRAP_T, GL_MIRRORED_REPEAT);
  glTexParameteri(t->type, GL_TEXTURE_WRAP_R, GL_MIRRORED_REPEAT);
  
  free(lut_data);
  
  return t;
  
}
*/
