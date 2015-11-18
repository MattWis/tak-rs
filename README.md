# tak-rs

Implementation of tak in Rust.

Tak is a game from The Kingkiller Chronicles, which has recently released a set
of beta rules. http://www.cheapass.com/sites/default/files/TAKBetaRules9-9.pdf

I'm not entirely sure where this will end up. It will start as a command line
application that will validate moves and declare a winner. From there it may
turn into an AI project, or a nice GUI, or a webapp for two people to play
online. (Or some combination of the three.)

## Writing games of tak

I need a good way to transcribe moves as strings, so I am borrowing from the
way chess games are transcribed. The vertical axis is labelled with numbers,
and the horizontal axis with letters. A turn is a location followed by a coded
letter. For the movements, the letter is followed by a list of numbers
describing the amount each piece was moved, from the bottom of the stack to the
top. For placing stones, the number following denotes whether the stone belongs
to player 1 (whoever goes first) or player 2.

* F - flat stone laid
* S - standing stone laid
* C - Capstone laid
* R - pile moved right
* L - pile moved left
* U - pile moved up
* D - pile moved down

To summarize a game, both players moves are put on the same line, separated by
a space. (This isn't necessary for computers, but it makes it more
human-readable).  It's worth remembering that the moves are pieces laid, so the
first line is weird, since the players are laying stones of the opposite color.
As an example, here's a game I played (poorly) against myself on a 5x5 board.
(But seriously, I haven't even figured out how to use standing stones or my
Capstone effectively.)

* a1F2 e1F1
* d2F1 c3F2
* d3F1 d4F2
* d5F1 c4F2
* e2F1 c3R1
* c3F1 b4F2
* e4F1 e3F2
* a4F1 b3F2
* d5D1 b1F2
* c3L1 b2F2
* a4R1 c3F2
* e4D1 c2F2
* d2L1 c5F2
* e3L12 d3L012

Breaking it down:

* a1F2 e1F1
* d2F1 c3F2

At this point the board looks like:

```
5|  |  |  |  |
4|  |  |  |  |
3|  |  |F2|  |
2|  |  |  |F1|
1|F2|  |  |  |F1
  a  b  c  d  e
```

* d3F1 d4F2
* d5F1 c4F2
* e2F1 c3R1

So that move took a piece from c3, and moved it right 1 space, onto d3.  Now
(with right being the top of piles), the board looks like:

```
5|    |    |    |F1  |
4|    |    |F2  |F2  |
3|    |    |    |F1F2|
2|    |    |    |F1  |F1
1|F2  |    |    |    |F1
  a   b   c   d   e
```

* c3F1 b4F2
* e4F1 e3F2
* a4F1 b3F2
* d5D1 b1F2

With a couple of questionable moves, P1 has gotten himself into a pickle:

```
5|    |    |    |F1  |
4|F1  |F2  |F2  |F2F1|F1
3|    |F2  |F1  |F1F2|F2
2|    |    |    |F1  |F1
1|F2  |    |    |    |F1
  a   b   c   d   e
```

* c3L1 b2F2
* a4R1 c3F2
* e4D1 c2F2
* d2L1 c5F2

P1 continuing to not even drop stones, but just try to move around to stay
alive, now makes a really stupid move to lose:

```
5|    |    |F2  |    |
4|F1  |F2F1|F2  |F2F1|
3|    |F2F1|F2  |F1F2|F2F1
2|    |F2  |F2F1|F1  |F1
1|F2  |F2  |    |    |F1
  a   b   c   d   e
```

* e3L12

```
5|      |      |F2    |      |
4|F1    |F2F1  |F2    |F2F1  |
3|      |F2F1  |F2F1  |F1F2F2|
2|      |F2    |F2F1  |F1    |F1
1|F2    |F2    |      |      |F1
  a      b      c      d      e
```

Leaving the easy win for P2 with:

* d3L012

```
5|      |      |F2    |      |
4|F1    |F2F1  |F2    |F2F1  |
3|      |F2F1F2|F2F1F2|F1    |
2|      |F2    |F2F1  |F1    |F1
1|F2    |F2    |      |      |F1
  a      b      c      d      e
```

Note that the winning path goes b1, b2, b3, c3, c4, c5.
