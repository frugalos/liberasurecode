#! /usr/bin/env bash

set -eux

DIR=`mktemp -d`
cp *.patch $DIR/
cd $DIR

#
# gf-complete
#
git clone https://github.com/ceph/gf-complete.git
cd gf-complete/
git checkout a6862d1
./autogen.sh
./configure --disable-shared --with-pic
make install
ldconfig
cd ../

#
# jerasure
#
git clone https://github.com/ceph/jerasure.git
cd jerasure/
git checkout de1739c
autoreconf --force --install
./configure --disable-shared --enable-static --with-pic
make install
cd ../

#
# liberasurecode
#
git clone https://github.com/openstack/liberasurecode.git
cd liberasurecode/
git checkout 1.5.0
./autogen.sh
CFLAGS="-I$PWD/../jerasure/include" LIBS="-lJerasure" LDFLAGS="-L/usr/local/lib" ./configure --disable-shared --with-pic
patch -p1 < ../liberasurecode.patch # Applies a patch for building static library
make install
ldconfig
