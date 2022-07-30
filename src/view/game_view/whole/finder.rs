use rich_sdl2_rust::ttf::font::{
  pen::{FontRenderExt, FontRenderOptions, TextAlign, TextAlignX},
  Font, RenderMode, StyleExt,
};
use rich_sdl2_rust::{
  color::Rgb,
  geo::{Point, Rect, Size},
  renderer::pen::Pen,
};
use std::rc::Rc;

use crate::{
  model::exp::sentence::{Sentence, TypingStr},
  view::Component,
};

#[derive(PartialEq)]
pub struct FinderProps {
  pub sentence: Sentence,
  pub remaining_ratio: f64,
}

pub struct Finder<'font> {
  props: FinderProps,
  client: Rect,
  font: Rc<Font<'font>>,
}

impl<'font> Finder<'font> {
  pub fn new(
    mut initial_props: FinderProps,
    font: Rc<Font<'font>>,
    client: Rect,
  ) -> Self {
    initial_props.remaining_ratio =
      initial_props.remaining_ratio.max(0.).min(1.);
    Self {
      props: initial_props,
      client,
      font,
    }
  }
}

impl<'font> Component for Finder<'font> {
  type Props = FinderProps;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    &self.props != new_props
  }

  fn update(&mut self, new_props: Self::Props) {
    self.props = new_props;
  }

  fn render(&self, pen: &Pen<'_>) {
    let Finder {
      props,
      font,
      client,
    } = &self;
    let FinderProps {
      remaining_ratio,
      sentence,
    } = &props;

    pen.set_color(Rgb {
      r: 230,
      g: 220,
      b: 200,
    });
    pen.fill_rect(*client);

    let remaining_width =
      (client.size.width as f64 * remaining_ratio) as u32;
    pen.set_color(Rgb {
      r: 203,
      g: 193,
      b: 176,
    });
    pen.fill_rect(Rect {
      size: Size {
        width: remaining_width,
        ..client.size
      },
      ..*client
    });

    const JAPANESE_HEIGHT: u32 = 30;
    let half_x = (client.size.width / 2) as i32;
    let will_input_japanese = sentence.origin();
    font.set_font_size(JAPANESE_HEIGHT).unwrap();
    pen.text(
      font,
      will_input_japanese,
      FontRenderOptions::new()
        .mode(RenderMode::Blended {
          foreground: 0x505050.into(),
        })
        .align(TextAlign {
          x: TextAlignX::Left,
          ..Default::default()
        })
        .pivot(client.up_left),
    );

    const ROMAN_HEIGHT: u32 = 40;
    font.set_font_size(ROMAN_HEIGHT).unwrap();
    {
      let TypingStr {
        will_input,
        inputted,
      } = sentence.roman();

      pen.text(
        font,
        &will_input,
        FontRenderOptions::new()
          .mode(RenderMode::Blended {
            foreground: 0.into(),
          })
          .align(TextAlign {
            x: TextAlignX::Left,
            ..Default::default()
          })
          .pivot(Point {
            x: half_x + 5,
            y: client.bottom() - ROMAN_HEIGHT as i32 - 20,
          }),
      );

      pen.text(
        font,
        &inputted,
        FontRenderOptions::new()
          .mode(RenderMode::Blended {
            foreground: 0x505050.into(),
          })
          .align(TextAlign {
            x: TextAlignX::Right,
            ..Default::default()
          })
          .pivot(Point {
            x: half_x - 5,
            y: client.bottom() - ROMAN_HEIGHT as i32 - 20,
          }),
      );
    }
    const YOMIGANA_HEIGHT: u32 = 80;
    font.set_font_size(YOMIGANA_HEIGHT).unwrap();
    {
      let TypingStr {
        will_input,
        inputted,
      } = sentence.yomiagana();

      pen.text(
        font,
        &will_input,
        FontRenderOptions::new()
          .align(TextAlign {
            x: TextAlignX::Left,
            ..Default::default()
          })
          .pivot(Point {
            x: half_x + 5,
            y: client.bottom()
              - ROMAN_HEIGHT as i32
              - YOMIGANA_HEIGHT as i32
              - 20,
          }),
      );

      pen.text(
        font,
        &inputted,
        FontRenderOptions::new()
          .mode(RenderMode::Blended {
            foreground: 0x505050.into(),
          })
          .align(TextAlign {
            x: TextAlignX::Right,
            ..Default::default()
          })
          .pivot(Point {
            x: half_x - 5,
            y: client.bottom()
              - ROMAN_HEIGHT as i32
              - YOMIGANA_HEIGHT as i32
              - 20,
          }),
      );
    }
  }
}
