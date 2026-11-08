#include "../src/app.hpp"
#include <catch2/catch_test_macros.hpp>
#include <format>


TEST_CASE ( "App::run returns zero (0).", "[app::App]" )
{
	int   argc{ 1 };
	char* argv[ 1 ] = { const_cast<char*> ( "./dynawall" ) };

	REQUIRE ( app::run ( argc, argv ) == 0 );
}

TEST_CASE ( "Make wallpaper xml content.", "[app]" )
{
	std::string path{ "/path/to/my/wallpaper.xml" };

	std::string expected{ std::format ( R"(<?xml version="1.0"?>
<!DOCTYPE wallpapers SYSTEM "gnome-wp-list.dtd">
<wallpapers>
    <wallpaper deleted="false">
        <name>my awesome wallpaper timed</name>
        <filename>{}</filename>
        <options>zoom</options>
        <pcolor>#ffffff</pcolor>
        <scolor>#000000</scolor>
    </wallpaper>
</wallpapers>)",
	                                    path ) };

	std::string actual{ app::make_wallpaper ( path ) };

	REQUIRE ( actual == expected );
}

TEST_CASE ( "Make slide show xml content (no transition).", "[app]" )
{
	std::vector<std::string_view> images{
		"/dir/img1.png",
		"/dir/img2.png",
		"/dir/img3.png",
	};

	auto opts{ options::Options() };

	std::string expected{ std::format ( R"(<background>
    <starttime>
        <year>2000</year>
        <month>1</month>
        <day>1</day>
        <hour>00</hour>
        <minute>00</minute>
        <second>00</second>
    </starttime>

    <static>
        <duration>{0}</duration>
        <file>{1}</file>
    </static>

    <static>
        <duration>{0}</duration>
        <file>{2}</file>
    </static>

    <static>
        <duration>{0}</duration>
        <file>{3}</file>
    </static>
</background>
)",
	                                    opts.duration(), images.at ( 0 ), images.at ( 1 ),
	                                    images.at ( 2 ) ) };

	std::string actual{ app::make_slide_show ( images, opts ) };

	REQUIRE ( actual == expected );
}
