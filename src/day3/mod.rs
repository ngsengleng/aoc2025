use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, io::Read};

use crate::common::{read_file, InputType};

const MAX_BATTERIES: i8 = 12;

fn traverse_string(s: String) -> i64 {
  let ptr1 = 0;
  let mut ptr2 = 1;
  let chars: Vec<char> = s.chars().collect();
  let mut first_digit = chars[ptr1];
  let mut second_digit = chars[ptr2];
  while ptr2 < s.len() {
    // edge case: if at end of bank, immediately replace second battery if necessary
    if ptr2 == s.len() - 1 {
      second_digit = std::cmp::max(second_digit, chars[ptr2]);
      break;
    }
    // if second battery bigger than first, replace first with second and move
    // one battery down
    if chars[ptr2] > first_digit {
      first_digit = chars[ptr2];
      second_digit = chars[ptr2 + 1];
    } else if chars[ptr2] > second_digit {
      second_digit = chars[ptr2];
    }
    ptr2 += 1;
  }
  let res: i64 = format!("{first_digit}{second_digit}").parse().expect("error parsing string");
  return res;
}

fn get_biggest_number(s: String) -> i64 {
  let mut this_string = s.as_bytes();
  let mut res: i64 = 0;
  // assume digits up to i from the back is correct, need to pick the biggest from the remaining digits at the front
  for i in (0..12).rev() { 
    let (best_idx, best_num) = this_string.iter().enumerate().rev().skip(i).max_by_key(|s| s.1).unwrap();
    this_string = &this_string[best_idx+1..];
    res = res * 10 + (best_num - b'0').to_string().parse().unwrap_or(0);
  }
  return res;
}

pub fn solve(input_path: String, is_part_1: bool) -> String {
  let contents = read_file(&input_path, InputType::Lines);
  let mut res = 0;
  for bank in contents {
    let curr = if is_part_1 {
      traverse_string(bank)
    } else {
      get_biggest_number(bank)
    };
    res += curr;
  }
  if is_part_1 {
    assert_eq!(17332, res);
  } else {
    assert_eq!(172516781546707, res);
  }
  return res.to_string();
}