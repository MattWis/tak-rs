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
and the horizontal axis with letters. A move is a location followed by a coded
letter. For the movements, the letter is followed by a list of numbers
describing the amount each piece was moved, from the bottom of the stack to the
top.

F - flat stone laid
S - standing stone laid
C - Capstone laid
R - pile moved right
L - pile moved left
U - pile moved up
D - pile moved down

To summarize a game, both players moves are put on the same line, separated by
a space. It's worth remembering that the moves are pieces laid, so the first
line is weird, since the players are laying stones of the opposite color. As
an example, here's a game I played (poorly) against myself on a 5x5 board. (But
seriously, I haven't even figured out how to use standing stones or my Capstone
effectively.)

a1F e1F
d2F c2F
d3F d4F
d5F c4F
e2F c3R1
c3F b4F
e4F e3F
a4F b3F
d5D1 b1F
c3L1 b2F
a4R1 c3F
e4D1 c2F
d2L1 c5F
e3L12 d3L012

Breaking it down:

a1F e1F
d2F c2F

At this point the board looks like:

5|   |   |   |   |
4|   |   |   |   |
3|   |   |o  |   |
2|   |   |   |x  |
1|o  |   |   |   |x
  a   b   c   d   e

d3F d4F
d5F c4F
e2F c3R1

So that move took a piece from c3, and moved it right 1 space, onto d3.  Now
(with right being the top of piles), the board looks like:

5|   |   |   |x  |
4|   |   |o  |o  |
3|   |   |   |xo |
2|   |   |   |x  |x
1|o  |   |   |   |x
  a   b   c   d   e

c3F b4F
e4F e3F
a4F b3F
d5D1 b1F

With a couple of questionable moves, P1 (x) has gotten himself into a pickle:

5|   |   |   |   |
4|x  |o  |o  |ox |x
3|   |o  |x  |xo |o
2|   |   |   |x  |x
1|o  |o  |   |   |x
  a   b   c   d   e

c3L1 b2F
a4R1 c3F
e4D1 c2F
d2L1 c5F

P1 continuing to not even drop stones, but just try to move around to stay
alive, now makes a really stupid move to lose:

5|   |   |o  |   |
4|   |ox |o  |ox |
3|   |ox |o  |xo |ox
2|   |o  |ox |   |x
1|o  |o  |   |   |x
  a   b   c   d   e

e3L12

Leaving the easy win for P2 with:

d3L012

5|   |o  |o  |   |
4|   |ox |o  |ox |
3|   |oxo|oxo|x  |
2|   |o  |ox |   |x
1|o  |o  |   |   |x
  a   b   c   d   e

