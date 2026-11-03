#pragma once
#include <string>


namespace app
{

// The necessary options to create a dynamic wallpaper.  Every option has a valid default value.
class Options
{
  public:
    // Empty constructor with default options.  Since every option is, well, "optional" (hence the
    // name ...) I've provided chaining methods instead of lots of overloaded constructors.
    Options ()
    {
    }

    ~Options ()
    {
    }

    auto with_name ( std::string name ) -> Options&;
    auto with_img_dir ( std::string img_dir ) -> Options&;
    auto with_install ( bool install ) -> Options&;
    auto with_copy ( bool copy ) -> Options&;
    auto with_recursive_search ( bool recursive ) -> Options&;
    auto with_with_transition ( bool with_transition ) -> Options&;
    auto with_overwrite ( bool overwrite ) -> Options&;
    auto with_duration ( unsigned duration ) -> Options&;
    auto with_transition_duration ( unsigned transition_duration ) -> Options&;

    auto name () -> std::string;
    auto img_dir () -> std::string;
    auto install () -> bool;
    auto copy () -> bool;
    auto recursive_search () -> bool;
    auto with_transition () -> bool;
    auto overwrite () -> bool;
    auto duration () -> unsigned;
    auto transition_duration () -> unsigned;

  private:
    // Wallpaper name.
    std::string _name{ "dynawall" };
    // Images directory.
    std::string _img_dir{ "." };
    // Install the wallpaper or create it locally for later use.
    bool _install{ false };
    // Copy the images to the wallpaper directory?
    bool _copy{ false };
    // Search recursively for images in the given directory?
    bool _recursive_search{ false };
    // Apply transitions between images?
    bool _with_transition{ false };
    // Overwrite if the wallpaper already exists?
    bool _overwrite{ false };
    // Duration of each image.
    unsigned _duration{ 300 };
    // Duration of the transition
    unsigned _transition_duration{ 2 };
};


// This will run the whole thing.
class App
{
  public:
    // Default constructor uses default options for everything.
    App ()
    {
    }

    // Optionally pass an options object.  Options can not be modified later on.
    App ( Options& options ) : _options{ options }
    {
    }

    ~App ()
    {
    }

    auto run () -> int;

  private:
    Options _options{ Options() };
};

}  // namespace app
