use super::seconds::Seconds;
use std::{cmp::Ordering, ops::SubAssign};

#[derive(Debug)]
pub enum DurationError {
  InvalidArguments(String),
}

pub type DurationResult<T> = Result<T, DurationError>;

#[derive(Debug, Clone, PartialEq)]
pub struct Duration {
  from: Seconds,
  to: Seconds,
}

impl Duration {
  pub fn new(from: f64, to: f64) -> DurationResult<Self> {
    if from >= to {
      return Err(DurationError::InvalidArguments(
        format!("`from`  must be before than `to`; Actual: {{ from: {}, to: {} }}", from, to),
      ));
    }
    Ok(Self {
      from: from.into(),
      to: to.into(),
    })
  }

  pub fn following(&self, length: f64) -> Self {
    Self {
      from: self.to,
      to: self.to + length.into(),
    }
  }

  #[allow(dead_code)]
  pub fn following_replace(&mut self, length: f64) -> Self {
    let following = self.following(length);
    *self = following.clone();
    following
  }

  pub fn concat(&self, other: &Self) -> Self {
    let start = self.from.min(other.from);
    let end = self.to.max(other.to);
    Self {
      from: start,
      to: end,
    }
  }

  pub fn includes(&self, time: &Seconds) -> bool {
    self.from <= *time && *time < self.to
  }

  pub fn remaining_ratio(&self, now: Seconds) -> f64 {
    let duration = self.to - self.from;
    let elapsed = now - self.from;
    elapsed / duration
  }
}

#[test]
fn duration() -> DurationResult<()> {
  assert!(Duration::new(1.0, 0.0).is_err());
  assert!(Duration::new(0.0, 0.0).is_err());
  assert!(Duration::new(0.0, 1.0).is_ok());

  let duration = Duration::new(0.0, 1.3)?;
  assert_eq!(0.0, duration.from);
  assert_eq!(1.3, duration.to);
  assert!(duration.includes(&0.7.into()));
  assert!(!duration.includes(&1.3.into()));
  assert!(!duration.includes(&Seconds::new(-0.1)));

  let duration = Duration::new(2.5, 3.1)?.concat(&duration);
  assert_eq!(0.0, duration.from);
  assert_eq!(3.1, duration.to);

  let duration = duration.following(2.4);
  assert_eq!(3.1, duration.from);
  assert_eq!(5.5, duration.to);
  Ok(())
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct MinuteSecond {
  minutes: u32,
  seconds: Seconds,
}

impl MinuteSecond {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn minutes(&self, minutes: u32) -> Self {
    MinuteSecond {
      minutes,
      seconds: self.seconds,
    }
  }

  pub fn seconds<T: Into<Seconds> + SubAssign>(
    &self,
    seconds: T,
  ) -> Self {
    let mut seconds: Seconds = seconds.into();
    while Seconds::new(60.0) <= seconds {
      seconds -= 60.0.into();
    }
    MinuteSecond {
      minutes: self.minutes,
      seconds,
    }
  }

  pub fn as_seconds(&self) -> Seconds {
    Seconds::new(self.minutes as f64 * 60.0) + self.seconds
  }
}

impl From<MinuteSecond> for Seconds {
  fn from(ms: MinuteSecond) -> Self {
    ms.as_seconds()
  }
}

impl PartialOrd for MinuteSecond {
  fn partial_cmp(&self, other: &MinuteSecond) -> Option<Ordering> {
    if self.minutes == other.minutes {
      self.seconds.partial_cmp(&other.seconds)
    } else {
      self.minutes.partial_cmp(&other.minutes)
    }
  }
}
