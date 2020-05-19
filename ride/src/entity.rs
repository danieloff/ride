use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use std::fmt::Write;

use crate::core::{error, debug};

/*
#include "centity.h"

#include "data/list.h"
#include "data/dict.h"

typedef struct {
  int type_id;
  void* (*new_func)();
  void (*del_func)();
} entity_handler;

*/

pub struct EntityHandler<T> {
  //type_id
  new_func: Box<dyn Fn() -> Rc<RefCell<T>>>,
//  del_func: Box<Fn(Rc<RefCell<T>>)>,
}

/*

#define MAX_ENTITY_HANDLERS 512
static entity_handler entity_handlers[MAX_ENTITY_HANDLERS];
static int num_entity_handlers = 0;

static list* entity_names = NULL;
static dict* entities = NULL;
static dict* entity_types = NULL;
*/

struct EntityStaticData {
  entity_handlers: Option<Vec<Rc<dyn Any>>>,
  entities: Option<HashMap<String, Rc<dyn Any>>>,
}

/*
void entity_init(void) {
  entities = dict_new(512);
  entity_types = dict_new(512);
  entity_names = list_new(512);
}
*/

static mut G_ENTITY_STATE: EntityStaticData = EntityStaticData {
  entities: None,
  entity_handlers: None,
};

pub fn entity_init() {
  unsafe {
    G_ENTITY_STATE.entities = Some(HashMap::new());
    G_ENTITY_STATE.entity_handlers = Some(Vec::new());
  }
}

/*
void entity_finish(void) {
  
  while (entity_names->num_items > 0) {
    entity_delete(list_get(entity_names, 0));
  }
  
  list_delete_with(entity_names, free);
  
  dict_delete(entities);
  
  dict_map(entity_types, free);
  dict_delete(entity_types);
  
}
*/

pub fn entity_finish() {
  if let Some(entities) = unsafe { G_ENTITY_STATE.entities.as_mut() } {
    entities.clear();
  }
}

/*
void entity_handler_cast(int type_id, void* entity_new_func() , void entity_del_func(void* entity)) {
  
  if (num_entity_handlers >= MAX_ENTITY_HANDLERS) {
    warning("Max number of entity handlers reached. Handler for type %s not added.", type_id_name(type_id));
    return;
  }
  
  entity_handler eh;
  eh.type_id = type_id;
  eh.new_func = entity_new_func;
  eh.del_func = entity_del_func;
  
  entity_handlers[num_entity_handlers] = eh;
  num_entity_handlers++;
}
*/

pub fn entity_handler<T>(
  entity_new_func: fn() -> Rc<RefCell<T>>,
//  entity_del_func: fn(Rc<RefCell<T>>),
) where
  T: Any,
{
  entity_handler_cast(Box::new(entity_new_func)); // Box::new(entity_del_func))
}

pub fn entity_handler_cast<T>(
  entity_new_func: Box<dyn Fn() -> Rc<RefCell<T>>>,
//  entity_del_func: Box<Fn(Rc<RefCell<T>>)>,
) where
  T: Any,
{

  let eh = EntityHandler {
    new_func: entity_new_func,
  //  del_func: entity_del_func,
  };

  let reh: Rc<dyn Any> = Rc::new(RefCell::new(eh));

  if let Some(entity_handlers) = unsafe { G_ENTITY_STATE.entity_handlers.as_mut() } {
    entity_handlers.push(reh);
  }
}

/*

entity* entity_new_type_id(char* fmt, int type_id, ...) {

  char entity_name_buff[512];
  
  va_list args;
  va_start(args, type_id);
  vsnprintf(entity_name_buff, 511, fmt, args);
  va_end(args);
  
  if ( dict_contains(entities, entity_name_buff) ) {
    error("Entity Manager already contains entity called %s!", entity_name_buff);
  }
  
  debug("Creating Entity %s (%s)", entity_name_buff, type_id_name(type_id));
  
  entity* e = NULL;
  
  for(int i = 0; i < num_entity_handlers; i++) {
    entity_handler eh = entity_handlers[i];
    if (eh.type_id == type_id) {
      e = eh.new_func();
      break;
    }
  }
  
  if (e == NULL) {
    error("Don't know how to create entity %s. No handler for type %s!", entity_name_buff, type_id_name(type_id));
  }
  
  dict_set(entities, entity_name_buff, e);
  
  int* type_ptr = malloc(sizeof(int));
  *type_ptr = type_id;
  dict_set(entity_types, entity_name_buff, type_ptr);
  
  char* name_copy = malloc(strlen(entity_name_buff) + 1);
  strcpy(name_copy, entity_name_buff);
  list_push_back(entity_names, name_copy);
  
  return e;
}
*/
pub fn entity_new<T>(name: &str, typestring: &str) -> Option<Rc<RefCell<T>>>
where
  T: Any,
{
  if let Some(entities) = unsafe { G_ENTITY_STATE.entities.as_mut() } {
    if entities.contains_key(name) {
      let mut error_full = String::new();
      let _ = write!(
        &mut error_full,
        "Entity manager already contains an entity called {}",
        name
      );
      error(&error_full);
      //TODO, should this return the old entity? return nothing? crash? the hashmap might crash
      match entities.get(name) {
        Some(rc) => {
          match rc.clone().downcast::<RefCell<T>>() {
            Ok(result) => return Some(result),
            Err(_) => return None, //the duplicate entity was probably of a different type, TODO display an error message?
          }

        }
        None => {} //?? This should be impossible
      }
    }

    {
      let mut debug_full = String::new();
      let _ = write!(&mut debug_full, "Creating Entity {} ({})", name, typestring);
      debug(&debug_full);
    }

    if let Some(entity_handlers) = unsafe { G_ENTITY_STATE.entity_handlers.as_ref() } {
      for handler in entity_handlers {
        match handler.clone().downcast::<RefCell<EntityHandler<T>>>() {
          Ok(ohandler) => {
            let e = (*ohandler.borrow().new_func)();

            //let rce:Rc<Any> = e.clone();

            let _ = entities.insert(name.into(), e.clone());

            return Some(e);
          }
          Err(_) => {}
        }
      }

    }

  }

  {
    let mut error_full = String::new();
    let _ = write!(
      &mut error_full,
      "Don't know how to create {}, no handler for type {}",
      name,
      typestring
    );
    error(&error_full);
  }

  return None;
}



