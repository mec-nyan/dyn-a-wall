#include "../src/app.hpp"
#include <catch2/catch_test_macros.hpp>


TEST_CASE ( "App::run returns zero (0).", "[app::App]" )
{
	REQUIRE ( app::App().run() == 0 );
}
