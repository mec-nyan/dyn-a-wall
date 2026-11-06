#pragma once

#include "options.hpp"
#include <expected>
#include <filesystem>
#include <optional>
#include <string>
#include <vector>


namespace app
{
	auto run ( int argc, char* argv[] ) -> int;

	// Parse the command line arguments (if any) and return Options or an Error in case of invalid
	// invocation.  All command line arguments are optional.  Sane defaults are provided.
	auto get_cli_opts ( int argc, char* argv[] ) -> std::expected<options::Options, std::string>;

	// Find images in the given directory.  There can be no images there.
	auto get_images ( std::string_view path ) -> std::optional<std::vector<std::string_view>>;

	// Get installation directory.  We should be able to get the user directory.
	auto get_install_dir ( bool install ) -> std::expected<std::string, std::string>;

	// Where to put the slide show.
	auto get_wallpaper_dir_name ( std::string base ) -> std::string;

	// Where to put the wallpaper file.
	auto get_properties_dir_name ( std::string base ) -> std::string;

	// Should return an error?
	auto make_properties_dir ( std::filesystem::path dir );

	// Should return an error?
	auto make_wallpaper_dir ( std::filesystem::path dir, bool install );

	// The next function should not fail.
	auto make_slide_show ( std::vector<std::string_view> images, options::Options opts )
		-> std::string;

	// The next function should not fail.
	auto make_wallpaper ( std::string path ) -> std::string;

}  // namespace app
