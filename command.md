消費ガス量測定
```
tezos-client --mode mockup run script ./examples/out/simple_while.tz on storage \
'Unit' and input 'Unit' --trace-stack \
| grep location \
| (head --lines=1; tail --lines=1) \
| awk '{ print $6 }'


tezos-client --mode mockup run script ./examples/out/simple_struct2.tz on storage \
'Unit' and input 'Unit' --trace-stack \
| grep location \
| (head --lines=1; tail --lines=1) \
| awk '{ print $6 }'

```
