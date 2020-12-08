#!/usr/bin/python3

TEST_LENGTH = 100

import time

start = time.time()

i = 0
while time.time() < start + 0.005*TEST_LENGTH:
    i += 1
    f = open('file{}'.format(i), 'w')
    f.write("random garbage alskfjsladkj and a happy number {}".format(i)*200)
    f.close()
print(i)
exit(i//TEST_LENGTH)
