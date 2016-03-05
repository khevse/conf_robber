@echo off
rem -*- coding:OEM -*-
Setlocal EnableDelayedExpansion

IF "%CPP_DIR%" == "" (
    SET "CPP_DIR=%1"
)

IF "%CPP_DIR%" == "" (
    SET "CPP_DIR=%CD%"
)

echo CPP_DIR - %CPP_DIR%

IF "%BUILD_TYPE%" == "" (
    SET "BUILD_TYPE=%2"
)

IF "%BUILD_TYPE%" == "" (
    SET "BUILD_TYPE=debug"
)

echo BUILD_TYPE - %BUILD_TYPE%


SET "BUILD_DIR_NAME=build"
SET "BUILD_DIR=%CPP_DIR%\build"

rem 32 or 64
SET "SYS_TYPE=64"

if "%MINGW_DIR%" == "" (
    SET "MINGW_DIR=C:/C++/mingw-w64-5-2-0/mingw64"
    SET "PATH=%PATH%;%MINGW_DIR%\bin"
)

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

if not "%BUILD_TYPE%" == "release" (
    if not "%BUILD_TYPE%" == "" (
        exit
    )
)

rem Remove temp files
for %%i in (bin, %BUILD_DIR_NAME%, include, lib, share) do (
     rmdir /s /q "%CPP_DIR%\%%i"
)

for %%i in (*.exe, libzlib.dll.a, libzlib.dll) do (
     del /f /s /q "%CPP_DIR%\%%i"
)