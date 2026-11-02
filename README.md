# Stalin sort — Python / Rust

左から値を読み、最後に残した値以上なら残し、小さければ除外する単純なスターリンソートです。
等しい値は残します。入力を変更せず、新しい配列を返します。

```text
入力: [3, 1, 4, 2, 5, 0, 2, 6]
結果: [3, 4, 5, 6]
```

**入力の全要素を保持するソートではありません。** 要素を保持する多段階版v1・v3は
[thirdthatch/multi-stalin](https://github.com/thirdthatch/multi-stalin) にあります。

## Python

Python 3.10以上。外部パッケージは不要です。

```sh
python3 python/example.py
python3 -m unittest discover -s python -v
```

`python/stalin_sort.py` を使います。
ルートから対話実行する場合は `PYTHONPATH=python python3` で起動してください。

```python
from stalin_sort import stalin_sort

stalin_sort([3, 1, 4, 2, 5])  # [3, 4, 5]
stalin_sort([(2, "a"), (1, "b"), (2, "c")], key=lambda row: row[0])
# [(2, "a"), (2, "c")]
```

入力は反復可能オブジェクトでも構いません。`key` は各要素につき1回評価します。
キーには一貫した全順序が必要です。浮動小数点のNaNキーは `ValueError` にします。

## Rust

Rust 1.82以上、外部クレート不要。

```sh
cargo test --manifest-path rust/Cargo.toml
cargo run --manifest-path rust/Cargo.toml --example basic
```

```rust
use stalin_sort::{stalin_sort, stalin_sort_by};

assert_eq!(stalin_sort(&[3, 1, 4, 2, 5]), vec![3, 4, 5]);
let output = stalin_sort_by(&[1.5_f64, -0.5, 2.5], f64::total_cmp);
assert_eq!(output, vec![1.5, 2.5]);
```

通常APIは `Ord + Clone`、比較関数版は `Clone` を要求します。
浮動小数点は `f64::total_cmp` など、一貫した全順序の比較関数を明示してください。
Rustの `total_cmp` はNaNと符号付きゼロにも順序を定義します。Pythonと同じ数値比較が必要なら、
NaNを除き、`|a, b| a.partial_cmp(b).expect("NaN")` を指定します。

## 計算量と検証

時間 O(n)、結果を含む追加領域 O(n)。空配列・逆順・重複・負数・小数に対応します。
両言語で7要素の全5,040順列と729の重複列を検証します。結果が接頭列の最大値だけを
残した列と一致すること、等値の保持、入力非変更、カスタムキー・比較関数を確認します。

Classic Stalin sort retains a nondecreasing subsequence and discards the rest.
For lossless sorting, see the multi-stage repository linked above.
