@echo off
rem -*- coding:OEM -*-
Setlocal EnableDelayedExpansion

SET "CURRENT_DIR=%CD%"
SET "BUILD_DIR_NAME=build"
SET "BUILD_DIR=%CURRENT_DIR%\build"

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
    del /f /s /q "%CURRENT_DIR%\%%i"
)

if "%1" == "clean" (
    exit
)


mkdir %BUILD_DIR_NAME%
cd %BUILD_DIR_NAME%

cmake -G "MinGW Makefiles" -DCMAKE_BUILD_TYPE=RELEASE ../

mingw32-make install
if "%ERRORLEVEL%" == "0" (
    cd ..
) Else (
    cd ..
    exit 1
)

if not "%1" == "release" (
    if not "%1" == "" (
        exit
    )
)

rem Remove temp files
for %%i in (bin, %BUILD_DIR_NAME%, include, lib, share) do (
     rmdir /s /q "%CURRENT_DIR%\%%i"
)

for %%i in (*.exe, libzlib.dll.a, libzlib.dll) do (
     del /f /s /q "%CURRENT_DIR%\%%i"
)