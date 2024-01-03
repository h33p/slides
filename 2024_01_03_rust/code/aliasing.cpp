#include <iostream>
#include <memory>

inline void mat_fma(float *a, const float *b, float c, int len) {
    if ((len % 8) != 0) {
        return;
    }

    for (int i = 0; i < len; i++) {
        a[i] += b[i % 4] * c;
    }
}

extern void noinline_mat_fma(float *a, const float *b, float c, size_t len) {
    mat_fma(a, b, c, len);
}

extern void noinline_mat_fma_restrict(float* __restrict__ a, const float* __restrict__ b, float c, size_t len) {
    mat_fma(a, b, c, len);
}

extern void noinline_mat_fma2(float *a, const float *b, float c, size_t len) {
    if (len != 8) {
        return;
    }

    mat_fma(a, b, c, len);
}

extern void noinline_mat_fma_restrict2(float* __restrict__ a, const float* __restrict__ b, float c, size_t len) {
    if (len != 8) {
        return;
    }

    mat_fma(a, b, c, len);
}

[[clang::optnone]]
extern void use(float *a) {}

void t1() {
    float a[8] = {1, 2, 3, 4, 5, 6, 7, 8};
    noinline_mat_fma2(a, a, 2.0, 8);

    std::cout << "T1:" << std::endl;
    for (int i = 0; i < 8; i++) {
        std::cout << " " << a[i] << std::endl;
    }
}

void t2() {
    float a[8] = {1, 2, 3, 4, 5, 6, 7, 8};
    noinline_mat_fma_restrict2(a, a, 2.0, 8);

    std::cout << "T2:" << std::endl;
    for (int i = 0; i < 8; i++) {
        std::cout << " " << a[i] << std::endl;
    }
}

int main() {
    float a[8] = {1, 2, 3, 4, 5, 6, 7, 8};
    float b[4] = {16, 14, 12, 10};

    // Mark the values as modified
    use(a);
    use(b);

    mat_fma(a, b, 2.0, 8);

    use(a);
    use(b);

    for (int i = 0; i < 8; i++) {
        std::cout << a[i] << " " << b[i] << std::endl;
    }

    t1();
    t2();

    return 0;
}
