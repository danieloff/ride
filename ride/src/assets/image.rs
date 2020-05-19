
//use ::alga::linear::Similarity;
use std::ops::Add;
use std::ops::Div;
use std::ops::Sub;

use crate::core::error;
use crate::engine::{Vec4, Vec3, Vec2, Dir2, Vec4Additional, Vec3Additional};

use cgmath::*;
//use cgmath::prelude::*;

//use ::na::Point4;
/*

/**
*** :: Image ::
***
***   Functions to use in CPU side image processing.
***
**/

#ifndef image_h
#define image_h

#include "cengine.h"

typedef struct {
  int width;
  int height;
  unsigned char* data;
  int repeat_type;
  int sample_type;
} image;
*/

pub struct Image {
  width: i32,
  height: i32,
  data: Vec<u8>,
  repeat_type: ImageRepeat,
  sample_type: ImageSample,
}


/*
enum {
  IMAGE_REPEAT_TILE   = 0,
  IMAGE_REPEAT_CLAMP  = 1,
  IMAGE_REPEAT_MIRROR = 2,
  IMAGE_REPEAT_ERROR  = 3,
  IMAGE_REPEAT_BLACK  = 4
};

*/

#[derive(Copy, Clone)]
pub enum ImageRepeat {
  ImageRepeatTile,
  ImageRepeatClamp,
  ImageRepeatMirror,
  ImageRepeatError,
  ImageRepeatBlack,
}

/*

enum {
  IMAGE_SAMPLE_LINEAR  = 0,
  IMAGE_SAMPLE_NEAREST = 1
};

*/

#[derive(Copy, Clone)]
pub enum ImageSample {
  ImageSampleLinear,
  ImageSampleNearest,
}

#[derive(Copy, Clone)]
enum ImageChannel {
  Red,
  Green,
  Blue,
  Alpha,
}

/*

image* image_new(int width, int height, unsigned char* data);
image* image_empty(int width, int height);
image* image_blank(int width, int height);
image* image_copy(image* src);
void image_delete(image* i);

vec4 image_get(image* i, int u, int v);
void image_set(image* i, int u, int v, vec4 c);
void image_map(image* i, vec4(*f)(vec4));

image* image_red_channel(image* src);
image* image_green_channel(image* src);
image* image_blue_channel(image* src);
image* image_alpha_channel(image* src);

void image_bgr_to_rgb(image* i);
void image_rotate_90_clockwise(image* i);
void image_rotate_90_counterclockwise(image* i);
void image_rotate_180(image* i);
void image_rotate_inplace(image* i, float amount);
void image_flip_horizontal(image* i);
void image_flip_vertical(image* i);

void image_fill(image* i, vec4 color);
void image_fill_black(image* i);
void image_fill_white(image* i);

void image_apply_gamma(image* i, float amount);
void image_to_gamma(image* i);
void image_from_gamma(image* i);

void image_rgb_to_hsv(image* i);
void image_hsv_to_rgb(image* i);
void image_hsv_scalar(image* i);

vec4 image_min(image* i);
vec4 image_max(image* i);
vec4 image_mean(image* i);
vec4 image_var(image* i);
vec4 image_std(image* i);
void image_auto_contrast(image* i);
void image_set_to_mask(image* i);
void image_set_brightness(image* i, float brightness);

vec4 image_alpha_mean(image* i);

image* image_get_subimage(image* src, int left, int top, int width, int height);

void image_set_subimage(image* i, int left, int top, image* src);
void image_paste_subimage(image* i, int left, int top, image* src);

vec4 image_sample(image* i, vec2 uv);
void image_paint(image* i, vec2 uv, vec4 color);

void image_scale(image* i, vec2 scale);

void image_mask_not(image* i);
void image_mask_binary(image* i0, image* i1, bool(*f)(bool,bool));
void image_mask_or(image* i0, image* i1);
void image_mask_and(image* i0, image* i1);
void image_mask_xor(image* i0, image* i1);
void image_mask_nor(image* i0, image* i1);
void image_mask_nand(image* i0, image* i1);
void image_mask_xnor(image* i0, image* i1);

image* image_mask_alpha(image* i);
image* image_mask_nearest(image* i);
image* image_mask_nearest(image* i);
image* image_mask_flood_fill(image* src, int u, int v, float tolerance);
image* image_mask_difference(image* src, vec4 color, float tolerance);

long image_mask_count(image* i);
void image_mask_median(image* i, int* u, int* v);
void image_mask_random(image* i, int* u, int* v);

image* image_read_from_file(char* filename);
image* image_tga_load_file(char* filename);
image* image_bmp_load_file(char* filename);

void image_write_to_file(image* i, char* filename);
void image_tga_save_file(image* i, char* filename);
void image_bmp_save_file(image* i, char* filename);

#endif

#include "assets/image.h"

#include "data/int_list.h"

image* image_new(int width, int height, unsigned char* data) {
  
  image* i = malloc(sizeof(image));
  i->data = malloc(width * height * 4);
  memcpy(i->data, data, width * height * 4);
  i->width = width;
  i->height = height;
  
  i->repeat_type = IMAGE_REPEAT_TILE;
  i->sample_type = IMAGE_SAMPLE_LINEAR;
  
  return i;
}

*/

pub fn image_new(width: i32, height: i32, data: &[u8]) -> Image {

  let mut data_new: Vec<u8> = vec![0; (width * height * 4) as usize]; //zero init for safety for now
  data_new.copy_from_slice(&data[0..((width * height * 4) as usize)]); //copy safely just the right size

  return Image {
    data: data_new,
    width: width,
    height: height,
    repeat_type: ImageRepeat::ImageRepeatTile,
    sample_type: ImageSample::ImageSampleLinear,
  };
}

/*

image* image_empty(int width, int height) {

  image* i = malloc(sizeof(image));
  i->data = malloc(width * height * 4);
  i->width = width;
  i->height = height;
  
  i->repeat_type = IMAGE_REPEAT_TILE;
  i->sample_type = IMAGE_SAMPLE_LINEAR;
  
  return i;
}

*/

pub fn image_empty(width: i32, height: i32) -> Image {

  let data_new: Vec<u8> = vec![0; (width * height * 4) as usize]; //zero init for safety for now

  return Image {
    data: data_new,
    width: width,
    height: height,
    repeat_type: ImageRepeat::ImageRepeatTile,
    sample_type: ImageSample::ImageSampleLinear,
  };
}

/*

image* image_blank(int width, int height) {
  
  image* i = malloc(sizeof(image));
  i->data = calloc(width * height * 4, 1);
  i->width = width;
  i->height = height;
  
  i->repeat_type = IMAGE_REPEAT_TILE;
  i->sample_type = IMAGE_SAMPLE_LINEAR;
  
  return i;
}

*/

