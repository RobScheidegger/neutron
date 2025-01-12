FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI += " \
    file://10-wired.network \
"

PACKAGECONFIG:append = " networkd resolved"

FILES:${PN} += " \
    /usr/lib/systemd/network/10-wired.network \
"

do_install:append() {
    install -d ${D}/usr/lib/systemd/network
    install -m 0644 ${UNPACKDIR}/10-wired.network ${D}/usr/lib/systemd/network/
}