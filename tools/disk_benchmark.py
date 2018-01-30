#!/usr/bin/python3

import time

start = time.time()

i = 0
while time.time() < start + 0.005:
    i += 1
    f = open('file{}'.format(i), 'w')
    f.write("zdravim tenhle soubor {}".format(i)*200)
    f.close()
print(i)
exit(i)
