import time

count = 100_000_000
data = 0

start = time.time()
for i in range(count):
    data += 1
end = time.time()
print(f"Time taken: {end - start}")
print(f"Result: {data}")
