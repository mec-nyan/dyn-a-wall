package main

func run(options Options) error {

	images, err := get_images(options.images_directory)
	if err != nil {
		return err
	}

	err = make_files(options.install, images)
	if err != nil {
		return err
	}

	return nil
}
