mod util;
mod unit;

use regex::Regex;
use lazy_static::lazy_static;
use crate::unit::Unit;
use crate::util::constants::*;

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

pub fn ms_to_string(n: usize) -> String {
  let mut s: Vec<String> = vec![];

  let mut r = n as f64;

  let u = (r / WEEK as f64).floor();
  r %= WEEK as f64;
  if u > 0.0 {
    let f = format!("{}w", u);
    s.push(f);
  }

  let u = (r / DAY as f64).floor();
  r %= DAY as f64;
  if u > 0.0 {
    let f = format!("{}d", u);
    s.push(f);
  }

  let u = (r / HOUR as f64).floor();
  r %= HOUR as f64;
  if u > 0.0 {
    let f = format!("{}h", u);
    s.push(f);
  }

  let u = (r / MINUTE as f64).floor();
  r %= MINUTE as f64;
  if u > 0.0 {
    let f = format!("{}m", u);
    s.push(f);
  }

  let u = (r / SECOND as f64).floor();
  r %= SECOND as f64;
  if u > 0.0 {
    let f = format!("{}s", u);
    s.push(f);
  }

  let u = (r / MILLISECOND as f64).floor();
  // r %= MILLISECOND as f64;
  if u > 0.0 {
    let f = format!("{}ms", u);
    s.push(f);
  }

  s.join(" ")
}

#[cfg(test)]
mod tests {
  use super::*;

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

  #[test]
  fn as_string_simple() {
    let s = ms_to_string(HOUR);
    assert_eq!(s, "1h");
  }

  #[test]
  fn as_string_complex() {
    let s = ms_to_string(WEEK + 2 * DAY + 7 * HOUR + 14 * MINUTE + 8 * SECOND + 10 * MILLISECOND);
    assert_eq!(s, "1w 2d 7h 14m 8s 10ms");
  }
}
