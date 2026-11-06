#include "../src/app.hpp"
#include <catch2/catch_test_macros.hpp>


TEST_CASE ( "App::run returns zero (0).", "[app::App]" )
{
	int   argc{ 0 };
	char* argv[] = { 0 };

	REQUIRE ( app::run ( app::to_sv ( argc, argv ) ) == 0 );
}
