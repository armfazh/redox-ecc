# redox-ecc

![Rustico](https://github.com/armfazh/redox-ecc/workflows/Rustico/badge.svg)

The purpose of this library is to provide mathematical operations used in elliptic curves.

**Features**
-   Prime field arithmetic.
-   Short Weierstrass over prime order groups.
-   Montgomery and twisted Edwards curves.

### Warning

This implementation is **not** protected against any kind of attack,
including side-channel attacks. Do not use this code for securing any application.

**Limitations**
-   No specify architecture optimizations.
-   No side-channel protection, see [Warning](#Warning) section.

### License

BSD 3-Clause License
