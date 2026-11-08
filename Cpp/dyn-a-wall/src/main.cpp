#include "app.hpp"
#include <print>


auto main( int argc, char* argv[] ) -> int
{
	try
	{
		app::run( argc, argv );
	}
	catch ( app::Error e )
	{
		if ( e == app::Error::not_implemented )
		{
			std::println( "Not implemented." );
		}
		return 1;
	}
	catch ( ... )
	{
		return 42;
	}

	return 0;
}
