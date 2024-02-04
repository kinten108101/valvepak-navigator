#!/usr/bin/env sh
meson setup --reconfigure build-meson &&
meson install -C build-meson &&
valvepak-navigator
