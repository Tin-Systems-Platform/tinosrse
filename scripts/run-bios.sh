#!/bin/sh
set -eu

kernel=$1
project_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
iso_root=$(mktemp -d)
iso_image=$(mktemp)

cleanup() {
    rm -rf "$iso_root" "$iso_image"
}
trap cleanup EXIT INT TERM

mkdir -p "$iso_root/boot/limine"
cp "$kernel" "$iso_root/boot/tinosrse"
cp "$project_root/limine.conf" "$iso_root/boot/limine/limine.conf"
cp /usr/local/share/limine/limine-bios.sys "$iso_root/boot/limine/"
cp /usr/local/share/limine/limine-bios-cd.bin "$iso_root/boot/limine/"
xorriso -as mkisofs \
    -b boot/limine/limine-bios-cd.bin \
    -no-emul-boot \
    -boot-load-size 4 \
    -boot-info-table \
    --protective-msdos-label \
    -o "$iso_image" "$iso_root" >/dev/null

exec qemu-system-x86_64 -cdrom "$iso_image" -display "${QEMU_DISPLAY:-gtk}"
