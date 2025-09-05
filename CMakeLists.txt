cmake_minimum_required(VERSION 3.30)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(leetcode LANGUAGES CXX)

function(problem id)
  add_executable(${id} problems/${id}.cpp)

  set_target_properties(${id} PROPERTIES CXX_EXTENSIONS OFF)
  target_compile_features(${id} PRIVATE cxx_std_23)
  target_compile_options(${id} PRIVATE -pedantic -Wextra -Wall -Werror)

  # Debug-only sanitizers.
  target_compile_options(${id} PRIVATE $<$<CONFIG:Debug>:-fno-omit-frame-pointer;-fsanitize=undefined;-fsanitize=address>)
  target_link_options(${id} PRIVATE $<$<CONFIG:Debug>:-fno-omit-frame-pointer;-fsanitize=undefined;-fsanitize=address>)
endfunction()

# Base 7
problem(504)

# Linked List Cycle
problem(141)

# Reverse String II
problem(541)

# Longest Happy Prefix
problem(1392)

# Minimize Xor
problem(2429)

# Number Of Even And Odd Bits
problem(2595)

# Make String A Subsequence Using Cyclic Increments
problem(2825)

# Maximum Value Of An Ordered Triplet I
problem(2873)
