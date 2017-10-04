#!/bin/bash

mkdir yoga-tmp
cd yoga-tmp

wget https://github.com/facebook/yoga/archive/1.6.0.tar.gz
tar -xf 1.6.0.tar.gz

cp -af yoga-1.6.0/yoga ../.

cd ..
rm -rf yoga-tmp

bindgen --whitelist-function "^YG.*" yoga/YGNodeList.h -o src/sys.rs
