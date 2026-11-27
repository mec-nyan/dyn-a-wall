package main

import (
	"flag"
	"fmt"
)

type Options struct {
	images_directory    string
	image_duration      int
	copy                bool
	install             bool
	overwrite           bool
	transition          bool
	transition_duration int
	debug               bool
}

// Don't worry about the flag names, they're just temporary.  I'll provide a nice CLI to select the
// options.
var (
	images_directory = flag.String("directory", ".", "Images directory.")

	image_duration = flag.Int("time", 600, "Image duration in seconds.")

	make_copies = flag.Bool("copy", false, "Copy images to slide show directory.")

	install = flag.Bool("install", false, "Install the wallpaper.")

	overwrite = flag.Bool("overwrite", false, "Overwrite existing wallpaper with the same name.")

	transition = flag.Bool("transition", false, "Apply transitions between images.")

	transition_duration = flag.Int("transition-time", 2, "Transition duration in seconds.")

	debug = flag.Bool("debug", false, "Print options values.")
)

func init() {
	flag.Parse()
}

func get_options() Options {
	return Options{
		images_directory:    *images_directory,
		image_duration:      *image_duration,
		copy:                *make_copies,
		install:             *install,
		overwrite:           *overwrite,
		transition:          *transition,
		transition_duration: *transition_duration,
		debug:               *debug,
	}
}

func print_options(options Options) {
	fmt.Printf("Options:\n")
	fmt.Printf("\t- Images directory   : %s\n", options.images_directory)
	fmt.Printf("\t- Image duration     : %ds\n", options.image_duration)
	fmt.Printf("\t- Copy images        : %t\n", options.copy)
	fmt.Printf("\t- Install wallpaper  : %t\n", options.install)
	fmt.Printf("\t- Overwrite          : %t\n", options.overwrite)
	fmt.Printf("\t- With transition    : %t\n", options.transition)
	fmt.Printf("\t- Transition duration: %ds\n", options.transition_duration)
}
