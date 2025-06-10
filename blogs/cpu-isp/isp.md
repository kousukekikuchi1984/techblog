# 知っておきたい CPU の ISP の特性


## Introduction

- 社長 > お前の記事はマニアックすぎる。利用価値のあるものにしてくれ。さもないと減給な。


- という話があったかどうかは定かではありませんが、減給されるくらいならその命令に従わなければなりません。
- コンピューターに関わるシステム全体を理解した上で設計や速度向上に役立てようと考えているため、まずは物理レイヤーから学習しています。
- その中で、max と min の演算に関して一部のエンジニア界隈がざわついた出来事があるので、


## 問題

### 数式による max と min の求め方と演算による max と min の求め方

数学的には x と y は下記のような式でも表すことができます。
``max(x, y) = ``
``min(x, y) = ``

上記の演算を行うのと、if 文で max min を求めるのはどちらのほうが速いでしょうか？

### 速度比較

これらを検証するために、C でこれらの式を計算するコードを記しました。なぜ C にしたのかは、後でアセンブリの確認がしやすいためです。

```c
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

static inline int max_if(int x, int y) {
    return x > y ? x : y;
}

static inline int min_if(int x, int y) {
    return x < y ? x : y;
}

static inline int max_formula(int x, int y) {
    return (x + y + abs(x - y)) / 2;
}

static inline int min_formula(int x, int y) {
    return (x + y - abs(x - y)) / 2;
}

int main(void) {
    const int N = 100000000;
    int sum = 0;
    int *a = malloc(sizeof(int) * N);
    int *b = malloc(sizeof(int) * N);
    if (!a || !b) return 1;
    srand(0);
    for (int i = 0; i < N; i++) {
        a[i] = rand();
        b[i] = rand();
    }
    clock_t start = clock();
    for (int i = 0; i < N; i++) {
        sum += max_if(a[i], b[i]);
        sum += min_if(a[i], b[i]);
    }
    clock_t end = clock();
    printf("if version: %f sec\n", (double)(end - start) / CLOCKS_PER_SEC);

    start = clock();
    for (int i = 0; i < N; i++) {
        sum += max_formula(a[i], b[i]);
        sum += min_formula(a[i], b[i]);
    }
    end = clock();
    printf("formula version: %f sec\n", (double)(end - start) / CLOCKS_PER_SEC);

    printf("ignore: %d\n", sum);
    free(a);
    free(b);
    return 0;
}
```

上記のコードは `max_if` / `min_if` のような if 文で場合分けする実装と、
絶対値を利用した数式ベースの実装をそれぞれ大量に実行して計測しています。

コンパイルと実行は以下の通りです。

```sh
$ gcc -O3 -march=native -o maxmin maxmin.c
$ ./maxmin
if version: 0.11 sec
formula version: 0.10 sec
```

環境によって多少結果は異なりますが、手元の x86_64 環境 (GCC 13) では
if 文よりも数式版の方が **約 10%** ほど速い数値となりました。
コンパイラはどちらのコードも AVX2 の `vpmaxsd` / `vpminsd` 命令へと
ベクトル化しており、分岐のない計算が効率的に実行されています。

## まとめ

- max/min の処理はシンプルな分岐でも十分高速だが、
  分岐を排した数式版はわずかながら有利になることがある。
- 近年のコンパイラは自動でベクトル命令を用いるため、
  CPU 命令レベルではどちらの実装も最適化されている。

