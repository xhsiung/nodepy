set PYTHONHOME=C:\nodepy\python
set PATH=%PYTHONHOME%;%PATH%
set PYO3_PYTHON=C:\nodepy\python\python.exe
set RUSTFLAGS=-L %PYTHONHOME%\libs

node/node.exe test.js
