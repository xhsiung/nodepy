@echo off
REM 請將下面的 PYTHONHOME 路徑修改為您自己的 Python 安裝路徑
set PYTHONHOME=C:\nodepy\python
set PATH=%PYTHONHOME%;%PATH%
set PYO3_PYTHON=C:\nodepy\python\python.exe
set RUSTFLAGS=-L %PYTHONHOME%\libs


cargo build
copy target\debug\nodepy.dll nodepy.node
node test.js

