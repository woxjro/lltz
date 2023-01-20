int main()
{
    int res = 0;
    for (int i = 0; i < 10; ++i) {
        int count = 0;
        while (count < 10) {
            res += 1;
            count += 1;
        }
    }
    return res;
}
