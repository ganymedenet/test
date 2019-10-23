import time

start_time = time.time()

i = 1
while i < 990000000:
  c = 999*999
  i += 1

elapsed_time = time.time() - start_time

print(elapsed_time)
