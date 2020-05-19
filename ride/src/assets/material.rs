use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::fmt::Write;

use std::cell::RefCell;
use std::cell::RefMut;
//use std::any::Any;
use std::rc::Rc;
//use std::ops::Deref;

use crate::asset::{AssetHandle, asset_hndl_new_load, asset_hndl_ptr};
use crate::engine::{Vec2, Vec3, Vec4, gl_check_error};
use crate::assets::shader::{Shader, ShaderProgram, shader_program_new, shader_program_attach_shader,
                     shader_program_link};
use crate::assets::texture::Texture;
use crate::core::error;

/*
/**
*** :: Material ::
***
***   Material in system. Provides shader.
***   Also provides shader "uniform" values
***
**/

#ifndef material_h
#define material_h

#include "cengine.h"
#include "casset.h"

#include "assets/shader.h"

typedef union {
  int as_int;
  float as_float;
  vec2 as_vec2;
  vec3 as_vec3;
  vec4 as_vec4;
  asset_hndl as_asset;
} material_item;
*/

#[derive(Clone)]
pub enum MaterialItem {
  AsInt(i32),
  AsFloat(f32),
  AsVec2(Vec2),
  AsVec3(Vec3),
  AsVec4(Vec4),
  AsShaderAsset(AssetHandle<Shader>),
  AsTextureAsset(AssetHandle<Texture>),
  AsNone,
}

/*
static const int mat_item_int = 0;
static const int mat_item_float = 1;
static const int mat_item_vec2 = 2;
static const int mat_item_vec3 = 3;
static const int mat_item_vec4 = 4;
static const int mat_item_shader = 5;
static const int mat_item_texture = 6;
*/

#[derive(Clone)]
pub enum MaterialItemType {
  MaterialItemInt = 0,
  MaterialItemFloat,
  MaterialItemVec2,
  MaterialItemVec3,
  MaterialItemVec4,
  MaterialItemShader,
  MaterialItemTexture,
  MaterialItemNone,
}

/*

typedef struct {
  shader_program* program;
  int num_items;
  int* types;
  char** names;
  material_item* items;
} material_entry;
*/

pub struct MaterialEntry {
  pub program: Option<ShaderProgram>,
  pub types: Vec<MaterialItemType>,
  pub names: Vec<String>,
  pub items: Vec<MaterialItem>,
}

/*

void material_entry_delete(material_entry* me);
material_item material_entry_item(material_entry* me, char* name);
bool material_entry_has_item(material_entry* me, char* name);
void material_entry_add_item(material_entry* me, char* name, int type, material_item mi);

typedef struct {
  int num_entries;
  material_entry** entries;
} material;
*/

pub struct Material {
  pub entries: Vec<MaterialEntry>, //why was this star star? ...
}

/*

material* material_new();
void material_delete(material* m);

material* mat_load_file(char* filename);

material_entry* material_get_entry(material* m, int index);
material_entry* material_add_entry(material* m);

shader_program* material_first_program(material* m);

#endif

#include "assets/material.h"

void material_entry_delete(material_entry* me) {
  shader_program_delete(me->program);
  for(int i = 0; i < me->num_items; i++) {
    free(me->names[i]);
  }
  free(me->names);
  free(me->types);
  free(me->items);
  free(me);
}
*/

//pub fn material_entry_delete(mut me: MaterialEntry) {
//  shader_program_delete(&mut me.program);
//}

/*

material_item material_entry_item(material_entry* me, char* name) {
  
  for(int i = 0; i < me->num_items; i++) {
    if (strcmp(me->names[i], name) == 0) {
      return me->items[i];
    }
  }
  
  material_item empty;
  memset(&empty, 0, sizeof(empty));
  
  return empty;
}
*/

pub fn material_entry_item(me: &MaterialEntry, name: &str) -> MaterialItem {
  for i in 0..me.items.len() {
    if me.names[i] == name {
      return me.items[i].clone(); //this could be an asset handle so it is more complicated, ie must clone instead of copy
    }
  }

  return MaterialItem::AsInt(0); //none TODO
}

/*

bool material_entry_has_item(material_entry* me, char* name) {
  for(int i = 0; i < me->num_items; i++) {
    if (strcmp(me->names[i], name) == 0) {
      return true;
    }
  }
  
  return false;
}
*/

/*
material* material_new() {
  material* m = malloc(sizeof(material));
  m->num_entries = 0;
  m->entries = NULL;
  return m;
}*/

