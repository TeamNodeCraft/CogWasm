#!/usr/bin/env bash

set -ex

# The toolchain scripts and their defconfigs live together in this directory;
# resolve the config from the script, not from whatever directory invokes it.
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [ $UID -eq 0 ]; then
    exec su rustbuild -c "$0"
fi

hide_output() {
  set +x
  trap 'echo "ERROR: An error was encountered with the build."; cat /tmp/build.log; exit 1' ERR
  bash -c "while true; do sleep 30; echo \$(date) - building ...; done" &
  PING_LOOP_PID=$!
  "$@" &> /tmp/build.log
  rm /tmp/build.log
  trap - ERR
  kill $PING_LOOP_PID
  set -x
}

mkdir build
cd build
cp "$script_dir"/crosstool.defconfig .config
ct-ng olddefconfig
hide_output ct-ng build
cd ..
rm -rf build
