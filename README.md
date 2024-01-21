Spellchecker
============
Simple spellchecker algorithms implemented in Rust.

Levenshtein distance algorithm (blazingly slow 🚀)
--------------------------------------------------
Never meant to be used by computers so it's recursive nature is very ineffective and slow.

Wagner-Fisher distance algorithm 
---------------------------------
Much more effective, using matrices.

Usage (Try it for yourself)
===========================
You can run the program and put the sentence, you want to correct, as argument like this

```bash
$ cargo run Helo mom, I luve you!
```
then the program will go word by word and if it detects an unknown word it will printed it out

```bash
Helo mom, I luve you!
^^^^ unknown word, maybe try
  helo -> hell
  helo -> held
  helo -> hero
  helo -> hello
```

Resources
=========
* [video that inspired this](https://www.youtube.com/watch?v=d-Eq6x1yssU&ab_channel=b001)
* Some articles about these algorithms
  * [wiki](https://en.wikipedia.org/wiki/Wagner%E2%80%93Fischer_algorithm#:~:text=The%20Wagner%E2%80%93Fischer%20algorithm%20computes,find%20the%20distance%20between%20the)
  * [pseudocode](https://hyperskill.org/learn/step/35106)