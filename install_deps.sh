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
git clone https://github.com/roentgen/jerasure.git
cd jerasure/
git checkout 87f052e 
autoreconf --force --install
CFLAGS="-I${BUILD_DIR}/include" LDFLAGS="-L${BUILD_DIR}/lib" ./configure --disable-shared --enable-static --with-pic --prefix $BUILD_DIR
make $MAKE_FLAGS install
cd ../

#
# liberasurecode
#
git clone https://github.com/roentgen/liberasurecode.git
cd liberasurecode/
git checkout 8331347
./autogen.sh
CFLAGS="-I${BUILD_DIR}/jerasure/include -I${BUILD_DIR}/include"
if [ "$(uname)" == "Darwin" ]; then
    CFLAGS="$CFLAGS -Wno-error=address-of-packed-member"
fi
CFLAGS=$CFLAGS LIBS="-lJerasure" LDFLAGS="-L${BUILD_DIR}/lib" ./configure --disable-shared --with-pic --prefix $BUILD_DIR
make $MAKE_FLAGS install
