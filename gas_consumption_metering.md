# 消費ガス量の測定に使うコマンド

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


- `simple_contract_and_operation` : ここに送金すると`couter`コントラクトへとoperationが飛ぶ
    ```
    simple_contract_and_operation: KT1QTP4uTnUENe7bzoUuL5g7paJnYJTLEFN6
    counter: KT1Vh2yUNseYabMc1c9EKiBbtQxbyoRWAFDv
    https://better-call.dev/ghostnet/KT1QTP4uTnUENe7bzoUuL5g7paJnYJTLEFN6/operations
    https://better-call.dev/ghostnet/KT1Vh2yUNseYabMc1c9EKiBbtQxbyoRWAFDv/storage
    ```
