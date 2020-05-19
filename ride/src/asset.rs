use std::cell::RefCell;
use std::any::Any;
use std::rc::Rc; //Weak
use std::collections::HashMap;
use std::path::Path;
use std::env::current_dir;
//use std::ops::AddAssign;
use std::ops::Add;
use std::fmt::Write;
use std::fs;

use time;
//use std::io;



use crate::core::error;
use crate::core::debug;

//.h
/*
#include "casset.h"

#include "data/dict.h"
#include "data/list.h"

typedef struct {
  fpath path;
  asset* ptr;
  uint32_t timestamp;
} asset_hndl;
*/

pub struct AssetHandle<T> {
  pub path: String,
  ptr: Option<Rc<RefCell<T>>>,
  timestamp: u64,
}

impl<T> Clone for AssetHandle<T> {
  fn clone(&self) -> Self {
    return AssetHandle {
      path: self.path.clone(),
      ptr: self.ptr.clone(),
      timestamp: self.timestamp.clone(),
    };
  }
}

//.cpp
/*
static dict* asset_dict;
static uint32_t asset_timestamp = 0;

enum {
  MAX_ASSET_HANDLERS = 512,
  MAX_PATH_VARIABLES = 512
};

static asset_handler asset_handlers[MAX_ASSET_HANDLERS];
static int num_asset_handlers = 0;

static path_variable path_variables[MAX_PATH_VARIABLES];
static int num_path_variables = 0;

*/

struct AssetStaticData {
  asset_dict: Option<HashMap<String, Rc<dyn Any>>>,
  asset_timestamp: u64,
  //max_asset_handlers:i32,
  //max_path_variables:i32,
  asset_handlers: Option<Vec<AssetHandler>>,
  //asset_handlers:Option<Vec<Box<Any>>>,
  path_variables: Option<Vec<PathVariable>>,
}

//no allocations on globals

static mut G_ASSET_STATE: AssetStaticData = AssetStaticData {
  asset_dict: None,
  asset_timestamp: 0,
  asset_handlers: None,
  path_variables: None,
};

/*
typedef struct {
  type_id type;
  char* extension;
  void* (*load_func)(const char*);
  void (*del_func)();
} asset_handler;
*/

struct AssetHandler {
  //asset_type: String, //this is a typename, the true type that the 'load' produces //TypeId, //this is an index into the type_names table
  extension: String,
  load_func: Box<dyn Fn(&str) -> Rc<dyn Any>>,
  //del_func: Box<Fn(Rc<Any>)>,
}

/*
typedef struct {
  fpath variable;
  fpath mapping;
} path_variable;
*/

struct PathVariable {
  variable: String,
  mapping: String,
}

/*
void asset_add_path_variable(fpath variable, fpath mapping) {
  
  if (num_path_variables == MAX_PATH_VARIABLES) {
    error("Already reached maximum num of path variables (%i)", MAX_PATH_VARIABLES);
  }
  
  if (variable.ptr[0] != '$') {
    error("Variables must start with a dollar sign e.g '$CORANGE'");
  }
  
  path_variable pv = { variable, mapping }; 
  
  path_variables[num_path_variables] = pv;
  num_path_variables++;
  
}
*/

pub fn asset_add_path_variable(variable: &str, mapping: &str) {
  if variable.chars().nth(0).unwrap() != '$' {
    error("Variables must start with a dollar sign e.g. '$RIDE'");
  }

  unsafe {
    match G_ASSET_STATE.path_variables.as_mut() {
      Some(path_variables) => {
        path_variables.push(PathVariable {
          variable: variable.into(),
          mapping: mapping.into(),
        })
      }
      None => error("Path variable storage missing"),
    }
  }
}

/*
static fpath asset_map_fullpath(fpath filename) {
  fpath out;
  SDL_PathFullName(out.ptr, filename.ptr);
  return out;
}
*/
//pub just to get rid of warning TODO
pub fn asset_map_fullpath(filename: &str) -> String {
  let path = Path::new(filename);
  let result = path.canonicalize();

  let string_result = match result {
    Ok(result_path) => {
      match result_path.to_str() {
        Some(string) => string.into(),
        None => {
          error("Bad path read (unicode problems?)");
          "".into()
        }
      }
    }
    Err(..) => {
      error("Not a real path");
      "".into()
    }
  };
  return string_result;

}

