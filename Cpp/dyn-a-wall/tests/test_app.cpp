#include "../src/app.hpp"
#include <catch2/catch_test_macros.hpp>


TEST_CASE ( "App::run returns zero (0).", "[app::App]" )
{
	int   argc{ 1 };
	char* argv[ 1 ] = { const_cast<char*> ( "./dynawall" ) };

	REQUIRE ( app::run ( argc, argv ) == 0 );
}
