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

You can then add cards to your deck by creating text files in the
`my-deck` directory. These files should have the extension `.cards`.

## Format of the flashcard file

Flashcards are kept `.cards` files. Each is a text file that is meant
to be edited by humans, though `mathema` will also edit it when a
`cards` file is first loaded (just to insert a uuid).

The way it is meant to work is like this. First, you assemble a file
that contains the list of words you would like to add to your deck.
The card file is formatted like so:

```
en lesson
gr το μάθημα

# this is a comment
en yes
gr ναι
gr μάλιστα
```

Then execute `mathema add newfile.cards`. This command will load your
words into the database. `mathema add` doesn't always succeed. If it detects words that it
thinks already exist in the database, it will stop and issue warnings
(e.g., duplicates).  You can edit the file and try again (or use `-f`
to force it to continue).

Once it succeeds, `mathema add` will edit `newfile.cards` in place to
add a `uuid` field to each word. It will also add `newfile.cards` to
git version control (presuming that hasn't already been done).

From here one out, `newfile.cards` will stay in place: you can edit
the file to make corrections, and `mathema` will notice them.

In fact, you can even add new words into the file later. In that case,
you need to re-run `mathema add` to bring in the new words (but
mathema will warn you if it finds new words that do not yet have a
uuid assigned).

## Testing and word selection

To be written. Still busy implementing the stuff before. =)


