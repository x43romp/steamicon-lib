use std::io;

/// Gets the directory path to steam
///
/// # Errors
///
/// This function errors if there is no SteamPath in the Windows Registry
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

/// Expands variables for Windows paths
///
/// # Panics
///
/// Panics if a env variable is used but cannot be found
#[cfg(target_os = "windows")]
pub fn expand_win_vars(s: &str) -> String {
    let re = regex::Regex::new(r"^\%([^\/]+)\%").unwrap();
    re.replace_all(s, |caps: &regex::Captures| {
        std::env::var(&caps[1]).unwrap_or_else(|_| caps[0].to_string())
    })
    .into_owned()
}

/// Gets the current user's desktop path
///
/// # Errors
///
/// Errors if it cannot find Desktop in the User Shell Folders
#[cfg(target_os = "windows")]
pub fn desktop_path() -> Result<std::path::PathBuf, io::Error> {
    let mut path = windows_registry::CURRENT_USER
        .open(r"Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\User Shell Folders")?
        .get_string("Desktop")?;

    if path.contains("%") {
        path = expand_win_vars(&path);
    }

    Ok(std::path::PathBuf::from(path))
}

#[cfg(target_os = "macos")]
#[cfg(target_os = "linux")]
pub fn desktop_path() -> Result<std::path::PathBuf, io::Error> {
    Ok(std::path::PathBuf::default())
}

/// Gets the current user's menu path
///
/// # Errors
///
/// It cannot find the desktop in the User Shell Folders
#[cfg(target_os = "windows")]
pub fn menu_path() -> Result<std::path::PathBuf, io::Error> {
    let mut path = windows_registry::CURRENT_USER
        .open(r"Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\User Shell Folders")?
        .get_string("Programs")?;

    if path.contains("%") {
        path = expand_win_vars(&path);
    }

    let mut buffer = std::path::PathBuf::from(path);
    buffer.push("Steam");

    Ok(buffer)
}

#[cfg(target_os = "macos")]
#[cfg(target_os = "linux")]
pub fn menu_path() -> Result<std::path::PathBuf, io::Error> {
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

    #[test]
    #[cfg(target_os = "windows")]
    fn test_desktop_path() {
        let binding = desktop_path().unwrap();
        let path = binding.to_str().unwrap();
        assert!(path.contains("Users"));
        assert!(path.contains("Desktop"));
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[cfg(target_os = "linux")]
    fn test_desktop_path() {
        let path = desktop_path().unwrap();
        assert_eq!(path, std::path::PathBuf::default());
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_menu_path() {
        let binding = menu_path().unwrap();
        let path = binding.to_str().unwrap();
        assert!(path.contains("Start Menu"));
        assert!(path.contains("Steam"));
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[cfg(target_os = "linux")]
    fn test_menu_path() {
        let path = menu_path().unwrap();
        assert_eq!(path, std::path::PathBuf::default());
    }
}
