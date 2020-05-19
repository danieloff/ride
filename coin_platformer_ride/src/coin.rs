/*
//coin.h

#ifndef coin_h
#define coin_h

#include "corange.h"

typedef struct {
  vec2 position;
} coin;
*/
use std::rc::Rc;
use std::cell::RefCell;

use cgmath::*;

use ride::asset::*;
use ride::assets::texture::*;
use ride::engine::*;

pub struct Coin {
  pub position: Vec2
}

impl Coin {
  fn new() -> Self {
    return Coin{position: Vec2::zero()};
  }
//why do I have to specifically make this public but not new()??
  pub fn new_rc() -> Rc<RefCell<Self>> {
    return Rc::new(RefCell::new(Self::new()));
  }

  fn render(&self, camera_position:Vec2) {
//projection identity
//->ortho
//modelview identity
//blend alpha, one minus source alpha
//texture2d
//
    let coin_tex = asset_get::<Texture>("./tiles/coin.dds");
 //bind texture 
//  glBegin(GL_QUADS);
   /* 
    glTexCoord2f(0, 1); glVertex3f(c->position.x, c->position.y + 32, 0);
    glTexCoord2f(1, 1); glVertex3f(c->position.x + 32, c->position.y + 32, 0);
    glTexCoord2f(1, 0); glVertex3f(c->position.x + 32, c->position.y, 0);
    glTexCoord2f(0, 0); glVertex3f(c->position.x, c->position.y, 0);
   */
   }
}

/*
coin* coin_new();
void coin_delete(coin* c);

void coin_render(coin* c, vec2 camera_position);


#endif
#include <stdlib.h>

#include "coin.h"

coin* coin_new() {
  coin* c = malloc(sizeof(coin));
  c->position = vec2_zero();
  return c;
}

void coin_delete(coin* c) {
  free(c);
}

/* Renders a quad to the screen with coin texture */

void coin_render(coin* c, vec2 camera_position) {

	glMatrixMode(GL_PROJECTION);
  glPushMatrix();
	glLoadIdentity();
	glOrtho(camera_position.x - graphics_viewport_width() / 2, 
          camera_position.x + graphics_viewport_width() / 2,
          -camera_position.y + graphics_viewport_height() / 2,
          -camera_position.y - graphics_viewport_height() / 2
          , 0, 1);
  
	glMatrixMode(GL_MODELVIEW);
  glPushMatrix();
	glLoadIdentity();
  
  glEnable(GL_BLEND);
  glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
  
  glEnable(GL_TEXTURE_2D);
  
  texture* coin_tex = asset_get(P("./tiles/coin.dds"));
  glBindTexture(GL_TEXTURE_2D, texture_handle(coin_tex));
  
  glBegin(GL_QUADS);
    
    glTexCoord2f(0, 1); glVertex3f(c->position.x, c->position.y + 32, 0);
    glTexCoord2f(1, 1); glVertex3f(c->position.x + 32, c->position.y + 32, 0);
    glTexCoord2f(1, 0); glVertex3f(c->position.x + 32, c->position.y, 0);
    glTexCoord2f(0, 0); glVertex3f(c->position.x, c->position.y, 0);
    
  glEnd();
  
  glDisable(GL_TEXTURE_2D);
  
  glDisable(GL_BLEND);
  
  glMatrixMode(GL_PROJECTION);
  glPopMatrix();
  
	glMatrixMode(GL_MODELVIEW);
  glPopMatrix();

}
*/

