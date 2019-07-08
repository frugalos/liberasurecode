#! /usr/bin/env bash

set -eux

BUILD_DIR=$PWD
MAKE_FLAGS=""

# Please try and add other distributions.
case "$(uname)" in
    "Linux") MAKE_FLAGS="-j$(nproc)";;
    "Darwin") MAKE_FLAGS="-j$(sysctl -n hw.ncpu)"
esac

#
# gf-complete
#
git clone https://github.com/ceph/gf-complete.git
cd gf-complete/
git checkout a6862d1
./autogen.sh
./configure --disable-shared --with-pic --prefix $BUILD_DIR
make $MAKE_FLAGS install
cd ../

#
# jerasure
#
git clone https://github.com/ceph/jerasure.git
cd jerasure/
git checkout de1739c
autoreconf --force --install
CFLAGS="-I${BUILD_DIR}/include" LDFLAGS="-L${BUILD_DIR}/lib" ./configure --disable-shared --enable-static --with-pic --prefix $BUILD_DIR
make $MAKE_FLAGS install
cd ../

#
# zlib, introduced in liberasurecode 1.5.0 -> 1.6.0
#

git clone https://github.com/madler/zlib.git
cd zlib
git checkout cacf7f1 # tag: v1.2.11
CFLAGS="-I${BUILD_DIR}/include -fPIC" LDFLAGS="-L${BUILD_DIR}/lib" ./configure --prefix $BUILD_DIR
make $MAKE_FLAGS install
cd ../

#
# liberasurecode
#
git clone https://github.com/openstack/liberasurecode.git
cd liberasurecode/
git checkout 1.6.1
./autogen.sh
CFLAGS="-I${BUILD_DIR}/jerasure/include -I${BUILD_DIR}/include"
CFLAGS=$CFLAGS LIBS="-lJerasure -lz" LDFLAGS="-L${BUILD_DIR}/lib" ./configure --disable-shared --with-pic --prefix $BUILD_DIR
patch -p1 < ../liberasurecode.patch # Applies a patch for building static library
make $MAKE_FLAGS install
