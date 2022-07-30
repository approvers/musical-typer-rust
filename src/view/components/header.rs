use rich_sdl2_rust::ttf::font::{
  pen::{FontRenderExt, FontRenderOptions, TextAlign, TextAlignX},
  Font, RenderMode, StyleExt,
};
use rich_sdl2_rust::{color::Rgba, geo::Rect, renderer::pen::Pen};
use std::rc::Rc;

use crate::{model::exp::scoremap::MusicInfo, view::Component};

#[derive(PartialEq)]
pub struct HeaderProps {
  pub music_info: MusicInfo,
  pub score_point: i32,
}

pub struct Header<'font> {
  props: HeaderProps,
  font: Rc<Font<'font>>,
  client: Rect,
}

impl<'font> Header<'font> {
  pub fn new(
    props: HeaderProps,
    font: Rc<Font<'font>>,
    client: Rect,
  ) -> Self {
    Self {
      props,
      font,
      client,
    }
  }
}

impl<'font> Component for Header<'font> {
  type Props = HeaderProps;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    &self.props != new_props
  }

  fn update(&mut self, new_props: Self::Props) {
    self.props = new_props;
  }

  fn render(&self, pen: &Pen<'_>) {
    let &Header {
      font,
      props,
      client,
    } = &self;
    let &HeaderProps {
      music_info,
      score_point,
    } = &props;

    let title = &music_info.title;
    let author = &music_info.song_author;

    font.set_font_size(30).unwrap();
    pen.text(
      font,
      title,
      FontRenderOptions::new()
        .mode(RenderMode::Blended {
          foreground: Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
          },
        })
        .align(TextAlign {
          x: TextAlignX::Right,
          ..Default::default()
        })
        .pivot(client.top_right().offset(-5, 5)),
    );

    font.set_font_size(15).unwrap();
    pen.text(
      font,
      author,
      FontRenderOptions::new()
        .mode(RenderMode::Blended {
          foreground: Rgba {
            r: 156,
            g: 156,
            b: 162,
            a: 255,
          },
        })
        .align(TextAlign {
          x: TextAlignX::Right,
          ..Default::default()
        })
        .pivot(client.bottom_right().offset(-5, -35)),
    );

    font.set_font_size(35).unwrap();
    pen.text(
      font,
      &format!("{:08}", score_point),
      FontRenderOptions::new()
        .mode(RenderMode::Blended {
          foreground: Rgba {
            r: 64,
            g: 79,
            b: 181,
            a: 255,
          },
        })
        .pivot(client.bottom_left().offset(5, -60)),
    );
  }
}
