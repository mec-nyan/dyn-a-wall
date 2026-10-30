//! dynawall
//!
//! Create and install dynamic wallpapers (aka background slide shows) for Gnome Shell.
//!

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
    let options = parse_args::parse()?;

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

    // 3. Check if a wallpaper with that name already exists.
    //    If so, abort.
    //    TODO: Instead of aborting, check if the about to be created slide show is present in the
    //    wallpaper.  If it is present, abort.  Otherwise, append to the wallpaper.

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