/*
static fpath asset_map_shortpath(fpath filename) {
  fpath out;
  SDL_PathRelative(out.ptr, filename.ptr);
  return out;
}
*/
//pub just to get rid of warning before impl TODO
pub fn asset_map_shortpath(filename: &str) -> String {
  let path = Path::new(filename);
  let cwd = current_dir().unwrap();
  let result = path.strip_prefix(&cwd);

  let string_result = match result {
    Ok(result_path) => {
      match result_path.to_str() {
        Some(string) => {
          let dot: String = ".".into();
          dot.add(string)
        }
        None => String::from(filename),
      }
    }
    Err(..) => String::from(filename), 
  };

  return string_result;
}

/*
fpath asset_unmap_filename(fpath filename) {
  
  fpath fullpath = asset_map_fullpath(filename);
  
  for (int i = 0; i < num_path_variables; i++) {
    
    fpath variable = path_variables[i].variable;
    fpath mapping  = path_variables[i].mapping; 
    fpath fullmapping = asset_map_fullpath(mapping);
    
    char* subptr = strstr(fullpath.ptr, fullmapping.ptr);
    
    if (subptr) {
    
      fpath sub; strcpy(sub.ptr, subptr);
    
      int replace_len = strlen(variable.ptr);
      int start_len = strlen(fullpath.ptr) - strlen(sub.ptr);
      int ext_len = strlen(sub.ptr) - strlen(fullmapping.ptr);
      
      fullpath.ptr[start_len] = '\0';
      strcat(fullpath.ptr, variable.ptr);
      strcat(fullpath.ptr, "/");
      strcat(fullpath.ptr, sub.ptr + strlen(fullmapping.ptr));
      
    }
    
  }
  
  return asset_map_shortpath(fullpath);
  
}
*/

//add the var back in?
//pub to hide unused for now TODO
pub fn asset_unmap_filename(filename: &str) -> String {
  //OK, THEY ARE JUST USING A CUSTOM ENVIRONMENT VARIABLE SYSTEM $CORANGE AT BEGINNING OF PATH
  let mut fullpath = asset_map_fullpath(filename);
  //let mut garbage = Vec::<PathVariable>::new();
  //

  //something tells me the question mark would be useful here

  let path_variables = unsafe {
    match G_ASSET_STATE.path_variables.as_mut() {
      Some(path_variables) => path_variables,
      None => return "".into(),//&mut garbage
    }
  };

  //unsafe {
  for path_variable in path_variables {
    let variable = &path_variable.variable;
    let mapping = &path_variable.mapping;

    let fullmapping = asset_map_fullpath(mapping); //get the real path that is contained in the $variable

    let contains = fullpath.contains(fullmapping.as_str());

    if contains {
      let fullpath2 = fullpath.clone();
      //TODO fix the separator for different os here
      let variablefull = variable.clone().add("/");
      let parts: Vec<&str> = fullpath2.split(mapping).collect(); //spit at variable and every in between put in mapping
      fullpath = parts.join(&variablefull);
    }
  }
  //}

  return asset_map_shortpath(fullpath.as_str());
}


/*

fpath asset_map_filename(fpath filename) {
  
  fpath out = filename;
  
  for(int i = 0; i < num_path_variables; i++) {
  
    fpath variable = path_variables[i].variable;
    fpath mapping = path_variables[i].mapping;
    
    char* subptr = strstr(out.ptr, variable.ptr);
    
    if (subptr) {
      
      fpath sub; strcpy(sub.ptr, subptr);
    
      int replace_len = strlen(mapping.ptr);
      int start_len = strlen(out.ptr) - strlen(sub.ptr);
      int ext_len = strlen(sub.ptr) - strlen(variable.ptr);
      
      out.ptr[start_len] = '\0';
      strcat(out.ptr, mapping.ptr);
      strcat(out.ptr, sub.ptr + strlen(variable.ptr));
    }
  
  }

  return asset_map_fullpath(out);
}

*/

//remove the $var? //assumes $var occurs once but not necessarily at the beginning
fn asset_map_filename(filename: &str) -> String {
  let mut out: String = filename.into();

  let path_variables = unsafe {
    match G_ASSET_STATE.path_variables.as_mut() {
      Some(path_variables) => path_variables,
      None => return "".into(),//&mut garbage
    }
  };

  for path_variable in path_variables {
    let variable = &path_variable.variable;
    let mapping = &path_variable.mapping;

    let contains = out.contains(variable);

    if contains {
      let out2 = out.clone();
      let parts: Vec<&str> = out2.split(variable).collect(); //spit at variable and every in between put in mapping
      out = parts.join(mapping);
    }
  }

  return out;
}

