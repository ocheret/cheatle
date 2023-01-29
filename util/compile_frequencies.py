#!/usr/bin/env python3

import os
import re

frequency_file = os.path.join(os.path.realpath(os.path.dirname(__file__)),
    "en_frequencies.txt")

fivechars_file = os.path.join(os.path.realpath(os.path.dirname(__file__)),
    "fivechars.txt")

def read_frequency_file():
    frequencies = {}
    prog = re.compile('^[a-z]{5}$')
    with open(frequency_file) as file:
        for line in file:
            line = line.strip()
            [word, frequency] = line.split()
            if prog.match(word):
                frequencies[word] = frequency
    return frequencies

def read_fivechars_file():
    with open(fivechars_file) as file:
        return [line.rstrip() for line in file]

def main():
    frequencies = read_frequency_file()
    words = read_fivechars_file()
    for word in words:
        frequency = frequencies.get(word, 0)
        print(f"{word},{frequency}")

if __name__ == "__main__":
    main()