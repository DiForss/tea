# The Tea Text Editor

## What is it?
Tea is, first and foremost, an extensible and configurable text editor. However,
it also aims to follow in Emacs's and Atom's footsteps and become a good
environment for text-based interfaces.

The early development will be focused on developing the text-editing
capabilities. Along with this, a plugin environment will be implemented.

Tea also aims to be performant. Plugins should not be able to block the UI, and
the editor should feel snappy. The early versions may have performance issues,
but they will be resolved before the first major version.

### Plugins 
The plugins will initially be implemented as shared libraries written in Rust
and leveraging Tea's own code, as well as external calls to shell
scripts. Eventually, this may be extended by implementing support for one (or
more) scripting languages.

For now, plugins will lack access to advanced rendering. This may change,
granting plugins full UI access.
