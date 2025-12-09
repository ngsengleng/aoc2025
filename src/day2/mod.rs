use crate::common::{InputType, read_file};

fn get_range(inputs: String) -> (String, String) {
  let values: Vec<String> = inputs.split("-").map(|v| v.to_string()).collect();
  return (values[0].clone(), values[1].clone());
}

fn is_candidate(value: i64, divisor: i64) -> bool {
  let test = value / (i64::pow(divisor, 2) / 10);
  return test != 0;
}

fn is_even_length(value: usize) -> bool {
  return value % 2 == 0;
}

fn parse_numeric_string(s: &String) -> i64 {
  return s.parse().expect("failed to parse non numeric string");
}

// first solution using math
fn find_invalid_ids(start_range: &String, end_range: &String) -> Vec<i64> {
  let mut invalids = Vec::<i64>::new();
  let start: i64 = parse_numeric_string(start_range);
  let end: i64 = parse_numeric_string(end_range);
  if !is_even_length(start_range.len()) && !is_even_length(end_range.len()) {
    return invalids;
  }
  let divisor: i64 = if is_even_length(end_range.len()) {
    i64::pow(10, ((end_range.len() / 2) as i64).try_into().unwrap())
  } else {
    i64::pow(10, ((start_range.len() / 2) as i64).try_into().unwrap())
  };
  // println!("range: {start_range}, {end_range}");
  for i in start..(end + 1) {
    if !is_candidate(i, divisor) {
      continue;
    }
    let first_half = i / divisor;
    let second_half = i % divisor;
    
    if first_half == second_half {      
      invalids.push(i);
    }
  }

  return invalids;
}

// some voodoo magic where if u concat the same string to itself and remove the start and end, 
// if its a repeated substring the original can be found, O(n) time

// brute force is O(n2) by getting all possible combinations of substrings, repeating up to 
// length and check if matches
fn find_repeating_substrings(start_range: &String, end_range: &String) -> Vec<i64> {
  let mut invalids: Vec<i64> = Vec::new();
  let start = parse_numeric_string(start_range);
  let end = parse_numeric_string(end_range) + 1;
  for i in start..end {
    let original = i.to_string();
    let mut s = original.clone();
    s.push_str(&original);
    let partial_string = &s[1..(s.len() - 1)];
    if partial_string.contains(&original) {
      invalids.push(i);
    }
  }
  return invalids;
}

pub fn solve(input_path: String, is_part_1: bool) -> String {
  let instructions: Vec<String> = read_file(&input_path, InputType::CSV);
  let mut sum: i64 = 0;
  for i in instructions {
    let (start, end) = get_range(i);
    let invalids = if is_part_1 {
      find_invalid_ids(&start, &end)
    } else {
      find_repeating_substrings(&start, &end)
    };
    sum += invalids.iter().sum::<i64>();
  }
  return sum.to_string();
}