#!/bin/bash

mkdir -p release

## Linux 
upx -q --best ./target/x86_64-unknown-linux-musl/release/qs-lite
echo -n "[*] Packaging x86_64-unknown-linux-musl binary -> "
tar czvf "./release/qs-netcat_linux_amd64.tar.gz" "./target/x86_64-unknown-linux-musl/release/qs-lite"

upx -q --best ./target/i686-unknown-linux-musl/release/qs-lite
echo -n "[*] Packaging i686-linux-musl binary -> "
tar czvf "./release/qs-netcat_linux_386.tar.gz" "./target/i686-unknown-linux-musl/release/qs-lite"

upx -q --best ./target/aarch64-unknown-linux-musl/release/qs-lite
echo -n "[*] Packaging aarch64-unknown-linux-musl binary -> "
tar czvf "./release/qs-netcat_linux_arm64.tar.gz" "./target/aarch64-unknown-linux-musl/release/qs-lite"

## Windows
upx -q --best ./target/x86_64-pc-windows-gnu/release/qs-lite.exe
echo -n "[*] Packaging x86_64-pc-windows-gnu binary -> "
tar czvf "./release/qs-netcat_windows_amd64.tar.gz" "./target/x86_64-pc-windows-gnu/release/qs-lite.exe"

upx -q --best ./target/i686-pc-windows-gnu/release/qs-lite.exe
echo -n "[*] Packaging i686-pc-windows-gnu binary -> "
tar czvf "./release/qs-netcat_windows_386.tar.gz" "./target/i686-pc-windows-gnu/release/qs-lite.exe"

## Android
upx -q --best ./target/aarch64-linux-android/release/qs-lite
echo -n "[*] Packaging aarch64-linux-android binary -> "
tar czvf "./release/qs-netcat_android_arm64.tar.gz" "./target/aarch64-linux-android/release/qs-lite"

upx -q --best ./target/x86_64-linux-android/release/qs-lite
echo -n "[*] Packaging x86_64-linux-android binary -> "
tar czvf "./release/qs-netcat_android_amd64.tar.gz" "./target/x86_64-linux-android/release/qs-lite"

## Darwin
upx -q --best ./target/x86_64-apple-darwin/release/qs-lite
echo -n "[*] Packaging x86_64-apple-darwin binary -> "
tar czvf "./release/x86_64-apple-darwin.tar.gz" "./target/x86_64-apple-darwin/release/qs-lite"

