use super::{parse, RomanChar, RomanParseError};
use std::fmt::{Debug, Formatter};

#[derive(Clone, PartialEq)]
pub struct RomanStr {
  yomigana: String,
  chars: Vec<RomanChar>,
  inputting_char: usize,
  inputted: String,
}

impl Debug for RomanStr {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{:?}",
      self
        .chars
        .iter()
        .map(|c: &RomanChar| format!("{:?}", c))
        .collect::<Vec<String>>()
        .join(" ")
    )
  }
}

impl RomanStr {
  pub fn new(yomigana: &str) -> Result<Self, RomanParseError> {
    let chars: Vec<char> = yomigana.chars().collect();
    let mut parsed: Vec<RomanChar> = vec![];
    parse(&mut parsed, chars.as_slice())?;
    Ok(RomanStr {
      yomigana: yomigana.into(),
      chars: parsed,
      inputting_char: 0,
      inputted: String::new(),
    })
  }

  fn inputted_yomigana_index(&self) -> usize {
    self
      .chars
      .iter()
      .take_while(|c| c.completed_input())
      .count()
  }

  pub fn will_input_yomigana(
    &self,
  ) -> impl Iterator<Item = char> + '_ {
    self.yomigana.chars().skip(self.inputted_yomigana_index())
  }

  pub fn inputted_yomigana(&self) -> impl Iterator<Item = char> + '_ {
    self.yomigana.chars().take(self.inputted_yomigana_index())
  }

  pub fn will_input_roman(&self) -> String {
    self
      .chars
      .iter()
      .map(|roman_char: &RomanChar| {
        roman_char.determined_style().into()
      })
      .collect::<Vec<String>>()
      .join("")[self.inputted_roman().len()..]
      .into()
  }

  pub fn inputted_roman(&self) -> &str {
    &self.inputted
  }

  pub fn input(&mut self, typed: char) -> bool {
    if self.chars[self.inputting_char].input(typed) {
      self.inputted.push(typed);
      if self.chars[self.inputting_char].completed_input() {
        self.inputting_char += 1;
      }
      if !self.completed() {
        self.chars[self.inputting_char].fix_style(typed);
      }
      true
    } else {
      false
    }
  }

  pub fn completed(&self) -> bool {
    self.chars.len() <= self.inputting_char
  }
}

#[test]
fn hello() -> Result<(), RomanParseError> {
  let mut hello = RomanStr::new("こんにちは")?;
  assert_eq!(hello.inputted_roman(), "");
  assert_eq!(hello.will_input_roman(), "konnnitiha");
  assert!(hello.input('k'));
  assert_eq!(hello.inputted_roman(), "k");
  assert_eq!(hello.will_input_roman(), "onnnitiha");
  assert!(hello.input('o'));
  assert_eq!(hello.inputted_roman(), "ko");
  assert_eq!(hello.will_input_roman(), "nnnitiha");
  assert!(hello.input('n'));
  assert_eq!(hello.inputted_roman(), "kon");
  assert_eq!(hello.will_input_roman(), "nnitiha");
  assert!(hello.input('n'));
  assert_eq!(hello.inputted_roman(), "konn");
  assert_eq!(hello.will_input_roman(), "nitiha");
  assert!(hello.input('n'));
  assert_eq!(hello.inputted_roman(), "konnn");
  assert_eq!(hello.will_input_roman(), "itiha");
  assert!(hello.input('i'));
  assert_eq!(hello.inputted_roman(), "konnni");
  assert_eq!(hello.will_input_roman(), "tiha");
  assert!(hello.input('t'));
  assert_eq!(hello.inputted_roman(), "konnnit");
  assert_eq!(hello.will_input_roman(), "iha");
  assert!(hello.input('i'));
  assert_eq!(hello.inputted_roman(), "konnniti");
  assert_eq!(hello.will_input_roman(), "ha");
  assert!(hello.input('h'));
  assert_eq!(hello.inputted_roman(), "konnnitih");
  assert_eq!(hello.will_input_roman(), "a");
  assert!(hello.input('a'));
  assert_eq!(hello.inputted_roman(), "konnnitiha");
  assert_eq!(hello.will_input_roman(), "");
  Ok(())
}

#[test]
fn toy() -> Result<(), RomanParseError> {
  let mut hello = RomanStr::new("おもちゃ")?;
  assert_eq!(hello.inputted_roman(), "");
  assert_eq!(hello.will_input_roman(), "omotya");
  assert!(hello.input('o'));
  assert_eq!(hello.inputted_roman(), "o");
  assert_eq!(hello.will_input_roman(), "motya");
  assert!(hello.input('m'));
  assert_eq!(hello.inputted_roman(), "om");
  assert_eq!(hello.will_input_roman(), "otya");
  assert!(hello.input('o'));
  assert_eq!(hello.inputted_roman(), "omo");
  assert_eq!(hello.will_input_roman(), "tya");
  assert!(hello.input('c'));
  assert_eq!(hello.inputted_roman(), "omoc");
  assert_eq!(hello.will_input_roman(), "ha");
  assert!(hello.input('y'));
  assert_eq!(hello.inputted_roman(), "omocy");
  assert_eq!(hello.will_input_roman(), "a");
  assert!(hello.input('a'));
  assert_eq!(hello.inputted_roman(), "omocya");
  assert_eq!(hello.will_input_roman(), "");
  Ok(())
}

#[test]
fn small() -> Result<(), RomanParseError> {
  let mut hello = RomanStr::new("ちっちゃい")?;
  assert_eq!(hello.inputted_roman(), "");
  assert_eq!(hello.will_input_roman(), "tittyai");
  assert!(hello.input('t'));
  assert_eq!(hello.inputted_roman(), "t");
  assert_eq!(hello.will_input_roman(), "ittyai");
  assert!(hello.input('i'));
  assert_eq!(hello.inputted_roman(), "ti");
  assert_eq!(hello.will_input_roman(), "ttyai");
  assert!(hello.input('t'));
  assert_eq!(hello.inputted_roman(), "tit");
  assert_eq!(hello.will_input_roman(), "tyai");
  assert!(!hello.input('c'));
  assert!(hello.input('t'));
  assert_eq!(hello.inputted_roman(), "titt");
  assert_eq!(hello.will_input_roman(), "yai");
  assert!(hello.input('y'));
  assert_eq!(hello.inputted_roman(), "titty");
  assert_eq!(hello.will_input_roman(), "ai");
  assert!(hello.input('a'));
  assert_eq!(hello.inputted_roman(), "tittya");
  assert_eq!(hello.will_input_roman(), "i");
  assert!(hello.input('i'));
  assert_eq!(hello.inputted_roman(), "tittyai");
  assert_eq!(hello.will_input_roman(), "");
  Ok(())
}
