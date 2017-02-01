<h1 align="center">
   <a href="https://github.com/xtensis-editor/xtensis-core">
    <img src="media/logos/PNG/xtensis-256.png" alt="Xtensis Editor" width="256" height="256"/>
    </a><br>
    <a href="https://github.com/xtensis-editor/xtensis-core">Xtensis Editor</a>
</h1>

<h4 align="center">Extendable editing: ignore the rules.</h4>

[![Build Status][jenkins-ci-status]][jenkins-ci]

# Xtensis

## History

I started work on Xtensis when I got tired of the current tooling for
text editing. I felt a new contenter was necessary that took
inspiration from everything else.

(Xtensis was previously known as Slacs. It was changed to prevent
ambiguity with Slack.)

Xtensis is currently being rewritten to be faster, more stable, and to
be in line with Rust idioms.

## Core goals

### Extendable

Xtensis is intended to be as extendable as possible for the
end-user. This is similar to GNU Emacs, but without the bloat and
unnecessary features. Xtensis can have plugins that extend it whilst
keeping the core subsystem small and lean.

### Minimal

Xtensis is designed to be minimal, and keep code in core to a minimum.

For example, GNU Emacs has the frontend and backend mixed together in
an monolithic style. Xtensis is the opposite- the core runs in the
background, and frontends can be attached to the core; the core does
_not_ care what frontend it is, as long as the communication between
the frontend & the core is consistent and conforming to the standards.

This means that you could use a terminal-based frontend, Qt, Gtk, or,
if you _really_ wanted to- an Android frontend!

### Fast

Xtensis _should_ not hang. Asynchronous processing is **vital** in
Xtensis, and it will make use of queues, and background processing.

## Documentation for xtensis

You can read the documentation (gracefully hosted by ReadTheDocs.io)
for Xtensis [here][rtd-xtensis].

Other documentation can be found in the `docs/` folder in the root of
this repo.

## Current status

At the time of writing this, Xtensis is in the process of being
rewritten from scratch.

## Licensing

Xtensis is licensed under the GPL 3.0.

See [here][GPL-3] for the included license file,
*or* [here][GPL-3-EXT], for the GNU project's web page on the license.

## Contributing to Xtensis

Please see [this][contributing] document for more information about
contributing to Xtensis.

## Credits

The logo for Xtensis was designed by a friend of mine, [@vktec][vktec]

The logo is licensed under the CC BY-SA 4.0 license. Please provide
attribution to @vktec for usage of it.

## Inspiration

I have several sources of ongoing inspiration for xtensis:

- [Xi Editor][xi]
- [Neovim][neovim]
- [GNU Emacs][emacs]
- [iota][iota]

[travis-ci-status]: https://img.shields.io/travis/xtensis-editor/xtensis.svg
[travis-ci]: https://travis-ci.org/xtensis-editor/xtensis-core

[jenkins-ci-status]: https://ci.shymega.org.uk/buildStatus/icon?job=xtensis-core
[jenkins-ci]: https://ci.shymega.org.uk/job/xtensis-core/

[GPL-3-EXT]: https://www.gnu.org/licenses/gpl.html
[GPL-3]: /LICENSE

[contributing]: /.github/CONTRIBUTING.md

[vktec]: https://github.com/vktec

[xi]: https://github.com/google/xi-editor
[neovim]: https://neovim.io
[emacs]: https://www.gnu.org/software/emacs
[iota]: https://github.com/gchp/iota
[swiboe]: https://github.com/swiboe/swiboe
