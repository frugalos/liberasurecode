#! /usr/bin/env bash

set -eux

INSTALL_DIR=""

if [ -z $FRUGALOS_DIR ]
then
   echo "Please set the environment FRUGALOS_DIR to install auxiliary artifacts."
   exit 1
fi

INSTALL_DIR=$FRUGALOS_DIR

case $INSTALL_DIR in
    /*)
	echo "We install dependent artifacts into FRUGALOS_DIR=$FRUGALOS_DIR"
	;;
    *)
	echo "You passed a relative path. Please use an absolute path."
	exit 1
	;;
esac

MAKE_FLAGS=""

# Please try and add other distributions.
case "$(uname)" in
    "Linux") MAKE_FLAGS="-j$(nproc)";;
    "Darwin") MAKE_FLAGS="-j$(sysctl -n hw.ncpu)"
esac

BUILD_DIR="build_working_directory"
mkdir $BUILD_DIR

#
# gf-complete
#
git clone https://github.com/ceph/gf-complete.git $BUILD_DIR/gf-complete
cd $BUILD_DIR/gf-complete
git checkout a6862d1
./autogen.sh
./configure --with-pic --prefix $INSTALL_DIR
make $MAKE_FLAGS install
cd ../..

#
# jerasure
#
git clone -b frugalos_dyn https://github.com/frugalos/jerasure.git $BUILD_DIR/jerasure
cd $BUILD_DIR/jerasure
autoreconf --force --install
CFLAGS="-I${INSTALL_DIR}/include" LDFLAGS="-L${INSTALL_DIR}/lib" ./configure --with-pic --prefix $INSTALL_DIR
make $MAKE_FLAGS install
cd ../..

#
# liberasurecode
#
git clone -b frugalos_dyn https://github.com/frugalos/openstack_liberasurecode.git $BUILD_DIR/openstack_liberasurecode
cd $BUILD_DIR/openstack_liberasurecode
./autogen.sh
CFLAGS="-I${INSTALL_DIR}/include -I${INSTALL_DIR}/include/jerasure"
if [ "$(uname)" == "Darwin" ]; then
    CFLAGS="$CFLAGS -Wno-error=address-of-packed-member"
fi
CFLAGS=$CFLAGS LDFLAGS="-L${INSTALL_DIR}/lib" ./configure --with-pic --prefix $INSTALL_DIR
make $MAKE_FLAGS install
