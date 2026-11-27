package main

import (
	"os"
)

func run(options Options) error {
	// First try:
	//     - No options.  No command line arguments.
	//     - Don't install.  Instead create the files under "./share/...".
	//     - Don't overwrite.

	// We should abort/exit in the following scenarios:
	//     - There are no images in the given directory (for now ".").
	//     - The wallpaper already exists.
	//     - The slide show description already exists.

	// NOTE: For now I'm using the current directory.  To install we'll use
	// "~/.local/share/backgrounds/".
	cwd, err := os.Getwd()
	if err != nil {
		return err
	}

	images, err := get_images(cwd)
	if err != nil {
		return err
	}

	err = make_files(cwd, images)
	if err != nil {
		return err
	}

	return nil
}
