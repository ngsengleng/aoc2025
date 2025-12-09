use std::io::{BufWriter, Write, stdout};
use std::env;

mod day2;
mod common;

// current active pkg, since dynamic imports doesnt seem to be a thing
use day2 as pkg;

fn main() {
  let stdout = stdout();
  let args: Vec<String> = env::args().collect();
  if args.len() != 3 {
    println!("File path was not passed as an argument\n");
    return;
  }
  let is_part_1 = args[2] == "1";
  let mut message = pkg::solve(args[1].clone(), is_part_1);
  let mut writer = BufWriter::new(stdout.lock());
  message.push('\n');
  writer.write(message.as_bytes()).unwrap();
  writer.flush().unwrap();
}