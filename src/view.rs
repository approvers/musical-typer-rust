use std::rc::Rc;

use crate::model::exp::{
  game_activity::GameScore,
  scoremap::{MusicInfo, Scoremap},
};
use crate::model::game::MusicalTyperError;
use game_view::GameView;
use player::PlayerError;
use result_view::ResultView;
use rich_sdl2_mixer_rust::{
  device::{MixDevice, MixDeviceBuilder},
  FormatFlag, Mix,
};
use rich_sdl2_rust::{
  renderer::{pen::Pen, Renderer},
  window::{WindowBuilder, WindowContextKind},
  Sdl, Video,
};
use rich_sdl2_ttf_rust::{font::Font, Ttf};

mod components;
mod game_view;
mod player;
mod result_view;

pub trait Component {
  type Props;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool;

  fn update(&mut self, new_props: Self::Props);

  fn render(&self, ctx: &Pen<'_>);
}

#[derive(Debug)]
pub enum ViewError {
  Model(MusicalTyperError),
  Font(String),
  Player(PlayerError),
  Render(String),
  Cache,
}

impl From<MusicalTyperError> for ViewError {
  fn from(err: MusicalTyperError) -> Self {
    ViewError::Model(err)
  }
}

pub trait View {
  fn run(&mut self) -> Result<ViewRoute, ViewError>;
}

#[allow(dead_code)]
pub enum ViewRoute {
  SelectMusic,
  Start(Scoremap),
  Retry,
  ResultView(GameScore, MusicInfo),
  Quit,
}

impl From<PlayerError> for ViewError {
  fn from(err: PlayerError) -> Self {
    ViewError::Player(err)
  }
}

struct Router<'router> {
  renderer: Renderer<'router>,
  video: &'router Video<'router>,
  font: Rc<Font<'router>>,
  mix_device: MixDevice<'router>,
}

impl<'router> Router<'router> {
  pub fn new(
    renderer: Renderer<'router>,
    video: &'router Video<'router>,
    font: Font<'router>,
    mix_device: MixDevice<'router>,
  ) -> Self {
    Self {
      renderer,
      video,
      font: Rc::new(font),
      mix_device,
    }
  }

  pub fn run(self, score: Scoremap) -> Result<(), ViewError> {
    let mut view: Option<Box<dyn View>> =
      Some(Box::new(ResultView::new(
        &self.renderer,
        GameScore::new(0, 0.0, 0.0),
        score.metadata.get_music_info(),
        Rc::clone(&self.font),
        self.video,
      )));
    while let Some(boxed_view) = view.as_mut() {
      let next = boxed_view.run()?;
      match next {
        ViewRoute::SelectMusic => {}
        ViewRoute::Start(_) => {}
        ViewRoute::Retry => {
          view.replace(Box::new(GameView::new(
            &self.renderer,
            score.clone(),
            Rc::clone(&self.font),
            &self.mix_device,
            self.video,
          )?));
        }
        ViewRoute::ResultView(score, info) => {
          view = None;
          view.replace(Box::new(ResultView::new(
            &self.renderer,
            score,
            info,
            Rc::clone(&self.font),
            self.video,
          )));
        }
        ViewRoute::Quit => {
          view = None;
        }
      };
    }

    Ok(())
  }
}

pub fn run_router(score: Scoremap) -> Result<(), ViewError> {
  let sdl = Sdl::new();
  let ttf = Ttf::new();
  let mix = Mix::new(FormatFlag::MP3).expect("mp3 loader not found");
  let mut builder = MixDeviceBuilder::new();
  builder.frequency(44100).chunk_size(1024);
  let dev =
    builder.build(&mix).expect("Fail to open an audio channel");

  let font =
    Font::new(&ttf, "./asset/mplus-1m-medium.ttf", 128, None)
      .expect("Font file is not found");

  let video = Video::new(&sdl);
  let window = WindowBuilder::builder()
    .title("Musical Typer")
    .width(800)
    .height(600)
    .context_kind(WindowContextKind::OpenGl)
    .allow_high_dpi(true)
    .build()
    .new_window(&video);
  window.show();

  let renderer = Renderer::new(&window);

  Router::new(renderer, &video, font, dev).run(score)?;
  Ok(())
}
