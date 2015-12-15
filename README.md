# tak-rs

Implementation of tak in Rust.

Tak is a game from The Kingkiller Chronicles, which has recently released a set
of beta rules. http://www.cheapass.com/sites/default/files/TAKBetaRules9-9.pdf

This project is being used as the validation and game engine for
[tak-server](https://https://github.com/MattWis/tak-server). An AI may be
added to this package in the future. At some point it may also support
branching games, though I haven't thought about that much yet.

## Writing games of tak

I am using a modified version of [Portable Tak Notation](https://www.reddit.com/r/Tak/comments/3o2omm/tak_game_notation/).

For placement, I add more context to the moves, since I intend to recieve a
move at a time, and need to validate it. (If Fa1 as a first move meant to play
one of his own stones, it would be weird if an opponent's stone, which is the
correct play, showed up.) So I do location + type + player, so a1F1 instead of
Fa1, or just a1.
