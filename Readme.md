# faiyels

Faiyels is a replacement for your text editor's crappy file tree. Some IDE's have good file trees but most text editors have really crappy ones.
Faiyels is designed to be in a window placed by your window manager next to your text editor and synced up with your text editor through some kind of editor
plugin and IPC calls.

**Unfortunately** I haven't actually implemented any of this yet. Currently this repo just runs a hello world window.
This project is currently blocked by the poor GUI libraries in Rust. I'd like to write this in Rust with Qt Quick but `qmlrs` doesn't
support the features I would need. Conrod also seems inadequate. I may solve this by first contributing to `qmlrs` or Conrod.

# Feature Ideas

- Support both tree and column style navigation.
- Be pretty and themable to match your editor
- Support file drag and drop.
- Support file icons.
- Support file operations some editors don't like duplicating files.
- Fully keyboard controllable and fully mouse controllable
- Discoverable keybindings through something like spacemacs/ranger style hints
