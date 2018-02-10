**mathema** (or, more properly, **μάθημα**) is a CLI for flashcards.
It's not really meant to be used by anyone other than its author, but
maybe you will enjoy it.

## Using mathema

To start, you need to create a new deck of cards. I suggest you manage
it with git so you don't lose information -- and in fact mathema will
help you with that, doing commits at regular times and pushing those
commits to the remote named `mathema`.

To create a deck of cards, you do:

```
mathema new my-deck
```

This will create a directory `my-deck` with a file containing
mathema's internal data:

```
my-deck/mathema.data
```

You can then add cards to your deck by doing

## Format of the flashcard file

Flashcards are kept in a pair of files that always have related names.
One is edited by humans (well, and the tool) and one exclusively by
the tool itself. The human edited file is formatted like so:

```
en lesson
gr το μάθημα

# this is a comment
en yes
gr ναι
gr μάλιστα
```

As you can see, each flashcard is separated by a blank line, and may have
many lines of data. The first word on each line identifies what kind of line
it is. The current lines are currently defined:

- `#` -- a comment line. This is ignored, but must nonetheless be attached to a word.
- `en` -- English meaning for a word
- `gr` -- Greek meaning for a word
- `uuid` -- the UUID for a word; this will be auto-inserted by mathema
  when your file is loaded (and then written back out). This will be used to
  track your words uniquely.

The idea is that you can add words to the file and then run `mathema load file`.
It will 
  
  
mathema new 
