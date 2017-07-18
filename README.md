[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Build Status](https://travis-ci.org/zovt/tea.svg?branch=master)](https://travis-ci.org/zovt/tea)

# The Tea Text Editor
Tea aims to be an extensible, fast, modern text editor.

Also, more information can be found at
[hack.withtea.party](https://hack.withtea.party).

## Features
- Efficient, modal- and mode-based text editing
- Define-your-own text objects
- Responsive (not in the web way, in the performance way), modern, customizeable
  user interface
- Easily configurable via text files
- Easy interoperability with external tools via both key-based triggers and
  text-based interfaces (using asynchronous processing when available)
- Ease-of-use features including autocompletion, documentation support, and
  semantic highlighting.
- Fine-grained syntax highlighting control

### No plugins?
Maybe. For right now, the plan is to simply use Tea macros and external tools,
but if that proves insufficient, a plugin system will be designed.

### Other UIs?
Maybe. For right now, Tea will be focused on creating an efficient UI toolkit
based on OpenGL. However, if there is a need to decouple the core from the UI
this will be done in a way that allows for different UI implementations.

## Contributing
Please see [CONTRIBUTING.md](./CONTRIBUTING.md) in the root of this directory.

## Prior art
Here are some projects that have heavily influenced Tea:
- Emacs
- Magit
- Vim
- Kakoune
- Sublime Text 3

## Licensing
The Tea source code and crates that are related directly to creating any
Tea-related binaries are licensed under GPLv3. Any Tea source code and crates
that are intended for or able to be used in public consumption for other
projects are licensed under MPLv2.

TODO fixup licenses and whatnot

## Contributors
Please see [CONTRIBUTORS.md](./CONTRIBUTORS.md) in the root of this directory.
