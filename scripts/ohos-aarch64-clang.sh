#!/bin/sh
NDK="${OHOS_NDK_HOME}"
exec "${NDK}/native/llvm/bin/clang" -target aarch64-linux-ohos --sysroot="${NDK}/native/sysroot" "$@"
