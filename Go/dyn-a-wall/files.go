package main

import (
	"os"
	"path/filepath"
)

// Create these files on the corresponding directory (i.e. under "~/.local/" to install the
// wallpaper or under "." to just create the files).
//
// share/
// ├── backgrounds
// │   └── dynawall
// │       └── dynawall_slide_show.xml
// └── gnome-background-properties
//     └── dynawall_wallpaper.xml

// NOTE: On naming convention here.  These names seem a bit "too much" i.e. why did I use
// "slide_show_file_name" and not just "slide_show"?  Well, the thing is, for each file I need these
// things:
//     - Directory name.
//     - Absolute directory name (path).
//     - File name.
//     - Absolute file name (path).
//     - File contents.
//     - Actual file descriptor (to write to).
//
// And we can't reuse "path" because we need the slide show path to be included in the wallpaper
// file.  Hence the detailed names for each of those items.

const (
	shared_dir_name      string = "share"
	wallpaper_dir_name   string = "gnome-background-properties"
	backgrounds_dir_name string = "backgrounds"
	slide_show_dir_name  string = "dynawall" // temporary name.
	wallpaper_file_name  string = "dynawall_wallpaper.xml"
	slide_show_file_name string = "dynawall_slide_show.xml"
)

func make_files(install_dir string, images []string) error {

	base_dir := filepath.Join(install_dir, shared_dir_name)
	var err error

	// Ensure the directories exist or create them.
	wallpaper_dir_path := filepath.Join(base_dir, wallpaper_dir_name)
	err = os.MkdirAll(wallpaper_dir_path, 0755)
	if err != nil {
		return err
	}

	slide_show_dir_path := filepath.Join(base_dir, backgrounds_dir_name, slide_show_dir_name)
	err = os.MkdirAll(slide_show_dir_path, 0755)
	if err != nil {
		return err
	}

	// Make the slide show description file.  It should not exist.
	slide_show_file_path := filepath.Join(slide_show_dir_path, slide_show_file_name)
	slide_show_file, err := os.OpenFile(slide_show_file_path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0644)
	if err != nil {
		return err
	}

	slide_show_xml := make_slide_show_description(images)

	_, err = slide_show_file.WriteString(slide_show_xml)
	if err != nil {
		return err
	}

	if err = slide_show_file.Close(); err != nil {
		return err
	}

	// Make the wallpaper file.  It should not exist.
	// TODO: If the file exist, check if it contains a section to the slide show.  If not, append a
	// new section for this slide show.
	wallpaper_file_path := filepath.Join(wallpaper_dir_path, wallpaper_file_name)
	wallpaper_file, err := os.OpenFile(wallpaper_file_path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0644)
	if err != nil {
		return err
	}

	wallpaper_xml := make_wallpaper_xml(slide_show_file_path)

	_, err = wallpaper_file.WriteString(wallpaper_xml)
	if err != nil {
		return err
	}

	if err = wallpaper_file.Close(); err != nil {
		return err
	}

	return nil
}
