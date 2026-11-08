#include "options.hpp"

namespace options
{
	auto Options::with_name( std::string name ) -> Options&
	{
		_name = name;
		return *this;
	}

	auto Options::with_img_dir( std::string img_dir ) -> Options&
	{
		_img_dir = img_dir;
		return *this;
	}

	auto Options::with_install( bool install ) -> Options&
	{
		_install = install;
		return *this;
	}

	auto Options::with_copy( bool copy ) -> Options&
	{
		_copy = copy;
		return *this;
	}

	auto Options::with_recursive_search( bool recursive ) -> Options&
	{
		_recursive_search = recursive;
		return *this;
	}

	auto Options::with_with_transition( bool with_transition ) -> Options&
	{
		_with_transition = with_transition;
		return *this;
	}

	auto Options::with_overwrite( bool overwrite ) -> Options&
	{
		_overwrite = overwrite;
		return *this;
	}

	auto Options::with_duration( unsigned duration ) -> Options&
	{
		_duration = duration;
		return *this;
	}

	auto Options::with_transition_duration( unsigned transition_duration ) -> Options&
	{
		_transition_duration = transition_duration;
		return *this;
	}

	auto Options::name() const -> std::string
	{
		return _name;
	}

	auto Options::img_dir() const -> std::string
	{
		return _img_dir;
	}

	auto Options::install() const -> bool
	{
		return _install;
	}

	auto Options::copy() const -> bool
	{
		return _copy;
	}

	auto Options::recursive_search() const -> bool
	{
		return _recursive_search;
	}

	auto Options::with_transition() const -> bool
	{
		return _with_transition;
	}

	auto Options::overwrite() const -> bool
	{
		return _overwrite;
	}

	auto Options::duration() const -> unsigned
	{
		return _duration;
	}

	auto Options::transition_duration() const -> unsigned
	{
		return _transition_duration;
	}

	auto operator <<( std::ostream& os, const Options& opts ) -> std::ostream&
	{
		return os << "Options:" << "\n- Name: " << opts.name()
		          << "\n- Images directory: " << opts.img_dir() << "\n- Install: " << std::boolalpha
		          << opts.install() << "\n- Copy: " << opts.copy()
		          << "\n- Recursive: " << opts.recursive_search()
		          << "\n- Overwrite: " << opts.overwrite()
		          << "\n- Image duration: " << opts.duration()
		          << "\n- Transition: " << opts.with_transition()
		          << "\n- Transition duration: " << opts.transition_duration() << "\n";
	}

}  // namespace options
