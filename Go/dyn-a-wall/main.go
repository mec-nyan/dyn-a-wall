// dyn-a-wall
//
// Create dynamic wallpapers (aka background slide shows) for Gnome Shell.
package main

import (
	"log"
)

// Inverse approach ("do it all").  Then extract.  Then test.
// This is a simple program ...
func main() {
	err := run()
	if err != nil {
		log.Fatalf("Oops! -> %s\n", err)
	}

	println("All done!")
}
