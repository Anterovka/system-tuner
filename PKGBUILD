# Maintainer: Your Name <your@email.com>
pkgname=system-tuner
pkgver=0.1.0
pkgrel=1
pkgdesc='GUI tool for Linux system tuning (sysctl, systemd, storage, performance)'
arch=('x86_64')
url='https://github.com/Anterovka/system-tuner'
license=('MIT')
makedepends=('cargo')
source=("$url/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$pkgname-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$pkgname-$pkgver"
  install -Dm755 target/release/system-tuner "$pkgdir/usr/bin/system-tuner"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
