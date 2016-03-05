@echo off

SET "CPP_DIR_NAME=cpp_src"

SET "CPP_DIR=%CD%\%CPP_DIR_NAME%"
SET "BUILD_TYPE=release"

rem go to the dirertory of cpp for output message of cmake
cd "%CPP_DIR%"
call "%CD%\build_C_lib"
cd ...