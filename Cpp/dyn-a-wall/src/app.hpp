#pragma once

#include "options.hpp"


namespace app
{


// This will run the whole thing.
class App
{
  public:
    // Default constructor uses default options for everything.
    App ()
    {
    }

    // Optionally pass an options object.  Options can not be modified later on.
    App ( options::Options& options ) : _options{ options }
    {
    }

    ~App ()
    {
    }

    auto run () -> int;

  private:
    options::Options _options{ options::Options() };
};

}  // namespace app
