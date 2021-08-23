use rich_sdl2_rust::{
  color::{Rgb, Rgba},
  delay,
  geo::{Point, Rect, Size},
  renderer::{pen::Pen, Renderer},
  EventBox, Video,
};
use rich_sdl2_ttf_rust::font::{
  pen::{
    FontRenderExt, FontRenderOptions, TextAlign, TextAlignX,
    TextAlignY,
  },
  Font, RenderMode, StyleExt,
};
use std::{
  cell::{Cell, RefCell},
  rc::Rc,
  time::Instant,
};

use super::{
  components::{
    Button, ButtonProps, Header, HeaderProps, Stats, StatsProps,
  },
  View, ViewRoute,
};
use crate::{
  model::exp::{game_activity::GameScore, scoremap::MusicInfo},
  view::Component,
};

pub struct ResultView<'view> {
  renderer: &'view Renderer<'view>,
  score: GameScore,
  music_info: MusicInfo,
  font: Rc<Font<'view>>,
  video: &'view Video<'view>,
}

impl<'view> ResultView<'view> {
  pub fn new(
    renderer: &'view Renderer<'view>,
    score: GameScore,
    music_info: MusicInfo,
    font: Rc<Font<'view>>,
    video: &'view Video<'view>,
  ) -> Self {
    Self {
      renderer,
      score,
      music_info,
      font,
      video,
    }
  }
}

impl<'view> View for ResultView<'view> {
  fn run(&mut self) -> Result<ViewRoute, super::ViewError> {
    let client = Rect {
      up_left: Default::default(),
      size: self.renderer.output_size(),
    };

    enum Dst {
      Game,
      Quit,
    }
    let will_navigate_to = Rc::new(RefCell::new(None));

    let stats_dim = Rect {
      up_left: Point {
        x: 0,
        y: client.size.height as i32 - 300,
      },
      size: Size {
        width: client.size.width,
        height: 200,
      },
    };
    let mut stats = Stats::new(
      StatsProps {
        type_per_second: 0.0,
        score: self.score.clone(),
      },
      Rc::clone(&self.font),
      stats_dim,
    );

    let header_dim = Rect {
      up_left: Point { x: 20, y: 50 },
      size: Size {
        width: client.size.width.saturating_sub(40),
        height: 100,
      },
    };
    let mut header = Header::new(
      HeaderProps {
        music_info: self.music_info.clone(),
        score_point: self.score.score_point,
      },
      Rc::clone(&self.font),
      header_dim,
    );

    const WIDTH: u32 = 240;
    const HEIGHT: u32 = 80;
    const MARGIN: u32 = 20;
    let retry_button_area = Rect {
      up_left: Point {
        x: client.size.width as i32 - WIDTH as i32 - MARGIN as i32,
        y: client.size.height as i32 - HEIGHT as i32 - MARGIN as i32,
      },
      size: Size {
        width: WIDTH,
        height: HEIGHT,
      },
    };
    let mut retry_button = Button::new(
      ButtonProps {
        border_color: 0x0a0d0a.into(),
        color_on_hover: 0xdce0dc.into(),
        mouse: None,
      },
      retry_button_area,
      || {
        will_navigate_to.borrow_mut().replace(Dst::Game);
      },
    );

    let should_quit = Cell::new(false);

    let mut event = EventBox::new(&self.video);

    event.handle_quit(Box::new(|_| should_quit.set(true)));
    event.handle_keyboard(Box::new(|_| should_quit.set(true)));

    loop {
      if should_quit.get() {
        will_navigate_to.borrow_mut().replace(Dst::Quit);
      }
      let time = Instant::now();

      {
        let pen = Pen::new(&self.renderer);
        pen.set_color(Rgb {
          r: 253,
          g: 243,
          b: 226,
        });
        pen.clear();

        header.update(HeaderProps {
          music_info: self.music_info.clone(),
          score_point: self.score.score_point,
        });
        header.render(&pen);

        stats.update(StatsProps {
          type_per_second: 0.0,
          score: self.score.clone(),
        });
        stats.render(&pen);

        {
          let new_props = ButtonProps {
            border_color: Rgb {
              r: 10,
              g: 14,
              b: 10,
            },
            color_on_hover: Rgb {
              r: 220,
              g: 224,
              b: 220,
            },
            mouse: None,
          };
          if retry_button.is_needed_redraw(&new_props) {
            retry_button.update(new_props);
          }
          retry_button.render(&pen);

          self.font.set_font_size(60);
          pen.text(
            &self.font,
            "再挑戦",
            FontRenderOptions::new()
              .mode(RenderMode::Blended {
                foreground: Rgba {
                  r: 36,
                  g: 141,
                  b: 255,
                  a: 255,
                },
              })
              .align(TextAlign {
                x: TextAlignX::Center,
                y: TextAlignY::Center,
              })
              .pivot(retry_button_area.center()),
          );
        }
      }

      let draw_time = time.elapsed().as_secs_f64();
      delay((1e3 / 60.0 - draw_time * 1e3).max(0.0) as u32);

      let will_navigate_to = will_navigate_to.borrow();
      if let Some(will_navigate_to) = will_navigate_to.as_ref() {
        match will_navigate_to {
          Dst::Game => return Ok(ViewRoute::Retry),
          Dst::Quit => return Ok(ViewRoute::Quit),
        }
      }
    }
  }
}
