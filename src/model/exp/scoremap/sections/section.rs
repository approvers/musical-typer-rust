use crate::model::exp::time::{Duration, Seconds};
use note::{Note, NoteId, TypeResult};

pub mod note;

#[derive(Debug, Clone)]
pub struct Section {
  notes: Vec<Note>,
  current_note_index: usize,
  duration: Duration,
}

impl Section {
  pub fn new(notes: Vec<Note>, duration: Duration) -> Self {
    Self {
      notes,
      current_note_index: 0,
      duration,
    }
  }

  pub fn current_note(&self) -> &Note {
    &self.notes[self.current_note_index]
  }

  pub fn id(&self) -> NoteId {
    self.current_note().id()
  }

  pub fn input(&mut self, typed: char) -> TypeResult {
    self.notes[self.current_note_index].input(typed)
  }

  pub fn accuracy(&self) -> f64 {
    let mut accuracies: Vec<_> =
      self.notes.iter().map(|note| note.accuracy()).collect();
    accuracies.sort_by(|ref a, ref b| a.partial_cmp(b).unwrap());

    if accuracies.len() % 2 == 0 {
      accuracies[accuracies.len() / 2]
    } else {
      (accuracies[accuracies.len() / 2 - 1]
        + accuracies[accuracies.len() / 2])
        / 2.0
    }
  }

  pub fn update(&mut self, time: &Seconds) -> bool {
    for (index, note) in self.notes.iter().enumerate() {
      if note.duration().includes(time) {
        self.current_note_index = index;
        return true;
      }
    }
    false
  }

  pub fn iter(&self) -> impl Iterator<Item = &Note> {
    self.notes.iter()
  }

  #[allow(dead_code)]
  pub fn len(&self) -> usize {
    self.notes.len()
  }
}
