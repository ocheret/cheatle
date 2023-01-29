# cheatle utils

An assortmet of scripts for building up the dictionary used for Cheatle.

## Important files

- `fivechars_freq.txt` - This is the full list of five character words
  along with their frequencies. This file should be copied to the base
  directory for each language implementation to keep them as
  self-contained units.

- `fivechars.txt` - This is the full list of five character words in
  the dictionary.

- `words.txt` - The current full list of words that `fivechars.txt` is
  derived from

- `en_frequencies.txt` - List of english words with a frequency metric

## Scripts

- `extract_fivechars.sh` - Takes a new source of words in a file
  `words.txt`, finds all of the alphanumeric five character words,
  merges it with the `fivechars.txt` file to produce
  `potential_fivechars.txt` and displays the diff of `fivechars.txt`
  and `potential_fivechars.txt`. Simply run as:

```
$ ./extract_fiverchars.sh`
```

- `compile_frequencies.py` - Uses `fivechars.txt` and
  `en_frequencies.txt` to produce `fivechars_freq.txt`. Outputs to
  stdout so use redirection like:

```
$ ./compile_frequencies.py > fivechars_freq.txt
```
