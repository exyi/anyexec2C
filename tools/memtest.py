#!/usr/bin/python3

# Part of anyexec2c project
# https://github.com/exyi/anyexec2C/tree/master/tools
#
# Tests memory limits of the sandbox, measured in blocks of BLOCK_SIZE
# Exit code 254 probably means that the limit has not been reached when that many blocks were allocated
# Exit code 255 is some error

import sys, os
import time

BLOCK_SIZE = 1024*1024

def child(pipe_write):
	for aloc in range(0, 254):
		aloc * BLOCK_SIZE * b'*'
		# child still alive - make parent know
		# try until a char is written or run out of time
		while os.write(pipe_write, b'.') != 1:
			pass
	
	return 0

def parent(pipe_read):
	nblocks = 0
	while True:
		c = os.read(pipe_read, 1)
		try:
			if c == b'': # eof
				return nblocks
			if c == b'.': # character read
				nblocks += 1
			pass
		except Exception:
			pass
		# try until a char is read or run out of time

def main():
	try:
		read_fd, write_fd = os.pipe()
		pid = os.fork()
		if pid == 0:
			os.close(read_fd)
			return child(write_fd)
		if pid > 0:
			os.close(write_fd)
			return parent(read_fd)
		return 255
	except Exception:
		return 255

sys.exit(main())
