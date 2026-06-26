#!/usr/bin/env python3
"""Print words from words.txt whose letters are exactly those given on the command line."""

import os
import sys

WORDS = os.path.join(os.path.dirname(os.path.abspath(__file__)), "words.txt")


def main():
    letters = set("".join(sys.argv[1:]).lower())
    with open(WORDS) as f:
        for line in f:
            word = line.strip()
            if letters == set(word.lower()):
                print(word)


if __name__ == "__main__":
    main()
