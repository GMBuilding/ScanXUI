#!/bin/bash

ASNs=("123")

# 遍历 ASN 数组
for ASN in "${ASNs[@]}"
do
  # 使用 curl 命令获取 IP 范围
  curl -s https://ipinfo.io/AS${ASN} -H "User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:58.0) Gecko/20100101 Firefox/58.0" |   grep -oE '\b[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+/[0-9]+\b' >> ipranges.txt
done

curl https://download.microsoft.com/download/7/1/D/71D86715-5596-4529-9B13-DA13A5DE5B63/ServiceTags_Public_20240729.json |   grep -oE '\b[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+/[0-9]+\b' > ipranges.txt
