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

▶実行結果 (10回計測した平均)

実装 | 時間 | 備考
--- | --- | ---
Python | 約7秒 | 純粋なPython実装
Rust | 約0.18秒 | バイナリを毎回起動
PyO3 | 約0.11秒 | モジュール初期化後は高速

▶ 初期化と再利用

PyO3モジュールは一度読み込めばそのまま関数呼び出しを再利用できます。初回呼び出し時のコストはほぼゼロで、継続して使うことでバイナリ起動よりも高速に処理できます。

逆に、処理ごとにRustバイナリを起動する方式はプロセス生成の分だけ遅くなります。簡単にRust化できるものの、数ミリ秒レベルの処理では起動コストが無視できません。

▶ オーバーヘッドの考察

PyO3 overhead ≈ PyO3時間 - Rust理論時間
              ≈ 0.11s - 0.05s 程度

Rust側の処理が 1ms 以上あるなら PyO3 のオーバーヘッドは気にしなくてよいでしょう。
0.1ms 以下の処理なら退化することも考慮します。

▶ まとめ

PyO3 は「手近なデータを Rust でも速くしたい」に答えられる強力な選択肢です。ただし銀の弾丸ではなく、初期化コストや呼び出し頻度を理解したうえで設計する必要があります。

Rust化の備考に「カスト」だけではなく、処理量と時間を入れよう。
次回予告："Pandas の groupby を Rust + PyO3 にしたら驚異の速度になった話"
あなたの Python、まだ本気出してないだけかもしれません。
