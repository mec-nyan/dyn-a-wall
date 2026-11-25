package main

import "fmt"

// Make the contents of the wallpaper xml file.
// NOTE: Based upon: https://gitlab.gnome.org/GNOME/gnome-backgrounds/-/blob/gnome-41/backgrounds/gnome-backgrounds.xml.in?ref_type=heads

func make_wallpaper_xml(slide_show_file_path string) string {
	return fmt.Sprintf(`<?xml version="1.0"?>
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
}
