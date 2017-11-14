#!/usr/bin/python3

import argparse
from sys import argv, stdin
from os import listdir
from os.path import isdir
from re import search

def gen_check(filename):
    print("#if __has_include(\"" + filename + "\")")
    print("#error y " + filename)
    print("#else")
    print("#error n " + filename)
    print("#endif")

def gen_check_recursive(name):
        if isdir(name):
            for item in listdir(name):
                gen_check_recursive(name.rstrip("/") + "/" + item)
        else:
            gen_check(name)


def generate(file):
    inp = stdin
    if file is not None:
        inp = open(file, 'r')

    for line in inp:
        gen_check_recursive(line[:-1])

def parse(file):
    inp = stdin
    if file is not None:
        inp = open(file, 'r')

    for line in inp:
        m = search("^ #error ([yn]) (.+)$", line)
        if m is not None:
            if m.group(1) == "y":
                print(m.group(2))

def main():
    parser = argparse.ArgumentParser(description='Tool for checking existence of files in ReCodEx')
    parser.add_argument('command', choices=['generate', 'parse'], help='select action')
    parser.add_argument('-f', '--file', help='Optional input file. If not provided, reads from stdin')
    args = parser.parse_args()

    if args.command == 'generate':
        generate(args.file)
    elif args.command == 'parse':
        parse(args.file)

main()