typedef int _mutez;
typedef int _int;
typedef int _nat;

int main()
{
    _mutez res = 0;
    for (_int i = 0; i < 10; ++i) {
        for (_int j = 0; j < 10; ++j) {
            _nat count = 0;
            while (count < 10) {
                res += 1;
                count += 1;
            }
        }

        for (_int j = 0; j < 10; ++j) {
        }
    }
    return res;
}
