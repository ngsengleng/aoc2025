pub enum InputType {
  Lines,
  CSV
}
pub fn read_file(input_path:  &String, input_type: InputType) -> Vec<String> {
  let contents: String = std::fs::read_to_string(input_path).expect("Error reading file");
  return match input_type {
    InputType::Lines => contents.lines().map(|s| s.to_string()).collect(),
    InputType::CSV => contents.split(",").map(|s| s.to_string()).collect(),
  }
}