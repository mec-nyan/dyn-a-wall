#pragma once

#include "files.hpp"
#include <string_view>
#include <vector>


namespace app
{
	auto run ( std::vector<std::string_view> args ) -> int;
	auto to_sv ( int argc, char* argv[] ) -> std::vector<std::string_view>;
}  // namespace app
