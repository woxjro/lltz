消費ガス量測定
```
tezos-client --mode mockup run script ./examples/out/simple_while.tz on storage \
'Unit' and input 'Unit' --trace-stack \
| grep location \
| (head --lines=1; tail --lines=1) \
| awk '{ print $6 }'

tezos-client --mode mockup run script ./examples/out/complex_smartcontract.tz on storage 'Pair 1 2 3 4 (Pair 5 6 7)' and input 'Pair 8 9 10 (Pair 11 12 13)' --trace-stack \
| grep location \
| (head --lines=1; tail --lines=1) \
| awk '{ print $6 }'


tezos-client --mode mockup run script ./examples/out/simple_struct2.tz on storage \
'Unit' and input 'Unit' --trace-stack \
| grep location \
| (head --lines=1; tail --lines=1) \
| awk '{ print $6 }'

tezos-client originate contract complex_smartcontract \
    transferring 0 from my_account \
    running ./examples/out/complex_smartcontract.tz \
    --init 'Pair 18 10 5 3 (Pair 2 0 300)' \
    --burn-cap 100

tezos-client transfer 0 from my_account to complex_smartcontract --arg 'Pair 3 6 9 (Pair 1 2 400)'
```
