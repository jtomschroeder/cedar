#!/bin/bash

TMP=/tmp/cef-fetch-$(date +%s)
ARC=/tmp/cef-archive-$(date +%s)

mkdir -p $TMP && cd $TMP

# CEF=cef_binary_3.3163.1671.g700dc25_macosx64
CEF=cef_binary_3.3163.1671.g700dc25_macosx64_minimal

wget http://opensource.spotify.com/cefbuilds/$CEF.tar.bz2
tar -xf $CEF.tar.bz2

cd -

mkdir -p $ARC/{lib,etc}

cp -a $TMP/$CEF/Release/'Chromium Embedded Framework.framework' $ARC/lib/.

cp -a lib/app/mac $ARC/etc/.
cp -a lib/etc/*   $ARC/etc/.

tar -czf archive-mac.tar.gz -C $ARC .

rm -rf $TMP $ARC
