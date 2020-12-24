pub struct Config {
    pub file_directory: String,
}

impl Config {
    pub fn default() -> Self {
        Config {
            file_directory: "/home/inab/tmp/note".to_string(),
        }
    }
}
