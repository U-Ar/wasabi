# wasabi

書籍 作って学ぶOSの仕組み1の写経リポジトリ

動作確認済み、第二巻の出版待ち

## 書籍との相違点

- 2025/10時点の最新nightlyで動作するように修正
  - static mutへの制限が厳しくなっていたのでフォントキャッシュもMutex管理に変更
  - 各種clippy警告へ対応

## How to use

```
git clone https://github.com/U-Ar/wasabi.git
cd wasabi
cargo fmt
cargo clippy
cargo test
cargo run
```