pub fn image_blank(width: i32, height: i32) -> Image {

  let data_new: Vec<u8> = vec![0; (width * height * 4) as usize]; //zero init for safety for now

  return Image {
    data: data_new,
    width: width,
    height: height,
    repeat_type: ImageRepeat::ImageRepeatTile,
    sample_type: ImageSample::ImageSampleLinear,
  };
}


/*

image* image_copy(image* src) {

  image* i = malloc(sizeof(image));
  i->data = malloc(src->width * src->height * 4);
  memcpy(i->data, src->data, src->width * src->height * 4);
  i->width = src->width;
  i->height = src->height;
  
  i->repeat_type = src->repeat_type;
  i->sample_type = src->sample_type;

  return i;
  
}

*/

pub fn image_copy(src: &Image) -> Image {

  let width = src.width;
  let height = src.height;

  let mut data_new: Vec<u8> = vec![0; (width * height * 4) as usize]; //zero init for safety for now
  data_new.copy_from_slice(&(src.data[0..((width * height * 4) as usize)])); //copy safely just the right size

  return Image {
    data: data_new,
    width: width,
    height: height,
    repeat_type: src.repeat_type,
    sample_type: src.sample_type,
  };
}



/*

void image_delete(image* i) {
  free(i->data);
  free(i);
}
*/

pub fn image_delete(_: Image) {}

/*

static int image_wrap(int u, int m, int type) {
  
  if (u < 0 || u >= m) {
  
    switch (type) {
      case IMAGE_REPEAT_TILE:
        while (u < 0) { u = m - 1 - u; }
        return u % m;
      break;
      
      case IMAGE_REPEAT_CLAMP:
        return u < 0 ? 0 : (u >= m ? m-1 : u);
      break;
      
      case IMAGE_REPEAT_MIRROR:
        u = abs(u) % (m * 2);
        return u >= m ? m - 1 - u : u;
      break;
      
      case IMAGE_REPEAT_ERROR:
        error("Image access out of bounds!");
      break;
      
      case IMAGE_REPEAT_BLACK:
        return -1;
      break;
    }
    
  }
  
  return u;
  
}
*/

pub fn image_wrap(u: i32, m: i32, repeat_type: &ImageRepeat) -> i32 {
  let mut u = u;
  if u < 0 || u >= m {
    match *repeat_type {
      ImageRepeat::ImageRepeatTile => {
        while u < 0 {
          u = m - 1 - u;
        }
        return u % m;
      }
      ImageRepeat::ImageRepeatClamp => {
        if u < 0 {
          return 0;
        }
        if u >= m {
          return m - 1;
        }
        return u;
      }
      ImageRepeat::ImageRepeatMirror => {
        u = u.abs() % (m * 2);
        if u >= m {
          return m - 1 - u;
        }
        return u;
      }
      ImageRepeat::ImageRepeatError => {
        error("Image access out of bounds!");
      }
      ImageRepeat::ImageRepeatBlack => {
        return -1;
      }
    }
  }

  return u;
}


/*

vec4 image_get(image* i, int u, int v) {

  u = image_wrap(u, i->width,  i->repeat_type);
  v = image_wrap(v, i->height, i->repeat_type);
  
  if (u == -1) { return vec4_zero(); }
  if (v == -1) { return vec4_zero(); }
  
  float r = (float)i->data[u * 4 + v * i->width * 4 + 0] / 255;
  float g = (float)i->data[u * 4 + v * i->width * 4 + 1] / 255;
  float b = (float)i->data[u * 4 + v * i->width * 4 + 2] / 255;
  float a = (float)i->data[u * 4 + v * i->width * 4 + 3] / 255;

  return vec4_new(r, g, b, a);
}

*/

pub fn image_get(i: &Image, u: i32, v: i32) -> Vec4 {
  let u = image_wrap(u, i.width, &i.repeat_type);
  let v = image_wrap(v, i.height, &i.repeat_type);

  if u == -1 {
    return Vec4::zero();
  }

  if v == -1 {
    return Vec4::zero();
  }

  let r = (i.data[(u * 4 + v * i.width + 0) as usize] as f32) / 255.0;
  let g = (i.data[(u * 4 + v * i.width + 1) as usize] as f32) / 255.0;
  let b = (i.data[(u * 4 + v * i.width + 2) as usize] as f32) / 255.0;
  let a = (i.data[(u * 4 + v * i.width + 3) as usize] as f32) / 255.0;

  return Vec4::new(r, g, b, a);
}

/*
void image_set(image* i, int u, int v, vec4 c) {
  
  u = image_wrap(u, i->width,  i->repeat_type);
  v = image_wrap(v, i->height, i->repeat_type);
  
  if (u == -1) { return; }
  if (v == -1) { return; }
  
  i->data[u * 4 + v * i->width * 4 + 0] = (c.x * 255);
  i->data[u * 4 + v * i->width * 4 + 1] = (c.y * 255);
  i->data[u * 4 + v * i->width * 4 + 2] = (c.z * 255);
  i->data[u * 4 + v * i->width * 4 + 3] = (c.w * 255);
}

void image_map(image* i, vec4(*f)(vec4)) {
  for (int x = 0; x < i->width; x++)
  for (int y = 0; y < i->height; y++) {
    image_set(i, x, y, f(image_get(i, x, y)));
  }
}
*/

pub fn image_set(i: &mut Image, u: i32, v: i32, c: &Vec4) {
  let u = image_wrap(u, i.width, &i.repeat_type);
  let v = image_wrap(v, i.height, &i.repeat_type);

  if u == -1 {
    return;
  }

  if v == -1 {
    return;
  }

  i.data[(u * 4 + v * i.width * 4 + 0) as usize] = (c.x * 255.0) as u8;
}

/*


static image* image_channel(image* src, int chan) {
  image* new = image_empty(src->width, src->height);
  
  for (int x = 0; x < src->width; x++)
  for (int y = 0; y < src->height; y++) {
    vec4 c = image_get(src, x, y);
    
    switch (chan) {
      case 0: image_set(new, x, y, vec4_new(c.x,c.x,c.x,c.x)); break;
      case 1: image_set(new, x, y, vec4_new(c.y,c.y,c.y,c.y)); break;
      case 2: image_set(new, x, y, vec4_new(c.z,c.z,c.z,c.z)); break;
      case 3: image_set(new, x, y, vec4_new(c.w,c.w,c.w,c.w)); break;
    }
    
  }
  
  return new;
}

*/

fn image_channel(src: &Image, chan: &ImageChannel) -> Image {
  let mut new = image_empty(src.width, src.height);

  for x in 0..src.width {
    for y in 0..src.height {
      let c = image_get(src, x, y);

      match *chan {
        ImageChannel::Red => image_set(&mut new, x, y, &Vec4::new(c.x, c.x, c.x, c.x)),
        ImageChannel::Green => image_set(&mut new, x, y, &Vec4::new(c.y, c.y, c.y, c.y)),
        ImageChannel::Blue => image_set(&mut new, x, y, &Vec4::new(c.z, c.z, c.z, c.z)),
        ImageChannel::Alpha => image_set(&mut new, x, y, &Vec4::new(c.w, c.w, c.w, c.w)),
      }
    }
  }

  return new;
}


