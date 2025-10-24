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

#[cfg(target_os = "windows")]
pub fn expand_win_vars(s: &str) -> String {
    let re = regex::Regex::new(r"^\%([^\/]+)\%").unwrap();
    re.replace_all(s, |caps: &regex::Captures| {
        std::env::var(&caps[1]).unwrap_or_else(|_| caps[0].to_string())
    })
    .into_owned()
}

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
}
