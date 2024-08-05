#!/bin/bash

ASNs=()

# 遍历 ASN 数组
for ASN in "${ASNs[@]}"
do
  # 使用 curl 命令获取 IP 范围
  curl -s https://ipinfo.io/AS${ASN} -H "User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:58.0) Gecko/20100101 Firefox/58.0" |   grep -oE '\b[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+/[0-9]+\b' >> ipranges.txt
done

curl https://raw.githubusercontent.com/femueller/cloud-ip-ranges/fa157ef53be42c0d9f916351ae6e93ddfef80276/vultr-all-ip-ranges-geofeed.constant.com.json  |   grep -oE '\b[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+/[0-9]+\b' >> ipranges.txt
