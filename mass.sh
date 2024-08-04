masscan -p54321 --max-rate 8000 -oG results.txt --exclude 255.255.255.255 \

grep -E -o '([0-9]{1,3}\.){3}[0-9]{1,3}' results.txt >> ips.txt