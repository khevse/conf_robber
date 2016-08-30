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

SET "PATH_TO_TARGET_PARSE_WITH_SETTINGS=%TARGET_DIR%\target_P_Settings"
SET "PATH_TO_TARGET_BUILD_WITH_SETTINGS=%TARGET_DIR%\target_B_Settings"

for %%f in ("%PATH_TO_TARGET_PARSE%", "%PATH_TO_TARGET_BUILD%", "%PATH_TO_TARGET_PARSE_WITH_SETTINGS%", "%PATH_TO_TARGET_BUILD_WITH_SETTINGS%") do (
    if exist "%%f" (
        rmdir /s /q "%%f"
    )
)

SET "CF=original.cf"

rem Full pack/unpack
rem %PATH_TO_EXE% -P "--cf=%CURRENT_DIR%\test_data\%CF%" "--target=%PATH_TO_TARGET_PARSE%" --log-level=info
rem %PATH_TO_EXE% -B "--dir=%PATH_TO_TARGET_PARSE%\unpack" "--target=%PATH_TO_TARGET_BUILD%" --log-level=info

rem Pack/unpack with settings
%PATH_TO_EXE% -P "--cf=%CURRENT_DIR%\test_data\%CF%" "--target=%PATH_TO_TARGET_PARSE_WITH_SETTINGS%" --log-level=info "--settings=%cd%/settings.xml"
%PATH_TO_EXE% -B "--dir=%PATH_TO_TARGET_PARSE_WITH_SETTINGS%\unpack" "--target=%PATH_TO_TARGET_BUILD_WITH_SETTINGS%" --log-level=info
