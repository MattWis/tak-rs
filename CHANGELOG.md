# Changelog

This project attempts to follow semantic versioning.

### UNRELEASED

### 0.5.2

- [added] Include PieceCounts in board struct

### 0.5.1

- [fixed] Removed possibility of killing Turn parser with bad input

### 0.5.0

- [changed] allow full shortening of basic slides, ala "a1+"
- [changed] when placing a piece, conform to PTN
- [added] full PTN compliance

### 0.4.0

- [changed] when making a sliding turn, string should conform to PTN

### 0.3.0

- [changed] game.play() now takes a string parameter and a player, for which player is making the move

### 0.2.3

- [added] All types now implement RustcDecodable, RustcEncodable
