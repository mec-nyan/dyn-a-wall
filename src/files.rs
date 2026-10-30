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

use crate::parse_args::Options;

pub struct Files {
    pub slide_show_name: String,
    pub wallpaper_name: String,
    pub wallpaper_dir: String,
    pub properties_dir: String,
}

pub fn get_paths(options: &Options) -> std::result::Result<Files, String> {
    match destination(options.install) {
        Ok(base) => Ok(Files {
            slide_show_name: format!("{}-slide-show.xml", options.name),
            wallpaper_name: format!("{}-wallpaper.xml", options.name),
            wallpaper_dir: format!("{base}/backgrounds/{}", options.name),
            properties_dir: format!("{base}/gnome-background-properties"),
        }),
        Err(msg) => Err(msg),
    }
}

const LOCAL_PATH: &str = "./dynawall/share";

fn destination(install: bool) -> std::result::Result<String, String> {
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
    use crate::parse_args::Options;

    use super::LOCAL_PATH;
    use super::destination;
    use super::get_paths;

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

    #[test]
    fn test_get_paths_install() {
        let mut options = Options::new();
        options.install = true;

        let paths = get_paths(&options).unwrap();

        assert!(paths.properties_dir.starts_with("/home/"));

        assert!(
            paths
                .properties_dir
                .ends_with("/.local/share/gnome-background-properties")
        );

        assert!(paths.wallpaper_dir.starts_with("/home/"));

        assert!(
            paths
                .wallpaper_dir
                .ends_with(&format!("/.local/share/backgrounds/{}", options.name))
        );

        assert_eq!(
            paths.slide_show_name,
            format!("{}-slide-show.xml", options.name)
        );

        assert_eq!(
            paths.wallpaper_name,
            format!("{}-wallpaper.xml", options.name)
        );
    }

    #[test]
    fn test_get_paths_local() {
        let options = Options::new();

        let paths = get_paths(&options).unwrap();

        assert_eq!(
            paths.wallpaper_dir,
            format!("./{0}/share/backgrounds/{0}", options.name)
        );

        assert_eq!(
            paths.properties_dir,
            format!("./{}/share/gnome-background-properties", options.name)
        );

        assert_eq!(
            paths.slide_show_name,
            format!("{}-slide-show.xml", options.name)
        );

        assert_eq!(
            paths.wallpaper_name,
            format!("{}-wallpaper.xml", options.name)
        );
    }
}