/*

asset_hndl asset_hndl_null() {
  asset_hndl ah;
  ah.path = P("");
  ah.ptr = NULL;
  ah.timestamp = 0;
  return ah;
}

*/

pub fn asset_hndl_null<T>() -> AssetHandle<T> {
  //TODO this rc should go away anc create a null ref like I want. Is there a better way?
  //let null: Rc<Any> = Rc::new(RefCell::new(0));
  let ah = AssetHandle {
    path: "".into(),
    ptr: None,
    timestamp: 0,
  }; //Rc::downgrade(&null), timestamp:0};
  return ah;
}

/*

asset_hndl asset_hndl_new(fpath path) {
  asset_hndl ah;
  ah.path = asset_map_filename(path);
  ah.ptr = NULL;
  ah.timestamp = 0;
  return ah;
}

*/

pub fn asset_hndl_new<T>(path: &str) -> AssetHandle<T> {
  return AssetHandle {
    path: asset_map_filename(path),
    ptr: None,
    timestamp: 0,
  };
}

/*

asset_hndl asset_hndl_new_load(fpath path) {
  asset_hndl ah = asset_hndl_new(path);
  if (!file_isloaded(ah.path)) {
    file_load(ah.path);
  }
  return ah;
}

*/

pub fn asset_hndl_new_load<T>(path: &str) -> AssetHandle<T> {
  let ah = asset_hndl_new(path);
  if !file_isloaded(&ah.path) {
    file_load(&ah.path);
  }
  return ah;
}

/*

asset_hndl asset_hndl_new_ptr(asset* as) {
  asset_hndl ah;
  ah.path = P(asset_ptr_path(as));
  ah.ptr = as;
  ah.timestamp = SDL_GetTicks();
  return ah;
}

*/

pub fn asset_hndl_new_ptr<T>(asset: &Rc<dyn Any>) -> AssetHandle<T>
where
  T: Any,
{
  return AssetHandle {
    path: asset_ptr_path(asset),
    ptr: asset.clone().downcast::<RefCell<T>>().ok(),
    timestamp: time::precise_time_ns(),
  };
}
/*

bool asset_hndl_isnull(asset_hndl* ah) {
  return (strcmp(ah->path.ptr, "") == 0);
}
*/

pub fn asset_hndl_isnull<T>(ah: &AssetHandle<T>) -> bool {
  return ah.path.eq("");
}

/*

fpath asset_hndl_path(asset_hndl* ah) {
  return ah->path;
}
*/

pub fn asset_hndl_path<T>(ah: &AssetHandle<T>) -> String {
  return ah.path.clone();
}

/*

bool asset_hndl_eq(asset_hndl* ah0, asset_hndl* ah1) {
  return (strcmp(ah0->path.ptr, ah1->path.ptr) == 0);
}

*/

pub fn asset_hndl_eq<T>(ah0: &AssetHandle<T>, ah1: &AssetHandle<T>) -> bool {
  return ah0.path.eq(&ah1.path);
}

/*
asset* asset_hndl_ptr(asset_hndl* ah) {

  if (unlikely(ah->path.ptr[0] == '\0')) {
    error("Cannot load NULL asset handle");
    return NULL;
  }
  
  if (likely(ah->timestamp > asset_timestamp)) {
    return ah->ptr;
  } else {
    
    ah->ptr = dict_get(asset_dict, ah->path.ptr);
    ah->timestamp = SDL_GetTicks();
    
    if (unlikely(ah->ptr == NULL)) {
      error("Failed to get Asset '%s', is it loaded yet?", ah->path.ptr);
      return NULL;
    }
    
    return ah->ptr;
  }
  
}
*/

