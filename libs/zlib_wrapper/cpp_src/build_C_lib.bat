@echo off
rem -*- coding:OEM -*-
Setlocal EnableDelayedExpansion

rem ----------------------------------------------------
rem The directory with source codes
IF "%CPP_DIR%" == "" (
    SET "CPP_DIR=%1"
)

IF "%CPP_DIR%" == "" (
    SET "CPP_DIR=%CD%"
)

echo CPP_DIR - %CPP_DIR%

rem ----------------------------------------------------
rem The build type
IF "%BUILD_TYPE%" == "" (
    SET "BUILD_TYPE=%2"
)

IF "%BUILD_TYPE%" == "" (
    SET "BUILD_TYPE=debug"
)

echo BUILD_TYPE - %BUILD_TYPE%

rem ----------------------------------------------------
rem The build directory
SET "BUILD_DIR_NAME=build"
SET "BUILD_DIR=%CPP_DIR%\build"

rem 32 or 64
SET "SYS_TYPE=32"

rem ----------------------------------------------------
rem Adding MINGW to the system variable 'PATH'
if "%MINGW_DIR%" == "" (
    SET "MINGW_DIR=C:\c++\mingw-w64\x86_64-5.3.0-win32-seh-rt_v4-rev0\mingw32"
    SET "PATH=%PATH%;%MINGW_DIR%\bin"
)

rem ----------------------------------------------------
rem Remove old files
if exist "%BUILD_DIR%" (
    rmdir /s /q "%BUILD_DIR%"
)

for %%i in (*.exe, *.a, *.dll) do (
    del /f /s /q "%CPP_DIR%\%%i"
)

if "%BUILD_TYPE%" == "clean" (
    exit
)

rem ----------------------------------------------------
rem The build
mkdir "%CPP_DIR%\%BUILD_DIR_NAME%"
cd "%CPP_DIR%\%BUILD_DIR_NAME%"

cmake -G "MinGW Makefiles" -DCMAKE_BUILD_TYPE=RELEASE "%CPP_DIR%"

mingw32-make install
if "%ERRORLEVEL%" == "0" (
    cd ..
) Else (
    cd ..
    exit 1
)

rem ----------------------------------------------------
rem Tests
if exist test_zlibwrapper.exe (
    if exist zlib_wrapper_log.txt (
        DEL zlib_wrapper_log.txt
        DEL zlib_wrapper_log_t.txt
    )

    test_zlibwrapper.exe
)

if not "%BUILD_TYPE%" == "release" (
    if not "%BUILD_TYPE%" == "" (
        exit
    )
)

rem ----------------------------------------------------
rem Remove temp files
for %%i in (bin, %BUILD_DIR_NAME%, include, lib, share) do (
     rmdir /s /q "%CPP_DIR%\%%i"
)

for %%i in (*.exe, libzlib.dll.a, libzlib.dll) do (
     del /f /s /q "%CPP_DIR%\%%i"
)