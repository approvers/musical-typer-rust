use std::{
  cmp::Ordering,
  fmt::{Display, Formatter, Result},
  ops::{Add, AddAssign, Div, Sub, SubAssign},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Seconds(i64); // owns milliseconds

impl Seconds {
  pub fn new(seconds: f64) -> Self {
    Self((seconds * 1000.0).ceil() as i64)
  }

  pub fn as_f64(&self) -> f64 {
    self.0 as f64 / 1000.0
  }

  pub fn max(self, other: Self) -> Self {
    Self(self.0.max(other.0))
  }

  pub fn min(self, other: Self) -> Self {
    Self(self.0.min(other.0))
  }
}

impl From<f64> for Seconds {
  fn from(v: f64) -> Self {
    Self::new(v)
  }
}

impl Add for Seconds {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0 + rhs.0)
  }
}

impl AddAssign for Seconds {
  fn add_assign(&mut self, rhs: Self) {
    self.0 += rhs.0;
  }
}

impl Sub for Seconds {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_sub(rhs.0))
  }
}

impl SubAssign for Seconds {
  fn sub_assign(&mut self, rhs: Self) {
    self.0 -= rhs.0;
  }
}

impl Div for Seconds {
  type Output = f64;

  fn div(self, rhs: Self) -> Self::Output {
    self.0 as f64 / rhs.0 as f64
  }
}

impl PartialEq<Seconds> for f64 {
  fn eq(&self, other: &Seconds) -> bool {
    ((self * 1e3) as i64 - other.0 as i64).abs() <= 10
  }
}

impl PartialEq<f64> for Seconds {
  fn eq(&self, other: &f64) -> bool {
    (self.0 as i64 - (other * 1e3) as i64) <= 10
  }
}

impl PartialEq for Seconds {
  fn eq(&self, other: &Self) -> bool {
    (self.0 as i64 - other.0 as i64) <= 10
  }
}

impl PartialOrd for Seconds {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl Display for Seconds {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:.2}", self.0)
  }
}
