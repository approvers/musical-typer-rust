use rich_sdl2_rust::{
  color::Rgb,
  geo::{Point, Rect, Size},
  renderer::pen::Pen,
};
use rich_sdl2_ttf_rust::font::{
  pen::{
    FontRenderExt, FontRenderOptions, TextAlign, TextAlignX,
    TextAlignY,
  },
  Font,
};
use std::rc::Rc;

use crate::view::Component;

const BLUE: Rgb = 0x4050b4.into();
const ORANGE: Rgb = 0xd19a1d.into();
const GREEN: Rgb = 0x144c40.into();
const BACK: Rgb = 0xfdf3e2.into();
const BLACK: Rgb = 0.into();
const GRAY: Rgb = 0xc3c3be.into();

struct KeyCell<'font> {
  font: Rc<Font<'font>>,
  key: char,
  is_highlighted: bool,
  is_pressed: bool,
  client: Rect,
}

impl PartialEq for KeyCell<'_> {
  fn eq(&self, other: &Self) -> bool {
    self.key == other.key
      && self.is_highlighted == other.is_highlighted
      && self.is_pressed == other.is_pressed
      && self.client == other.client
  }
}

impl KeyCell<'_> {
  fn bg_color(&self) -> Rgb {
    if self.is_highlighted {
      GREEN
    } else {
      BACK
    }
  }

  fn text_color(&self) -> Rgb {
    if self.is_pressed {
      ORANGE
    } else if self.is_highlighted {
      GRAY
    } else if self.key == 'f' || self.key == 'j' {
      BLUE
    } else {
      BLACK
    }
  }
}

impl Component for KeyCell<'_> {
  type Props = Self;

  fn update(&mut self, new_props: Self::Props) {
    *self = new_props;
  }

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    self != new_props
  }

  fn render(&self, pen: &Pen<'_>) {
    let border_dim = Rect::from_center(
      self.client.center(),
      Size {
        width: self.client.size.width - 5,
        height: self.client.size.height - 5,
      },
    );
    pen.set_color(self.bg_color());
    pen.fill_rect(border_dim);
    pen.set_color(BLACK);
    pen.stroke_rect(border_dim);

    pen.text(
      &self.font,
      &self.key.to_string().to_uppercase(),
      FontRenderOptions::new()
        .align(TextAlign {
          x: TextAlignX::Center,
          y: TextAlignY::Center,
        })
        .pivot(self.client.center()),
    );
  }
}

#[derive(PartialEq)]
pub struct KeyboardProps {
  pub pressed_keys: Vec<char>,
  pub highlighted_keys: Vec<char>,
}

pub struct Keyboard<'font> {
  props: KeyboardProps,
  cells: Vec<KeyCell<'font>>,
  font: Rc<Font<'font>>,
}

impl<'font> Keyboard<'font> {
  pub fn new(
    initial_props: KeyboardProps,
    font: Rc<Font<'font>>,
    client: Rect,
  ) -> Self {
    const CELL_ASPECT: f64 = 1.0;
    const KEY_CHARS_ROWS: &[&str] = &[
      "1234567890-^¥",
      "qwertyuiop@[",
      "asdfghjkl;:]",
      "zxcvbnm,./\\",
    ];

    let cell_height =
      client.size.height as f64 / KEY_CHARS_ROWS.len() as f64;
    let cell_width = cell_height * CELL_ASPECT;

    let mut cells = vec![];

    for (y, key_chars_row) in KEY_CHARS_ROWS.iter().enumerate() {
      let y = y as f64;
      let row_amount = key_chars_row.len() as f64;
      let margin = client.size.width as f64 - row_amount * cell_width;
      for (x, key_char) in key_chars_row.chars().enumerate() {
        let x = x as f64 + 1.0;
        let center = Point {
          x: (x * cell_width + client.left() as f64 + margin / 2.0)
            as i32,
          y: (y * cell_height
            + client.top() as f64
            + cell_height / 2.0) as i32,
        };
        let key_cell_client = Rect::from_center(
          center,
          Size {
            width: cell_width as u32,
            height: cell_height as u32,
          },
        );
        cells.push(KeyCell {
          font,
          key: key_char,
          is_highlighted: initial_props
            .highlighted_keys
            .contains(&key_char),
          is_pressed: initial_props.pressed_keys.contains(&key_char),
          client: key_cell_client,
        });
      }
    }

    Self {
      cells,
      props: initial_props,
      font,
    }
  }
}

impl<'font> Component for Keyboard<'font> {
  type Props = KeyboardProps;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    &self.props != new_props
  }

  fn update(&mut self, new_props: KeyboardProps) {
    self.props = new_props;
  }

  fn render(&self, ctx: &Pen<'_>) {
    for cell in &self.cells {
      cell.render(ctx);
    }
  }
}
