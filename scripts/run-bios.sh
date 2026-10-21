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

mkdir -p "$iso_root/boot/grub"
cp "$kernel" "$iso_root/boot/tinosrse"
cp "$project_root/grub.cfg" "$iso_root/boot/grub/grub.cfg"
grub-mkrescue -o "$iso_image" "$iso_root" >/dev/null

exec qemu-system-x86_64 -cdrom "$iso_image" -display "${QEMU_DISPLAY:-gtk}"
