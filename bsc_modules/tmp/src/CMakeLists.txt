project(project)

set(EXECUTABLE_OUTPUT_PATH bin/${CMAKE_BUILD_TYPE})

## Add source files ##
	file (GLOB_RECURSE source_files ./*)
## End of adding source files ##

## Remove main.cc files of modules ##
## End of removing main.cc files of modules ##

## Add executables ##
	add_executable (project ${source_files})
## End of adding executables ##