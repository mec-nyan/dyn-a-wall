#include "app.hpp"
#include <iostream>


namespace app
{
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

		std::cout << _options;

		return 0;
	}

}  // namespace app
