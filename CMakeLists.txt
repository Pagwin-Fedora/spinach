# Note the minimum version is probably higher than the actual minimum it's just the version I have at the time of starting this project
cmake_minimum_required(VERSION 3.24.3)
project(spinach)

add_library(spinach STATIC runtime.c thread.c)
