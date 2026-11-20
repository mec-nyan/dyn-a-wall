// dyn-a-wall
//
// Create dynamic wallpapers (aka background slide shows) for Gnome Shell.
package main

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

	// List current dir images.  Abort if there is none.

	// Make the wallpaper (xml) file.
	// Ensure parent dir exists.
	// Abort if file is already there.

	// Make the slide show description (xml) file.
	// Ensure parent dir exists.
	// Abort if file is already there.

	// Make the content of the wallpaper (xml).
	// Abort on errors (unlikely).  Clean up?

	// Make the content of the slide show description (xml).
	// Abort on errors (unlikely).  Clean up?

	// Save content to file(s).
	// Abort on errors (unlikely).  Clean up?

	// Print success message with information about the files written.
}
