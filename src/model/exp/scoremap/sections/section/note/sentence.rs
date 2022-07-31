use roman::{RomanParseError, RomanStr};
use std::fmt::{Debug, Formatter};

pub mod roman;

pub struct TypingStr {
  pub will_input: String,
  pub inputted: String,
}

#[derive(Clone, PartialEq)]
pub struct Sentence {
  origin: String,
  hiragana: RomanStr,
}

impl Debug for Sentence {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} <-> {:?}", self.origin, self.hiragana)
  }
}

impl Sentence {
  #[allow(dead_code)]
  pub fn new(
    origin: &str,
    to_input: &str,
  ) -> Result<Self, RomanParseError> {
    Ok(Sentence {
      origin: origin.into(),
      hiragana: RomanStr::new(to_input)?,
    })
  }

  #[allow(dead_code)]
  pub fn new_with_inputted(
    origin: &str,
    to_input: &str,
    inputted: &str,
  ) -> Result<Self, RomanParseError> {
    let mut roman_str = RomanStr::new(to_input)?;
    for inputted in inputted.chars() {
      roman_str.input(inputted);
    }
    Ok(Sentence {
      origin: origin.into(),
      hiragana: roman_str,
    })
  }

  pub fn empty() -> Self {
    Sentence {
      origin: "".into(),
      hiragana: RomanStr::new("").unwrap(),
    }
  }

  pub fn from(origin: &str, yomigana: RomanStr) -> Self {
    Sentence {
      origin: origin.into(),
      hiragana: yomigana,
    }
  }

  pub fn origin(&self) -> &str {
    self.origin.as_str()
  }

  pub fn yomiagana(&self) -> TypingStr {
    TypingStr {
      will_input: self.hiragana.will_input_yomigana().collect(),
      inputted: self.hiragana.inputted_yomigana().collect(),
    }
  }

  pub fn roman(&self) -> TypingStr {
    TypingStr {
      will_input: self.hiragana.will_input_roman(),
      inputted: self.hiragana.inputted_roman().into(),
    }
  }

  pub fn input(&mut self, typed: char) -> bool {
    self.hiragana.input(typed)
  }

  pub fn completed(&self) -> bool {
    self.hiragana.completed()
  }
}
