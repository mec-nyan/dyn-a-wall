#include "app.hpp"


auto main ( int argc, char* argv[] ) -> int
{
	return app::run ( app::to_sv ( argc, argv ) );
}
