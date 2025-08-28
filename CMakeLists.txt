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

# https://leetcode.com/problems/base-7/
problem(504)

# https://leetcode.com/problems/reverse-string-ii/
problem(541)

# https://leetcode.com/problems/longest-happy-prefix/
problem(1392)

# https://leetcode.com/problems/minimize-xor/
problem(2429)

# https://leetcode.com/problems/number-of-even-and-odd-bits/
problem(2595)

# https://leetcode.com/problems/make-string-a-subsequence-using-cyclic-increments/
problem(2825)

# https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-i/
problem(2873)
