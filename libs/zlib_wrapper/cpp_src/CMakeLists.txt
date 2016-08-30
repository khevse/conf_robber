cmake_minimum_required(VERSION 2.8.4)

set(PROJECT zlibwrapper)

project(${PROJECT})

set(PROJECT_SOURCE_DIR ${${PROJECT}_SOURCE_DIR})
set(${PROJECT}_BINARY_DIR "${PROJECT_SOURCE_DIR}")
set(CMAKE_INSTALL_PREFIX  "${PROJECT_SOURCE_DIR}")

# Каталог в который будут компилироваться исполняемые файлы проекта
set (EXECUTABLE_OUTPUT_PATH "${PROJECT_SOURCE_DIR}")
# Каталог в который будут компилироваться библиотеки проекта
set (LIBRARY_OUTPUT_PATH "${PROJECT_SOURCE_DIR}")

set (CMAKE_VERBOSE_MAKEFILE ON)
set(SYSTEM WIN$ENV{SYS_TYPE})

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Os -std=c++11 -Wall -Wextra -D${SYSTEM} -pthread -static -static-libgcc -static-libstdc++")

if(MINGW)
   set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -mwindows")
   set(MINGW_DIR "$ENV{MINGW_DIR}")
   set(CMAKE_LIBRARY_PATH ${CMAKE_LIBRARY_PATH} "${MINGW_DIR}/x86_64-w64-mingw32/lib;${CMAKE_LIBRARY_PATH};${LIBRARY_OUTPUT_PATH}")
   set(CMAKE_INCLUDE_PATH ${CMAKE_INCLUDE_PATH} "${CMAKE_INCLUDE_PATH};${MINGW_DIR}/x86_64-w64-mingw32/include")

   include(Platform/Windows-GNU-CXX)
endif(MINGW)

# Расположение заголовочных файлов
INCLUDE_DIRECTORIES(
    .
    zlib
    )

# Добавление подпроекта (указывается имя дирректории)
add_subdirectory(zlib)

# Создание основной библиотеки
add_library(${PROJECT} STATIC zlibwrapper.cpp zlibwrapper.h)

# Тестирование
if ("$ENV{BUILD_TYPE}" MATCHES "debug")
    set (TESTS_SOURCES test.cpp)
    set (TEST test_${PROJECT})
    add_executable (${TEST} ${TESTS_SOURCES})
    target_link_libraries (${TEST} ${PROJECT} zlib)
endif($ENV{BUILD_TYPE} = "debug")