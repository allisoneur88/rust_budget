use std::path::Path;

pub struct FileHelper {}

impl FileHelper {
    pub fn save_to_file<P: AsRef<Path>, T: serde::Serialize>(
        path: &P,
        data: &T,
    ) -> std::io::Result<()> {
        std::fs::write(path, serde_json::to_string_pretty(data)?)
    }
}