/*

image* image_red_channel(image* src)   { return image_channel(src, 0); }
image* image_green_channel(image* src) { return image_channel(src, 1); }
image* image_blue_channel(image* src)  { return image_channel(src, 2); }
image* image_alpha_channel(image* src) { return image_channel(src, 3); }

*/

pub fn image_red_channel(src: &Image) -> Image {
  return image_channel(src, &ImageChannel::Red);
}

pub fn image_green_channel(src: &Image) -> Image {
  return image_channel(src, &ImageChannel::Green);
}

pub fn image_blue_channel(src: &Image) -> Image {
  return image_channel(src, &ImageChannel::Blue);
}

pub fn image_alpha_channel(src: &Image) -> Image {
  return image_channel(src, &ImageChannel::Alpha);
}

/*
void image_bgr_to_rgb(image* i) {
  for (int x = 0; x < i->width; x++)
  for (int y = 0; y < i->height; y++) {
    vec4 p = image_get(i, x, y);
    image_set(i, x, y, vec4_new(p.z, p.y, p.x, p.w));
  } 
}
*/

pub fn image_bgr_to_rgb(i: &mut Image) {
  for x in 0..i.width {
    for y in 0..i.height {
      let p = image_get(i, x, y);
      image_set(i, x, y, &Vec4::new(p.z, p.y, p.x, p.w));
    }
  }
}

/*
static void image_data_swap(void **x, void **y) {
	void *t = *x;
	*x = *y;
	*y = t;
}
*/

/*

void image_rotate_90_clockwise(image* i) {
  
  image* j = image_blank(i->height, i->width);
  
  for (int x = 0; x < i->width; x++)
  for (int y = 0; y < i->height; y++) {
    image_set(j, x, y, image_get(i, (j->height-1) - y, x));
  }
  
  i->width  = j->width;
  i->height = j->height;
  image_data_swap((void**)&i->data, (void**)&j->data);
  image_delete(j);
  
}
*/

pub fn image_rotate_90_clockwise(i: &mut Image) {
  let mut j = image_blank(i.height, i.width);
  let j_height = j.height;

  for x in 0..i.width {
    for y in 0..i.height {
      image_set(&mut j, x, y, &image_get(i, (j_height - 1) - y, x));
    }
  }

  i.width = j.width;
  i.height = j.height;

  i.data.swap_with_slice(&mut j.data);

  image_delete(j);
}

/*

void image_rotate_90_counterclockwise(image* i) {

  image* j = image_blank(i->height, i->width);
  
  for (int x = 0; x < i->width; x++)
  for (int y = 0; y < i->height; y++) {
    image_set(j, x, y, image_get(i, y, (j->width-1) - x));
  }
  
  i->width  = j->width;
  i->height = j->height;
  image_data_swap((void**)&i->data, (void**)&j->data);
  image_delete(j);

}
*/

pub fn image_rotate_90_counterclockwise(i: &mut Image) {
  let mut j = image_blank(i.height, i.width);
  let j_width = j.width;

  for x in 0..i.width {
    for y in 0..i.height {
      image_set(&mut j, x, y, &image_get(i, y, (j_width - 1) - x));
    }
  }

  i.width = j.width;
  i.height = j.height;

  i.data.swap_with_slice(&mut j.data);

  image_delete(j);
}

/*
void image_rotate_inplace(image* i, float amount) {
  
  image* j = image_blank(i->height, i->width);
  int repeat = i->repeat_type;
  i->repeat_type = IMAGE_REPEAT_BLACK;
    
  for (int x = 0; x < i->width; x++)
  for (int y = 0; y < i->height; y++) {
    float u = ((float)x / i->width)  - 0.5;
    float v = ((float)y / i->height) - 0.5;
    vec2 uv = mat2_mul_vec2(mat2_rotation(amount), vec2_new(u, v));
    uv = vec2_add(uv, vec2_new(0.5, 0.5));
    image_set(j, x, y, image_sample(i, uv));
  }
  
  i->repeat_type = repeat;
  image_data_swap((void**)&i->data, (void**)&j->data);
  image_delete(j);
  
}
*/

pub fn image_rotate_inplace(i: &mut Image, amount: f32) {
  let mut j = image_blank(i.height, i.width);
  let repeat = i.repeat_type;
  i.repeat_type = ImageRepeat::ImageRepeatBlack;
  let rotation: Basis2<f32> = Rotation2::from_angle(Deg(amount));
  let offset = Dir2::new(0.5, 0.5);

  for x in 0..i.width {
    for y in 0..j.height {
      let u = (x as f32) / (i.width as f32) - 0.5;
      let v = (y as f32) / (i.height as f32) - 0.5;
      let uv = rotation.rotate_vector(Vec2::new(u, v)).add(offset);
      image_set(&mut j, x, y, &image_sample(i, &uv));
    }
  }

  i.repeat_type = repeat;
  i.data.swap_with_slice(&mut j.data);
  image_delete(j);
}



/*

void image_rotate_180(image* i) {
  image_flip_vertical(i);
  image_flip_horizontal(i);
}

*/

pub fn image_rotate_180(i: &mut Image) {
  image_flip_vertical(i);
  image_flip_horizontal(i);
}

/*

void image_flip_horizontal(image* i) {

  for (int y = 0; y < i->height;    y++)
  for (int x = 0; x < i->width / 2; x++) {
     vec4 l = image_get(i, x, y);
     vec4 r = image_get(i, (i->width-1) - x, y);
     image_set(i, x, y, r);
     image_set(i, (i->width-1) - x, y, l);
  }

}

*/

pub fn image_flip_horizontal(i: &mut Image) {
  for y in 0..i.height {
    for x in 0..(i.width / 2) {
      let l = image_get(i, x, y);
      let r = image_get(i, (i.width - 1) - x, y);
      image_set(i, x, y, &r);
      let i_width = i.width;
      image_set(i, (i_width - 1) - x, y, &l);
    }
  }
}

/*
void image_flip_vertical(image* i) {

  for (int x = 0; x < i->width;      x++)
  for (int y = 0; y < i->height / 2; y++) {
     vec4 t = image_get(i, x, y);
     vec4 b = image_get(i, x, (i->height-1) - y);
     image_set(i, x, y, b);
     image_set(i, x, (i->height-1) - y, t);
  }

}
*/

fn image_flip_vertical(i: &mut Image) {
  for x in 0..i.width {
    for y in 0..(i.height / 2) {
      let t = image_get(i, x, y);
      let b = image_get(i, x, (i.height - 1) - y);
      image_set(i, x, y, &b);
      let i_height = i.height;
      image_set(i, x, (i_height - 1) - y, &t);
    }
  }
}

/*

void image_fill(image* i, vec4 color) {
  for (int x = 0; x < i->width;  x++)
  for (int y = 0; y < i->height; y++) {
    image_set(i, x, y, color);
  }
}

*/

