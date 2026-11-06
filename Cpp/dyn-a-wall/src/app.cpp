/*
We need to:
- Parse command line arguments.

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

#include "app.hpp"
#include <print>

using namespace std::literals::string_literals;

namespace app
{
	// Let's try to do some compile driven development, with a more functional approach.
	auto run ( int argc, char* argv[] ) -> int
	{
		// Parse command line arguments.
		// Set program options.
		options::Options opts;
		if (const auto o = get_cli_opts ( argc, argv ); o.has_value())
		{
			opts = *o;
		}
		else
		{
			std::println ( "Error: {}", o.error() );
			return 1;
		}


		// Check pre-condition: images exist.
		// Make images list.
		std::vector<std::string_view> images{};
		if (const auto img = get_images ( opts.img_dir() ); img.has_value())
		{
			images = img.value();
		}
		else
		{
			std::println ( "No images found in {}.", opts.img_dir() );
			return 1;
		}


		// Make paths and file names.
		std::string install_dir{};
		if (const auto id = get_install_dir ( opts.install() ); id.has_value())
		{
			install_dir = id.value();
		}
		else
		{
			std::println ( "Error: {}", id.error() );
			return 1;
		}

		auto wallpaper_dir_name{ get_wallpaper_dir_name ( install_dir ) };

		auto properties_dir_name{ get_properties_dir_name ( install_dir ) };


		// Check pre-conditions: wallpaper directory, wallpaper, slide show.  Overwrite?
		// TODO.


		// Create xml file content.  Checking previous conditions may have opened/created the
		// necessary files.


		// Create wallpaper directories (if needed) and files.  (These may have already been created
		// when checking pre-conditions.)


		// Copy images?


		// Write files.


		// Success message.


		return 0;
	}

}  // namespace app
