use std::rc::Rc;

use crate::{
  model::exp::{
    game_activity::GameScore, scoremap::MusicInfo, sentence::Sentence,
  },
  view::{
    components::{Header, HeaderProps, Stats, StatsProps},
    Component,
  },
};

mod finder;
mod keyboard;

use finder::{Finder, FinderProps};
use keyboard::{Keyboard, KeyboardProps};
use rich_sdl2_rust::{
  color::Rgb,
  geo::{Point, Rect, Size},
  renderer::pen::Pen,
};
use rich_sdl2_ttf_rust::font::Font;

#[derive(PartialEq)]
pub struct WholeProps {
  pub pressed_keys: Vec<char>,
  pub sentence: Sentence,
  pub music_info: MusicInfo,
  pub type_per_second: f64,
  pub score: GameScore,
  pub section_remaining_ratio: f64,
}

pub struct Whole<'font> {
  keyboard: Keyboard<'font>,
  finder: Finder<'font>,
  header: Header<'font>,
  stats: Stats<'font>,
  client: Rect,
}

impl<'font> Whole<'font> {
  pub fn new(
    props: WholeProps,
    font: Rc<Font<'font>>,
    client: Rect,
  ) -> Self {
    let hint = {
      let roman = props.sentence.roman();
      roman.will_input.chars().next().map_or(vec![], |c| vec![c])
    };
    let keyboard_dim = Rect {
      up_left: Point {
        x: 0,
        y: client.size.height as i32 - 350,
      },
      size: Size {
        width: client.size.width,
        height: 200,
      },
    };

    let keyboard = Keyboard::new(
      KeyboardProps {
        pressed_keys: props.pressed_keys.clone(),
        highlighted_keys: hint,
      },
      Rc::clone(&font),
      keyboard_dim,
    );

    let finder_dim = Rect {
      up_left: Point { x: 0, y: 100 },
      size: Size {
        width: client.size.width,
        height: 150,
      },
    };
    let finder = Finder::new(
      FinderProps {
        sentence: props.sentence.clone(),
        remaining_ratio: props.section_remaining_ratio,
      },
      Rc::clone(&font),
      finder_dim,
    );

    let header_dim = Rect {
      up_left: Point { x: 0, y: 0 },
      size: Size {
        width: client.size.width,
        height: 100,
      },
    };
    let header = Header::new(
      HeaderProps {
        music_info: props.music_info.clone(),
        score_point: props.score.score_point,
      },
      Rc::clone(&font),
      header_dim,
    );

    let stats_dim = Rect {
      up_left: Point {
        x: 0,
        y: client.size.height as i32 - 150,
      },
      size: Size {
        width: client.size.width,
        height: 150,
      },
    };
    let stats = Stats::new(
      StatsProps {
        type_per_second: props.type_per_second,
        score: props.score,
      },
      Rc::clone(&font),
      stats_dim,
    );

    Self {
      keyboard,
      finder,
      header,
      stats,
      client,
    }
  }
}

impl<'font> Component for Whole<'font> {
  type Props = WholeProps;

  fn is_needed_redraw(&self, _: &Self::Props) -> bool {
    true
  }

  fn update(&mut self, props: Self::Props) {
    let hint = {
      let roman = props.sentence.roman();
      roman.will_input.chars().next().map_or(vec![], |c| vec![c])
    };

    self.keyboard.update(KeyboardProps {
      pressed_keys: props.pressed_keys.clone(),
      highlighted_keys: hint,
    });

    self.finder.update(FinderProps {
      sentence: props.sentence.clone(),
      remaining_ratio: props.section_remaining_ratio,
    });

    self.stats.update(StatsProps {
      type_per_second: props.type_per_second,
      score: props.score,
    });
  }

  fn render(&self, pen: &Pen<'_>) {
    let &Whole { client, .. } = &self;

    pen.set_color(Rgb {
      r: 253,
      g: 243,
      b: 226,
    });
    pen.clear();

    {
      let header_dim = Rect {
        up_left: Point::default(),
        size: Size {
          width: client.size.width,
          height: 100,
        },
      };
      self.header.render(pen);
      pen.set_color(0.into());
      pen.stroke_rect(header_dim);
    }

    self.finder.render(pen);

    {
      let keyboard_dim = Rect {
        up_left: Point {
          x: 0,
          y: client.size.height as i32 - 350,
        },
        size: Size {
          width: client.size.width,
          height: 200,
        },
      };
      self.keyboard.render(pen);

      pen.set_color(0.into());
      pen.stroke_rect(keyboard_dim);
    }

    self.stats.render(pen);
  }
}
