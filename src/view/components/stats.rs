use rich_sdl2_rust::{
  color::{Rgb, Rgba},
  geo::{Point, Rect, Size},
  renderer::pen::Pen,
};
use rich_sdl2_ttf_rust::font::{
  pen::{
    FontRenderExt, FontRenderOptions, TextAlign, TextAlignX,
    TextAlignY,
  },
  Font, RenderMode, StyleExt,
};
use std::rc::Rc;

use crate::{model::exp::game_activity::GameScore, view::Component};

mod rank;

#[derive(PartialEq)]
pub struct StatsProps {
  pub type_per_second: f64,
  pub score: GameScore,
}

pub struct Stats<'font> {
  props: StatsProps,
  font: Rc<Font<'font>>,
  client: Rect,
}

impl<'font> Stats<'font> {
  pub fn new(
    props: StatsProps,
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

impl<'font> Component for Stats<'font> {
  type Props = StatsProps;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    &self.props != new_props
  }

  fn update(&mut self, new_props: Self::Props) {
    self.props = new_props;
  }

  fn render(&self, pen: &Pen<'_>) {
    let Stats {
      props,
      font,
      client,
    } = &self;
    let StatsProps {
      type_per_second,
      score,
    } = &props;

    let accuracy = score.accuracy;
    let achievement_rate = score.achievement_rate;

    let speed_indicator_color = if 4.0 < *type_per_second {
      Rgb {
        r: 250,
        g: 119,
        b: 109,
      }
    } else {
      Rgb {
        r: 178,
        g: 255,
        b: 89,
      }
    };

    let rank = rank::rank(accuracy * 200.0);

    let speed_indicator_center = Point {
      x: client.size.width as i32 / 2,
      y: client.up_left.y + 15,
    };
    pen.set_color(speed_indicator_color);
    pen.fill_rect(Rect::from_center(
      speed_indicator_center,
      Size {
        width: client.size.width - 20,
        height: 20,
      },
    ));

    font.set_font_size(20);
    pen.text(
      font,
      &format!("{:04.2} Type/s", type_per_second),
      FontRenderOptions::new()
        .align(TextAlign {
          x: TextAlignX::Center,
          y: TextAlignY::Center,
        })
        .pivot(speed_indicator_center),
    );

    font.set_font_size(30);
    pen.text(
      font,
      "正解率",
      FontRenderOptions::new()
        .mode(RenderMode::Blended {
          foreground: Rgba {
            r: 160,
            g: 160,
            b: 165,
            a: 255,
          },
        })
        .pivot(client.up_left.offset(10, 30)),
    );
    font.set_font_size(client.size.height - 20);
    pen.text(
      font,
      &format!("{:05.1}%", accuracy * 100.0),
      FontRenderOptions::new()
        .mode(RenderMode::Blended {
          foreground: Rgba {
            r: (250.0 * accuracy) as u8,
            g: (120.0 * accuracy) as u8,
            b: (110.0 * accuracy) as u8,
            a: 255,
          },
        })
        .pivot(client.up_left.offset(10, 30)),
    );

    pen.set_color(Rgb {
      r: 250,
      g: 120,
      b: 110,
    });
    pen.stroke_rect(Rect {
      up_left: Point {
        x: client.left() + 10,
        y: client.bottom() - 10,
      },
      size: Size {
        width: (client.size.width as f64 * 0.5 * accuracy) as u32,
        height: 2,
      },
    });

    font.set_font_size(30);
    pen.text(
      font,
      "達成率",
      FontRenderOptions::new()
        .mode(RenderMode::Blended {
          foreground: Rgba {
            r: 160,
            g: 160,
            b: 165,
            a: 255,
          },
        })
        .pivot(Point {
          x: client.center().x + client.left() + 10,
          y: client.up_left.y + 30,
        }),
    );
    font.set_font_size(client.size.height - 20);
    pen.text(
      font,
      &format!("{:05.1}%", achievement_rate * 100.0),
      FontRenderOptions::new()
        .mode(RenderMode::Blended {
          foreground: Rgba {
            r: 64,
            g: 79,
            b: 181,
            a: 255,
          },
        })
        .pivot(Point {
          x: client.center().x + client.left() + 10,
          y: client.up_left.y + 30,
        }),
    );

    font.set_font_size(25);
    pen.text(
      font,
      "ランク",
      FontRenderOptions::new()
        .mode(RenderMode::Blended {
          foreground: Rgba {
            r: 160,
            g: 160,
            b: 165,
            a: 255,
          },
        })
        .pivot(client.up_left.offset(10, -60)),
    );
    font.set_font_size(35);
    pen.text(
      font,
      rank.0,
      FontRenderOptions::new()
        .mode(RenderMode::Blended {
          foreground: Rgba {
            r: 64,
            g: 79,
            b: 181,
            a: 255,
          },
        })
        .pivot(client.up_left.offset(10, -30)),
    );
  }
}
