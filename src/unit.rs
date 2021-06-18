use crate::util::constants::{
  WEEK,
  DAY,
  HOUR,
  MINUTE,
  SECOND,
  MILLISECOND,
};

#[derive(Debug, PartialEq)]
pub enum Unit {
  Week,
  Day,
  Hour,
  Minute,
  Second,
  Millisecond,
}

impl Unit {
  pub fn from_str(s: &str) -> Option<Self> {
    let u = s.to_lowercase();
    if u == "w" || u == "wk" || u.starts_with("week") {
      Some(Self::Week)
    } else if u == "d" || u.starts_with("day") {
      Some(Self::Day)
    } else if u == "h" || u == "hr" || u.starts_with("hour") {
      Some(Self::Hour)
    } else if u == "s" || u.starts_with("sec") {
      Some(Self::Second)
    } else if u == "ms" || u.starts_with("millisecond") {
      Some(Self::Millisecond)
    } else if u == "m" || u.starts_with("min") {
      Some(Self::Minute)
    } else {
      None
    }
  }

  pub fn denomination(&self) -> usize {
    match self {
      Self::Week => WEEK,
      Self::Day => DAY,
      Self::Hour => HOUR,
      Self::Minute => MINUTE,
      Self::Second => SECOND,
      Self::Millisecond => MILLISECOND,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_longhand() {
    let u = Unit::from_str("hours");
    assert_eq!(u, Some(Unit::Hour));
  }

  #[test]
  fn parse_shorthand() {
    let u = Unit::from_str("hr");
    assert_eq!(u, Some(Unit::Hour));
  }

  #[test]
  fn reject_nonsense() {
    let u = Unit::from_str("cheeseburger");
    assert_eq!(u, None);
  }
}
