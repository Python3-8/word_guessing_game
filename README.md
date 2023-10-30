# Word guessing game

## Idea
About a week ago I started learning Rust on YouTube while my friend was learning Python. He would often ask me for project ideas so he could put his new Python skills to the test, and similarly I asked him for a Rust project idea. He told me to make a word guessing game, and the project was born. 

## Development
At first I thought I didn't have the necessary skillset, which was true. However, I did end up making the game the same way I learned Python— trying, and Googling when I failed. The first version only took a few hours to code, and the next morning I [posted my code on Reddit](https://www.reddit.com/r/rust/comments/17hsc79/i_just_wrote_my_first_rust_project_do_you_have/?utm_source=share&utm_medium=web2x&context=3) asking for tips to improve it. I got many responses, and implemented the commenters' suggestions along with a new feature (I added hints on every tenth guess, and therefore also a way to lose when you run out of hints).

## The arduous part
Now that the project was done, I had to compile for macOS (the operating system I'm using), Windows (for the vast majority of people, including the friend who suggested the project), and Linux (for chads like my dad lol). It was of course very simple to compile for macOS, but it took over an entire day to figure out how to and cross-compile for Windows and Linux. I've [documented the painstaking process here](https://www.reddit.com/r/rust/comments/17iiyad/how_do_i_cross_compile_on_macos/?utm_source=share&utm_medium=web2x&context=3) for future reference. The next morning I followed the solution I'd written for Windows, and in a matter of minutes, cross-compiled for Linux.

## Install
Go to [the latest release](https://github.com/Python3-8/word_guessing_game/releases/latest) and download the appropriate executable for your operating system. They are all 64-bit, and if you're using a 32-bit machine, maybe it's time to upgrade bruh.

## Conclusion
This project may not be much, but it's important to realize that this is my first project, and I'm very proud of it.