pub fn image_fill(i: &mut Image, color: &Vec4) {
  for x in 0..i.width {
    for y in 0..i.height {
      image_set(i, x, y, color);
    }
  }
}

/*
///wouldn't this be transparent??
void image_fill_black(image* i) {
  memset(i->data, 0, i->width * i->height * 4);
}

*/


pub fn image_fill_black(i: &mut Image) {
  i.data.resize(0, 0);
  i.data.resize((i.width * i.height * 4) as usize, 0);
}

/*
void image_fill_white(image* i) {
  memset(i->data, 255, i->width * i->height * 4);
}
*/

pub fn image_fill_white(i: &mut Image) {
  i.data.resize(0, 255);
  i.data.resize((i.width * i.height * 4) as usize, 255);
}

/*
void image_apply_gamma(image* i, float amount) {
  for (int x = 0; x < i->width;  x++)
  for (int y = 0; y < i->height; y++) {
    image_set(i, x, y, vec4_pow(image_get(i, x, y), amount));
  }
}
*/

pub fn image_apply_gamma(i: &mut Image, amount: f32) {
  for x in 0..i.width {
    for y in 0..i.height {
      let set = image_get(i, x, y).vec4_pow(amount);
      image_set(i, x, y, &set); //is there a powf trait I can use in some way?
    }
  }
}

/*

void image_to_gamma(image* i) { image_apply_gamma(i, 2.2); }
void image_from_gamma(image* i) { image_apply_gamma(i, 1.0/2.2); }
*/

pub fn image_to_gamma(i: &mut Image) {
  image_apply_gamma(i, 2.2);
}

pub fn image_from_gamma(i: &mut Image) {
  image_apply_gamma(i, 1.0 / 2.2);
}

/*
void image_rgb_to_hsv(image* i) {
  
  for (int x = 0; x < i->width;  x++)
  for (int y = 0; y < i->height; y++) {
    
      vec4 curr = image_get(i, x, y);
      vec4 rslt = curr;

      float vmin = min(min(curr.x, curr.y), curr.z);
      float vmax = max(max(curr.x, curr.y), curr.z);
      float vdif = (vmax - vmin) + 0.0001;
      
      if (curr.x == vmax) {
        rslt.x = (curr.y - curr.z) / vdif;
        if (rslt.x < 0) { rslt.x += 6; }
      } else if (curr.y == vmax) {
        rslt.x = ((curr.z - curr.x) / vdif) + 2;
      } else {
        rslt.x = ((curr.x - curr.y) / vdif) + 4;
      }
      
      rslt.x = (rslt.x * 60) / 360.0;
      rslt.y = vdif / (vmax + 0.0001);
      rslt.z = vmax;
      
      image_set(i, x, y, rslt);
  
  }
  
}
*/

pub fn image_rgb_to_hsv(i: &mut Image) {
  for x in 0..i.width {
    for y in 0..i.height {
      let curr = image_get(i, x, y);
      let mut rslt = curr;

      let vmin = curr.x.min(curr.y.min(curr.z));
      let vmax = curr.x.max(curr.y.max(curr.z));

      let vdif = (vmax - vmin) + 0.0001;

      if curr.x == vmax {
        rslt.x = (curr.y - curr.z) / vdif;
        if rslt.x < 0.0 {
          rslt.x = rslt.x + 6.0;
        }
      } else if curr.y == vmax {
        rslt.x = ((curr.z - curr.x) / vdif) + 2.0;
      } else {
        rslt.x = ((curr.x - curr.y) / vdif) + 4.0;
      }

      rslt.x = (rslt.x * 60.0) / 360.0;
      rslt.y = vdif / (vmax + 0.0001);
      rslt.z = vmax;

      image_set(i, x, y, &rslt);
    }
  }
}


/*

void image_hsv_to_rgb(image* i) {

  for (int x = 0; x < i->width;  x++)
  for (int y = 0; y < i->height; y++) {
    
    vec4 curr = image_get(i, x, y);
    
    if (curr.y <= 0) {
      image_set(i, x, y, vec4_new(curr.z, curr.z, curr.z, curr.w));
      continue;
    }
    
    float hh = (curr.x * 360) / 60;
    
    long j = (long)hh;
    float ff = hh - j;
    float p = curr.z * (1 -  curr.y);
    float q = curr.z * (1 - (curr.y * ff));
    float t = curr.z * (1 - (curr.y * (1 - ff)));

    switch(j) {
      case 0:  curr = vec4_new(curr.z, t, p, curr.w); break;
      case 1:  curr = vec4_new(q, curr.z, p, curr.w); break;
      case 2:  curr = vec4_new(p, curr.z, t, curr.w); break;
      case 3:  curr = vec4_new(p, q, curr.z, curr.w); break;
      case 4:  curr = vec4_new(t, p, curr.z, curr.w); break;
      case 5:  curr = vec4_new(curr.z, p, q, curr.w); break;
      default: curr = vec4_zero(); break;
    }
    
    image_set(i, x, y, curr);
    
  }

}

*/

pub fn image_hsv_to_rgb(i: &mut Image) {
  for x in 0..i.width {
    for y in 0..i.height {
      let mut curr = image_get(i, x, y);

      if curr.y <= 0.0 {
        image_set(i, x, y, &Vec4::new(curr.z, curr.z, curr.z, curr.w));
        continue;
      }

      let hh = (curr.x * 360.0) / 60.0;

      let j: i64 = hh as i64; //does this round? I hope not. also i64 == long?
      let ff = hh - (j as f32);
      let p = curr.z * (1.0 - curr.y);
      let q = curr.z * (1.0 - (curr.y * ff));
      let t = curr.z * (1.0 - (curr.y * (1.0 - ff)));

      match j {
        0 => {
          curr = Vec4::new(curr.z, t, p, curr.w);
        }
        1 => {
          curr = Vec4::new(q, curr.z, p, curr.w);
        }
        2 => {
          curr = Vec4::new(p, curr.z, t, curr.w);
        }
        3 => {
          curr = Vec4::new(p, q, curr.z, curr.w);
        }
        4 => {
          curr = Vec4::new(t, p, curr.z, curr.w);
        }
        5 => {
          curr = Vec4::new(curr.z, p, q, curr.w);
        }
        _ => {
          curr = Vec4::zero();
        } //does this zero? I think so..
      }

      image_set(i, x, y, &curr);
    }
  }
}



/*

void image_hsv_scalar(image* i) {

  for (int x = 0; x < i->width;  x++)
  for (int y = 0; y < i->height; y++) {
    
    vec4 curr = image_get(i, x, y);
    float amount = (curr.x + curr.y * 2 + curr.z * 5) / 8;
    image_set(i, x, y, vec4_new(amount, amount, amount, amount));
  }

}

*/

