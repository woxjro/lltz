__attribute__((always_inline))
int sub_func(int a, int b) {
    int res = 0;
    if (a < 10) {
        res += a + b;
    } else {
        res -= a - b;
    }
    return res;
}

__attribute__((always_inline))
int func(int a, int b) {
    int res = 0;
    if (a < 10) {
        res += sub_func(a, b);
    } else {
        res -= sub_func(a, b);
    }
    return res;
}

//__attribute__((flatten))
int main()
{
    return func(5, 3);
}