//really refcell any
pub fn asset_hndl_ptr<T>(ah: &mut AssetHandle<T>) -> Option<Rc<RefCell<T>>>
where
  T: Any,
{
  if ah.path.eq("") {
    error("Cannot load NULL asset handle");
    return None;
  }

  let g_timestamp = unsafe { G_ASSET_STATE.asset_timestamp };

  if ah.timestamp > g_timestamp {
    return ah.ptr.clone();
  } else {
    ah.ptr = None; //conceptually it is none, the ptr has expired, free it up

    let asset_dict = unsafe {
      match G_ASSET_STATE.asset_dict.as_ref() {
        Some(dict) => dict,
        None => {
          error("Asset dictionary unavailable");
          return None;
        }
      }
    };

    match asset_dict.get(&ah.path) {
      Some(ptr) => {
        ah.ptr = ptr.clone().downcast::<RefCell<T>>().ok();
        ah.timestamp = time::precise_time_ns();
        return ah.ptr.clone();
      }
      None => {
        let mut error_full = String::new();
        let _ = write!(
          &mut error_full,
          "Failed to get asset '{}', is it loaded yet?",
          ah.path
        );
        error(&error_full);
        return None;
      }
    }
  }

}

/*
void asset_cache_flush(void) {
  asset_timestamp = SDL_GetTicks();
}
*/

pub fn asset_cache_flush() {
  let g_timestamp = time::precise_time_ns();
  unsafe {
    G_ASSET_STATE.asset_timestamp = g_timestamp;
  }
}

/*

void asset_init(void) {
  asset_dict = dict_new(1024);
  asset_cache_flush();
}

*/

pub fn asset_init() {
  unsafe {
    G_ASSET_STATE.asset_dict = Some(HashMap::new());
    //trash the asset handlers to? yes
    G_ASSET_STATE.asset_handlers = Some(Vec::new());
    G_ASSET_STATE.path_variables = Some(Vec::new());
  }
  asset_cache_flush();
}

/*

void asset_handler_delete(asset_handler* h) {

  free(h->extension);
  free(h);

}
*/

/*
fn asset_handler_delete(_: AssetHandler) {

}
*/

/*

static void delete_bucket_list(struct bucket* b) {
  
  if(b == NULL) {
    return;
  }
  
  delete_bucket_list(b->next);
  
  debug("Unloading: '%s'", b->key);
  
  fpath ext;
  SDL_PathFileExtension(ext.ptr, b->key);
  
  for(int i = 0; i < num_asset_handlers; i++) {
  
    asset_handler handler = asset_handlers[i];
    if (strcmp(ext.ptr, handler.extension) == 0) {
      
      bucket_delete_with(b, handler.del_func);
      
      break;
    }
    
  }
  
}
*/

//fn delete_bucket_list() {
//call the delete function on every loaded asset?
//}

/*
void asset_finish() {

  for(int i=0; i <asset_dict->size; i++) {
    struct bucket* b = asset_dict->buckets[i];
    delete_bucket_list(b);
  }
  
  for(int i=0; i < num_asset_handlers; i++) {
    free(asset_handlers[num_asset_handlers].extension);
  }
  
}
*/

pub fn asset_finish() {
  let asset_dict = unsafe {
    match G_ASSET_STATE.asset_dict.as_mut() {
      Some(dict) => dict,
      None => {
        error("Asset dictionary unavailable");
        return;
      }
    }
  };

  asset_dict.clear();
  //go through the asset dictionary and destroy all the assets using the delete function that was
  //provided?

  //then destroy all the asset handler extensions?
}

/*
// Create handler for asset type. Requires type, file extension, and load/unload functions.
#define asset_handler(type, extension, loader, deleter) \
  asset_handler_cast(typeid(type), extension, \
  (asset*(*)(const char*))loader , \
  (asset(*)(void*))deleter)

*/




/*

void asset_handler_cast(type_id type, const char* extension, void* asset_loader(const char* filename) , void asset_deleter(void* asset) ) {
  
  if(num_asset_handlers == MAX_ASSET_HANDLERS) {
    warning("Max number of asset handlers reached. Handler for extension '%s' not added.", extension);
    return;
  }
  
  asset_handler h;
  char* c = malloc(strlen(extension) + 1);
  strcpy(c, extension);
  h.type = type;
  h.extension = c;
  h.load_func = asset_loader;
  h.del_func = asset_deleter;

  asset_handlers[num_asset_handlers] = h;
  num_asset_handlers++;
  
}

*/

