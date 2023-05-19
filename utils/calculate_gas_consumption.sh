tezos-client --mode mockup --base-dir /tmp/mockup \
    run script $1 \
    on storage "$2" and input "$3" --trace-stack 2> /dev/null \
    | grep location | awk 'NR==1; END{print}' \
    | awk '{ print $6 }' \
    | awk 'NR==1 { first=$1 } NR==2 { second=$1; print first - second }'
