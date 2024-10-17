use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]

fn test_database_creation()
{
    // Create a temporary directory for our test
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("config");
    let cache_dir = temp_dir.path().join("cache");
    let data_dir = temp_dir.path().join("data");

    // Set environment variables to use our temporary directories
    std::env::set_var("XDG_CONFIG_HOME", config_dir.to_str().unwrap());
    std::env::set_var("XDG_CACHE_HOME", cache_dir.to_str().unwrap());
    std::env::set_var("XDG_DATA_HOME", data_dir.to_str().unwrap());

    // Run your CLI application
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.assert().success();

    // Check if the database file was created
    let expected_db_path = if cfg!(debug_assertions)
    {
        cache_dir.join("dev.db")
    }
    else
    {
        data_dir.join("journal.db")
    };

    assert!(expected_db_path.exists(), "Database file was not created");

    // Optionally, check if the database file is not empty
    let metadata = fs::metadata(expected_db_path).unwrap();
    assert!(metadata.len() > 0, "Database file is empty");

    // Clean up (tempfile will automatically delete the temporary directory)
}
