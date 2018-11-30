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
./configure --with-pic --prefix $BUILD_DIR
make $MAKE_FLAGS install
cd ../

#
# jerasure
#
git clone -b frugalos_dyn https://github.com/frugalos/jerasure.git
cd jerasure/
autoreconf --force --install
CFLAGS="-I${BUILD_DIR}/include" LDFLAGS="-L${BUILD_DIR}/lib" ./configure --with-pic --prefix $BUILD_DIR
make $MAKE_FLAGS install
cd ../

#
# liberasurecode
#
git clone -b frugalos_dyn https://github.com/frugalos/openstack_liberasurecode.git
cd openstack_liberasurecode/
./autogen.sh
CFLAGS="-I${BUILD_DIR}/jerasure/include -I${BUILD_DIR}/include"
if [ "$(uname)" == "Darwin" ]; then
    CFLAGS="$CFLAGS -Wno-error=address-of-packed-member"
fi
CFLAGS=$CFLAGS LDFLAGS="-L${BUILD_DIR}/lib" ./configure --with-pic --prefix $BUILD_DIR
make $MAKE_FLAGS install
