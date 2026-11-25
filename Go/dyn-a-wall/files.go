package main

import (
	"os"
	"path/filepath"
)

func make_files(cwd string, images []string) error {
	// Make the wallpaper (xml) file.
	// Ensure parent dir exists.
	// Abort if file is already there.
	// NOTE: For now I'm using current directory.  To install we'll use
	// "~/.local/share/backgrounds/".

	base_directory := "share"
	var err error

	properties_directory := filepath.Join(cwd, base_directory, "gnome-background-properties")
	err = os.MkdirAll(properties_directory, 0755)
	if err != nil {
		return err
	}

	// Make the slide show description (xml) file.
	// Ensure parent dir exists.
	// Abort if file is already there.
	// NOTE: "dynawall" is just a temporary/default name.

	wallpaper_directory := filepath.Join(cwd, base_directory, "backgrounds/dynawall")
	err = os.MkdirAll(wallpaper_directory, 0755)
	if err != nil {
		return err
	}

	// Make the content of the slide show description (xml).
	// Abort on errors (unlikely).  Clean up?

	slide_show_xml := make_slide_show_description(images)

	// Save content to file(s).
	// Abort on errors (unlikely).  Clean up?
	slide_show_file_path := filepath.Join(wallpaper_directory, "dynawall_slide_show.xml")
	slide_show_file, err := os.OpenFile(slide_show_file_path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0644)
	if err != nil {
		return err
	}

	_, err = slide_show_file.WriteString(slide_show_xml)
	if err != nil {
		return err
	}

	if err = slide_show_file.Close(); err != nil {
		return err
	}

	// Make the content of the wallpaper (xml).
	// Abort on errors (unlikely).  Clean up?

	wallpaper_xml := make_wallpaper_xml(slide_show_file_path)

	wallpaper_file_path := filepath.Join(properties_directory, "dynawall_wallpaper.xml")
	wallpaper_file, err := os.OpenFile(wallpaper_file_path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0644)
	if err != nil {
		return err
	}

	_, err = wallpaper_file.WriteString(wallpaper_xml)
	if err != nil {
		return err
	}

	if err = wallpaper_file.Close(); err != nil {
		return err
	}

	return nil
}
