#!/bin/bash

TMP=/tmp/yoga-fetch-$(date +%s)

mkdir -p $TMP
cd $TMP

wget https://github.com/facebook/yoga/archive/1.6.0.tar.gz
tar -xf 1.6.0.tar.gz

cd -

mkdir -p lib
mv $TMP/yoga-1.6.0/yoga lib/yoga
# mv $TMP/yoga-1.6.0/YogaKit/Source lib/YogaKit

rm -rf $TMP

bindgen --whitelist-function "^YG.*" lib/yoga/YGNodeList.h -o src/yoga/sys.rs
