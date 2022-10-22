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

```
