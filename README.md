# cheatle
Cheat at WORDLE and other 5 letter word games

Very simple command line interface. Type help to see the commands.

NOTE: The dictionary here might not be complete. Who knows what
dictionaries these various games use? For example,
/usr/share/dict/words doesn't seem to contain the word 'women'. If you
encounter any words that are missing her feel free to file an issue.

You still have to guess words to discover the presence and potential
locations of letters.

Sample session to guess the unknown word 'while'...

```
$ ./cheatle.py 
cheatle> help
reset : start a new cheating session
- <LETTERS> : letters not in solution
-N <LETTER> : letter in solution but not in Nth position
+N <LETTER> : letter is in Nth position
list : list remaining solutions
help : this messsage
```

Let's say your first guess is 'cheat' and you discover that 'h' is
correctly in the 2nd position, 'e' is in the word but is not in the
3rd position, and that 'c', 'a', and 't' are not in the word. Here's
how you let cheatle know this...

```
cheatle> +2 h
cheatle> -3 e
cheatle> - cat
```

If you were to use the list command now you would see that there are
still 32 words that have not been filtered out...

```
cheatle> list
dhole
phone
phose
phyle
rhine
rhyme
shide
shied
shiel
shier
shies
shine
shire
shive
shode
shoer
shole
shone
shore
shove
shred
shree
shrew
shune
shure
shyer
while
whine
whole
whone
whore
whose
count: 32
```

You probably don't even need to look at the list for a few guesses. It
is not necessary to use the list command if you can easily come up
with some good words to guess.

Let's guess the word 'shine' next. We already know that the 'h' is in
place. We learn that the 'i' is correct in the 3rd position, the 'e'
is correct in the 5th position, and that 's' and 'n' are not in the
word. So you futher refine your search with...

```
cheatle> +3 i
cheatle> +5 e
cheatle> - sn
cheatle> list
while
count: 1
```

You have successfully cheated.

You can use the 'reset' command to start a new cheat without exiting
the program.
