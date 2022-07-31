use rich_sdl2_rust::mixer::device::MixDevice;
use rich_sdl2_rust::ttf::font::Font;
use rich_sdl2_rust::{
  delay,
  event::keyboard::key_code::KeyCode,
  geo::Rect,
  renderer::{pen::Pen, Renderer},
  EventBox, Video,
};
use std::{
  cell::{Cell, RefCell},
  collections::{BTreeSet, VecDeque},
  rc::Rc,
  time::Instant,
};
use whole::{Whole, WholeProps};

use super::{
  player::{Player, SEKind},
  View, ViewError, ViewRoute,
};
use crate::{
  model::{
    exp::{scoremap::Scoremap, sentence::Sentence, time::Seconds},
    game::{
      MusicalTypeResult, MusicalTyper, MusicalTyperConfig,
      MusicalTyperEvent,
    },
  },
  view::Component,
};

mod whole;

pub struct GameView<'view> {
  renderer: &'view Renderer<'view>,
  model: MusicalTyper,
  font: Rc<Font<'view>>,
  device: &'view MixDevice<'view>,
  video: &'view Video<'view>,
}

impl<'view> GameView<'view> {
  pub fn new(
    renderer: &'view Renderer<'view>,
    score: Scoremap,
    font: Rc<Font<'view>>,
    device: &'view MixDevice<'view>,
    video: &'view Video<'view>,
  ) -> Result<Self, ViewError> {
    Ok(GameView {
      renderer,
      model: MusicalTyper::new(score, MusicalTyperConfig::default())?,
      font,
      device,
      video,
    })
  }
}

impl<'canvas> View for GameView<'canvas> {
  fn run(&mut self) -> Result<ViewRoute, ViewError> {
    struct TypeTimePoint(Seconds);

    let mut mt_events = vec![];
    let mut player = Player::new(self.device);
    let mut sentence = Sentence::empty();
    let mut time_points = VecDeque::new();
    let mut ended = None;

    let pressed_key_buf = Rc::new(RefCell::new(BTreeSet::new()));
    let typed_key_buf = Rc::new(RefCell::new(vec![]));
    let should_quit = Cell::new(false);

    let client = Rect {
      up_left: Default::default(),
      size: self.renderer.output_size().unwrap(),
    };
    let mut whole_view = Whole::new(
      WholeProps {
        pressed_keys: pressed_key_buf
          .borrow()
          .iter()
          .cloned()
          .collect(),
        sentence: sentence.clone(),
        music_info: self.model.music_info(),
        type_per_second: 0.0,
        score: self.model.activity().score().clone(),
        section_remaining_ratio: self.model.section_remaining_ratio(),
      },
      Rc::clone(&self.font),
      client,
    );

    let mut event = EventBox::new(self.video);

    event.handle_quit(Box::new(|_| {
      should_quit.set(true);
    }));

    event.handle_keyboard(Box::new(|e| {
      if e.is_repeated {
        return;
      }
      let key_code = e.symbol.key_code;
      if e.is_pressed {
        let key = keycode_to_char(key_code);
        if pressed_key_buf.borrow_mut().insert(key) {
          typed_key_buf.borrow_mut().push(key);
        }
      } else {
        pressed_key_buf
          .borrow_mut()
          .remove(&keycode_to_char(key_code));
      }
    }));

    let game_start_time = Instant::now();

    loop {
      if should_quit.get() {
        player.stop_bgm(50)?;
        return Ok(ViewRoute::Quit);
      }
      let render_start_time = Instant::now();
      {
        for mt_event in mt_events.iter() {
          use MusicalTyperEvent::*;
          match mt_event {
            PlayBgm(bgm_name) => {
              player.change_bgm(bgm_name)?;
            }
            UpdateSentence(new_sentence) => {
              sentence = new_sentence.clone();
            }
            Typed(result) => match result {
              MusicalTypeResult::Missed => {
                player.play_se(SEKind::Fail)?;
              }
              MusicalTypeResult::Correct => {
                time_points.push_back(TypeTimePoint(
                  self.model.current_time(),
                ));
                player.play_se(SEKind::Correct)?;
              }
              MusicalTypeResult::Vacant => {
                player.play_se(SEKind::Vacant)?;
              }
            },
            MissedSentence(_sentence) => {
              player.play_se(SEKind::MissedSentence)?;
              // TODO: Queue a missed animation
            }
            CompletedSentence(_sentence) => {
              player.play_se(SEKind::PerfectSentence)?;
              // TODO: Queue a completed animation
            }
            DidPerfectSection => {
              player.play_se(SEKind::PerfectSection)?;
              // TODO: Queue a perfect animation
            }
            EndOfScore => {
              if ended.is_none() {
                ended = Some(self.model.current_time() + 2.0.into());
              }
            }
          }
        }
      }
      event.poll();
      {
        let expire_limit = self.model.current_time() - 5.0.into();
        while let Some(front) = time_points.front() {
          if front.0 < expire_limit {
            time_points.pop_front();
          } else {
            break;
          }
        }
      }

      let type_per_second = time_points.len() as f64 / 5.0;
      {
        let pen = Pen::new(self.renderer);
        whole_view.update(WholeProps {
          pressed_keys: pressed_key_buf
            .borrow()
            .iter()
            .cloned()
            .collect(),
          sentence: sentence.clone(),
          music_info: self.model.music_info(),
          type_per_second,
          score: self.model.activity().score().clone(),
          section_remaining_ratio: self
            .model
            .section_remaining_ratio(),
        });
        whole_view.render(&pen);
      }

      let typed_key_buf_cloned = typed_key_buf.borrow().clone();
      typed_key_buf.borrow_mut().clear();
      mt_events =
        self.model.key_press(typed_key_buf_cloned.into_iter());

      let draw_time = render_start_time.elapsed().as_secs_f64();

      delay((1e3 / 60.0 - draw_time * 1e3).max(0.0) as u32);

      let new_time = game_start_time.elapsed().as_secs_f64();

      mt_events.append(&mut self.model.set_time(new_time.into()));
      print!("\rFPS: {}     ", 1.0 / draw_time);

      if ended
        .as_ref()
        .map_or(false, |ended| ended < &self.model.current_time())
      {
        return Ok(ViewRoute::ResultView(
          self.model.activity().score().clone(),
          self.model.music_info(),
        ));
      }
    }
  }
}

fn keycode_to_char(keycode: KeyCode) -> char {
  use KeyCode::*;
  match keycode {
    A => 'a',
    B => 'b',
    C => 'c',
    D => 'd',
    E => 'e',
    F => 'f',
    G => 'g',
    H => 'h',
    I => 'i',
    J => 'j',
    K => 'k',
    L => 'l',
    M => 'm',
    N => 'n',
    O => 'o',
    P => 'p',
    Q => 'q',
    R => 'r',
    S => 's',
    T => 't',
    U => 'u',
    V => 'v',
    W => 'w',
    X => 'x',
    Y => 'y',
    Z => 'z',
    Minus => '-',
    _ => '\0',
  }
}
