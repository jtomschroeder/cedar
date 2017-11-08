#!/bin/bash

TMP=/tmp/cef-fetch-$(date +%s)

mkdir -p $TMP
cd $TMP

CEF=cef_binary_3.3163.1671.g700dc25_macosx64
# CEF=cef_binary_3.3163.1671.g700dc25_macosx64_minimal

wget http://opensource.spotify.com/cefbuilds/$CEF.tar.bz2
tar -xf $CEF.tar.bz2

cd -

mkdir -p lib
mv $TMP/$CEF lib/cef

rm -rf $TMP