pub fn image_hsv_scalar(i: &mut Image) {
  for x in 0..i.width {
    for y in 0..i.height {
      let curr = image_get(i, x, y);
      let amount = (curr.x + curr.y * 2.0 + curr.z * 5.0) / 8.0;
      image_set(i, x, y, &Vec4::new(amount, amount, amount, amount));
    }
  }
}


/*

vec4 image_min(image* i) {
  vec4 vmin = vec4_one();

  for (int x = 0; x < i->width;  x++)
  for (int y = 0; y < i->height; y++) {
    vmin = vec4_min(vmin, image_get(i, x, y));
  }

  return vmin;
}

*/

pub fn image_min(i: &Image) -> Vec4 {
  let mut vmin = Vec4::vec4_one();

  for x in 0..i.width {
    for y in 0..i.height {
      vmin = vmin.vec4_min(&image_get(i, x, y));
    }
  }

  return vmin;
}

/*

vec4 image_max(image* i) {
  vec4 vmax = vec4_one();

  for (int x = 0; x < i->width;  x++)
  for (int y = 0; y < i->height; y++) {
    vmax = vec4_max(vmax, image_get(i, x, y));
  }

  return vmax;
}

*/

pub fn image_max(i: &Image) -> Vec4 {
  let mut vmax = Vec4::vec4_one(); //why is this one? shouldn't it be zero??

  for x in 0..i.width {
    for y in 0..i.height {
      vmax = vmax.vec4_max(&image_get(i, x, y));
    }
  }

  return vmax;
}


/*
vec4 image_mean(image* i) {
  vec4 total = vec4_zero();
  for (int x = 0; x < i->width;  x++)
  for (int y = 0; y < i->height; y++) {
    total = vec4_add(total, vec4_div(image_get(i, x, y), i->width * i->height));
  }
  return total;
}
*/

pub fn image_mean(i: &Image) -> Vec4 {
  let mut total = Vec4::zero();

  for x in 0..i.width {
    for y in 0..i.height {
      total = total.add(image_get(i, x, y).div((i.width * i.height) as f32));
    }
  }
  return total;
}


/*
vec4 image_var(image* i) {
  vec4 mean = image_mean(i);
  vec4 total = vec4_zero();
  for (int x = 0; x < i->width;  x++)
  for (int y = 0; y < i->height; y++) {
    vec4 diff = vec4_sub(image_get(i, x, y), mean);
    total = vec4_add(total, vec4_div(vec4_mul_vec4(diff, diff), i->width * i->height));
  }
  return total;
}
*/

pub fn image_var(i: &Image) -> Vec4 {
  let mean = image_mean(i);
  let mut total = Vec4::zero();

  for x in 0..i.width {
    for y in 0..i.height {
      let diff = image_get(i, x, y).sub(mean);
      total = total.add(diff.mul_element_wise(diff).div((i.width * i.height) as f32));
    }
  }

  return total;
}

/*

vec4 image_std(image* i) {
  return vec4_sqrt(image_var(i));
}

*/

pub fn image_std(i: &Image) -> Vec4 {
  return image_var(i).vec4_sqrt();
}

/*

void image_auto_contrast(image* i) {
  vec4 mean = image_mean(i);
  vec4 std  = image_std(i);
  
  float smean = vec4_dot(mean, vec4_new(0.33, 0.33, 0.33, 0));
  float sstd  = vec4_dot(std, vec4_new(0.33, 0.33, 0.33, 0));
  
  for (int x = 0; x < i->width;  x++)
  for (int y = 0; y < i->height; y++) {
    vec4 curr = image_get(i, x, y);
    curr = vec4_sub(curr, vec4_new(smean, smean, smean, 0));
    curr = vec4_div(curr, sstd*4);
    curr = vec4_saturate(vec4_add(curr, vec4_new(0.5, 0.5, 0.5, 0)));
    image_set(i, x, y, curr);
  }
}

*/

pub fn image_auto_contrast(i: &mut Image) {
  let mean = image_mean(i);
  let std = image_std(i);

  let smean = mean.dot(Vec4::new(0.33, 0.33, 0.33, 0.0));
  let sstd = std.dot(Vec4::new(0.33, 0.33, 0.33, 0.0));

  for x in 0..i.width {
    for y in 0..i.height {
      let mut curr = image_get(i, x, y);
      curr = curr.sub(Vec4::new(smean, smean, smean, 0.0));
      curr = curr.div(sstd * 4.0);
      curr = curr.add(Vec4::new(0.5, 0.5, 0.5, 0.0)).vec4_saturate();
      image_set(i, x, y, &curr);
    }
  }
}

/*

vec4 image_alpha_mean(image* i) {
  vec4  total = vec4_zero();
  float scale = 0.0;
  
  for (int x = 0; x < i->width;  x++)
  for (int y = 0; y < i->height; y++) {
    vec4 c = image_get(i, x, y);
    c.x = c.x * c.w;
    c.y = c.y * c.w;
    c.z = c.z * c.w;
    total = vec4_add(total, c);
    scale = scale + c.w;
  }
  
  return vec4_div(total, scale);
}

*/

pub fn image_alpha_mean(i: &Image) -> Vec4 {
  let mut total = Vec4::zero();
  let mut scale: f32 = 0.0;

  for x in 0..i.width {
    for y in 0..i.height {
      let mut c = image_get(i, x, y);
      c.x = c.x * c.w;
      c.y = c.y * c.w;
      c.z = c.z * c.w;
      total = total.add(c);
      scale = scale + c.w;
    }
  }

  return total.div(scale);
}


/*

void image_set_brightness(image* i, float b) {
  for (int x = 0; x < i->width;  x++)
  for (int y = 0; y < i->height; y++) {
    image_set(i, x, y, vec4_saturate(vec4_add(image_get(i, x, y), vec4_new(b, b, b, b))));
  }
}

*/

pub fn image_set_brightness(i: &mut Image, b: f32) {
  for x in 0..i.width {
    for y in 0..i.height {
      let set = image_get(i, x, y)
        .add(Vec4::new(b, b, b, b))
        .vec4_saturate();
      image_set(i, x, y, &set);
    }
  }
}

/*

void image_set_to_mask(image* i) {
  for (int x = 0; x < i->width;  x++)
  for (int y = 0; y < i->height; y++) {
    float v = vec4_dot(image_get(i, x, y), vec4_new(0.333, 0.333, 0.333, 0));
    if (v > 0.5) {
      image_set(i, x, y, vec4_one());
    } else {
      image_set(i, x, y, vec4_zero());
    }
  }
}

*/

pub fn image_set_to_mask(i: &mut Image) {
  for x in 0..i.width {
    for y in 0..i.height {
      let v = image_get(i, x, y).dot(Vec4::new(0.333, 0.333, 0.333, 0.0));
      if v > 0.5 {
        image_set(i, x, y, &Vec4::vec4_one());
      } else {
        image_set(i, x, y, &Vec4::zero());
      }
    }
  }
}

