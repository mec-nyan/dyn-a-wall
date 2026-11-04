cmake_minimum_required(VERSION 4.2)

project(Dynawall
	VERSION 0.0.1
	DESCRIPTION "Create dynamic wallpapers (background slide shows) for Gnome Shell."
	LANGUAGES CXX
)

add_executable(dynawall)

target_sources(dynawall
	PRIVATE
		src/options.cpp
		src/app.cpp
		src/main.cpp
)

find_package(Catch2 REQUIRED)

add_executable(tests)

target_sources(tests
	PRIVATE
		tests/test_app.cpp
		src/options.cpp
		src/app.cpp
)

target_link_libraries(tests
	PRIVATE
		Catch2::Catch2WithMain
)

include(CTest)
include(Catch)
catch_discover_tests(tests)