pub fn asset_handler<'a, T>(
  //  asset_type: &str,
  extension: &str,
  asset_loader: fn(&str) -> Rc<RefCell<T>>, //  asset_deleter: fn(Rc<RefCell<T>>)
) where
  T: Any + 'a,
{

  let asset_handlers = unsafe {
    match G_ASSET_STATE.asset_handlers.as_mut() {
      Some(handlers) => handlers,
      None => {
        return;
      }
    }
  };

  let asset_loader_any = move |x: &str| {
    let y = asset_loader(x);
    let z: Rc<dyn Any> = y;
    z
  };

  /*
  let asset_deleter_any = move |x: Rc<Any>| {
    if let Ok(y) = x.downcast::<RefCell<T>>() {
      asset_deleter(y); //this had better work, I don't see why it wouldn't
    }
    //assumes this downcast will work
    else {
      error("Bad downcast");
    }
  };
  */

  asset_handlers.push(AssetHandler {
  //  asset_type: asset_type.into(),
    extension: extension.into(),
    load_func: Box::new(asset_loader_any)//,
    //del_func: Box::new(asset_deleter_any),
  });
}

/*
void file_load(fpath filename) {
    
  filename = asset_map_filename(filename);
  
  if (dict_contains(asset_dict, filename.ptr)) {
    error("Asset '%s' already loaded", filename.ptr);
  }
  
  fpath ext;
  SDL_PathFileExtension(ext.ptr, filename.ptr);
  
  for(int i=0; i < num_asset_handlers; i++) {
    asset_handler handler = asset_handlers[i];
    
    if (strcmp(ext.ptr, handler.extension) == 0) {
      debug("Loading: '%s'", filename.ptr);
      asset* a = handler.load_func(filename.ptr);
      dict_set(asset_dict, filename.ptr, a);
      break;
    }
    
  }
  
}
*/

pub fn file_load(filename: &str) {
  let filename2 = asset_map_filename(filename);

  let asset_dict = unsafe {
    match G_ASSET_STATE.asset_dict.as_mut() {
      Some(dict) => dict,
      None => {
        return;
      }
    }
  };

  if asset_dict.contains_key(&filename2) {
    let mut full_error: String = String::new();
    let _ = write!(&mut full_error, "Asset '{}' already loaded", filename2);
    error(&full_error);
  }

  let file_path = Path::new(&filename2);
  let ext_opt = file_path.extension();

  match ext_opt {
    Some(ext) => {
      let asset_handlers = unsafe {
        match G_ASSET_STATE.asset_handlers.as_ref() {
          Some(handlers) => handlers,
          None => {
            return;
          }
        }
      };

      let ext_str: String = ext.to_str().unwrap().into();
      for asset_handler in asset_handlers {
        if ext_str == asset_handler.extension {
          let mut full_debug = String::new();
          let _ = write!(&mut full_debug, "Loading: '{}'", filename2);
          debug(&full_debug);

          let asset = (asset_handler.load_func)(&filename2);
          let _ = asset_dict.insert(filename2.clone(), asset);
          break;
        }
      }
    }
    None => {}//no extension?
  }
}

/*

bool file_exists(fpath filename) {

  filename = asset_map_filename(filename);
  SDL_RWops* file = SDL_RWFromFile(filename.ptr, "r");
  if (file) {
    SDL_RWclose(file);
    return true;
  } else {
    return false;
  }

}
*/

pub fn file_exists(filename: &str) -> bool {
  let filename2 = asset_map_filename(filename);

  let pathfilename = Path::new(&filename2);

  return pathfilename.exists(); //TODO this doesn't include checking read/write permissions
}

/*
void folder_load(fpath folder) {
  
  folder = asset_map_filename(folder);
  debug("Loading Folder: '%s'", folder.ptr);
  
  DIR* dir = opendir(folder.ptr);
  if (dir == NULL) {
    error("Could not open directory '%s' to load.", folder.ptr);
  }
  
  struct dirent* ent;
  while ((ent = readdir(dir)) != NULL) {
  
    if ((strcmp(ent->d_name,".") != 0) && 
        (strcmp(ent->d_name,"..") != 0)) {
    
      fpath filename = folder;
      
      // If does not end in "/" then copy it.
      if (folder.ptr[strlen(folder.ptr)-1] != '/') {
        strcat(filename.ptr, "/");
      }
      
      strcat(filename.ptr, ent->d_name);
      
      if (!file_isloaded(filename)) {
        file_load(filename);
      }
    } 
  }
  
  closedir(dir);
}
*/

