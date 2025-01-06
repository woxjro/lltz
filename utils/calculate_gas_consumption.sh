grep location | grep -oP '(?<=just consumed gas: )[\d\.]+' | awk '{sum += $1} END {print sum}'
