typedef int _mutez;

int main()
{
    _mutez res = 0;
    for (_mutez i = 0; i < 10; ++i) {
        for (_mutez j = 0; j < 10; ++j) {
            _mutez count = 0;
            while (count < 10) {
                res += 1;
                count += 1;
            }
        }
    }
    return res;
}
