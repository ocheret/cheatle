#!/usr/bin/env python3

import fileinput
import os
import re

dictionary_file = os.path.join(
    os.path.split(os.path.realpath(os.path.dirname(__file__)))[0],
    "fivechars.txt")

alphabet = "[abcdefghijklmnopqrstuvwxyz]"


class Cheatle:
    """Search a dictionary based on Wordle clues"""

    def __init__(self, filename=dictionary_file):
        self.dictionary_file = filename
        self.reset()

    def reset(self):
        self.read_dictionary(self.dictionary_file)
        self.remaining = [alphabet] * 5
        self.required = {}

    def read_dictionary(self, filename):
        with open(filename) as file:
            self.words = [line.rstrip() for line in file]

    def remove_letter(self, pos, letter):
        self.remaining[pos] = self.remaining[pos].replace(letter, '')

    def not_in_word(self, letter):
        for pos in range(0, 5):
            self.remove_letter(pos, letter)

    def placed_in_word(self, pos, letter):
        self.remaining[pos] = letter

    def misplaced_in_word(self, pos, letter):
        self.remove_letter(pos, letter)
        if letter not in self.required:
            self.required[letter] = 1

    def set_min_occurrences(self, letter, count):
        self.required[letter] = count

    def has_required(self, word):
        for letter, count in self.required.items():
            if count > sum([1 for c in word if c == letter]):
                return False
        return True

    def filter(self):
        search_expression = "".join(self.remaining)
        prog = re.compile(search_expression)
        x = self.words
        x = [w for w in x if prog.match(w) and self.has_required(w)]
        self.words = x

    def get_words(self):
        return self.words


def prompt():
    print("cheatle> ", end='', flush=True)


def print_help():
    print("reset : start a new cheating session")
    print("- <LETTERS> : letters not in solution")
    print("-N <LETTER> : letter in solution but not in Nth position")
    print("+N <LETTER> : letter is in Nth position")
    print(">=N <LETTER>: letter occurs at least N time in word")
    print("list : list remaining solutions")
    print("help : this messsage")


if __name__ == '__main__':
    cheatle = Cheatle()
    prompt()
    for line in fileinput.input():
        line = line.lower()
        tokens = line.split()
        if len(tokens) == 0:
            prompt()
            continue
        command = tokens[0]
        if command == 'reset':
            cheatle.reset()
        elif command == 'help':
            print_help()
        elif command == '-':
            if len(tokens) < 2:
                print_help()
                continue
            for c in tokens[1]:
                cheatle.not_in_word(c)
        elif command in {'-1', '-2', '-3', '-4', '-5'}:
            # XXX - if a letter is misplaced more than 1 time in a single
            # guess then the letter must be repeated at least that many
            # times in the word. What's a good way to capture this here?
            if len(tokens) != 2:
                print_help()
                continue
            pos = ord(command[1]) - ord('1')
            cheatle.misplaced_in_word(pos, tokens[1])
        elif command in {'+1', '+2', '+3', '+4', '+5'}:
            if len(tokens) != 2:
                print_help()
                continue
            pos = ord(command[1]) - ord('1')
            cheatle.placed_in_word(pos, tokens[1])
        elif command in {">=1", ">=2", ">=3", ">=4", ">=5"}:
            if len(tokens) != 2:
                print_help()
                continue
            count = ord(command[2]) - ord('0')
            cheatle.set_min_occurrences(tokens[1], count)
        elif command == 'list':
            cheatle.filter()
            words = cheatle.get_words()
            print("\n".join(words))
            print("count:", len(words))
        else:
            print("Invalid command")
            print_help()
        prompt()
    print("\nBye")
