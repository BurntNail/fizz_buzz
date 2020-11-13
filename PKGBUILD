# Maintainer: Jack Maguire <jackmaguire1234@gmail.com>

_pkgname=fizz_buzz
pkgname=${_pkgname}-git
pkgver=1.0.2
pkgdesc='A Simple Fizzbuz'
arch=('i686' 'x86_64')
url="https://github.com/ogham/exa"
license=('MIT')
depends=('libgit2')
makedepends=('rust' 'git')
provides=('fizz_buzz')
conflicts=('fizz_buzz')
source=("git+https://github.com/ogham/exa.git")
md5sums=('SKIP')

pkgver() {
    printf("1.0.2")
}

build() {
    cd $_pkgname
    cargo build --locked --release
}

package() {
    cd $_pkgname
    install -Dm755 "target/release/$_pkgname" \
        -t "$pkgdir/usr/bin"
    install -Dm644 LICEN?E \
        "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
