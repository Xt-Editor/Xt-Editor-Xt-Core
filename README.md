<h1 align="center">
    <a href="https://github.com/xtensis-editor/xtensis-core">
    <img src="media/logos/PNG/xtensis-256.png" alt="Xtensis Editor" width="256" height="256"/>
    </a><br>
    <a href="https://github.com/xtensis-editor/xtensis-core">Xtensis Editor</a>
</h1>

<h4 align="center">Extendable editing: ignore the rules.</h4>

![Travis-CI Build Status][travis-ci]
![Codecov.io coverage][codecov]
![Crates.io][cratesio]
![GHReleases][ghrel]

## Why create Xtensis?

### Dissatisfaction with the current options

I've tried a lot of tools for text editing: Eclipse, IDEA, GNU Emacs,
Neovim, Vim, nano and yes, even Atom.

I never felt that I could get the **most** out of these editors, which
made using them a sub-par experience. The only editor I did get the
most out of was Emacs, but it had several deficiencies, such as the
single threaded nature of Emacs.

I kept complaining for a while about the problems I was having, but
decided to *do* something about it- make my own editor. Xtensis.

### Core goals

#### Extendable

xtensis is intended to be as extendable as possible, much in the same
manner as GNU Emacs, but without the added bloat. (Hint: Tetris?!)

Of course, this is a difficult balance to keep, but by keeping the
core small and extendable, and using plugins to manage additional
functions- this _can_ be achieved.

#### Minimal

xtensis is designed to be minimal. It aims to provide APIs for
extensions, and not implement the frontend in the Core subsystem.

Instead, frontends are of a modular nature. You can use a Qt frontend,
a Terminal frontend, or even GTK!

#### Fast

Xtensis _should_ be fast to run, without too much of a delay.

Additionally, background processes should be put into a queue for
asynchronous processing.

### Disclaimer

It goes without saying, but this project is heavily in development.

### Documentation for xtensis

Documentation of xtensis is a little convoluted at the moment.

The document for ideas I've thought of for xtensis is [here][ideas].

The document for concepts that are specific to xtensis
is [here][concepts]

The document for TODO items for xtensis is [here][todo]. Bear in mind-
this *should* be on a issue tracker, but it's here for legacy reasons.

### Licensing

Xtensis is licensed under the GPL 3.0. This license upholds the four
software freedoms, which myself, as a free software advocate, regard
in the strongest manner.

See [here][GPL-3] for the included license file,
*or* [here][GPL-3-EXT], for the GNU project's web page on the license.

### Contributing to Xtensis

See [this][contributing] document for more information.

### Attribution

The logo for Xtensis was designed by Samadi van Koten- aka @vktec.

The logo is licensed under the CC BY-SA 4.0 license.

## Inspiration

I had several sources of inspiration for xtensis:

- [Xi Editor][xi]
- [Neovim][neovim]
- [GNU Emacs][emacs]
- [iota][iota]

Go check 'em out! They are all really awesome projects, and we could
all learn from them, be it negative or positive :-)

[travis-ci]: https://img.shields.io/travis/xtensis-editor/xtensis.svg
[codecov]: https://img.shields.io/codecov/c/github/xtensis-editor/xtensis.svg
[cratesio]: https://img.shields.io/crates/d/xtensis.svg
[ghrel]: https://img.shields.io/github/downloads/xtensis-editor/xtensis/total.svg

[ideas]: /doc/IDEAS.md
[concepts]: /doc/CONCEPTS.md
[todo]: /doc/TODO.md

[GPL-3-EXT]: https://www.gnu.org/licenses/gpl.html
[GPL-3]: /LICENSE

[contributing]: /.github/CONTRIBUTING.md

[xi]: https://github.com/google/xi-editor
[neovim]: https://neovim.io
[emacs]: https://www.gnu.org/software/emacs
[iota]: https://github.com/gchp/iota
