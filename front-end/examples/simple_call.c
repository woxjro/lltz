typedef int _mutez;
typedef int _int;
typedef int _nat;

__attribute__((always_inline))
_int sub_func(_int a, _int b) {
    _int res = 0;
    if (a < 10) {
        res += a + b;
    } else {
        res -= a - b;
    }
    return res;
}

__attribute__((always_inline))
_int func(_int a, _int b) {
    _int res = 0;
    if (a < 10) {
        res += sub_func(a, b);
    } else {
        res -= sub_func(a, b);
    }
    return res;
}

//__attribute__((flatten))
_int main()
{
    return func(5, 3);
}
