#!/bin/bash
export PYTHONHOME=/home/linuxbrew/.linuxbrew/Cellar/python@3.13/3.13.5
export RUSTFLAGS="-L /home/linuxbrew/.linuxbrew/Cellar/python@3.13/3.13.5/lib -lpython3.13"

cargo build
cp target/debug/libnodepy.so nodepy.node
node test.js