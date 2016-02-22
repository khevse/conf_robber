@echo off

SET "BUILD_TYPE=%1"
if not "%BUILD_TYPE%" == "release" (
    SET "BUILD_TYPE=debug"
)

SET "CURRENT_DIR=%CD%\.."
SET "TARGET_DIR=%CURRENT_DIR%\target\%BUILD_TYPE%"

SET "PATH_TO_EXE=%TARGET_DIR%\conf_robber.exe"
SET "PATH_TO_TARGET_PARSE=%TARGET_DIR%\target_P"
SET "PATH_TO_TARGET_BUILD=%TARGET_DIR%\target_B"

if exist "%PATH_TO_TARGET_PARSE%" (
    rmdir /s /q "%PATH_TO_TARGET_PARSE%"
)

if exist "%PATH_TO_TARGET_BUILD%" (
    rmdir /s /q "%PATH_TO_TARGET_BUILD%"
)

%PATH_TO_EXE% -P "--cf=%CURRENT_DIR%\test_data\original.cf" "--target=%PATH_TO_TARGET_PARSE%" --log-level=info
%PATH_TO_EXE% -B "--dir=%PATH_TO_TARGET_PARSE%\unpack" "--target=%PATH_TO_TARGET_BUILD%" --log-level=info