/*

void image_set_subimage(image* i, int left, int top, image* src) {

  for (int x = left; x < left + src->width;  x++)
  for (int y = top;  y < top  + src->height; y++) {
    image_set(i, x, y, image_get(src, x-left, y-top));
  }

}

*/

pub fn image_set_subimage(i: &mut Image, left: i32, top: i32, src: &Image) {
  for x in left..left + src.width {
    for y in top..top + src.height {
      image_set(i, x, y, &image_get(src, x - left, y - top));
    }
  }
}

/*

void image_paste_subimage(image* i, int left, int top, image* src) {

  for (int x = left; x < left + src->width;  x++)
  for (int y = top;  y < top  + src->height; y++) {
    vec4 t = image_get(src, x-left, y-top);
    vec4 b = image_get(i,   x,      y);
    vec3 c = vec3_lerp(vec3_new(b.x, b.y, b.z), vec3_new(t.x, t.y, t.z), t.w);
    image_set(i, x, y, vec4_new(c.x, c.y, c.z, t.w));
  }

}

*/

pub fn image_paste_subimage(i: &mut Image, left: i32, top: i32, src: &Image) {
  for x in left..left + src.width {
    for y in top..top + src.height {
      let t = image_get(src, x - left, y - top);
      let b = image_get(i, x, y);
      let c = Vec3::new(b.x, b.y, b.z).vec3_lerp(&Vec3::new(t.x, t.y, t.z), t.w);
      image_set(i, x, y, &Vec4::new(c.x, c.y, c.z, t.w));
    }
  }
}


/*

image* image_get_subimage(image* src, int left, int top, int width, int height) {
  
  image* i = image_blank(width, height);
  i->repeat_type = src->repeat_type;
  i->sample_type = src->sample_type;
  
  for (int x = 0; x < i->width;  x++)
  for (int y = 0; y < i->height; y++) {
    image_set(i, x, y, image_get(src, left+x, top+y));
  }
  
  return i;
}

*/

pub fn image_get_subimage(src: &Image, left: i32, top: i32, width: i32, height: i32) -> Image {
  let mut i = image_blank(width, height);
  i.repeat_type = src.repeat_type;
  i.sample_type = src.sample_type;

  for x in 0..i.width {
    for y in 0..i.height {
      image_set(&mut i, x, y, &image_get(src, left + x, top + y));
    }
  }
  return i;
}

/*

vec4 image_sample(image* i, vec2 uv) {

  float u = i->width  * uv.x;
  float v = i->height * uv.y;
  
  int s1_u, s1_v, s2_u, s2_v, s3_u, s3_v, s4_u, s4_v;  
  s1_u = floor(u); s1_v = floor(v);
  s2_u =  ceil(u); s2_v = floor(v);
  s3_u = floor(u); s3_v = ceil(v);
  s4_u =  ceil(u); s4_v = ceil(v);
  
  float amount_x = fmod(u, 1.0);
  float amount_y = fmod(v, 1.0);
  
  vec4 s1, s2, s3, s4;
  s1 = image_get(i, s1_u, s1_v);
  s2 = image_get(i, s2_u, s2_v);
  s3 = image_get(i, s3_u, s3_v);
  s4 = image_get(i, s4_u, s4_v);
  
  switch (i->sample_type) {
    case IMAGE_SAMPLE_LINEAR:  return vec4_bilinear_interp(s1, s2, s3, s4, amount_x, amount_y);
    case IMAGE_SAMPLE_NEAREST: return vec4_binearest_interp(s1, s2, s3, s4, amount_x, amount_y);
    default: return vec4_zero();
  }
}

*/

pub fn image_sample(i: &Image, uv: &Vec2) -> Vec4 {
  let u = (i.width as f32) * uv.x;
  let v = (i.height as f32) * uv.y;

  let s1_u = u.floor();
  let s2_u = u.ceil();
  let s3_u = u.floor();
  let s4_u = u.ceil();

  let s1_v = v.floor();
  let s2_v = v.floor();
  let s3_v = v.ceil();
  let s4_v = v.ceil();

  let amount_x = u % 1.0;
  let amount_y = v % 1.0;

  let s1 = image_get(i, s1_u as i32, s1_v as i32);
  let s2 = image_get(i, s2_u as i32, s2_v as i32);
  let s3 = image_get(i, s3_u as i32, s3_v as i32);
  let s4 = image_get(i, s4_u as i32, s4_v as i32);

  match i.sample_type {
    ImageSample::ImageSampleLinear => {
      return Vec4::vec4_bilinear_interp(&s1, &s2, &s3, &s4, amount_x, amount_y);
    }
    ImageSample::ImageSampleNearest => {
      return Vec4::vec4_binearest_interp(&s1, &s2, &s3, &s4, amount_x, amount_y);
    }
  }
}

