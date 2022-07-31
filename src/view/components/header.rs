use rich_sdl2_rust::{
  color::Rgb,
  ttf::font::{
    pen::{FontRenderExt, FontRenderOptions, TextAlign, TextAlignX},
    Font, RenderMode, StyleExt,
  },
};
use rich_sdl2_rust::{color::Rgba, geo::Rect, renderer::pen::Pen};
use std::{rc::Rc, time::Instant};

use crate::{model::exp::scoremap::MusicInfo, view::Component};

#[derive(PartialEq)]
pub struct HeaderProps {
  pub music_info: MusicInfo,
  pub score_point: i32,
  pub sentence_result: Option<SentenceResult>,
}

#[derive(PartialEq)]
pub enum SentenceResult {
  Completed,
  Missed,
}

pub struct Header<'font> {
  props: HeaderProps,
  font: Rc<Font<'font>>,
  animating_texts: Vec<AnimatedText>,
  client: Rect,
}

pub struct AnimatedText {
  created_at: Instant,
  text: String,
  color: Rgb,
}
const ANIMATION_DURATION: f64 = 2.0;
const ANIMATION_SPEED: f64 = 8.0;

impl<'font> Header<'font> {
  pub fn new(
    props: HeaderProps,
    font: Rc<Font<'font>>,
    client: Rect,
  ) -> Self {
    Self {
      props,
      font,
      animating_texts: vec![],
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
    let filtered = self
      .animating_texts
      .drain(..)
      .filter(|text| {
        text.created_at.elapsed().as_secs_f64() < ANIMATION_DURATION
      })
      .collect();
    self.animating_texts = filtered;
    self.props = new_props;
    if let Some(res) = self.props.sentence_result.take() {
      let text = match res {
        SentenceResult::Completed => AnimatedText {
          created_at: Instant::now(),
          text: "AC".into(),
          color: Rgb {
            r: 0x72,
            g: 0xb5,
            b: 0x66,
          },
        },
        SentenceResult::Missed => AnimatedText {
          created_at: Instant::now(),
          text: "TLE".into(),
          color: Rgb {
            r: 0xe7,
            g: 0xb0,
            b: 0x5f,
          },
        },
      };
      self.animating_texts.push(text);
    }
  }

  fn render(&self, pen: &Pen<'_>) {
    let &Header {
      font,
      props,
      animating_texts,
      client,
    } = &self;
    let &HeaderProps {
      music_info,
      score_point,
      ..
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

    for AnimatedText {
      created_at,
      color,
      text,
    } in animating_texts
    {
      let time = created_at.elapsed().as_secs_f64();
      let norm_time = time / ANIMATION_DURATION;

      let y_offset = 1.0 - (ANIMATION_SPEED * norm_time).exp2();
      let opacity = (255.0 * (1.0 - norm_time)) as u8;

      font.set_font_size(28).unwrap();
      pen.text(
        font,
        text,
        FontRenderOptions::new()
          .mode(RenderMode::Blended {
            foreground: Rgba {
              r: color.r,
              g: color.g,
              b: color.g,
              a: opacity,
            },
          })
          .pivot(
            client
              .bottom_left()
              .offset(180, (-60.0 + y_offset) as i32),
          ),
      );
    }
  }
}
