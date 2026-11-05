#pragma once

#include "files.hpp"
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
		App ( options::Options& options, files::FileOps& file_ops )
			: _options{ options }, _file_ops{ file_ops }
		{
		}

		~App ()
		{
		}

		auto run () -> int;

	  private:
		options::Options _options{ options::Options() };
		// NOTE: FileOps needs access to the Options object.
		files::FileOps _file_ops{ files::FileOps() };
	};

}  // namespace app
