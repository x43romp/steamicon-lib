use std::{fmt::Debug, io, path::PathBuf};

use regex::Regex;

pub struct Shortcut {
    path: PathBuf,
    pub appid: u32,
    pub name: String,
    pub icon: Icon,
}

#[derive(Debug)]
pub struct Icon {
    pub path: Option<PathBuf>,
    pub hash: Option<String>,
}

impl Shortcut {
    // create a new instance
    pub fn new(path: PathBuf) -> Self {
        Shortcut {
            path,
            appid: 123,
            name: "placeholder".to_string(),
            icon: Icon {
                path: None,
                hash: None,
            },
        }
    }

    /// Returns the read of this [`Shortcut`].
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn read(&mut self) -> Result<(), std::io::Error> {
        if self.path.exists() == false {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Game ID not found"));
        }
        {
            // name
            self.name = self.path.file_stem().unwrap().to_string_lossy().to_string();
        }

        let content = std::fs::read_to_string(&self.path)?;

        {
            // appid
            let pattern = Regex::new(r"URL=steam:\/\/rungameid\/(\d+)").unwrap();
            let captures = pattern.captures(&content).unwrap();
            if let Some(appid) = captures.get(1) {
                self.appid = appid.as_str().parse::<u32>().unwrap();
            }
        }

        {
            // icon path and hash
            let pattern = Regex::new(r"IconFile=(.*[\\\/]([\d\w]+)\.ico)").unwrap();
            let captures = pattern.captures(&content).unwrap();
            if let Some(path) = captures.get(1) {
                self.icon.path = Some(path.as_str().to_owned().into());
            }
            if let Some(hash) = captures.get(2) {
                self.icon.hash = Some(hash.as_str().to_owned());
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for Shortcut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Shortcut")
            .field("path", &self.path)
            .field("appid", &self.appid)
            .field("name", &self.name)
            .field("icon", &self.icon)
            .finish()
    }
}
