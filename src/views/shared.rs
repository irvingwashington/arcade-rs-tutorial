use sdl2::render::Renderer;
use phi::data::Rectangle;
use phi::gfx::{CopySprite, Sprite};

#[derive(Clone)]
pub struct Background {
    pub pos: f64,
    pub vel: f64,
    pub sprite: Sprite,
}

impl Background {
    pub fn render(&mut self, renderer: &mut Renderer, elapsed: f64) {
        let size = self.sprite.size();
        self.pos += self.vel * elapsed;
        if self.pos > size.0 {
            self.pos -= size.0;
        }

        let (win_w, win_h) = renderer.output_size().unwrap();
        let scale: f64 = (win_h as f64) / size.1;

        let mut physical_left = -self.pos * scale;

        while physical_left < win_w as f64 {
            renderer.copy_sprite(&self.sprite, Rectangle {
                x: physical_left,
                y: 0.0,
                w: size.0 * scale,
                h: win_h as f64,
            });

            physical_left += size.0 * scale;
        }
    }
}