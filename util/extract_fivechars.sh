#!/bin/bash

# Some list of words
WORDS=words.txt

# The current set of five character words
FIVECHARS=fivechars.txt

# The new potential set of five character words
POTENTIALFIVECHARS=potential_fivechars.txt

# Extract only alpha words of length 5
egrep '^[a-z]{5}$' "$WORDS" > tmp_fivechars.txt

# Merge existing word list and new word list
cat fivechars.txt tmp_fivechars.txt | sort | uniq > "$POTENTIALFIVECHARS"
rm tmp_fivechars.txt

# Display the difference between the files
diff "$FIVECHARS" "$POTENTIALFIVECHARS"