pub fn folder_load(folder: &str) {
  let folder2 = asset_map_filename(folder);
  let mut debug_full = String::new();
  let _ = write!(debug_full, "Loading folder: '{}'", folder2);
  debug(&debug_full);

  let dir = Path::new(&folder2);
  //I think this is NOT recursive and should ignore sub directories
  if dir.is_dir() {
    for entry in fs::read_dir(dir).unwrap() {
      let entry = entry.unwrap();
      let path = entry.path();
      if path.is_dir() {
        //visit_dirs(&path, cb)?;
      } else {
        //cb(&entry);
        let pathstring = path.to_str().unwrap();
        if !file_isloaded(pathstring) {
          file_load(pathstring);
        }
      }
    }
  }
}


/*


void folder_load_recursive(fpath folder) {

  folder = asset_map_filename(folder);
  debug("Loading Folder: '%s'", folder.ptr);
  
  DIR* dir = opendir(folder.ptr);
  if (dir == NULL) {
    error("Could not open directory '%s' to load.", folder.ptr);
  }
  
  struct dirent* ent;
  while ((ent = readdir(dir)) != NULL) {
  
    if ((strcmp(ent->d_name,".") != 0) && 
        (strcmp(ent->d_name,"..") != 0)) {
    
      fpath filename = folder;
      
      // If does not end in "/" then copy it.
      if (folder.ptr[strlen(folder.ptr)-1] != '/') {
        strcat(filename.ptr, "/");
      }
      
      strcat(filename.ptr, ent->d_name);
      
      DIR* sub = opendir(filename.ptr);
      if (sub) {
        folder_load_recursive(filename);
      }
      
      if (!file_isloaded(filename)) {
        file_load(filename);
      }
    } 
  }
  
  closedir(dir);

}
*/

pub fn folder_load_recursive(folder: &str) {
  let folder2 = asset_map_filename(folder);
  let mut debug_full = String::new();
  let _ = write!(debug_full, "Loading folder recursively: '{}'", folder2);
  debug(&debug_full);

  let dir = Path::new(&folder2);
  //I think this is NOT recursive and should ignore sub directories
  if dir.is_dir() {
    for entry in fs::read_dir(dir).unwrap() {
      let entry = entry.unwrap();
      let path = entry.path();
      if path.is_dir() {
        let pathstring = path.to_str().unwrap();
        folder_load_recursive(pathstring);
      //visit_dirs(&path, cb)?;
      } else {
        //cb(&entry);
        let pathstring = path.to_str().unwrap();
        if !file_isloaded(pathstring) {
          file_load(pathstring);
        }
      }
    }
  }
}


/*

void file_reload(fpath filename) {
  file_unload(filename);
  file_load(filename);
  asset_cache_flush();
}

*/

pub fn file_reload(filename: &str) {
  file_unload(filename);
  file_load(filename);
  asset_cache_flush();
}

/*

void folder_reload(fpath folder) {
  folder_unload(folder);
  folder_load(folder);
  asset_cache_flush();
}

*/

pub fn folder_reload(folder: &str) {
  folder_unload(folder);
  folder_load(folder);
  asset_cache_flush();
}

/*
void file_unload(fpath filename) {
  
  filename = asset_map_filename(filename);
  
  fpath ext;
  SDL_PathFileExtension(ext.ptr, filename.ptr);
  
  for(int i=0; i < num_asset_handlers; i++) {
  
    asset_handler handler = asset_handlers[i];
    if (strcmp(ext.ptr, handler.extension) == 0) {
      debug("Unloading: '%s'", filename.ptr);
      dict_remove_with(asset_dict, filename.ptr, handler.del_func);
      break;
    }
    
  }
}
*/

pub fn file_unload(filename: &str) {
  let filename2 = asset_map_filename(filename);

  let pathfilename = Path::new(&filename2);

  let ext_opt = pathfilename.extension();

  let asset_dict = unsafe {
    match G_ASSET_STATE.asset_dict.as_mut() {
      Some(dict) => dict,
      None => {
        return;
      }
    }
  };

  let asset_handlers = unsafe {
    match G_ASSET_STATE.asset_handlers.as_ref() {
      Some(handlers) => handlers,
      None => {
        return;
      }
    }
  };


  match ext_opt {
    Some(ext) => {
      let ext_str: String = ext.to_str().unwrap().into();
      for asset_handler in asset_handlers {
        if ext_str.eq(&asset_handler.extension) {
          let mut full_debug = String::new();
          let _ = write!(&mut full_debug, "Unloading: '{}'", filename2);
          debug(&full_debug);
          let _ = asset_dict.remove(&filename2);
          //match asset {
          //  Some(asset) => (asset_handler.del_func)(asset),
          //  None => {}
          //}
          break;
        }
      }
    }
    None => {}//no extension?
  }
}