/*

void image_paint(image* i, vec2 uv, vec4 color) {
  
  float u = i->width * uv.x;
  float v = i->height * uv.y;
  
  int s1_u, s1_v, s2_u, s2_v, s3_u, s3_v, s4_u, s4_v;  
  s1_u = floor(u); s1_v = floor(v);
  s2_u = ceil(u);  s2_v = floor(v);
  s3_u = floor(u); s3_v = ceil(v);
  s4_u = ceil(u);  s4_v = ceil(v);
  
  float amount_x = fmod(u, 1.0);
  float amount_y = fmod(v, 1.0);
  
  vec4 s1, s2, s3, s4;
  s1 = image_get(i, s1_u, s1_v);
  s2 = image_get(i, s2_u, s2_v);
  s3 = image_get(i, s3_u, s3_v);
  s4 = image_get(i, s4_u, s4_v);
  
  switch (i->sample_type) {
    
    case IMAGE_SAMPLE_LINEAR:
    
      s1 = vec4_lerp(s1, color, (1-amount_x + 1-amount_y)/2);
      s2 = vec4_lerp(s2, color, (amount_x + 1-amount_y)/2);
      s3 = vec4_lerp(s3, color, (amount_x + amount_y)/2);
      s3 = vec4_lerp(s4, color, (1-amount_x + amount_y)/2);
      
      image_set(i, s1_u, s1_v, s1);
      image_set(i, s2_u, s2_v, s2);
      image_set(i, s3_u, s3_v, s3);
      image_set(i, s4_u, s4_v, s4);
    
    break;
    
    case IMAGE_SAMPLE_NEAREST:
    
      s1 = vec4_nearest_interp(s1, color, (1-amount_x + 1-amount_y)/2);
      s2 = vec4_nearest_interp(s2, color, (amount_x + 1-amount_y)/2);
      s3 = vec4_nearest_interp(s3, color, (amount_x + amount_y)/2);
      s3 = vec4_nearest_interp(s4, color, (1-amount_x + amount_y)/2);
      
      image_set(i, s1_u, s1_v, s1);
      image_set(i, s2_u, s2_v, s2);
      image_set(i, s3_u, s3_v, s3);
      image_set(i, s4_u, s4_v, s4);  
    
    break;
    
  }
  
}

void image_scale(image* i, vec2 scale) {
  
  image* j = image_empty( i->width * scale.x , i->height * scale.y );
  
  for (int x = 0; x < j->width;  x++)
  for (int y = 0; y < j->height; y++) {
     image_set(j, x, y, image_sample( i, vec2_new((float)x / j->width, (float)y / j->height)));
  }
  
  i->width = j->width;
  i->height = j->height;
  image_data_swap((void**)&i->data, (void**)&j->data);
  image_delete(j);
}

void image_mask_not(image* i) {
  for (int x = 0; x < i->width;  x++) 
  for (int y = 0; y < i->height; y++) {
    if (image_get(i, x, y).w == 1.0) {
      image_set(i, x, y, vec4_zero());
    } else {
      image_set(i, x, y, vec4_one());
    }
  }
}

void image_mask_binary(image* i0, image* i1, bool(*f)(bool,bool)) {
  
  for(int x = 0; x < i0->width;  x++) 
  for(int y = 0; y < i0->height; y++) {
    if (f((image_get(i0, x, y).w == 1.0), 
          (image_get(i1, x, y).w == 1.0))) {
      image_set(i0, x, y, vec4_one());
    } else {
      image_set(i0, x, y, vec4_zero());
    }
  }
}

static bool image_op_and(bool x, bool y)  { return   x && y; }
static bool image_op_or(bool x, bool y)   { return   x || y; }
static bool image_op_xor(bool x, bool y)  { return   x != y; }
static bool image_op_nor(bool x, bool y)  { return !(x || y); }
static bool image_op_nand(bool x, bool y) { return !(x && y); }
static bool image_op_xnor(bool x, bool y) { return !(x != y); }

void image_mask_or(image* i0, image* i1)   { image_mask_binary(i0, i1, image_op_or); }
void image_mask_and(image* i0, image* i1)  { image_mask_binary(i0, i1, image_op_and); }
void image_mask_xor(image* i0, image* i1)  { image_mask_binary(i0, i1, image_op_xor); }
void image_mask_nor(image* i0, image* i1)  { image_mask_binary(i0, i1, image_op_nor); }
void image_mask_nand(image* i0, image* i1) { image_mask_binary(i0, i1, image_op_nand); }
void image_mask_xnor(image* i0, image* i1) { image_mask_binary(i0, i1, image_op_xnor); }

image* image_mask_alpha(image* i) {
  image* j = image_blank(i->width, i->height);
  for (int x = 0; x < i->width;  x++) 
  for (int y = 0; y < i->height; y++) {
    if (image_get(i, x, y).w > 0.5) {
      image_set(j, x, y, vec4_one());
    } else {
      image_set(j, x, y, vec4_zero());
    }
  }
  return j;
}

image* image_mask_threshold(image* i, float amount) {

  image* j = image_blank(i->width, i->height);
  for (int x = 0; x < i->width;  x++) 
  for (int y = 0; y < i->height; y++) {
    if (vec4_dot(image_get(i, x, y), 
        vec4_new(0.333, 0.333, 0.333, 0.0)) >= amount) {
      image_set(j, x, y, vec4_one());
    } else {
      image_set(j, x, y, vec4_zero());
    }
  }
  return j;
}

image* image_mask_nearest(image* i) { return image_mask_threshold(i, 0.5); }

image* image_flood_fill_mask(image* src, int u, int v, float tolerance) {
  
  image* mask = image_blank(src->width, src->height);
  
  int_list* q_x = int_list_new();
  int_list* q_y = int_list_new();
  
  vec4 base = image_get(src, u, v);
  float base_val = (base.x + base.y + base.z + base.w) / 4;
  
  int_list_push_back(q_x, u);
  int_list_push_back(q_y, v);
  
  while ( !int_list_is_empty(q_x) ) {
    
    int u = int_list_pop_back(q_x);
    int v = int_list_pop_back(q_y);
    
    image_set(mask, u, v, vec4_one() );
    
    if (u > 0) {
      vec4 left = image_get(src, u-1, v);
      vec4 left_mask = image_get(mask, u-1, v);
      float left_val = (left.x + left.y + left.z + left.w) / 4;
      
      if ( ( fabs( base_val - left_val ) <= tolerance ) && (left_mask.x != 1.0) ) {
        int_list_push_back(q_x, u-1);
        int_list_push_back(q_y, v);
      }
    }
    
    if (u < src->width-1) {
      vec4 right = image_get(src, u+1, v);
      vec4 right_mask = image_get(mask, u+1, v);
      float right_val = (right.x + right.y + right.z + right.w) / 4;
      
      if ( ( fabs( base_val - right_val ) <= tolerance ) && (right_mask.x != 1.0) ) {
        int_list_push_back(q_x, u+1);
        int_list_push_back(q_y, v);
      }
    }
    
    if (v > 0) {
      vec4 top = image_get(src, u, v-1);
      vec4 top_mask = image_get(mask, u, v-1);
      float top_val = (top.x + top.y + top.z + top.w) / 4;
      
      if ( ( fabs( base_val - top_val ) <= tolerance ) && (top_mask.x != 1.0) ) {
        int_list_push_back(q_x, u);
        int_list_push_back(q_y, v-1);
      }
    }
    
    if (v < src->height-1) {
      vec4 bottom = image_get(src, u, v+1);
      vec4 bottom_mask = image_get(mask, u, v+1);
      float bottom_val = (bottom.x + bottom.y + bottom.z + bottom.w) / 4;
      
      if ( ( fabs( base_val - bottom_val ) <= tolerance ) && (bottom_mask.x != 1.0) ) {
        int_list_push_back(q_x, u);
        int_list_push_back(q_y, v+1);
      }
    }
    
  }
  
  int_list_delete(q_x);
  int_list_delete(q_y);
  
  return mask;
  
}

image* image_difference_mask(image* src, vec4 color, float tolerance) {

  image* mask = image_blank(src->width, src->height);
  
  float base_val = (color.x + color.y + color.z + color.w) / 4;
  
  for (int x = 0; x < src->width; x++)
  for (int y = 0; y < src->height; y++) {
  
    vec4 col = image_get(src, x, y);
    float val = (col.x + col.y + col.z + col.w) / 4;
    
    if ( fabs(val - base_val) > tolerance) {
      image_set( mask, x, y, vec4_one() );
    }
  
  }
  
  return mask;

}

long image_mask_count(image* i) {
  long total = 0;
  for (int x = 0; x < i->width; x++)
  for (int y = 0; y < i->height; y++) {
    if (image_get(i, x, y).w == 1.0) { total++; }
  }
  return total;
}

void image_mask_median(image* i, int* u, int* v) {
  
  long count = image_mask_count(i);
  long total = 0;
  *u = 0; *v = 0;
  
  for (int x = 0; x < i->width; x++)
  for (int y = 0; y < i->height; y++) {
    if (image_get(i, x, y).w == 1.0) { total ++; }
    if (total >= count/2) {
      *u = x; *v = y;
      return;
    }
  }
  
}

void image_mask_random(image* i, int* u, int* v) {
  
  long count = image_mask_count(i);
  long total = 0;
  long index = (rand() % count) + 1;

  *u = 0; *v = 0;
  
  for (int x = 0; x < i->width; x++)
  for (int y = 0; y < i->height; y++) {
    if (image_get(i, x, y).w == 1.0) { total ++; }
    if (total == index) {
      *u = x; *v = y;
      return;
    }
  }
  
}

void image_write_to_file(image* i, char* filename) {
  
  fpath ext;
  SDL_PathFileExtension(ext.ptr, filename);
  
       if ( strcmp(ext.ptr, "tga") == 0 ) { image_tga_save_file(i, filename); }
  else if ( strcmp(ext.ptr, "bmp") == 0 ) { image_bmp_save_file(i, filename); }
  else { error("Cannot save texture to %s, unknown file extension %s. Try .tga!\n", filename, ext.ptr); }
}

void image_tga_save_file(image* i, char* filename) {
  
  image_flip_vertical(i);
  image_bgr_to_rgb(i);
  
  unsigned char xa= i->width % 256;
  unsigned char xb= (i->width-xa)/256;
  unsigned char ya= i->height % 256;
  unsigned char yb= (i->height-ya)/256;
  unsigned char header[18] = {0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, xa, xb, ya, yb, 32, 0};
  
  SDL_RWops* file = SDL_RWFromFile(filename, "wb");
  
  if (file == NULL) {
    error("Could not write to file %s", filename);
  }
  
  SDL_RWwrite(file, header, sizeof(header), 1);
  SDL_RWwrite(file, i->data, i->width * i->height * 4, 1);
  SDL_RWclose(file);
  
  image_bgr_to_rgb(i);
  image_flip_vertical(i);
  
}

void image_bmp_save_file(image* i, char* filename) {
  
  SDL_Surface* surface = SDL_CreateRGBSurfaceFrom(i->data, i->width, i->height, 32, 4 * i->width, 0xFF000000, 0x00FF0000, 0x0000FF00, 0x000000FF);
  SDL_SaveBMP(surface, filename);
  SDL_FreeSurface(surface);
  
}

image* image_read_from_file(char* filename) {
  
  fpath ext;
  SDL_PathFileExtension(ext.ptr, filename);
  
       if ( strcmp(ext.ptr, "tga") == 0 ) { return image_tga_load_file(filename); }
  else if ( strcmp(ext.ptr, "bmp") == 0 ) { return image_bmp_load_file(filename); } 
  else { error("Cannot save texture to %s, unknown file extension %s. Try .tga!\n", filename, ext.ptr); return NULL; }
}

image* image_tga_load_file(char* filename) {

  SDL_RWops* file = SDL_RWFromFile(filename, "rb");
  
	if (file == NULL) {
		error("Cannot open file %s", filename);
	}
	
  uint16_t width, height;
  char depth;
	
	/* Seek to the width */
	SDL_RWseek(file, 12, SEEK_SET);
	SDL_RWread(file, &width, sizeof(uint16_t), 1);
	
	/* Seek to the height */
	SDL_RWseek(file, 14, SEEK_SET);
	SDL_RWread(file, &height, sizeof(uint16_t), 1);
	
	/* Seek to the depth */
	SDL_RWseek(file, 16, SEEK_SET);
	SDL_RWread(file, &depth, sizeof(char), 1);
  
  image* i = image_empty(width, height);
  
  int channels;
	if (depth == 24) {
		channels = 3;
	} else if (depth == 32) {
		channels = 4;
	} else {
    error("Cannot load file '%s', it has depth of %i", filename, depth);
    return NULL;
  }

	int size = height * width * channels;
	unsigned char* image_data = malloc(sizeof(unsigned char) * size);

	/* Seek to the image data. */
	SDL_RWseek(file, 18, SEEK_SET);
	SDL_RWread(file, image_data, sizeof(unsigned char) * size, 1);
  SDL_RWclose(file);

  if (channels == 4) {
    
    int x, y;
    for( x = 0; x < i->width; x++)
    for( y = 0; y < i->height; y++) {
      i->data[x * 4 + y * i->width * 4 + 0] = image_data[x * 4 + y * width * 4 + 2];
      i->data[x * 4 + y * i->width * 4 + 1] = image_data[x * 4 + y * width * 4 + 1];
      i->data[x * 4 + y * i->width * 4 + 2] = image_data[x * 4 + y * width * 4 + 0];
      i->data[x * 4 + y * i->width * 4 + 3] = image_data[x * 4 + y * width * 4 + 3];
    }
    
  } else if (channels == 3) {
    
    int x, y;
    for( x = 0; x < i->width; x++)
    for( y = 0; y < i->height; y++) {
      i->data[x * 4 + y * i->width * 4 + 0] = image_data[x * 3 + y * width * 3 + 2];
      i->data[x * 4 + y * i->width * 4 + 1] = image_data[x * 3 + y * width * 3 + 1];
      i->data[x * 4 + y * i->width * 4 + 2] = image_data[x * 3 + y * width * 3 + 0];
      i->data[x * 4 + y * i->width * 4 + 3] = 0;
    }
    
  }
    
  free(image_data);
  
  image_flip_vertical(i);

  return i;
  
}

