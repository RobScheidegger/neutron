if [ -z "$1" ]; then
    echo "Usage: $0 /dev/<partition>"
    exit 1
fi

sudo bmaptool copy release/build/neutron-image.wic.bz2 $1