import os
import time

files = os.listdir(".\\temporary-powershell-scripts")

while True:
    for i in files:
        stats = os.stat(f".\\temporary-powershell-scripts\\{i}")
        delta = time.time() - stats.st_mtime
        if delta > 10_000:
            os.remove(f".\\temporary-powershell-scripts\\{i}")
    time.sleep(1000)