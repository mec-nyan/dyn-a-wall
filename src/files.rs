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

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::parse_args::Options;

pub struct Files {
    pub slide_show_name: String,
    pub wallpaper_name: String,
    pub wallpaper_dir: String,
    pub properties_dir: String,
}

/// Search for images in the specified directory.  Return an error if no images were found.
// TODO: Option to search recursively.
pub fn find_images(dir: &str) -> Result<Vec<PathBuf>, String> {
    let path = if dir == "." {
        std::env::current_dir().unwrap()
    } else {
        PathBuf::from(dir)
    };

    let mut images: Vec<PathBuf> = Vec::new();

    // TODO: Get rid of all these `unwrap` and handle errors properly.
    if path.is_dir() {
        for entry in fs::read_dir(&path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let extension = path.extension().unwrap();
            if extension == "png" || extension == "jpg" {
                images.push(path);
            }
        }
    }

    if images.len() == 0 {
        Err(format!("There are no images in {}", path.to_string_lossy()))
    } else {
        Ok(images)
    }
}

/// Create the wallpaper directory.  This directory will contain the slide show description and the
/// images if the "copy" option is selected.  Options are included to implement "overwrite".
pub fn make_wallpaper_directory(files: &Files, _: &Options) -> Result<(), String> {
    // Get the parent directory.  We must ensure it exists (we'll create it if it doesn't).
    let wallpaper_path = Path::new(&files.wallpaper_dir);
    let backgrounds_path = wallpaper_path.parent().unwrap();

    // Ensure the parent directory exists.
    make_backgrounds_directory(backgrounds_path)?;

    // create_dir_all won't fail if the directory is already present, and "exists" is not safe.
    // We'll use the atomic "create_dir" to create the wallpaper directory if it doesn't exist and
    // exit the program if it does.  We don't want to overwrite the wallpaper unless the user
    // explicitly says so.
    let wallpaper_dir = fs::create_dir(wallpaper_path);
    match wallpaper_dir {
        Ok(_) => Ok(()),
        Err(msg) => Err(format!("Failed to create wallpaper directory: {msg}")),
    }
}

fn make_backgrounds_directory(path: &Path) -> Result<(), String> {
    let result = fs::create_dir_all(path);
    match result {
        Ok(_) => Ok(()),
        Err(msg) => Err(format!("Failed to create backgrounds directory: {msg}")),
    }
}

pub fn get_paths(options: &Options) -> Result<Files, String> {
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

fn destination(install: bool) -> Result<String, String> {
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
