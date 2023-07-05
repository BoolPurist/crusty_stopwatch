use std::{fs, path::PathBuf};

use crate::AppResult;

pub fn get_path_to_data() -> AppResult<PathBuf> {
    if cfg!(debug_assertions) {
        let folder_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".date_during_dev");
        let data_path = folder_path.join("data.ron");

        if !folder_path.exists() {
            fs::create_dir_all(&folder_path)?;
            info!(
                "Created directory at {:?} for loading data from",
                folder_path
            );
        }
        if !data_path.exists() {
            fs::write(&data_path, "")?;
            info!(
                "Created empty data at {:?} for loading data from",
                data_path
            );
        }

        Ok(data_path)
    } else {
        todo!("Not implemented for production");
    }
}

pub fn handle_env_file() {
    if cfg!(debug_assertions) {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".env");
        dotenv::from_path(path.as_path()).expect("Unable to read .env file");
    }
}
