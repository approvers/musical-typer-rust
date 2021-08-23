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
use rich_sdl2_rust::{color::Rgb, geo::Rect, renderer::pen::Pen};
use rich_sdl2_ttf_rust::font::Font;

#[derive(PartialEq)]
pub struct WholeProps<'font> {
  pub font: Rc<Font<'font>>,
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
  pub fn new(props: WholeProps<'font>, client: Rect) -> Self {
    let hint = {
      let roman = props.sentence.roman();
      roman.will_input.chars().next().map_or(vec![], |c| vec![c])
    };
    let keyboard_dim =
      Rect::new(0, client.height() as i32 - 350, client.width(), 200);

    let keyboard = Keyboard::new(
      KeyboardProps {
        font: Rc::clone(&props.font),
        pressed_keys: props.pressed_keys.clone(),
        highlighted_keys: hint,
      },
      keyboard_dim,
    );

    let finder_dim = Rect::new(0, 100, client.width(), 150);
    let finder = Finder::new(
      FinderProps {
        font: Rc::clone(&props.font),
        sentence: props.sentence.clone(),
        remaining_ratio: props.section_remaining_ratio,
      },
      finder_dim,
    );

    let header_dim = Rect::new(0, 0, client.width(), 100);
    let header = Header::new(
      HeaderProps {
        font: Rc::clone(&props.font),
        music_info: props.music_info.clone(),
        score_point: props.score.score_point,
      },
      header_dim,
    );

    let stats_dim =
      Rect::new(0, client.height() as i32 - 150, client.width(), 150);
    let stats = Stats::new(
      StatsProps {
        font: Rc::clone(&props.font),
        type_per_second: props.type_per_second,
        score: props.score,
      },
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
  type Props = WholeProps<'font>;

  fn is_needed_redraw(&self, _: &Self::Props) -> bool {
    true
  }

  fn update(&mut self, props: Self::Props) {
    let hint = {
      let roman = props.sentence.roman();
      roman.will_input.chars().next().map_or(vec![], |c| vec![c])
    };

    self.keyboard.update(KeyboardProps {
      font: Rc::clone(&props.font),
      pressed_keys: props.pressed_keys.clone(),
      highlighted_keys: hint,
    });

    self.finder.update(FinderProps {
      font: Rc::clone(&props.font),
      sentence: props.sentence.clone(),
      remaining_ratio: props.section_remaining_ratio,
    });

    self.stats.update(StatsProps {
      font: Rc::clone(&props.font),
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
      let header_dim = Rect::new(0, 0, client.width(), 100);
      self.header.render(pen);
      pen.set_color(0.into());
      pen.stroke_rect(header_dim);
    }

    self.finder.render(pen);

    {
      let keyboard_dim = Rect::new(
        0,
        client.height() as i32 - 350,
        client.width(),
        200,
      );
      self.keyboard.render(pen);

      pen.set_color(0.into());
      pen.stroke_rect(keyboard_dim);
    }

    self.stats.render(pen);
  }
}
