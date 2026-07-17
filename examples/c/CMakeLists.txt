# Build and run the C and C++ examples against the wickra-darwin C ABI.
#
#   cargo build --release -p wickra-darwin-c
#   cmake -S examples/c -B examples/c/build
#   cmake --build examples/c/build --config Release
#   ctest --test-dir examples/c/build -C Release --output-on-failure
#
# WICKRA_DARWIN_LIB_DIR defaults to the workspace target/release directory and can
# be overridden for out-of-tree builds.

cmake_minimum_required(VERSION 3.15)
project(wickra_darwin_examples C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(WICKRA_DARWIN_LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../target/release"
    CACHE PATH "Directory containing the built wickra-darwin C ABI library")

set(WICKRA_DARWIN_INCLUDE "${CMAKE_CURRENT_SOURCE_DIR}/../../bindings/c/include")

find_library(WICKRA_DARWIN_LIB
    NAMES wickra_darwin libwickra_darwin
    HINTS "${WICKRA_DARWIN_LIB_DIR}"
    REQUIRED)

add_executable(evolve evolve.c)
target_include_directories(evolve PRIVATE "${WICKRA_DARWIN_INCLUDE}")
target_link_libraries(evolve PRIVATE "${WICKRA_DARWIN_LIB}")

add_executable(evolvepp evolve.cpp)
target_include_directories(evolvepp PRIVATE "${WICKRA_DARWIN_INCLUDE}")
target_link_libraries(evolvepp PRIVATE "${WICKRA_DARWIN_LIB}")

# Both examples build the demo sine universe with sin() from <math.h>. On Linux
# libm is a separate object that ld does not pull in implicitly, so link it
# explicitly. macOS folds libm into libSystem and Windows into the CRT, where
# the linker resolves it without a flag — but linking m there is a harmless
# no-op, so guard only against Windows.
if(NOT WIN32)
    target_link_libraries(evolve PRIVATE m)
    target_link_libraries(evolvepp PRIVATE m)
endif()

# On Windows the runtime DLL must sit next to the executable (there is no rpath);
# without it the loader pops a modal "DLL not found" dialog that hangs ctest. Copy
# it next to each example after build. (Unix resolves the .so/.dylib via the
# linker's embedded path, so this is Windows-only.)
if(WIN32)
    foreach(example evolve evolvepp)
        add_custom_command(TARGET ${example} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                "${WICKRA_DARWIN_LIB_DIR}/wickra_darwin.dll"
                "$<TARGET_FILE_DIR:${example}>"
            VERBATIM)
    endforeach()
endif()

enable_testing()
add_test(NAME evolve COMMAND evolve)
add_test(NAME evolvepp COMMAND evolvepp)
# A missing runtime dependency must fail fast, never block the runner for hours.
set_tests_properties(evolve evolvepp PROPERTIES TIMEOUT 60)
