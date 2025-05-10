# php-roaring-bitmap

一个高性能的 PHP 位图扩展，基于 Rust 的 [roaring-rs](https://github.com/RoaringBitmap/roaring-rs) 和 [phper](https://github.com/phper-framework/phper)。

## 特性
- 超高速的集合操作：并集、交集、差集、对称差集
- 高效存储和查询大规模整数集合
- 简单易用的 PHP 面向对象 API
- 完善的测试覆盖（Rust + PHP）

## 构建与安装

### 依赖
- PHP 7.4+（需 php-dev、phpize 等）
- Rust（nightly 或 stable）
- Linux/macOS（Windows 暂不支持）

### 构建
```sh
cargo build --release
```

### 安装
将编译好的扩展复制到 PHP 扩展目录：
```sh
cp target/release/lib_roaring_bitmap.so $(php-config --extension-dir)
```
在 `php.ini` 里添加：
```
extension=lib_roaring_bitmap.so
```

### 测试
Rust 单元测试：
```sh
cargo test --release
```
PHP 功能测试：
```sh
php -d "extension=target/release/libphp_roaring_bitmap.so" test.php
```

## 使用示例
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

## 许可协议
MIT
