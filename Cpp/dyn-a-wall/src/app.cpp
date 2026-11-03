#include "app.hpp"

using namespace app;

auto App::run () -> int
{
    /*
    We need to:

    - Find the images:
        - In the current or specified directory.
            - Optionally recursive.

    - Create two files:
        - [name]-slide-show.xml (on ${prefix}/share/backgrounds/[name]/)
            - This file refers to the images.
        - [name]-wallpaper.xml (on ${prefix}/share/gnome-background-properties/)
            - This file refers to the previous file.

    NOTE: When installing the wallpaper the prefix should be "/home/${USER}/.local".

    Options:
    - Name
    - Images directory
    - Recursive search
    - Install
    - Copy the images to the wallpaper directory (default action is don't copy).
    - Image duration
    - Transition
    - Transition duration
    - Overwrite

    Errors:

    - Wallpaper with the same name already exists.
        - Unless the user explicitly chooses to overwrite it.
    - No images found.

    Flow:

    - Make paths/file names.

    - Does the wallpaper file exist?
        - No -> Continue
        - Yes:
            - Overwrite?
                - No -> Exit
                - Yes -> Continue

    - Does the wallpaper directory exists?
        - No: Continue
        - Yes:
            - Should we overwrite?
                - No:
                    - Exit.
                - Yes:
                    - Clean up.
                    - Continue.

    - Are there any images in the selected directory?
        - No -> Exit
        - Yes:
            - Get list of images.

    - Make the contents of the slide show file.
    - Make the contents of the wallpaper file.

    - Open/create the wallpaper file:
        NOTE: This file may already be opened if we opened or created it when checking for
        existence.
        - Error: -> Exit
        - Write contents to file.

    - Open/create the slide show file:
        NOTE: This file may already be opened if we opened or created it when checking for
        existence.
        - Error: -> Exit
        - Write contents to file.

    - Optionally copy the images to the wallpaper directory.
    */

    return 0;
}

auto Options::with_name ( std::string name ) -> Options&
{
    _name = name;
    return *this;
}

auto Options::with_img_dir ( std::string img_dir ) -> Options&
{
    _img_dir = img_dir;
    return *this;
}

auto Options::with_install ( bool install ) -> Options&
{
    _install = install;
    return *this;
}

auto Options::with_copy ( bool copy ) -> Options&
{
    _copy = copy;
    return *this;
}

auto Options::with_recursive_search ( bool recursive ) -> Options&
{
    _recursive_search = recursive;
    return *this;
}

auto Options::with_with_transition ( bool with_transition ) -> Options&
{
    _with_transition = with_transition;
    return *this;
}

auto Options::with_overwrite ( bool overwrite ) -> Options&
{
    _overwrite = overwrite;
    return *this;
}

auto Options::with_duration ( unsigned duration ) -> Options&
{
    _duration = duration;
    return *this;
}

auto Options::with_transition_duration ( unsigned transition_duration ) -> Options&
{
    _transition_duration = transition_duration;
    return *this;
}

auto Options::name () -> std::string
{
    return _name;
}

auto Options::img_dir () -> std::string
{
    return _img_dir;
}

auto Options::install () -> bool
{
    return _install;
}

auto Options::copy () -> bool
{
    return _copy;
}

auto Options::recursive_search () -> bool
{
    return _recursive_search;
}

auto Options::with_transition () -> bool
{
    return _with_transition;
}

auto Options::overwrite () -> bool
{
    return _overwrite;
}

auto Options::duration () -> unsigned
{
    return _duration;
}

auto Options::transition_duration () -> unsigned
{
    return _transition_duration;
}
