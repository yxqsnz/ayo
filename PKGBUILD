# Maintainer: Moizes J. Sousa <yxqsnz@gmail.com>

pkgname="ayo"
pkgver=0.1.0
pkgrel=1
pkgdesc="A simple nice daemon"
license=("MIT")
arch=('i686' 'amd64')
source=("git+https://github.com/yxqsnz/ayo.git")
sha256sums=("SKIP")



build() {
  cd "$srcdir/ayo" || exit
  cargo build --release
}

package() {
  mkdir -p $pkgdir/etc/ $pkgdir/usr/bin $pkgdir/etc/systemd/system/
  cd "$srcdir/ayo" || exit
  mv ayo.toml $pkgdir/etc/ayo.toml
  mv examples $pkgdir/etc/ayo.d
  mv target/release/ayo $pkgdir/usr/bin/
  mv ayo.service /etc/systemd/system/
}
