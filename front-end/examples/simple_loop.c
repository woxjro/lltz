typedef int MyInt;
typedef int Mutez;

MyInt main()
{
    Mutez res = 0;
    for (Mutez i = 0; i < 10; ++i) {
        for (Mutez j = 0; j < 10; ++j) {
            Mutez count = 0;
            while (count < 10) {
                res += 1;
                count += 1;
            }
        }
    }
    return res;
}
