FLAGS=RUSTFLAGS="-C target-feature=+crt-static"
NIGHTLY_FLAGS=RUSTFLAGS="-C target-feature=+crt-static -Zlocation-detail=none"
BUILD=cargo build --release
CROSS=cross build --release
default:
	${BUILD}
nightly:
	${NIGHTLY_FLAGS} ${BUILD}
windows:
	${BUILD} --target x86_64-pc-windows-gnu
	${BUILD} --target i686-pc-windows-gnu
	${BUILD} --target aarch64-pc-windows-msvc
linux:
	${FLAGS} ${BUILD} --target x86_64-unknown-linux-musl
	${FLAGS} ${CROSS} --target i686-unknown-linux-musl
	${FLAGS} ${CROSS} --target aarch64-unknown-linux-musl
	${FLAGS} ${CROSS} --target mips-unknown-linux-gnu
	${FLAGS} ${CROSS} --target mips64-unknown-linux-gnuabi64
	${FLAGS} ${CROSS} --target mips64el-unknown-linux-gnuabi64
# ${BUILD} --target mipsel-unknown-linux-gnu
# ${BUILD} --target powerpc-unknown-linux-gnu
# ${BUILD} --target powerpc64-unknown-linux-gn
# ${BUILD} --target powerpc64le-unknown-linux-gnu
freebsd:
	${FLAGS} ${CROSS} --target x86_64-unknown-freebsd
openbsd:
# ${BUILD} --target x86_64-unknown-openbsd
netbsd:
# ${BUILD} --target x86_64-unknown-netbsd
android:
	${FLAGS} ${CROSS} --target aarch64-linux-android
	${FLAGS} ${CROSS} --target x86_64-linux-android
ios:
# ${BUILD} --target aarch64-apple-ios
# ${BUILD} --target aarch64-apple-ios-sim
darwin:
	${FLAGS} ${BUILD} --target aarch64-apple-darwin
	${FLAGS} ${BUILD} --target x86_64-apple-darwin
solaris:
# ${BUILD} --target x86_64-pc-solaris
illumos:
# ${BUILD} --target x86_64-unknown-illumos
dragonfly:
# ${BUILD} --target x86_64-unknown-dragonfly


all: linux windows darwin freebsd openbsd netbsd solaris aix dragonfly illumos # ${BUILD} android ios 
