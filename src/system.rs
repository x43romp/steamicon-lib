use std::io;

#[cfg(target_os = "windows")]
pub fn steam_path() -> Result<std::path::PathBuf, io::Error> {
    let steam_path = windows_registry::CURRENT_USER
        .open(r"Software\\Valve\\Steam")?
        .get_string("SteamPath")?;

    Ok(std::path::PathBuf::from(steam_path))
}

#[cfg(target_os = "macos")]
#[cfg(target_os = "linux")]
pub fn steam_path() -> Result<std::path::PathBuf, io::Error> {
    Ok(std::path::PathBuf::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[cfg(target_os = "windows")]
    fn test_steam_path() {
        let path = steam_path().unwrap();
        let target = "c:/program files (x86)/steam";
        println!("steam path: {}", path.to_str().unwrap());
        assert_eq!(path.to_str().unwrap(), target);
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[cfg(target_os = "linux")]
    fn test_steam_path() {
        let path = steam_path().unwrap();
        assert_eq!(path, std::path::PathBuf::default());
    }
}
