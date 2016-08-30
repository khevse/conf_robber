@echo off
rem -*- coding:OEM -*-

SET "BUILD_TYPE=release"

rem go to the dirertory of cpp for output message of cmake
cmd /c "cd /d ""%cd%/cpp_src"" && build_C_lib.bat"