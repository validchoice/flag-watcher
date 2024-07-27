@echo off
title installer

setlocal enabledelayedexpansion
set x=
for /f %%e in ('echo prompt $E ^| cmd') do set "x=%%e"

set red=!x![31m
set green=!x![32m
set reset=!x![0m
set dir1=%localappdata%\flag watcher
set dir=%userprofile%\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Flag Watcher

:: Check if the application directory already exists
if not exist "!dir1!" (
    goto confirm
)

:confirm
cls
echo It seems like you are using Flag Watcher for the first time. Flag Watcher will be installed in !green!%localappdata%\flag watcher!reset!
echo.
echo Click !green!Enter!reset! to continue
set /p x=

:: Proceed to installation if Enter is pressed
if "!x!" equ "" (
    goto install
)

:install
cls
echo Installing...
:: Create the application directory and move the executable file
mkdir "!dir1!" & move "%~dp0\flag watcher.exe" "!dir1!" >nul

:: Create the Start Menu directory if it does not exist
if not exist "!dir!" mkdir "!dir!"

set "target=%localappdata%\flag watcher\flag watcher.exe"
set "shortcut=%dir%\Flag Watcher.lnk"

:: Create a VBScript file to generate a shortcut
echo Set objShell = WScript.CreateObject("WScript.Shell") > "%temp%\CreateShortcut.vbs"
echo Set objShortCut = objShell.CreateShortcut("%shortcut%") >> "%temp%\CreateShortcut.vbs"
echo objShortCut.TargetPath = "%target%" >> "%temp%\CreateShortcut.vbs"
echo objShortCut.WorkingDirectory = "%target%" >> "%temp%\CreateShortcut.vbs"
echo objShortCut.WindowStyle = 1 >> "%temp%\CreateShortcut.vbs"
echo objShortCut.Save >> "%temp%\CreateShortcut.vbs"

:: Execute the VBScript to create the shortcut
cscript "%temp%\CreateShortcut.vbs" >nul 2>&1
del "%temp%\CreateShortcut.vbs"

echo Installation completed.
echo.
echo Flag Watcher will appear in your Start menu. To check a flag, click on the Windows icon, type "Flag Watcher," and it will be there!
timeout /t 6 /nobreak >nul & exit