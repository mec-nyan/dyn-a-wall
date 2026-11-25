package main

import (
	"errors"
	"os"
	"path/filepath"
	"strings"
)

const (
	png string = "png"
	jpg string = "jpg"
)

// TODO: Add all suitable image extensions.  For that we need to know what image files are supported
// by Gnome Shell.  I know that both png and jpg files are supported, and optionally large files are
// supported too (i.e. jxg or something like that).  Not quite sure if other formats are supported
// (i.e. svg, gif, etc).
// NICE: Check if it's a valid image.
func is_image(file_name string) bool {
	return strings.HasSuffix(file_name, png) || strings.HasSuffix(file_name, jpg)
}

func get_images(directory string) ([]string, error) {
	files, err := os.ReadDir(directory)
	if err != nil {
		return nil, err
	}

	var images []string

	for _, file := range files {
		file_name := file.Name()
		if is_image(file_name) {
			images = append(images, filepath.Join(directory, file_name))
		}
	}

	if len(images) == 0 {
		return nil, errors.New("No images found.")
	}

	return images, nil
}
