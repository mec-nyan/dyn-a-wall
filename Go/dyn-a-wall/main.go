// dyn-a-wall
//
// Create dynamic wallpapers (aka background slide shows) for Gnome Shell.
package main

import (
	"log"
	"os"
)

// Inverse approach ("do it all").  Then extract.  Then test.
// This is a simple program ...
func main() {
	// First try:
	//     - No options.  No command line arguments.
	//     - Don't install.  Instead create the files under "./share/...".
	//     - Don't overwrite.

	// We should abort/exit in the following scenarios:
	//     - There are no images in the given directory (for now ".").
	//     - The wallpaper already exists.
	//     - The slide show description already exists.
	print_options()

	// NOTE: For now I'm using current directory.  To install we'll use
	// "~/.local/share/backgrounds/".
	cwd, err := os.Getwd()
	if err != nil {
		log.Fatal(err)
	}

	images, err := get_images(cwd)
	if err != nil {
		log.Fatal(err)
	}

	err = make_files(cwd, images)
	if err != nil {
		log.Fatal(err)
	}

	// Print success message with information about the files written.
	println("All done!")
}
