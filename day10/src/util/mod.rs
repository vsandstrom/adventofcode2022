
pub fn parse(path: &str) -> Result<String, std::io::Error>{
    return std::fs::read_to_string(path);
}
