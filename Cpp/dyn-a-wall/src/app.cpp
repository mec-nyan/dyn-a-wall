#include "app.hpp"
#include <print>

using namespace app;

auto App::run () -> int
{
    std::println ( "This is gonna be buried in the commit history haha" );

    return 0;
}
