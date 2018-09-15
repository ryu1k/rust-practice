
# Rust の勉強メモ


# お試し環境の Setup

2018/8 時点

## 条件
* Debian @ WSL 上にセットアップ
* apt などではなく https://sh.rustup.rs を利用
  * ~/.cargo 以下にインストールされる
* emacs で company + racer を利用

## Setup
```shell
curl https://sh.rustup.rs -sSf | sh
```
これだけで、 rustc, cargo, rustup が使える。

## racer の導入
racer はコード補完用のツール。 vim なり emacs なりから使える。

racer は rustc のバージョンが defalut ではエラーとなり導入できなかったので、nightly を入れた。

```shell
rustup install nightly
rustup default nightly
cargo install racer
rustup default stable
rustup component add rust-src
```

nightly でなく、 beta では NG であった。 @2018-09-12

### 参考とした URL

* https://tydk27.hatenadiary.com/entry/20160713/1468416790
* https://qiita.com/chikoski/items/b6461367e8c3875bb235

## emacs 用設定
