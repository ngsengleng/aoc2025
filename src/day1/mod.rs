const MAX: i32 = 100;

fn rotate_left(current_value: i32, rotation: i32) -> (i32, i32) {
  let mut rounds = rotation / MAX;
  let mut new_value = (current_value - rotation) % MAX;
  if new_value < 0 {
    new_value = MAX + new_value;
  }
  if new_value > current_value && current_value != 0 && new_value != 0 {
    rounds += 1;
  }
  return (rounds, new_value);
}

fn rotate_right(current_value: i32, rotation: i32) -> (i32, i32) {
  let mut rounds = rotation / MAX;
  let new_value = (current_value + rotation) % MAX;
  if new_value < current_value && current_value != 0 && new_value != 0 {
    rounds += 1;
  }
  return (rounds, new_value);
}

fn rotate(direction: char, current_value: i32, distance: i32) -> (i32, i32) {
  if direction == 'L' {
    return rotate_left(current_value, distance);
  }
  return rotate_right(current_value, distance);
}
fn read_file(input_path:  &String) -> Vec<String> {
  let contents: String = std::fs::read_to_string(input_path).expect("Error reading file");
  let instructions = contents.lines().map(|s| s.to_string()).collect(); 
  return instructions;
}

pub fn solve(input_path: String, is_part_1: bool) -> String {
  let mut res = 0;
  let mut curr = 50;
  let instructions: Vec<String> = read_file(&input_path);
  for instruction in instructions {
    let direction = &instruction.chars().nth(0);
    let distance = &instruction[1..].parse::<i32>().expect("Error parsing distance");
    match direction {
      None => println!("failed"),
      Some(direction) => {
        let (r, new) = rotate(*direction, curr, *distance);
        println!("{direction}: {curr}, {distance}, {new}");
        curr = new;
        if !is_part_1 {
          res += r;
        }
      }
    }
    if curr == 0 {
      res += 1;
    }
    println!("{res}, {curr}");

  }
  return res.to_string();
}

