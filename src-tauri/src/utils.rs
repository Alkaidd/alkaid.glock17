use dirs::home_dir;
use std::fs;

pub const MC_DB_FILENAME: &str = ".glock17db";

pub fn get_db_file_path() -> std::path::PathBuf {
    home_dir()
        .map(|it| it.join(MC_DB_FILENAME))
        .unwrap_or_default()
}

pub fn db_exists() -> bool {
    let dir = get_db_file_path();
    fs::metadata(dir).is_ok()
}

pub fn create_db_file() -> std::io::Result<()> {
    let dir = get_db_file_path();
    fs::File::create(dir)?;
    Ok(())
}

pub fn check_db_file() -> std::io::Result<()> {
    if !db_exists() {
        create_db_file()?;
    }
    Ok(())
}
