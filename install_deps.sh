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
git clone https://github.com/frugalos/jerasure.git
cd jerasure/
git checkout c9851639f4830e516e48797832ec30fe82a43e28
autoreconf --force --install
CFLAGS="-I${BUILD_DIR}/include" LDFLAGS="-L${BUILD_DIR}/lib" ./configure --disable-shared --enable-static --with-pic --prefix $BUILD_DIR
make $MAKE_FLAGS install
cd ../

#
# liberasurecode
#
git clone https://github.com/frugalos/openstack_liberasurecode.git
cd openstack_liberasurecode/
git checkout 240333686eeae71ea1b94669242a8a6f4f51e260
./autogen.sh
CFLAGS="-I${BUILD_DIR}/jerasure/include -I${BUILD_DIR}/include"
if [ "$(uname)" == "Darwin" ]; then
    CFLAGS="$CFLAGS -Wno-error=address-of-packed-member"
fi
<<<<<<< HEAD
CFLAGS=$CFLAGS LDFLAGS="-L${BUILD_DIR}/lib" ./configure --disable-shared --with-pic --prefix $BUILD_DIR
=======
CFLAGS=$CFLAGS LIBS="-lJerasure" LDFLAGS="-L${BUILD_DIR}/lib" ./configure --disable-shared --with-pic --prefix $BUILD_DIR
>>>>>>> d318ebdd469e8868cc7300d310cb8c821e75dc4c
make $MAKE_FLAGS install
