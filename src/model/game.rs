use super::exp::{
  game_activity::GameActivity,
  note::TypeResult,
  scoremap::{
    lexer::ScoremapLexError, MusicInfo, Scoremap, ScoremapError,
    ScoremapMetadata,
  },
  sentence::{roman::RomanParseError, Sentence},
  time::Seconds,
};
use std::io::Error;
use MusicalTyperError::*;
use MusicalTyperEvent::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq)]
pub enum MusicalTypeResult {
  Correct,
  Missed,
  Vacant,
}

impl From<TypeResult> for MusicalTypeResult {
  fn from(res: TypeResult) -> Self {
    match res {
      TypeResult::Succeed => MusicalTypeResult::Correct,
      TypeResult::Mistaken => MusicalTypeResult::Missed,
      TypeResult::Vacant => MusicalTypeResult::Vacant,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MusicalTyperEvent {
  PlayBgm(String),
  UpdateSentence(Sentence),
  MissedSentence(Sentence),
  CompletedSentence(Sentence),
  DidPerfectSection,
  Typed(MusicalTypeResult),
  EndOfScore,
}

#[derive(Debug)]
pub enum MusicalTyperError {
  SongDataNotFound,
  FileReadError { reason: String },
  ScoremapBuildError(ScoremapError),
}

impl From<Error> for MusicalTyperError {
  fn from(err: Error) -> Self {
    MusicalTyperError::FileReadError {
      reason: err.to_string(),
    }
  }
}

impl From<ScoremapError> for MusicalTyperError {
  fn from(err: ScoremapError) -> Self {
    MusicalTyperError::ScoremapBuildError(err)
  }
}
impl From<RomanParseError> for MusicalTyperError {
  fn from(_err: RomanParseError) -> Self {
    MusicalTyperError::ScoremapBuildError(ScoremapError::LexError(
      ScoremapLexError::InvalidStatementDefinition {
        line_num: 1,
        reason: "ふりがなでのそのような平仮名の並びは未対応です。",
      },
    ))
  }
}

pub type Point = u32;

pub struct MusicalTyperConfig {
  wrong_type: Point,
  correct_type: Point,
  missed_sentence: Point,
  complete_sentence: Point,
  perfect_sentence: Point,
  perfect_section: Point,
}

impl Default for MusicalTyperConfig {
  fn default() -> Self {
    MusicalTyperConfig {
      wrong_type: 30,
      correct_type: 10,
      missed_sentence: 2,
      complete_sentence: 50,
      perfect_sentence: 100,
      perfect_section: 300,
    }
  }
}

pub struct MusicalTyper {
  activity: GameActivity,
  metadata: ScoremapMetadata,
  current_time: Seconds,
  event_queue: Vec<MusicalTyperEvent>,
  config: MusicalTyperConfig,
}

impl MusicalTyper {
  pub fn new(
    score: Scoremap,
    config: MusicalTyperConfig,
  ) -> Result<Self, MusicalTyperError> {
    let mut event_queue = vec![];
    let activity = GameActivity::new(score.sections);

    let metadata = score.metadata;
    if let Some(song_data) = metadata.get("song_data") {
      event_queue.push(PlayBgm(song_data.into()));
    } else {
      return Err(SongDataNotFound);
    }

    Ok(MusicalTyper {
      activity,
      metadata,
      current_time: 0.0.into(),
      event_queue,
      config,
    })
  }

  pub fn activity(&self) -> &GameActivity {
    &self.activity
  }

  #[must_use]
  pub fn key_press(
    &mut self,
    typed: impl Iterator<Item = char>,
  ) -> Vec<MusicalTyperEvent> {
    let prev_sentence = self.activity.current_sentence();
    let prev_completed = prev_sentence.completed();
    for typed in typed {
      use super::exp::section::note::TypeResult::*;
      let result = self.activity.input(typed);
      let point = match result {
        Succeed => self.config.correct_type as i32,
        Mistaken => -(self.config.wrong_type as i32),
        _ => 0,
      };
      self.activity.point(point);
      self.event_queue.push(Typed(result.into()));
    }
    let curr_sentence = self.activity.current_sentence();
    let curr_completed = curr_sentence.completed();

    let mut events = vec![];
    if !prev_completed && curr_completed {
      if self
        .activity
        .current_section()
        .map_or(false, |section| 1.0 <= section.accuracy())
      {
        self.activity.point(self.config.perfect_section as i32);
        events.push(DidPerfectSection);
      }
      if self
        .activity
        .current_note()
        .map_or(false, |note| 1.0 <= note.accuracy())
      {
        self.activity.point(self.config.perfect_sentence as i32);
      }
      self.activity.point(self.config.complete_sentence as i32);
      events.push(CompletedSentence(prev_sentence));
    }

    [self.pack_events(), events].concat()
  }

  #[must_use]
  pub fn set_time(
    &mut self,
    new_time: Seconds,
  ) -> Vec<MusicalTyperEvent> {
    self.current_time = new_time;

    let completed = self.activity.current_sentence().completed();
    let prev_sentence = self.activity.current_sentence();
    let prev_note_id = self.activity.current_note_id();

    self.activity.update_time(self.current_time);

    if self.activity.is_game_over() {
      return vec![EndOfScore];
    }

    let curr_note_id = self.activity.current_note_id();

    let mut events = vec![];
    if !completed && (prev_note_id != curr_note_id) {
      self.activity.point(-(self.config.missed_sentence as i32));
      events.push(MissedSentence(prev_sentence));
    }

    [self.pack_events(), events].concat()
  }

  fn pack_events(&mut self) -> Vec<MusicalTyperEvent> {
    let sentence = self.activity.current_sentence();
    self.event_queue.push(UpdateSentence(sentence));

    let res = self.event_queue.to_vec();
    self.event_queue.clear();
    res
  }

  pub fn current_time(&self) -> Seconds {
    self.current_time
  }

  pub fn section_remaining_ratio(&self) -> f64 {
    self.activity.remaining_ratio(self.current_time)
  }

  pub fn music_info(&self) -> MusicInfo {
    self.metadata.get_music_info()
  }
}
