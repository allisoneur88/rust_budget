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

    pub fn load_from_file<P, T>(path: &P) -> AppResult<T>
    where
        P: AsRef<Path>,
        T: serde::de::DeserializeOwned + Default,
    {
        if !path.as_ref().exists() {
            return Ok(T::default());
        }

        let contents = fs::read_to_string(path)?;
        let data = serde_json::from_str(&contents)?;
        Ok(data)
    }
}
