use std::env;
use std::fs;
use std::path::PathBuf;


pub fn initialize() -> Result<(), String>
{
    let application_directory = get_application_directory()?;

    let data_directory = application_directory.join("AppData");
    let cache_directory = application_directory.join("cache");

    fs::create_dir_all(&data_directory)
        .map_err(|error|
        {
            format!(
                "Failed to create AppData directory '{}': {}",
                data_directory.display(),
                error
            )
        })?;

    fs::create_dir_all(&cache_directory)
        .map_err(|error|
        {
            format!(
                "Failed to create cache directory '{}': {}",
                cache_directory.display(),
                error
            )
        })?;

    unsafe
    {
        env::set_var("XDG_DATA_HOME", &data_directory);
        env::set_var("XDG_CACHE_HOME", &cache_directory);
    }

    Ok(())
}


fn get_application_directory() -> Result<PathBuf, String>
{
    #[cfg(target_os = "linux")]
    {
        if let Some(appimage) = env::var_os("APPIMAGE")
        {
            let appimage = PathBuf::from(appimage);

            return appimage
                .parent()
                .map(|path| path.to_path_buf())
                .ok_or_else(|| {
                    "Failed to determine AppImage directory.".to_string()
                });
        }
    }

    let executable = env::current_exe()
        .map_err(|error|
        {
            format!(
                "Failed to determine worker executable path: {}",
                error
            )
        })?;

    executable
        .parent()
        .map(|path| path.to_path_buf())
        .ok_or_else(|| {
            "Failed to determine worker directory.".to_string()
        })
}