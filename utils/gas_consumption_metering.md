# 消費ガス量の測定に使うコマンド

```
$ octez-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/tstz/sha256.tz on storage '0x' and input '777' --trace-stack | sh ./utils/calculate_gas_consumption.sh
30.653
```

```
$ octez-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/tstz/boomerang.tz on storage 'Unit' and input 'Unit' --trace-stack | sh ./utils/calculate_gas_consumption.sh
69.169
```

```
$ octez-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/tstz/check_sig.tz on storage 'Pair "edsigu3QszDjUpeqYqbvhyRxMpVFamEnvm9FYnt7YiiNt9nmjYfh8ZTbsybZ5WnBkhA7zfHsRVyuTnRsGLR6fNHt1Up1FxgyRtF" "hello"' and input '"edpkuBknW28nW72KG6RoHtYW7p12T6GKc7nAbwYX5m8Wd9sDVC9yav"' --trace-stack | sh ./utils/calculate_gas_consumption.sh
208.813
```

```
$ octez-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/pytz/sha256.tz on storage '0x' and input '777' --trace-stack | sh ./utils/calculate_gas_consumption.sh
30.653
```

```
$ octez-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/pytz/boomerang.tz on storage 'Unit' and input 'Unit' --trace-stack | sh ./utils/calculate_gas_consumption.sh
69.169
```

```
$ octez-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/pytz/check_sig.tz on storage 'Pair "edsigu3QszDjUpeqYqbvhyRxMpVFamEnvm9FYnt7YiiNt9nmjYfh8ZTbsybZ5WnBkhA7zfHsRVyuTnRsGLR6fNHt1Up1FxgyRtF" "hello"' and input '"edpkuBknW28nW72KG6RoHtYW7p12T6GKc7nAbwYX5m8Wd9sDVC9yav"' --trace-stack | sh ./utils/calculate_gas_consumption.sh
208.813
```

- 階乗計算のコントラクト（`simple_while`）

  ```
  tezos-client --mode mockup run script ./examples/out/simple_while.tz on storage \
  'Unit' and input 'Unit' --trace-stack \
  | grep location \
  | (head --lines=1; tail --lines=1) \
  | awk '{ print $6 }'
  ```

- 釣果を記録するコントラクト（`complex_smartcontract`）
  ```
  tezos-client --mode mockup run script ./examples/out/complex_smartcontract.tz on storage 'Pair 1 2 3 4 (Pair 5 6 7)' and input 'Pair 8 9 10 (Pair 11 12 13)' --trace-stack \
  | grep location \
  | (head --lines=1; tail --lines=1) \
  | awk '{ print $6 }'
  ```

```
tezos-client --mode mockup run script ./examples/out/simple_struct2.tz on storage \
'Unit' and input 'Unit' --trace-stack \
| grep location \
| (head --lines=1; tail --lines=1) \
| awk '{ print $6 }'
```

```
tezos-client originate contract complex_smartcontract \
    transferring 0 from my_account \
    running ./examples/out/complex_smartcontract.tz \
    --init 'Pair 18 10 5 3 (Pair 2 0 300)' \
    --burn-cap 100
```

```
tezos-client transfer 0 from my_account to complex_smartcontract --arg 'Pair 3 6 9 (Pair 1 2 400)'

https://better-call.dev/ghostnet/KT1Vh2yUNseYabMc1c9EKiBbtQxbyoRWAFDv/operations

tezos-client originate contract simple_contract_and_operation \
    transferring 200 from my_account \
    running ./examples/out/simple_contract_and_operation.tz \
    --init 'Unit' \
    --burn-cap 100

tezos-client transfer 0 from my_account to simple_contract_and_operation --arg 'Unit'

```

- `simple_contract_and_operation` : ここに送金すると`couter`コントラクトへと operation が飛ぶ
  ```
  simple_contract_and_operation: KT1QTP4uTnUENe7bzoUuL5g7paJnYJTLEFN6
  counter: KT1Vh2yUNseYabMc1c9EKiBbtQxbyoRWAFDv
  https://better-call.dev/ghostnet/KT1QTP4uTnUENe7bzoUuL5g7paJnYJTLEFN6/operations
  https://better-call.dev/ghostnet/KT1Vh2yUNseYabMc1c9EKiBbtQxbyoRWAFDv/storage
  ```
