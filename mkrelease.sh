#!/bin/bash

mkdir -p release
declare -a arcs=("amd64" "386" "arm" "arm64" "mips" "mips64" "mips64le" "mipsle" "ppc64" "ppc64le" "s390x")
for arc in "${arcs[@]}"
do
    echo -n "[*] Packaging linux-$arc binary -> "
    tar czvf "./release/qs-netcat_linux_$arc.tar.gz" "./build/linux/qs-netcat-$arc"
done

declare -a arcs=("amd64" "386" "arm" "arm64")
for arc in "${arcs[@]}"
do
    echo -n "[*] Packaging windows-$arc binary -> "
    tar czvf "./release/qs-netcat_windows_$arc.tar.gz" "./build/windows/qs-netcat-$arc.exe"
done

declare -a arcs=("amd64" "arm64")
for arc in "${arcs[@]}"
do
    echo -n "[*] Packaging darwin-$arc binary -> "
    tar czvf "./release/qs-netcat_darwin_$arc.tar.gz" "./build/darwin/qs-netcat-$arc"
done

declare -a arcs=("amd64" "arm64")
for arc in "${arcs[@]}"
do
    echo -n "[*] Packaging ios-$arc binary -> "
    tar czvf "./release/qs-netcat_ios_$arc.tar.gz" "./build/darwin/qs-netcat-$arc"
done

declare -a arcs=("amd64" "386" "arm" "arm64")
for arc in "${arcs[@]}"
do
    echo -n "[*] Packaging android-$arc binary -> "
    tar czvf "./release/qs-netcat_android_$arc.tar.gz" "./build/linux/qs-netcat-$arc"
done

declare -a arcs=("amd64" "386" "arm" "arm64")
for arc in "${arcs[@]}"
do
    echo -n "[*] Packaging freebsd-$arc binary -> "
    tar czvf "./release/qs-netcat_freebsd_$arc.tar.gz" "./build/freebsd/qs-netcat-$arc"
done

declare -a arcs=("amd64" "arm" "arm64" "mips64")
for arc in "${arcs[@]}"
do
    echo -n "[*] Packaging openbsd-$arc binary -> "
    tar czvf "./release/qs-netcat_openbsd_$arc.tar.gz" "./build/openbsd/qs-netcat-$arc"
done

declare -a arcs=("amd64" "386" "arm" "arm64")
for arc in "${arcs[@]}"
do
    echo -n "[*] Packaging netbsd-$arc binary -> "
    tar czvf "./release/qs-netcat_netbsd_$arc.tar.gz" "./build/netbsd/qs-netcat-$arc"
done

echo -n "[*] Packaging solaris-amd64 binary -> "
tar czvf "./release/qs-netcat_solaris_amd64.tar.gz" "./build/solaris/qs-netcat-amd64"
echo -n "[*] Packaging illumos-amd64 binary -> "
tar czvf "./release/qs-netcat_illumos_amd64.tar.gz" "./build/illumos/qs-netcat-amd64"
echo -n "[*] Packaging dragonfly-amd64 binary -> "
tar czvf "./release/qs-netcat_dragonfly_amd64.tar.gz" "./build/dragonfly/qs-netcat-amd64"
echo -n "[*] Packaging aix-ppc64 binary -> "
tar czvf "./release/qs-netcat_aix_ppc64.tar.gz" "./build/aix/qs-netcat-ppc64"