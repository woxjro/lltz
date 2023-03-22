#!/bin/sh

if [ $# -eq 0 ]; then
    echo "Usage: $0 file1 file2 file3 ..."
    exit 1
fi


for file in "$@"; do
    filename=$file
    count=0
    pair_count=0
    unpair_count=0
    car_count=0
    cdr_count=0
    get_count=0
    update_count=0

    while read line; do
        if echo "$line" | grep -qv "^#"; then
            count=$((count+1))
        fi

        if echo "$line" | grep -qv "^#"; then
            pair_count=$(($pair_count + `echo "$line" | grep -o "PAIR" | wc -l`))
            unpair_count=$(($unpair_count + `echo "$line" | grep -o "UNPAIR" | wc -l`))
            car_count=$(($car_count + `echo "$line" | grep -o "CAR" | wc -l`))
            cdr_count=$(($cdr_count + `echo "$line" | grep -o "CDR" | wc -l`))
            get_count=$(($get_count + `echo "$line" | grep -o "GET" | wc -l`))
            update_count=$(($update_count + `echo "$line" | grep -o "UPDATE" | wc -l`))
        fi
    done < "$filename"

echo "filename: $filename"
echo "instruction count: $count"
echo "       PAIR count: $pair_count"
echo "     UNPAIR count: $unpair_count"
echo "        CAR count: $car_count"
echo "        CDR count: $cdr_count"
echo "        GET count: $get_count"
echo "     UPDATE count: $update_count"
done
