#pragma once

#include "options.hpp"
#include <expected>
#include <optional>
#include <vector>


namespace app
{
	auto run ( int argc, char* argv[] ) -> int;

	// Parse the command line arguments (if any) and return Options or an Error in case of invalid
	// invocation.  All command line arguments are optional.  Sane defaults are provided.
	auto get_cli_opts ( int argc, char* argv[] ) -> std::expected<options::Options, std::string>;

	// Find images in the given directory.  There can be no images there.
	auto get_images ( std::string_view path ) -> std::optional<std::vector<std::string_view>>;
}  // namespace app
