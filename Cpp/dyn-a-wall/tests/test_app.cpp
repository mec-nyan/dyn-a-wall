#include "../src/app.hpp"
#include <catch2/catch_test_macros.hpp>
#include <format>


TEST_CASE( "App::run returns zero (0).", "[app::App]" )
{
	int   argc{ 1 };
	char* argv[ 1 ] = { const_cast<char*>( "./dynawall" ) };

	REQUIRE( app::run( argc, argv ) == 0 );
}


TEST_CASE( "Make wallpaper XML content.", "[app]" )
{
	std::string path{ "/path/to/my/wallpaper.xml" };

	std::string expected{ std::format( R"(<?xml version="1.0"?>
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

	std::string actual{ app::make_wallpaper( path ) };

	REQUIRE( actual == expected );
}


TEST_CASE( "Make slide show XML content (no transition).", "[app]" )
{
	std::vector<std::string_view> images{
		"/dir/img1.png",
		"/dir/img2.png",
		"/dir/img3.png",
	};

	auto opts{ options::Options() };

	std::string expected{ std::format( R"(<background>
    <starttime>
        <year>2000</year>
        <month>1</month>
        <day>1</day>
        <hour>00</hour>
        <minute>00</minute>
        <second>00</second>
    </starttime>

    <static>
        <duration>{0}.0</duration>
        <file>{1}</file>
    </static>

    <static>
        <duration>{0}.0</duration>
        <file>{2}</file>
    </static>

    <static>
        <duration>{0}.0</duration>
        <file>{3}</file>
    </static>
</background>
)",
	                                   opts.duration(), images.at( 0 ), images.at( 1 ),
	                                   images.at( 2 ) ) };

	std::string actual{ app::make_slide_show( images, opts ) };

	REQUIRE( actual == expected );
}


TEST_CASE( "Make slide show XML content (with transition).", "[app]" )
{
	std::vector<std::string_view> images{
		"/dir/img1.png",
		"/dir/img2.png",
		"/dir/img3.png",
	};

	auto opts{ options::Options().with_transition( true ).with_transition_duration( 10 ) };

	std::string expected{ std::format( R"(<background>
    <starttime>
        <year>2000</year>
        <month>1</month>
        <day>1</day>
        <hour>00</hour>
        <minute>00</minute>
        <second>00</second>
    </starttime>

    <static>
        <duration>{0}.0</duration>
        <file>{1}</file>
    </static>

    <transition type="overlay">
        <duration>{4}.0</duration>
        <from>{1}</from>
        <to>{2}</to>
    </transition>

    <static>
        <duration>{0}.0</duration>
        <file>{2}</file>
    </static>

    <transition type="overlay">
        <duration>{4}.0</duration>
        <from>{2}</from>
        <to>{3}</to>
    </transition>

    <static>
        <duration>{0}.0</duration>
        <file>{3}</file>
    </static>

    <transition type="overlay">
        <duration>{4}.0</duration>
        <from>{3}</from>
        <to>{1}</to>
    </transition>
</background>
)",
	                                   opts.duration(), images.at( 0 ), images.at( 1 ),
	                                   images.at( 2 ), opts.transition_duration() ) };

	std::string actual{ app::make_slide_show( images, opts ) };

	REQUIRE( actual == expected );
}
