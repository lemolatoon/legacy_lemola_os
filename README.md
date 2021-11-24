# lemola_os
lemolaOS

# 実行
まずRustの実行環境を用意してください。


次に下記の操作を行います。**１階層上の /lemola_os/lemola_os/ 内で走らせてください。**


### nightlyの導入

このコマンドでnightlyを使えるようにしてください。


```sh
$ rustup override set nightly
```




また、OSから切り離して標準ライブラリをcompileするために、rust-srcを追加します。


```sh
$ rustup component add rust-src
```


### bootimage 作成ツールを cargo に install

次にbootimage作成に用いるbootimageをinstallします。


```sh
$ cargo install bootimage
```


bootimageのcompileに必要なllvm-tools-previewを追加します。


```sh
$ rustup component add llvm-tools-preview
```

### build


次のコマンドでkernelをbuildし、bootimageを作ります。


```sh
$ cargo bootimage
```

### run

QEMUで実行する場合には、次のように実行できます。


```sh
$ qemu-system-x86_64 -drive format=raw,file=target/x86_64-lemola_os/debug/bootimage-lemola_os.bin
```


QEMUのダウンロードについては[ここ](https://www.qemu.org/download/#linux)を参照してください。Windowsの場合Pathを通す必要があります。


```sh
$ cargo run
```
上記のコマンドによってbuildとQEMUの起動を同時に行うこともできます。




