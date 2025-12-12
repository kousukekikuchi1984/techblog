## Introduction

- はるか昔に Rust で配列の参照の仕方で速度が大きく異なるという話をしました。
- しかしながら、実はコメントにもいただいた通り、これは言語仕様に大きく依存するため、どのような条件でも速度が向上するとは限らない
- Python は特にこのような Cache Locality の恩恵はあまり起きないですが、これは Python の標準データ構造では cache locality が聞かせられないことに由来しています

## Python で Cache Locality による速度を捨てている

- Python で実測
- 結果をふまえPython のリストは動的配列だが、CCPython の実装においては PyObject への参照が格納されている。
- なぜcache locality 捨てているのか設計思想を紐解く

## Python で Cache Locality を活かす方法

- Numpy
  - 簡単な説明
  - 実測
- PyO3 + Rust
  - 簡単な説明
  - 実測

## まとめ

- python の配列にはオブジェクトへの参照が記載されているので、cache locality は働きづらい
- 無理やり聞かせたい場合は、内部的に C 実装のライブラリや Rust を適宜用いる必要がある。
