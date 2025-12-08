pub fn read_file(input_path:  &String) -> Vec<String> {
  let contents: String = std::fs::read_to_string(input_path).expect("Error reading file");
  let instructions = contents.lines().map(|s| s.to_string()).collect(); 
  return instructions;
}