/*

void folder_unload(fpath folder) {
    
  folder = asset_map_filename(folder);
  
  debug("Unloading Folder: '%s'", folder.ptr);
  DIR* dir = opendir(folder.ptr);
  
  if (dir == NULL) {
    error("Could not open directory '%s' to unload.\n", folder.ptr);
  }
  
  struct dirent* ent;
  while ((ent = readdir(dir)) != NULL) {
  
    if ((strcmp(ent->d_name,".") != 0) && 
        (strcmp(ent->d_name,"..") != 0)) {
    
      fpath filename = folder;
      strcat(filename.ptr, ent->d_name);
      
      if(dict_contains(asset_dict, filename.ptr) ) {
        file_unload(filename);
      }
      
    } 
  }
  closedir(dir);
}

*/

pub fn folder_unload(folder: &str) {
  let folder2 = asset_map_filename(folder);

  let mut debug_full = String::new();
  let _ = write!(debug_full, "Unloading folder: '{}'", folder2);
  debug(&debug_full);

  let asset_dict = unsafe {
    match G_ASSET_STATE.asset_dict.as_mut() {
      Some(dict) => dict,
      None => {
        return;
      }
    }
  };

  let dir = Path::new(&folder2);
  //I think this is NOT recursive and should ignore sub directories
  if dir.is_dir() {
    for entry in fs::read_dir(dir).unwrap() {
      let entry = entry.unwrap();
      let path = entry.path();
      if path.is_dir() {
        //visit_dirs(&path, cb)?;
      } else {
        let pathstring = path.to_str().unwrap();

        if asset_dict.contains_key(pathstring) {
          file_unload(pathstring);
        }
      }
    }
  }
}
/*

bool file_isloaded(fpath path) {
  path = asset_map_filename(path);
  return dict_contains(asset_dict, path.ptr);
}
*/

pub fn file_isloaded(path: &str) -> bool {
  let path2 = asset_map_filename(path);

  let asset_dict = unsafe {
    match G_ASSET_STATE.asset_dict.as_ref() {
      Some(dict) => dict,
      None => {
        return false;
      }
    }
  };

  return asset_dict.contains_key(&path2);
}

/*

asset* asset_get_load(fpath path) {
  asset_hndl ah = asset_hndl_new_load(path);
  return asset_hndl_ptr(&ah);
}

*/

pub fn asset_get_load<'a, T>(path: &str) -> Option<Rc<RefCell<T>>>
where
  T: Any,
{
  let mut ah = asset_hndl_new_load(path);
  return asset_hndl_ptr(&mut ah);
}

/*

asset* asset_get(fpath path) {
  asset_hndl ah = asset_hndl_new(path);
  return asset_hndl_ptr(&ah);
}
*/

pub fn asset_get<'a, T>(path: &str) -> Option<Rc<RefCell<T>>>
where
  T: Any,
{
  let mut ah = asset_hndl_new(path);
  return asset_hndl_ptr(&mut ah);
}

/*
asset* asset_get_as_type(fpath path, type_id type) {
  /* TODO: Type checking */
  return asset_get(path);
}
*/

//pub fn asset_get_as_type(path: &str, asset_type: &str) -> Rc<Any> {
//TODO: Type checking
//  return asset_get(path);
//}

/*

void asset_reload_type_id(type_id type) {

  debug("Reloading Assets of type '%s'...", type_id_name(type));
  
  fpath ext;
  
  /* Find the file extension for given type */
  for(int i=0; i < num_asset_handlers; i++) {
    asset_handler handler = asset_handlers[i];
    if (handler.type == type) {
      strcpy(ext.ptr, handler.extension);
      break;
    }
  }
  
  list* asset_names = list_new();
  
  for(int i = 0; i < asset_dict->size; i++) {
    struct bucket* b = asset_dict->buckets[i];
    while(b != NULL) {
      fpath bucket_ext;
      SDL_PathFileExtension(bucket_ext.ptr, b->key);
      
      if (strcmp(bucket_ext.ptr, ext.ptr) == 0) {
        char* new_name = malloc(strlen(b->key)+1);
        strcpy(new_name, b->key);
        list_push_back(asset_names, new_name);
      }
      
      b = b->next;
    }
  }

  for(int i = 0; i < asset_names->num_items; i++) {
    file_unload(P(list_get(asset_names, i)));
  }
  
  for(int i = 0; i < asset_names->num_items; i++) {
    /*
    ** Assets can reload their child assets before we do
    ** So it is important we check before loading again
    */
    if (!file_isloaded(P(list_get(asset_names, i)))) {
      file_load(P(list_get(asset_names, i)));
    }
  }
  
  list_delete_with(asset_names, free);
  
  asset_cache_flush();
}
*/

