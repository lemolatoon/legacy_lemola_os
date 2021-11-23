# lemola_os
lemolaOS

# 実行
まずRustの実行環境を用意してください。


次に下記の操作を行います。**１階層下の lemola_os/ 内で走らせてください。**

このコマンドでnightlyを使えるようにしてください。
```
rustup overide set nightly
```  


また、OSから切り離して標準ライブラリをcompileするために、rust-srcを追加します。
```
rustup component add rust-src
```  


次にbootimage作成に用いるbootimageをinstallします。
```
cargo install bootimage
```

bootimageのcompileに必要なllvm-tools-previewを追加します。
```
rustup component add llvm-tools-preview
```  


次のコマンドでkernelをbuildし、bootimageを作ります。
```
cargo bootimage
```  



QEMUで実行する場合には、次のように実行できます。
```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-lemola_os/debug/bootimage-lemola_os.bin
```

QEMUのダウンロードについては[ここ](https://www.qemu.org/download/#linux)を参照してください。Windowsの場合Pathを通す必要があります。




