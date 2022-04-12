cmake_minimum_required(VERSION 3.10)
project(cs6420_project)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include_directories(include)

FILE(GLOB ALG_CPP src/alg_*.cpp)
add_executable(main src/main.cpp ${ALG_CPP})
add_executable(tester test/tester.cpp ${ALG_CPP})