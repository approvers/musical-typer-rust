use crate::exp::game_stat::GameActivity;
use crate::exp::note::{Note, Section};
use crate::exp::scoremap::{Scoremap, ScoremapMetadata};
use crate::exp::string_to_input::StringToInput;

pub trait Controller {
  fn key_press(&mut self) -> char;
  fn elapse_time(&mut self) -> f64;
}
pub trait Presenter {
  fn play_bgm(&mut self, name: &str);
  fn decrease_remaining_time(&mut self, delta_time: f64);
  fn update_string_to_input(&mut self, string: &StringToInput);
  fn mistyped(&mut self);
  fn flush_screen(&mut self);
}

pub struct MusicalTyper {
  score: Scoremap,
  activity: GameActivity,
}

impl MusicalTyper {
  pub fn new(score: Scoremap) -> Self {
    let notes = score.notes();
    let shifted_notes = notes.iter().cloned().skip(1);
    let sections = notes
      .iter()
      .zip(shifted_notes)
      .map(|(prev, note): (&Note, Note)| {
        Section::new(prev.id(), note.id())
      })
      .collect();
    MusicalTyper {
      score,
      activity: GameActivity::new(sections),
    }
  }

  pub fn run_game(
    &mut self,
    controller: &mut impl Controller,
    presenter: &mut impl Presenter,
  ) -> Result<(), String> {
    let metadata = self.score.metadata();
    if let Some(ref bgm) = metadata.get("bgm") {
      presenter.play_bgm(bgm);
    }
    while let Some(section) = self.activity.current_section() {
      let delta_time = controller.elapse_time();
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::Controller;
  use super::Presenter;
  use crate::exp::string_to_input::StringToInput;

  struct KeyPress(f64, &'static str);

  struct MockController {
    key_press_index: usize,
    key_char_index: usize,
    called_key_press: bool,
    called_elapse_time: bool,
    key_press_schedule: &'static [KeyPress],
  }

  impl MockController {
    fn new(key_press_schedule: &'static [KeyPress]) -> Self {
      MockController {
        key_press_index: 0,
        key_char_index: 0,
        called_key_press: false,
        called_elapse_time: false,
        key_press_schedule,
      }
    }
  }

  impl Controller for MockController {
    fn key_press(&mut self) -> char {
      self.called_key_press = true;
      let res = self.key_press_schedule[self.key_press_index].1;
      if self.called_key_press && self.called_elapse_time {
        self.key_press_index += 1;
        self.called_key_press = false;
        self.called_elapse_time = false;
      }
      res.chars().nth(self.key_char_index).unwrap_or_default()
    }
    fn elapse_time(&mut self) -> f64 {
      self.called_elapse_time = true;
      let res = self.key_press_schedule[self.key_press_index].0;
      if self.called_key_press && self.called_elapse_time {
        self.key_press_index += 1;
        self.called_key_press = false;
        self.called_elapse_time = false;
      }
      res
    }
  }

  #[derive(Debug, PartialEq)]
  enum PresentLog {
    PlayBGM(String),
    DecreateRemainingTime(f64),
    UpdateStringToInput(StringToInput),
    Mistyped,
  }

  use PresentLog::*;

  struct MockPresenter {
    log: Vec<PresentLog>,
  }

  impl MockPresenter {
    fn new() -> Self {
      MockPresenter { log: vec![] }
    }

    fn log(&self) -> &[PresentLog] {
      &self.log
    }
  }

  impl Presenter for MockPresenter {
    fn play_bgm(&mut self, name: &str) {
      self.log.push(PlayBGM(name.to_owned()));
    }
    fn decrease_remaining_time(&mut self, delta_time: f64) {
      self.log.push(DecreateRemainingTime(delta_time));
    }
    fn update_string_to_input(&mut self, string: &StringToInput) {
      self.log.push(UpdateStringToInput(string.clone()));
    }
    fn mistyped(&mut self) {
      self.log.push(Mistyped)
    }
    fn flush_screen(&mut self) {}
  }

  #[test]
  fn op1() {
    use crate::exp::scoremap::{Scoremap, ScoremapLoadConfig};
    use crate::op::on_game::MusicalTyper;

    let test_score = Scoremap::from_file(
      std::fs::File::open(std::path::Path::new(
        "example/sampleScore.tsc",
      ))
      .unwrap(),
      ScoremapLoadConfig::new().ignore_invalid_properties(true),
    )
    .unwrap();

    let mut game = MusicalTyper::new(test_score);

    let mut controller = MockController::new(&[
      KeyPress(3.0, "moudamedasonnnatokiha"),
      KeyPress(6.5, "anosorawomiagetegorann"),
      KeyPress(11.0, "yorunoyamiwoosiagete"),
      KeyPress(14.5, "taiyougamatahohoemikureru"),
      KeyPress(19.0, "maedakemitetemortukarerune"),
      KeyPress(22.5, "tamanihatatidomatteiinndayo"),
      KeyPress(27.0, "muneippaikuukisuttara"),
      KeyPress(30.75, "mataashiwohumidasouyo"),
      KeyPress(35.0, "bokuranoyumesorawokoete"),
      KeyPress(39.0, "hateshinakuhirogatteikuyo"),
      KeyPress(43.0, "namidanoatomomunenoitamimo"),
      KeyPress(47.0, "kiminochikaraninaru"),
      KeyPress(51.0, "maltukuradanagedasumaeni"),
      KeyPress(54.0, "anosorawomiagetegorann"),
      KeyPress(59.0, "yorunoyamimewokoraseba"),
      KeyPress(62.5, "hoshitachinodannsupa-texi-"),
      KeyPress(67.0, "tuyogaribakarijatukarerune"),
      KeyPress(71.0, "namidawokoboshitemoiinndayo"),
      KeyPress(75.0, "omoikirinaitaatoniha"),
      KeyPress(78.75, "mataegaowomisetene"),
      KeyPress(83.0, "minnnanoyumetokiwokoete"),
      KeyPress(87.0, "dokomademotunagaxtuteikuyo"),
      KeyPress(91.0, "namidanoatomomunenoitamimo"),
      KeyPress(95.0, "kiminochikaraninaru"),
      KeyPress(99.0, "sukoshidutumaenisusumou"),
      KeyPress(103.0, "miraihazuttomatteirukara"),
      KeyPress(107.0, "omoikirinaitayorusase"),
      KeyPress(110.5, "itukaomoidenikawaruyo"),
      KeyPress(117.0, "bokuranoyumesorawokoete"),
      KeyPress(121.0, "hateshinakuhirogatteikuyo"),
      KeyPress(125.0, "namidanoatomomunenoitamimo"),
      KeyPress(129.0, "kiminochikaraninaru"),
      KeyPress(133.0, "minnnanoyumetokiwokoete"),
      KeyPress(137.0, "dokomademotunagatteikuyo"),
      KeyPress(141.0, "namidanoatomomunenoitamimo"),
      KeyPress(145.0, "kiminochikaraninaru"),
      KeyPress(149.0, "namidanoatomomunenoitamimo"),
      KeyPress(153.0, "kiminochikaraninaru"),
    ]);
    let mut presenter = MockPresenter::new();

    game.run_game(&mut controller, &mut presenter).unwrap();

    assert_eq!(
      presenter.log(),
      &[
        PlayBGM("kkiminochikara-edited.wav".to_owned()),
        DecreateRemainingTime(3.0),
        UpdateStringToInput(
          StringToInput::new(
            "もうダメだ そんな時は",
            "もうだめだそんなときは"
          )
          .unwrap()
        )
      ]
    );
  }
}
