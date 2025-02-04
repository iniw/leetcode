cmake_minimum_required(VERSION 3.30)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(p504)

add_executable(out main.cpp)

set_target_properties(out PROPERTIES CXX_EXTENSIONS OFF)
target_compile_features(out PRIVATE cxx_std_23)
target_compile_options(out PRIVATE -pedantic -Wall -Werror)

