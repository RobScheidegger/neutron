set -x
set -e

cd release

kas build kas-neutron.yml

WIC_BZ2_FILE="build/neutron-image.wic.bz2"
WIC_BMAP_FILE="build/neutron-image.wic.bmap"

# Create links if they don't exist
if [ ! -f "$WIC_BZ2_FILE" ]; then
    ln -s $(pwd)/build/tmp/deploy/images/raspberrypi4/neutron-image-raspberrypi4.rootfs.wic.bz2 $(pwd)/build/neutron-image.wic.bz2
fi

if [ ! -f "$WIC_BMAP_FILE" ]; then
    ln -s $(pwd)/build/tmp/deploy/images/raspberrypi4/neutron-image-raspberrypi4.rootfs.wic.bmap $(pwd)/build/neutron-image.wic.bmap
fi