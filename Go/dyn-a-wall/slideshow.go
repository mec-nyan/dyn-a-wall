package main

// Make the slide show description (xml) content.
// NOTE: Based upon: https://gitlab.gnome.org/GNOME/gnome-backgrounds/-/blob/gnome-41/backgrounds/adwaita-timed.xml.in?ref_type=heads

import "fmt"

func make_slide_show_description(images []string) string {
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

	return slide_show_xml
}
