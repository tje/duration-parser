mod util;
mod unit;

use regex::Regex;
use lazy_static::lazy_static;
use crate::unit::Unit;

pub fn str_to_ms(s: &str) -> Result<usize, std::num::ParseIntError> {
  lazy_static! {
    static ref RX: Regex = Regex::new(r"(\d+)\s?([a-zA-Z]+)").unwrap();
  }

  let mut acc: usize = 0;
  for cap in RX.captures_iter(s) {
    let amt = &cap[1].parse::<usize>()?;
    if let Some(unit) = Unit::from_str(&cap[2]) {
      acc = acc + unit.denomination() * amt;
    }
  }
  Ok(acc)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::util::constants::*;

  #[test]
  fn two_part_compact() {
    let t = str_to_ms("1hr 30m").unwrap();
    assert_eq!(t, HOUR + 30 * MINUTE);
  }

  #[test]
  fn consolidate_duplicate_parts() {
    let t = str_to_ms("2hr 2 hours").unwrap();
    assert_eq!(t, 4 * HOUR);
  }

  #[test]
  fn case_insensitive() {
    let t = str_to_ms("100 SECONDS!!!").unwrap();
    assert_eq!(t, 100 * SECOND);
  }
}
