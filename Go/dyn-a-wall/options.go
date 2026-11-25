package main

import (
	"flag"
	"fmt"
)

type Options struct {
	images_directory    string
	image_duration      int
	make_copies         bool
	install             bool
	overwrite           bool
	transition          bool
	transition_duration int
}

// Don't worry about the flag names, they're just temporary.  I'll provide a nice CLI to select the
// options.
var images_directory = flag.String("where", ".", "Images directory.")

var image_duration = flag.Int("time", 600, "Image duration in seconds.")

var make_copies = flag.Bool("copy", false, "Copy images to slide show directory.")

var install = flag.Bool("install", false, "Install the wallpaper.")

var overwrite = flag.Bool("overwrite", false, "Overwrite existing wallpaper with the same name.")

var transition = flag.Bool("transition", false, "Apply transitions between images.")

var transition_duration = flag.Int("transition-time", 2, "Transition duration in seconds.")

var options = Options{}

func init() {

	flag.Parse()

	options.images_directory = *images_directory
	options.image_duration = *image_duration
	options.make_copies = *make_copies
	options.install = *install
	options.overwrite = *overwrite
	options.transition = *transition
	options.transition_duration = *transition_duration
}

func print_options() {
	fmt.Printf("Options:\n")
	fmt.Printf("\t- Images directory   : %s\n", options.images_directory)
	fmt.Printf("\t- Image duration     : %ds\n", options.image_duration)
	fmt.Printf("\t- Copy images        : %t\n", options.make_copies)
	fmt.Printf("\t- Install wallpaper  : %t\n", options.install)
	fmt.Printf("\t- Overwrite          : %t\n", options.overwrite)
	fmt.Printf("\t- With transition    : %t\n", options.transition)
	fmt.Printf("\t- Transition duration: %ds\n", options.transition_duration)
}
