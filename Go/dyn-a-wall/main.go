// dyn-a-wall
//
// Create dynamic wallpapers (aka background slide shows) for Gnome Shell.
package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

const (
	png string = "png"
	jpg string = "jpg"
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

	// List current dir images.  Abort if there is none.
	// NOTE: "." is just temporary/default.
	files, err := os.ReadDir(".")
	if err != nil {
		log.Fatal(err)
	}

	cwd, err := os.Getwd()
	if err != nil {
		log.Fatal(err)
	}

	var images []string

	for _, file := range files {
		file_name := file.Name()
		if is_image(file_name) {
			images = append(images, cwd+"/"+file_name)
		}
	}

	if len(images) == 0 {
		println("No images found.")
		os.Exit(0)
	}

	fmt.Printf("I've found %d images!\n", len(images))

	for i, img := range images {
		fmt.Printf("\t%2d - %s\n", i+1, img)
	}

	// Make the wallpaper (xml) file.
	// Ensure parent dir exists.
	// Abort if file is already there.
	err = os.MkdirAll("share/backgrounds/gnome-background-properties", 0755)
	if err != nil {
		log.Fatal(err)
	}

	// Make the slide show description (xml) file.
	// Ensure parent dir exists.
	// Abort if file is already there.
	// NOTE: "dynawall" is just a temporary/default name.
	err = os.MkdirAll("share/backgrounds/dynawall", 0755)
	if err != nil {
		log.Fatal(err)
	}

	// Make the content of the slide show description (xml).
	// Abort on errors (unlikely).  Clean up?
	// NOTE: Based on https://gitlab.gnome.org/GNOME/gnome-backgrounds/-/blob/gnome-41/backgrounds/adwaita-timed.xml.in?ref_type=heads
	slide_show_xml := `<background>
	<starttime>
		<year>2001</year>
		<month>01</month>
		<day>01</day>
		<hour>00</hour>
		<minute>00</minute>
		<second>00</second>
	</starttime>`

	// We'll add transitions and other options later.
	for _, img := range images {
		slide_show_xml += fmt.Sprintf(`

	<static>
		<duration>300.0</duration>
		<file>%s</file>
	</static>`, img)
	}

	slide_show_xml += "\n</background>\n"

	// Save content to file(s).
	// Abort on errors (unlikely).  Clean up?
	slide_show_file_path := "share/backgrounds/dynawall/dynawall_slide_show.xml"
	slide_show_file, err := os.OpenFile(slide_show_file_path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0644)
	if err != nil {
		log.Fatal(err)
	}

	_, err = slide_show_file.WriteString(slide_show_xml)
	if err != nil {
		log.Fatal(err)
	}

	if err = slide_show_file.Close(); err != nil {
		log.Fatal(err)
	}

	// Make the content of the wallpaper (xml).
	// Abort on errors (unlikely).  Clean up?

	wallpaper_xml := fmt.Sprintf(`<?xml version="1.0"?>
<!DOCTYPE wallpapers SYSTEM "gnome-wp-list.dtd">
<wallpapers>
	<wallpaper deleted="false">
		<name>Dynawall</name>
		<filename>%s</filename>
		<options>zoom</options>
		<pcolor>#ffffff</pcolor>
		<scolor>#000000</scolor>
	</wallpaper>
</wallpapers>
`, slide_show_file_path)

	wallpaper_file_path := "share/backgrounds/gnome-background-properties/dynawall_wallpaper.xml"
	wallpaper_file, err := os.OpenFile(wallpaper_file_path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0644)
	if err != nil {
		log.Fatal(err)
	}

	_, err = wallpaper_file.WriteString(wallpaper_xml)
	if err != nil {
		log.Fatal(err)
	}

	if err = wallpaper_file.Close(); err != nil {
		log.Fatal(err)
	}

	// Print success message with information about the files written.
	println("All done!")
}

// TODO: Add all suitable image extensions.
// NICE: Check if is a valid image.
func is_image(file_name string) bool {
	return strings.HasSuffix(file_name, png) || strings.HasSuffix(file_name, jpg)
}
