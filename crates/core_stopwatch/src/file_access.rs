use std::{fs, path::PathBuf};

use crate::prelude::*;

const ENV_DATA: &str = ".env";
const PATH: &str = "data.ron";
const DEV_FOLDER_END: &str = ".date_during_dev";

pub fn get_path_to_data(project_root: &str) -> AppResult<PathBuf> {
    if cfg!(debug_assertions) {
        let folder_path = PathBuf::from(project_root).join(DEV_FOLDER_END);
        let data_path = folder_path.join(PATH);

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

pub fn handle_env_file(project_root: &str) {
    if cfg!(debug_assertions) {
        let path = PathBuf::from(project_root).join(ENV_DATA);
        dotenv::from_path(path.as_path()).expect("Unable to read .env file");
    }
}