pub fn material_new<'a>() -> Material {
  return Material { entries: Vec::new() };
}
/*

void material_delete(material* m) {
  for(int i = 0; i < m->num_entries; i++) {
    material_entry_delete(m->entries[i]);
  }
  free(m->entries);
  free(m);
}

*/

/*
pub fn material_delete(m:Rc<RefCell<Material>>) {
  while m.borrow().entries.len() > 0 {
    let me = m.borrow_mut().entries.pop().unwrap();
    material_entry_delete(me);
  }
}
*/

//impl Drop for Material {
//  fn drop (&mut self) {
//println!("Dropping Material");
//debug("Dropping Material");
//  }
//}

/*
static void material_generate_programs(material* m) {
  
  for(int i = 0; i < m->num_entries; i++) {
  
    material_entry* me = m->entries[i];
    me->program = shader_program_new();
    
    bool attached = false;
    for(int j = 0; j < me->num_items; j++) {
      
      if (me->types[j] == mat_item_shader) {
        asset_hndl ah = me->items[j].as_asset;
        
        shader_program_attach_shader(me->program, asset_hndl_ptr(&ah));
        attached = true;
      }
      
    }
    
    if (attached) { shader_program_link(me->program); }
    
  }
  
  SDL_GL_CheckError();
    
}
*/

fn material_generate_programs(m: &mut Material) {
  for i in 0..m.entries.len() {
    //let me2 = &;

    let me = &mut m.entries[i];
    //let me = &mut me2.borrow_mut();
    //me.program.swap(&shader_program_new()); //this also avoids mutability
    me.program = shader_program_new();

    let mut attached = false;

    let items_len = me.items.len();

    for j in 0..items_len {
      match me.types[j] {
        MaterialItemType::MaterialItemShader => {
          //let ah = me.items[j].clone(); //this appears to be the key, cloning so I don't have to borrow mutably
          //ok got around that
          //match ah {
          match me.items.get_mut(j) {
            Some(&mut MaterialItem::AsShaderAsset(ref mut value)) => {
              if let Some(shader) = asset_hndl_ptr::<Shader>(value) {
                shader_program_attach_shader(&mut me.program.as_ref().unwrap(), &shader.borrow());
                attached = true;
              } else {
                let mut error_full = String::new();
                let _ = write!(
                  &mut error_full,
                  "Could not find a shader for input '{}'",
                  value.path
                );
                error(&error_full);
              }
            }
            _ => {}
          }
        }
        _ => {}
      }
    }

    if attached {
      //this is how to extract from an option: get an asref option (clone), then unwrap it
      shader_program_link(&me.program.as_ref().unwrap())
    }
  }
}

/*
void material_entry_add_item(material_entry* me, char* name, int type, material_item mi) {
  me->num_items++;
  
  me->types = realloc(me->types, sizeof(int) * me->num_items);
  me->names = realloc(me->names, sizeof(char*) * me->num_items);
  me->items = realloc(me->items, sizeof(material_item) * me->num_items);
  
  me->items[me->num_items-1] = mi;
  me->types[me->num_items-1] = type;
  me->names[me->num_items-1] = malloc(strlen(name)+1);
  strcpy(me->names[me->num_items-1], name);  
}
*/

fn material_entry_add_item(
  me: &mut MaterialEntry,
  name: &str,
  mat_type: &MaterialItemType,
  mi: &MaterialItem,
) {
  //let mut me = me.borrow_mut(); //any outstanding refcell borrows and this will crash
  let mat_type = mat_type.clone();
  me.items.push(mi.clone());
  me.types.push(mat_type);
  me.names.push(name.into());
}

/*

material_entry* material_add_entry(material* m) {
  
  m->num_entries++;
  m->entries = realloc(m->entries, sizeof(material_entry*) * m->num_entries);
  m->entries[m->num_entries-1] = malloc(sizeof(material_entry));
  
  material_entry* me = m->entries[m->num_entries-1];
  me->program = NULL;
  me->num_items = 0;
  me->types = malloc(sizeof(int) * me->num_items);
  me->names = malloc(sizeof(char*) * me->num_items);
  me->items = malloc(sizeof(material_item) * me->num_items);
  
  return me;
}
*/

pub fn material_last_entry<'a>(m: &'a mut Material) -> Option<&'a mut MaterialEntry> {
  return m.entries.last_mut();
}

