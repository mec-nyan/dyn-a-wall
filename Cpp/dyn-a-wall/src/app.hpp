#pragma once
#include <string>


namespace app
{

class App
{
  public:
    App ()
    {
    }
    ~App ()
    {
    }

    auto run () -> int;
};

class Options
{
  public:
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
    std::string _name{ "dynawall" };
    std::string _img_dir{ "." };
    bool        _install{ false };
    bool        _copy{ false };
    bool        _recursive_search{ false };
    bool        _with_transition{ false };
    bool        _overwrite{ false };
    unsigned    _duration{ 300 };
    unsigned    _transition_duration{ 2 };
};

}  // namespace app
