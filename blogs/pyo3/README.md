PyO3のオーバーヘッドはどこまで無視できるのか？実測ベンチマークで検証してみた

▶テーマ

"Rust は Python より速い" これはもはやデファクトです。

じゃあ、Pythonで遅い処理を Rust + PyO3 にしたら速くなるのか？

実はここに、大きな落とし穴があるんです。

PyO3の FFI 呼び出しのオーバーヘッド、マジで気にした方がいい。

▶ベンチ設計

同じアルゴリズムで3通りの実装を作成し、実行時間を比較します。

① Python単体実装: Pythonの経緯性

② Rust単体バイナリ: Rustの理論最速値

③ Python + Rust (PyO3): 実用として最も使われるパターン

▶コード

Python

def py_sum(n):
    return sum(i for i in range(n) if i % 3 == 0)

Rust (standalone)

fn main() {
    let n = 10_000_000;
    let sum: u64 = (0..n).filter(|x| x % 3 == 0).sum();
    println!("{}", sum);
}

PyO3 経由 Rust

#[pyfunction]
fn rs_sum(n: u64) -> u64 {
    (0..n).filter(|x| x % 3 == 0).sum()
}

#[pymodule]
fn fastsum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rs_sum, m)?)?;
    Ok(())
}

▶実行結果 (例)

実装

時間

備考

Python

85ms

基準

Rust

5ms

理論値。とにかく速い

PyO3

7ms

Rust+FFI。オーバーヘッドは約2ms

▶ オーバーヘッドの実証

PyO3 overhead ≈ PyO3時間 - Rust理論時間
              ≈ 7ms - 5ms = 2ms

Rust側の処理が 1ms 以上ある なら PyO3 のオーバーヘッドは気にしなくてよい

0.1ms 以下の処理なら退化することも考慮

▶ まとめ

PyO3 は「手近なデータを Rust でも速くしたい」に答えられる強力な選択肢

1ms以上の処理には、PyO3 のオーバーヘッドは気にする必要なし

Rust化の備考に「カスト」だけではなく、処理量と時間を入れよう

次回予告："Pandas の groupby を Rust + PyO3 にしたら驚異の速度になった話"

あなたの Python、まだ本気出してないだけかもしれません。