pub fn material_add_entry(m: &mut Material) {
  //-> Option<&mut MaterialEntry> {
  //this should be nullable I think
  let me = MaterialEntry {
    program: None,
    types: Vec::new(),
    names: Vec::new(),
    items: Vec::new(),
  };

  //let rcme = Rc::new(RefCell::new(me));
  //m.entries.push(rcme.clone());
  m.entries.push(me);

  //return m.entries.last_mut(); //rcme;
}

/*

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

material* mat_load_file(char* filename) {
  
  SDL_RWops* file = SDL_RWFromFile(filename, "r");
  if(file == NULL) {
    error("Cannot load file %s", filename);
  }
  
  material* m = material_new();
  material_entry* me = material_add_entry(m);
  
  char line[1024];
  while(SDL_RWreadline(file, line, 1024)) {
    
    if (line[0] == '#') { continue; }
    if (line[0] == '\r') { continue; }
    if (line[0] == '\n') { continue; }
    
    if (strstr(line, "submaterial")) {
      
      /* Skip Empty Submaterials */
      if (me->num_items == 0) {
        continue;
      } else {
        me = material_add_entry(m);
        continue;
      }
      
    }
    
    char type[512]; char name[512]; char value[512];
    int matches = sscanf(line, "%511s %511s = %511s", type, name, value);
    
    if (matches != 3) continue;
    
    material_item mi;
    int type_id;
    char* end;
    float f0, f1, f2, f3;
    
    if (strcmp(type, "shader") == 0) {
    
      mi.as_asset = asset_hndl_new_load(P(value));
      type_id = mat_item_shader;
      
    } else if (strcmp(type, "texture") == 0) {
    
      mi.as_asset = asset_hndl_new_load(P(value));
      type_id = mat_item_texture;
    
    } else if (strcmp(type, "int") == 0) {
    
      mi.as_int = atoi(value);
      type_id = mat_item_int;
    
    } else if (strcmp(type, "float") == 0) {
      
      mi.as_float = atof(value);
      type_id = mat_item_float;
      
    } else if (strcmp(type, "vec2") == 0) {
    
      f0 = strtod(value, &end); f1 = strtod(end, NULL);
      mi.as_vec2 = vec2_new(f0, f1);
      type_id = mat_item_vec2;
      
    } else if (strcmp(type, "vec3") == 0) {
      
      f0 = strtod(value, &end); f1 = strtod(end, &end);
      f2 = strtod(end, NULL);
      mi.as_vec3 = vec3_new(f0, f1, f2);
      type_id = mat_item_vec3;
      
    } else if (strcmp(type, "vec4") == 0) {
    
      f0 = strtod(value, &end); f1 = strtod(end, &end);
      f2 = strtod(end, &end); f3 = strtod(end, NULL);
      mi.as_vec4 = vec4_new(f0, f1, f2, f3);
      type_id = mat_item_vec4;
      
    } else {
      error("Unknown material item type '%s'", type);
      return NULL;
    }
    
    material_entry_add_item(me, name, type_id, mi);
    
  }
  
  SDL_RWclose(file);
  
  material_generate_programs(m);
  
  SDL_GL_CheckError();
  
  return m;
}
*/

