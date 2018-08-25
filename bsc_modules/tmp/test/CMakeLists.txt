project(test_project)

set(EXECUTABLE_OUTPUT_PATH bin/${CMAKE_BUILD_TYPE})

## Add source files ##
	file (GLOB_RECURSE testing_files ./*)
	file (GLOB_RECURSE testing_source_files ../src/*)
## End of adding source files ##

## Remove main.cc files of modules ##
	FOREACH(item ${testing_source_files})
		IF(${item} MATCHES "main.cc")
			LIST(REMOVE_ITEM testing_source_files ${item})
		ENDIF(${item} MATCHES "main.cc")
	ENDFOREACH(item)
## End of removing main.cc files of modules ##

## Add executables ##
	add_executable (test_project ${testing_files} ${testing_source_files})
## End of adding executables ##