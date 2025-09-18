pub fn read_file_to_string(path: &String) -> String {
    std::fs::read_to_string(path).expect("Failed to read file!")
}
