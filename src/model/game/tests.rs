use super::{
  super::exp::{scoremap::Scoremap, sentence::Sentence},
  MusicalTypeResult, MusicalTyper, MusicalTyperConfig,
  MusicalTyperError, MusicalTyperEvent,
};

enum Input {
  Wait(f64),
  KeyPress(&'static str),
}

use Input::*;

#[test]
fn op1() -> Result<(), MusicalTyperError> {
  let test_score = Scoremap::from_str(
    r#"
# Sample 1
:title TEST
:score_author Mikuro さいな
:song_data void.ogg
:bpm 222.22

[start]
*2.22
打鍵テスト
:だけんてすと

*3.0
[end]
"#,
    |config| config.ignore_unsupported_property(true),
  )?;

  let inputs = &[Wait(2.22), KeyPress("dakentesuto"), Wait(1.0)];
  use MusicalTyperEvent::*;
  let expected_events = vec![
    PlayBgm("void.ogg".into()),
    UpdateSentence(Sentence::new_with_inputted(
      "打鍵テスト",
      "だけんてすと",
      "",
    )?),
    Typed(MusicalTypeResult::Correct),
    Typed(MusicalTypeResult::Correct),
    Typed(MusicalTypeResult::Correct),
    Typed(MusicalTypeResult::Correct),
    Typed(MusicalTypeResult::Correct),
    Typed(MusicalTypeResult::Correct),
    Typed(MusicalTypeResult::Correct),
    Typed(MusicalTypeResult::Correct),
    Typed(MusicalTypeResult::Correct),
    Typed(MusicalTypeResult::Correct),
    Typed(MusicalTypeResult::Correct),
    UpdateSentence(Sentence::new_with_inputted(
      "打鍵テスト",
      "だけんてすと",
      "dakentesuto",
    )?),
    DidPerfectSection,
    CompletedSentence(Sentence::new_with_inputted(
      "打鍵テスト",
      "だけんてすと",
      "",
    )?),
    UpdateSentence(Sentence::empty()),
  ]; // 560 points

  let mut game =
    MusicalTyper::new(test_score, MusicalTyperConfig::default())?;

  let actual_events: Vec<_> = inputs
    .into_iter()
    .flat_map(|input| match input {
      Wait(time) => game.set_time((*time).into()),
      KeyPress(key) => game.key_press(key.chars()),
    })
    .collect();

  for (i, (expected, actual)) in
    expected_events.iter().zip(actual_events.iter()).enumerate()
  {
    assert_eq!(expected, actual, "index: {}", i);
  }
  assert_eq!(expected_events.len(), actual_events.len());

  assert_eq!(game.activity().score().score_point, 560);

  Ok(())
}
