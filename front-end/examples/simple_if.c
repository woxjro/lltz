typedef int _mutez;
typedef int _int;
typedef int _nat;

_int main()
{
    _int a = 10;
    if (a < 5) {
        a += 100;
    } else {
        a -= 100;
    }
    return a;
}
