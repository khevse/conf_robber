@echo off
rem -*- coding:OEM -*-

if exist "%cd%/target" (
    rmdir /s /q "%cd%/target"
)

cmd /c "cd /d ""%cd%/libs/zlib_wrapper"" && build.bat"
cargo build --release