cmake_minimum_required (VERSION 3.9)

## Include libraries ##
	include_directories ("${PROJECT_BINARY_DIR}/../include")
## End of include libraries ##

set(EXECUTABLE_OUTPUT_PATH bin/${CMAKE_BUILD_TYPE})

if (MSVC)
	set(EXECUTABLE_OUTPUT_PATH bin/)
endif (MSVC)

## Add executables ##
	add_subdirectory (src)
	add_subdirectory (test)
## End of adding executables ##