pub struct FileHelper {}

impl FileHelper {
    pub fn save_to_file<T: serde::Serialize>(path: &str, data: &T) -> std::io::Result<()> {
        std::fs::write(path, serde_json::to_string_pretty(data)?)
    }
}
