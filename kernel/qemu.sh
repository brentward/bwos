#!/bin/sh

qemu-system-aarch64 \
    -nographic \
    -M raspi3 \
    -serial null -serial mon:stdio \
    -kernel \
    "$@"
