// File operations for dynawall.


#include <filesystem>
#include <vector>
namespace files
{

	class FileOps
	{
	  public:
		// TODO: Should we load everything in the constructor?
		FileOps()
		{
		}

		// TODO: We'll have two file handles.  Should we be explicit on the constructor?
		~FileOps()
		{
		}

	  private:
		std::vector<std::filesystem::path> _images{};
		std::string                        slide_show_name{};
		std::string                        wallpaper_name{};
		// TODO: If we put here the necessary file handles, ~FileOps will automatically manage the
		// resources.
		std::filesystem::directory_entry images_dir{};
	};
}  // namespace files
