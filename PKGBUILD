pkgname=noted
pkgver=1.0.0
pkgrel=1
pkgdesc="NOTED - aesthetic notes, drawing and code editor"
arch=('x86_64')
url="https://github.com/YOURNAME/noted"
license=('MIT')
depends=('webkit2gtk-4.1' 'gtk3')
makedepends=('rust' 'nodejs' 'npm')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')
build() { cd "$pkgname-$pkgver"; npm install; npm run icons; npm run tauri build -- --no-bundle; }
package() { cd "$pkgname-$pkgver"; install -Dm755 src-tauri/target/release/noted "$pkgdir/usr/bin/noted"; }
