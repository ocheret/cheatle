#!/usr/bin/env python3

import fileinput
import re

dictionary_file = "fivechars.txt"

alphabet = "[abcdefghijklmnopqrstuvwxyz]"

def read_dictionary(dictionary_file = dictionary_file):
    with open(dictionary_file) as file:
        words = [line.rstrip() for line in file]
    return words

def prompt():
    print("cheatle> ", end='', flush=True)

def reset():
    global words
    global remaining_letters
    global required_letters
    words = read_dictionary()
    remaining_letters = [alphabet for i in range(0, 5)]
    required_letters = set()

def print_help():
    print("reset : start a new cheating session")
    print("- <LETTER> : letter not in solution")
    print("-N <LETTER> : letter in solution but not in Nth position")
    print("+N <LETTER> : letter is in Nth position")
    print("list : list remaining solutions")
    print("debug : debug state dump")
    print("help : this messsage")

def remove_letter(pos, letter):
    global remaining_letters
    remaining_letters[pos] = remaining_letters[pos].replace(letter, '')

def fix_letter(pos, letter):
    global remaining_letters
    remaining_letters[pos] = letter
    if letter in required_letters:
        required_letters.remove(letter)

def has_required(word):
    for l in required_letters:
        if l not in word:
            return False
    return True

if __name__ == '__main__':
    global words
    global remaining_letters
    global required_letters
    reset()
    prompt()
    for line in fileinput.input():
        tokens = line.split()
        if len(tokens) == 0:
            prompt()
            continue
        command = tokens[0]
        if command == 'reset':
            reset()
        elif command == 'help':
            print_help()
        elif command == '-':
            if len(tokens) != 2:
                print_help()
                continue
            for pos in range(0, 5):
                remove_letter(pos, tokens[1])
        elif command in {'-1', '-2', '-3', '-4', '-5'}:
            if len(tokens) != 2:
                print_help()
                continue
            pos = ord(command[1]) - ord('1')
            remove_letter(pos, tokens[1])
            required_letters.add(tokens[1])
        elif command in {'+1', '+2', '+3', '+4', '+5'}:
            if len(tokens) != 2:
                print_help()
                continue
            pos = ord(command[1]) - ord('1')
            fix_letter(pos, tokens[1])
        elif command == 'list':
            search_expression = "".join(remaining_letters)
            prog = re.compile(search_expression)
            words = [w for w in words if prog.match(w) and has_required(w)]
            print("\n".join(words))
            print("count:", len(words))
        elif command == 'debug':
            search_expression = "".join(remaining_letters)
            print("Regex:", search_expression)
            print("Required:", required_letters)
            prog = re.compile(search_expression)
            words = [w for w in words if prog.match(w) and has_required(w)]
            print("word count:", len(words))
        else:
            print("Invalid command")
            help()
        prompt()
    print("\nBye")
