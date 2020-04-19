use sdl2::pixels::Color;
use sdl2::rect::Rect;

use super::ViewError;
use crate::{
  model::exp::sentence::Sentence, view::renderer::Renderer,
};

mod finder;
mod header;
mod keyboard;

use finder::Finder;
use header::Header;
use keyboard::Keyboard;

pub struct WholeProps<'a> {
  pub pressed_keys: &'a [char],
  pub sentence: &'a Option<Sentence>,
  pub title: &'a str,
  pub song_author: &'a str,
  pub score_point: i32,
}

pub fn render(
  mut canvas: &mut Renderer,
  client: Rect,
  props: &WholeProps,
) -> Result<(), ViewError> {
  canvas.set_draw_color(Color::RGB(253, 243, 226));
  canvas.clear();

  {
    let header =
      Header::new(props.title, props.song_author, props.score_point);
    let header_dim = Rect::new(0, 0, client.width(), 100);
    header.draw(&mut canvas)?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.draw_rect(header_dim)?;
  }

  {
    let finder = Finder::new(props.sentence, 0.2);
    let finder_dim = Rect::new(0, 100, client.width(), 200);
    finder.draw(&mut canvas, finder_dim)?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.draw_rect(finder_dim)?;
  }

  {
    let keyboard = Keyboard::new(props.pressed_keys, &[]);
    let keyboard_dim =
      Rect::new(0, client.height() as i32 - 300, client.width(), 300);
    keyboard.draw(&mut canvas, keyboard_dim)?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.draw_rect(keyboard_dim)?;
  }

  Ok(())
}
