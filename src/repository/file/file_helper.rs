use crate::util::error::AppResult;
use std::{fs, path::Path};

pub struct FileHelper {}

impl FileHelper {
    pub fn save_to_file<P, T>(path: &P, data: &T) -> AppResult<()>
    where
        P: AsRef<Path>,
        T: serde::Serialize,
    {
        let json = serde_json::to_string_pretty(data)?;

        let tmp_path = path.as_ref().with_extension("tmp");
        fs::write(&tmp_path, json)?;
        fs::rename(&tmp_path, path)?;

        Ok(())
    }
}
