#! /usr/bin/env bash

set -eux

BUILD_DIR=$PWD
MAKE_FLAGS=""
export CPATH="${BUILD_DIR}/include:${BUILD_DIR}/include/jerasure:${CPATH:-}"
export LIBRARY_PATH="${BUILD_DIR}/lib:${LIBRARY_PATH:-}"

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
./configure --disable-shared --enable-static --with-pic --prefix $BUILD_DIR
make $MAKE_FLAGS install
cd ../

#
# liberasurecode
#
git clone https://github.com/frugalos/openstack_liberasurecode.git liberasurecode
cd liberasurecode/
git checkout tmp/test-modify-rs-cauchy
if [ "$(uname)" == "Darwin" ]; then
    # if the compiler has the feature to check `address-of-packed-member`, we suppress it.
    # it is only annoying for liberasurecode v1.5.0.
    patch -p1 < ../for_darwin_to_detect_compiler_flag.patch
fi
./autogen.sh
LIBS="-lJerasure" LDFLAGS="-L${BUILD_DIR}/lib" ./configure --disable-shared --with-pic --prefix $BUILD_DIR
patch -p1 < ../liberasurecode.patch # Applies a patch for building static library
make $MAKE_FLAGS install
