import random
import string
import time
import shutil
import os

i = 0
while True:
	i += 1
	filename = f".\\files\\evil_file_{i}.exe"
	print(f"Making {filename}")
	with open(filename, "w") as outfile:
		b = ""
		for _ in range(10000):
			b += random.choice(string.ascii_lowercase)
		outfile.write(b)
	time.sleep(0.1)
	shutil.move(f".\\files\\evil_file_{i}.exe", f".\\moved-files\\evil_file_{i}.exe")
	if i % 10 == 0:
		files = list(filter(lambda x: "evil" in x, os.listdir(".\\moved-files")))
		for k in files[:-5]:
			os.remove(f".\\moved-files\{k}")
			time.sleep(0.05)

