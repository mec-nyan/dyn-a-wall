//! dynawall
//!
//! Create and install dynamic wallpapers (aka background slide shows) for Gnome Shell.
//!

use std::{fs, path::Path};

use crate::{files::get_paths, parse_args::parse};

mod files;
mod parse_args;
mod slide_show;

fn main() -> Result<(), String> {
    // 1. Parse command line arguments:
    //    - Name:                           string
    //    - Images directory:               string
    //    - Image duration:                 number
    //    - Copy or link to images:         maybe boolean
    //    - With transition, in which case: boolean
    //        - Transition duration:        number
    //    - Do install:                     boolean
    let options = parse()?;

    // 2. Generate the file names and paths.  Consider the value of the "install" flag.
    //    - true:
    //        - For the slide show description (and images if "copy" == true):
    //            - /home/${USER}/.local/share/backgrounds/${NAME}
    //        - For the wallpaper (xml) file:
    //            - /home/${USER}/.local/share/gnome-background-properties/
    //    - false:
    //        - Use a "share" directory on the working directory, then:
    //        - For the slide show description (and images if "copy" == true):
    //            - ./share/backgrounds/${NAME}
    //        - For the wallpaper (xml) file:
    //            - ./share/gnome-background-properties/
    let files = get_paths(&options)?;

    // 3. Check if a wallpaper with that name already exists.
    //    If so, abort.
    //    TODO: Instead of aborting, check if the about to be created slide show is present in the
    //    wallpaper.  If it is present, abort.  Otherwise, append to the wallpaper.
    //
    // TODO: Extract this logic.
    //
    // Get the parent directory.  We must ensure it exists (we'll create it if it doesn't).
    let wallpaper_path = Path::new(&files.wallpaper_dir);
    let backgrounds_path = wallpaper_path.parent().unwrap();
    let backgrounds_dir = fs::create_dir_all(backgrounds_path);
    match backgrounds_dir {
        Ok(_) => println!(
            "== backgrounds directory created in {}.",
            backgrounds_path.to_string_lossy()
        ),
        Err(msg) => {
            eprintln!("== error: {}", msg);
            return Err("couldn't create backgrounds directory.".to_string());
        }
    }

    // create_dir_all won't fail if the directory is already present, and "exists" is not safe.
    // We'll use the atomic "create_dir" to create the wallpaper directory if it doesn't exist and
    // exit the program if it does.  We don't want to overwrite the wallpaper unless the user
    // explicitly says so.
    let wallpaper_dir = fs::create_dir(wallpaper_path);
    match wallpaper_dir {
        Ok(_) => println!(
            "== wallpaper directory created in {}.",
            wallpaper_path.to_string_lossy()
        ),
        Err(msg) => {
            eprintln!("== error: {}", msg);
            return Err("couldn't create wallpaper directory.".to_string());
        }
    }

    // 3.  Check if the slide show exists.  If so, abort.

    // 4. Find the images in the given path.
    //    - If no images are found, return here.

    // 5. Generate the slide show description.
    //    5.1 Make the slide show directory.
    //    5.2 Create the slide show file.  Write its content.
    //    5.3 If the "copy" option is true, copy the images to this directory.

    // 6. Generate the wallpaper description.
    //    6.1 Make the directory if it doesn't exist.
    //    6.2 Create the wallpaper xml file.

    // 7. Print final message and exit.

    Ok(())
}