//rc refcell material
pub fn mat_load_file(filename: &str) -> Rc<RefCell<Material>> {
  //Rc<Any> {
  let file = File::open(filename);

  let file = match file {
    Ok(file) => file,
    Err(_) => {
      let mut error_full = String::new();
      let _ = write!(&mut error_full, "Cannot load file {}", filename);
      error(&error_full);
      let null = Rc::new(RefCell::new(material_new())); // I could make this smarter to return a Result?? TODO
      return null;
    }
  };

  let file = BufReader::new(&file);

  let mut m = material_new();

  material_add_entry(&mut m); //first entry
  //let mut me = material_add_entry(&mut m).unwrap();

  for line in file.lines() {
    let l = line.unwrap();
    let mut chars = l.chars();
    let next = chars.next();

    match next {
      Some('#') => continue,
      Some('\r') => continue,
      Some('\n') => continue,
      Some(_) => {}
      None => continue, //this shouldn't happen??
    }

    if l.contains("submaterial") {
      let items_len = {
        let me = material_last_entry(&mut m).unwrap();
        me.items.len()
      };

      if items_len == 0 {
        //avoid creating something empty, so don't move on yet
        continue;
      } else {
        material_add_entry(&mut m);

        //me = m.entries.last_mut().unwrap();
        //me = material_add_entry(&mut m).unwrap(); //this doesn't work because it is a non lexical lifetime :(!
        continue;
      }
    }

    let (mat_type, name, value) = scan_fmt!(&l, "{} {} = {}", String, String, String);

    if mat_type.is_none() || name.is_none() || value.is_none() {
      continue;
    }

    let mat_type = mat_type.unwrap();
    let name = name.unwrap();
    let value = value.unwrap();

    let mi: MaterialItem; // = MaterialItem::AsNone;
    let type_id: MaterialItemType; // = MaterialItemType::MaterialItemNone;
    /*
    let mut f0 = 0.0;
    let mut f1 = 0.0;
    let mut f2 = 0.0;
    let mut f3 = 0.0;
    */

    match mat_type.as_str() { //this looks like it was possibly made up?
      "shader" => {
        mi = MaterialItem::AsShaderAsset(asset_hndl_new_load(&value)); //load up the shader based on the value, the asset shader loading needs to be intact
        type_id = MaterialItemType::MaterialItemShader;
      }
      "texture" => {
        mi = MaterialItem::AsTextureAsset(asset_hndl_new_load(&value));
        type_id = MaterialItemType::MaterialItemTexture;
      }
      "int" => {
        mi = MaterialItem::AsInt(value.parse::<i32>().unwrap());
        type_id = MaterialItemType::MaterialItemInt;
      }
      "float" => {
        mi = MaterialItem::AsFloat(value.parse().unwrap());
        type_id = MaterialItemType::MaterialItemFloat;
      } //the other types don't appear to be used (yet)
      _ => {
        //mi = MaterialItem::AsNone;
        //type_id = MaterialItemType::MaterialItemNone;

        let mut error_full = String::new();
        let _ = write!(&mut error_full, "Unknown material item type '{}'", mat_type);
        error(&error_full);
        let null = Rc::new(RefCell::new(material_new()));
        return null;
        //return NULL;

      }
    }

    {
      let mut me = material_last_entry(&mut m).unwrap(); //I can't change me in the middle of the loop due to the borrow checker
      material_entry_add_item(&mut me, &name, &type_id, &mi);
    }
  }


  material_generate_programs(&mut m);

  gl_check_error();

  //let rcm: Rc<Any> = Rc::new(RefCell::new(m));
  let rcm = Rc::new(RefCell::new(m));
  return rcm;
}

/*
material_entry* material_get_entry(material* m, int index) {
  return m->entries[(int)clamp(index, 0, m->num_entries-1)];
}

shader_program* material_first_program(material* m) {
  if (m->num_entries <= 0) {
    error("No entries in material!");
    return NULL;
  } else {
    return m->entries[0]->program;
  }
}
*/

//lifetime in rust is all scope!!
pub struct MaterialPtr<'a> {
  refmut: RefMut<'a, Material>,
}

impl<'a> MaterialPtr<'a> {
  pub fn material_first_program(&mut self) -> Option<&mut ShaderProgram> {
    //(RefMut<'b, Material>, Option<&mut ShaderProgram>) {
    return self.refmut.entries[0].program.as_mut();

    //let mat:RefMut<'b, Material> = m.borrow_mut();
    //let program:Option<&'a mut ShaderProgram> = mat.deref().entries[0].program.as_mut();
    //return (mat, program);
    //return None;
  }
}
///everywhere that is getting a refcell material as a parameter should be a method I think. that way I can borrow()->do the method...??
///
/*
pub fn material_ptr(a:&Rc<RefCell<Material>>) -> MaterialPtr {
  let ptr = MaterialPtr{refmut:a.borrow_mut()};
  return ptr;
}

pub fn material_first_borrow(a:&Rc<RefCell<Material>>) -> Box<MaterialPtr> {//, Option<&mut ShaderProgram>) {
  let ptr = MaterialPtr{refmut:a.borrow_mut()};
  
  //let s = ptr.material_first_program();
  //let mut refmut = a.borrow_mut();

  //let s = material_first_program(&mut refmut);

 return Box::new(ptr)//, s);
}
*/

pub fn material_first_program(m: &mut Material) -> Option<&mut ShaderProgram> {
  //(RefMut<'b, Material>, Option<&mut ShaderProgram>) {
  if m.entries.len() < 1 {
    return None;
  }
  return m.entries[0].program.as_mut();
}