image* image_bmp_load_file(char* filename) {
  
  SDL_Surface *surface = SDL_LoadBMP(filename);
  
  if (!surface) { error("Could not load file %s\n", filename); }
  
  SDL_LockSurface(surface);
  
  unsigned char* image_data = malloc(sizeof(unsigned char) * 4 * surface->w * surface->h);
  
  if (surface->format->BytesPerPixel == 3) {
    
    int x, y;
    for(x = 0; x < surface->w; x++)
    for(y = 0; y < surface->h; y++) {
      image_data[x * 4 + y * surface->w * 4 + 0] = ((unsigned char*)surface->pixels)[x * 3 + y * surface->w * 3 + 0];
      image_data[x * 4 + y * surface->w * 4 + 1] = ((unsigned char*)surface->pixels)[x * 3 + y * surface->w * 3 + 1];
      image_data[x * 4 + y * surface->w * 4 + 2] = ((unsigned char*)surface->pixels)[x * 3 + y * surface->w * 3 + 2];
      image_data[x * 4 + y * surface->w * 4 + 3] = 0;
    }
    
  } else if (surface->format->BytesPerPixel == 4) {
    memcpy(image_data, surface->pixels, sizeof(unsigned char) * 4 * surface->w * surface->h);
  } else {
    error("File %s has %i channels, don't know how to load it!", filename, surface->format->BytesPerPixel);
  }

  image* i = image_new(surface->w, surface->h, image_data);
  
  free(image_data);
  
  SDL_UnlockSurface(surface);
  SDL_FreeSurface(surface);
  
  return i;
}

*/