/*

bool entity_exists(char* fmt, ...) {

  char entity_name_buff[512];

  va_list args;
  va_start(args, fmt);
  vsnprintf(entity_name_buff, 511, fmt, args);
  va_end(args);
  
  return dict_contains(entities, entity_name_buff);
  
}

entity* entity_get(char* fmt, ...) {
  
  char entity_name_buff[512];
  
  va_list args;
  va_start(args, fmt);
  vsnprintf(entity_name_buff, 511, fmt, args);
  va_end(args);
  
  if ( !dict_contains(entities, entity_name_buff) ) {
    error("Entity %s does not exist!", entity_name_buff);
  }

  return dict_get(entities, entity_name_buff);
  
}
*/

pub fn entity_get<T>(name: &str) -> Option<Rc<RefCell<T>>>
where
  T: Any,
{
  if let Some(entities) = unsafe { G_ENTITY_STATE.entities.as_mut() } {
    if let Some(e) = entities.get(name) {
      let f = e.clone().downcast::<RefCell<T>>().ok();
      return f;
    }
  }
  return None;
}

/*
entity* entity_get_as_type_id(char* fmt, int type_id, ...) {
  
  char entity_name_buff[512];
  
  va_list args;
  va_start(args, type_id);
  vsnprintf(entity_name_buff, 511, fmt, args);
  va_end(args);
  
  if ( !entity_exists(entity_name_buff) ) {
    error("Entity %s does not exist!", entity_name_buff);
  }
  
  int* entity_type = dict_get(entity_types, entity_name_buff);
  
  if (*entity_type != type_id) {
    error("Entity %s was created/added as a %s, but you requested it as a %s!", entity_name_buff, type_id_name(*entity_type), type_id_name(type_id));
  }
  
  return dict_get(entities, entity_name_buff);
}

void entity_delete(char* fmt, ...) {
  
  char entity_name_buff[512];
  
  va_list args;
  va_start(args, fmt);
  vsnprintf(entity_name_buff, 511, fmt, args);
  va_end(args);
  
  int* type_ptr = dict_get(entity_types, entity_name_buff);
  int type_id = *type_ptr;
  
  debug("Deleting Entity %s (%s)", entity_name_buff, type_id_name(type_id));
  
  for(int i = 0; i < num_entity_handlers; i++) {
    entity_handler eh = entity_handlers[i];
    if (eh.type_id == type_id) {
      dict_remove_with(entities, entity_name_buff, eh.del_func);
      break;
    }
  }
  
  for(int i = 0; i < entity_names->num_items; i++) {
    if ( strcmp(list_get(entity_names, i), entity_name_buff) == 0 ) {
      char* name = list_pop_at(entity_names, i);
      free(name);
      break;
    }
  }
  
  if (entity_exists(entity_name_buff)) {
    error("Don't know how to delete entity %s. No handler for type %s!", entity_name_buff, type_id_name(type_id));
  }
  
}

int entity_type_count_type_id(int type_id) {
  
  int count = 0;
  
  for(int i = 0; i < entity_names->num_items; i++) {
    char* name = list_get(entity_names, i);
    int* type = dict_get(entity_types, name);
    
    if (*type == type_id) {
      count++;
    }
  }
  
  return count;
  
}

char* entity_name(entity* e) {
  
  for(int i = 0; i < entity_names->num_items; i++) {
    char* name = list_get(entity_names, i);
    entity* ent = dict_get(entities, name);
    
    if (ent == e) {
      return name;
    }
  }
  
  error("Object at %p not loaded into entity manager. Cannot fetch name.", e);
  return NULL;
}

char* entity_typename(entity* e) {
  
  for(int i = 0; i < entity_names->num_items; i++) {
    char* name = list_get(entity_names, i);
    entity* ent = dict_get(entities, name);
    
    if (ent == e) {
      int* type = dict_get(entity_types, name);
      return type_id_name(*type);
    }
  }
  
  error("Object at %p not loaded into entity manager. Cannot fetch type name.", e);
  return NULL;
}

void entities_new_type_id(const char* name_format, int count, int type_id) {
  
  char entity_name[512];
  
  if(strlen(name_format) - 2 + ((count+1)/10) > 512) {
    error("Name pattern and count are potentially longer than %i characters. Wont fit in buffer.", 512);
  }
  
  if(!strstr(name_format, "%i")) {
    error("Name format must be like a sprintf format string and contain a %%i symbol for the entity index. E.G \"entity_%%i\"");
  }
  
  for(int i = 0; i < count; i++) {
    sprintf(entity_name, name_format, i);
    entity_new_type_id(entity_name, type_id);
  }
  
}

void entities_get_type_id(entity** out, int* returned, int type_id) {
  
  int count = 0;
  
  for(int i = 0; i < entity_names->num_items; i++) {
    char* name = list_get(entity_names, i);
    int* type = dict_get(entity_types, name);
    entity* ent = dict_get(entities, name);
    
    if (*type == type_id) {
      out[count] = ent;
      count++;
    }
  }
  
  if (returned != NULL) {
    *returned = count;
  }
  
}
*/
