set -x
set -e

cd release

kas build kas-neutron.yml

WIC_BZ2_FILE="build/raspberrypi-image.wic.bz2"
WIC_BMAP_FILE="build/raspberrypi-image.wic.bmap"

# Create links if they don't exist
if [ ! -f "$WIC_BZ2_FILE" ]; then
    ln -s build/tmp/deploy/images/raspberrypi4/core-image-base-raspberrypi4.rootfs.wic.bz2 build/raspberrypi-image.wic.bz2
fi

if [ ! -f "$WIC_BMAP_FILE" ]; then
    ln -s build/tmp/deploy/images/raspberrypi4/core-image-base-raspberrypi4.rootfs.wic.bmap build/raspberrypi-image.wic.bmap
fi