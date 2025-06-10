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
