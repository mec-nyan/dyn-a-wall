//! Generate names and paths for the required files.
//!
//! 1. Where to put the files:
//!    - /home/{USER}/.local/share/... for installing the wallpaper.
//!    - ./{WALLPAPER_NAME}/share/... to generate the files without installing.
//!
//! 2. The generated files are:
//!    - destination/backgrounds/{WALLPAPER_NAME}/SLIDE_SHOW_NAME.xml
//!        - If the user opts to copy the images, they will be copied to this directory.
//!    - describing/gnome-background-properties/WALLPAPER_NAME.xml

const LOCAL_PATH: &str = "./dynawall/share";

pub fn destination(install: bool) -> std::result::Result<String, String> {
    if install {
        let home = std::env::home_dir();
        match home {
            Some(path) => Ok(format!("{}/.local/share", path.to_string_lossy())),
            None => Err(String::from("error getting home directory")),
        }
    } else {
        // TODO: Maybe use a different name here (probably the user supplied wallpaper name).
        Ok(String::from(LOCAL_PATH))
    }
}

#[cfg(test)]
mod tests {
    use super::LOCAL_PATH;
    use super::destination;

    #[test]
    fn test_destination_install() {
        let path = destination(true).unwrap();

        assert!(path.starts_with("/home/"));
        assert!(path.ends_with("/.local/share"))
    }

    #[test]
    fn test_destination_local() {
        let path = destination(false).unwrap();

        assert_eq!(path, LOCAL_PATH);
    }
}
