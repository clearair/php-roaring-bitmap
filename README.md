# php-roaring-bitmap

A high-performance PHP extension for compressed bitmap operations, powered by Rust's [roaring-rs](https://github.com/RoaringBitmap/roaring-rs) and [phper](https://github.com/phper-framework/phper).

## Features
- Blazing fast bitmap set operations: union, intersection, difference, symmetric difference
- Efficient storage and querying of large sets of integers
- Easy-to-use PHP OOP API
- Full test coverage (Rust + PHP)

## Build & Install

### Prerequisites
- PHP 7.4+ (with php-dev, phpize, etc)
- Rust (nightly or stable)
- Linux/macOS (Windows not officially supported)

### Build
```sh
cargo build --release
```

### Install
Copy the compiled extension to your PHP extension dir:
```sh
cp target/release/lib_roaring_bitmap.so $(php-config --extension-dir)
```
Add to your `php.ini`:
```
extension=lib_roaring_bitmap.so
```

### Test
Rust unit tests:
```sh
cargo test --release
```
PHP functional tests:
```sh
php -d "extension=target/release/libphp_roaring_bitmap.so" test.php
```

## Usage Example
```php
$rbm = new RoaringBitmap();
$rbm->insert(1);
$rbm->insert(2);
$rbm->insert(3);
echo $rbm->count(); // 3
```

## API
- `insert(int $value): bool`
- `contains(int $value): bool`
- `remove(int $value): bool`
- `count(): int`
- `isEmpty(): bool`
- `min(): ?int`
- `max(): ?int`
- `union(RoaringBitmap $other): void`
- `intersect(RoaringBitmap $other): void`
- `difference(RoaringBitmap $other): void`
- `symmetricDifference(RoaringBitmap $other): void`
- `isSubset(RoaringBitmap $other): bool`
- `isSuperset(RoaringBitmap $other): bool`
- `isDisjoint(RoaringBitmap $other): bool`

## License
MIT
