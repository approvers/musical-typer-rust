use rich_sdl2_rust::{color::Rgba, geo::Rect, renderer::pen::Pen};
use rich_sdl2_ttf_rust::font::{
  pen::{FontRenderExt, FontRenderOptions, TextAlign, TextAlignX},
  Font, RenderMode, StyleExt,
};
use std::rc::Rc;

use crate::{model::exp::scoremap::MusicInfo, view::Component};

#[derive(PartialEq)]
pub struct HeaderProps {
  pub font: Rc<Font>,
  pub music_info: MusicInfo,
  pub score_point: i32,
}

pub struct Header {
  props: HeaderProps,
  client: Rect,
}

impl Header {
  pub fn new(props: HeaderProps, client: Rect) -> Self {
    Self { props, client }
  }
}

impl Component for Header {
  type Props = HeaderProps;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    &self.props != new_props
  }

  fn update(&mut self, new_props: Self::Props) {
    self.props = new_props;
  }

  fn render(&self, pen: &Pen<'_>) {
    let &Header { props, client } = &self;
    let &HeaderProps {
      font,
      music_info,
      score_point,
    } = &props;

    let title = &music_info.title;
    let author = &music_info.song_author;

    font.set_font_size(60);
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

    font.set_font_size(30);
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

    font.set_font_size(70);
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
