# faiyels

Faiyels is a source code visualizer that shows the code in all the files of your program in a massive overview kind of like
Sublime Text's minimap so you can see the size and texture of all the files. It uses my [syntect](https://github.com/trishume/syntect) library to do syntax highlighting.
It is written in Rust using Gfx and Conrod. It is currently quite basic and just shows all the files in a directory and lets you zoom around.

It's reasonably fast and can render millions of boxes at 60fps thanks to instancing, and the fact that each character is just 2 polygons.

![Demo GIF](/assets/images/faiyels-demo2.gif)

## Roadmap
- [x] Draw lots of rectangles
- [x] Draw lots of rectangles in the same places as a piece of text
- [x] Draw lots of rectangles in the same places as the text in a file
- [x] Add controls to navigate and zoom around
- [x] Draw many files at once in a useful arrangement
- [x] Colour the rectangles according to syntax highlighting
- [ ] Show actual letters when you zoom in enough