pub fn asset_reload_type_id(asset_type: &str) {
  let mut debug_full = String::new();
  let _ = write!(debug_full, "Reloading assets of type '{}'...", asset_type);

  let mut asset_names: Vec<String> = Vec::new();

  {
    let asset_dict = unsafe {
      match G_ASSET_STATE.asset_dict.as_ref() {
        Some(dict) => dict,
        None => {
          return;
        }
      }
    };

    for strpath in asset_dict.keys() {
      let path = Path::new(strpath);
      let ext_str: String = path.extension().unwrap().to_str().unwrap().into();
      if ext_str.eq(asset_type) {
        asset_names.push(strpath.clone());
      }
    }
  }

  for strpath in &asset_names {
    file_unload(&strpath)
  }

  for strpath in &asset_names {
    if !file_isloaded(&strpath) {
      file_load(&strpath);
    }
  }

  asset_cache_flush();
}

/*

void asset_reload_all() {
  
  debug("Reloading All Assets...");
  
  list* asset_names = list_new();
  
  for(int i = 0; i < asset_dict->size; i++) {
    struct bucket* b = asset_dict->buckets[i];
    while(b != NULL) {
      char* new_name = malloc(strlen(b->key)+1);
      strcpy(new_name, b->key);
      list_push_back(asset_names, new_name);
      b = b->next;
    }
  }
  
  for(int i = 0; i < asset_names->num_items; i++) {
    file_unload(P(list_get(asset_names, i)));
  }
  
  for(int i = 0; i < asset_names->num_items; i++) {
    /*
    ** Assets can reload their child assets before we do
    ** So it is important we check before loading again
    */
    if (!file_isloaded(P(list_get(asset_names, i)))) {
      file_load(P(list_get(asset_names, i)));
    }
  }
  
  list_delete_with(asset_names, free);
  
  asset_cache_flush();
}
*/

pub fn asset_reload_all() {
  debug("Reloading All Assets...");

  let mut asset_names: Vec<String> = Vec::new();

  {
    let asset_dict = unsafe {
      match G_ASSET_STATE.asset_dict.as_mut() {
        Some(dict) => dict,
        None => {
          return;
        }
      }
    };

    for strpath in asset_dict.keys() {
      asset_names.push(strpath.clone());
    }
  }

  for strpath in &asset_names {
    //apparently .iter is a thing. If I don't ref it, it eats the vector
    file_unload(&strpath);
  }

  for strpath in &asset_names {
    if !file_isloaded(&strpath) {
      file_load(&strpath);
    }
  }

  asset_cache_flush();
}

/*

char* asset_ptr_path(asset* a) {
  char* path = dict_find(asset_dict, a);
  if (path == NULL) {
    error("Asset dict doesn't contain asset pointer %p", a);
    return NULL;
  } else {
    return path; 
  }
}

*/

pub fn asset_ptr_path(a: &Rc<dyn Any>) -> String {
  /*let strong = Weak::upgrade(&a);

    let strong = match strong {
        Some(strong) => strong,
        None => return String::new() //doesn't exist anywhere
    };


    */

  {
    let asset_dict = unsafe {
      match G_ASSET_STATE.asset_dict.as_mut() {
        Some(dict) => dict,
        None => {
          return String::new();
        }
      }
    };

    for (key, val) in asset_dict.iter() {
      if Rc::ptr_eq(a, val) {
        return key.clone().into();
      }
    }
  }

  return String::new();
}

/*

char* asset_ptr_typename(asset* a) {
  char* path = dict_find(asset_dict, a);
  if (path == NULL) {
    error("Asset dict doesn't contain asset pointer %p", a);
    return NULL;
  }
  
  fpath ext;
  SDL_PathFileExtension(ext.ptr, path);
  
  for(int i=0; i < num_asset_handlers; i++) {
    asset_handler handler = asset_handlers[i];
    if (strcmp(ext.ptr, handler.extension) == 0) {
      return type_id_name(handler.type);
    }
  }
  
  return NULL;
}
*/
