# xtensis

## Goals

- Minimal
- Extendable
- Speedy

## Key features

- Package/plugin support.
  - JSON-API interface to `xtensis/core`

- Encourage customisation.
  xtensis should encourage customisation of the core, plugins and
  other components.

- Provide a set of defaults.
  These *could* be Emacs-style for now, in terms of keybindings.

- Written in Rust.

- Extendable and fast.

## Core functions

- Decentralised package manager

  This would be inspired in part from QUELPA & MELPA.

  QUELPA is a package manager for GNU Emacs based upon the MELPA package
  scheme.

  It is my aim to provide both a decentralised package manager, where
  packages can be retrieved from 'nodes' in a P2P fashion, and also
  where the source code (from `master`, or perhaps a commit hash), is
  stored - for example, on Notabug (A freedom respecting code hosting
  service) - *or* Github, a non-free code hosting service, but commonly
  used all the same.

- Major & Minor modes
    Emacs has support for 'major modes', which essentially define the
    syntax colouring schema, hooks to be run, programming indentation,
    etc.

    Emacs *also* has support for 'minor modes' which essentially
    modify behaviour in each major-mode (buffer-local basis) - this
    could be 'hard wrapping', syntax checking or spell checking.

    It is my intention to base xtensis upon this model of major and minor
    modes, and use that to the highest advantage available.

### Optional client/server architecture

We should aim for a high startup speed for both the core, and
frontend, but the client/server architecture would be good to have as
well, a la `emacsclient` and `emacs` pair.

### Backend / Frontend infrastructure

This is inspired from the xi-editor project, which I found to
be an excellent way to have enhanced UI flexibility.

The backend(core) and frontend would communicate via JSON-RPC, also
inspired from xi-editor.

The frontend should request the bare minimum of buffer contents to
save transfers. For example, if the frontend is instructed to move
down five lines, it will retrieve 5 lines worth of data in an JSON
array, similar to pagination.

(I got this idea from @spudowiar who mentioned Xi's editor method of
transferring data frontend <--> backend, and how it was
inefficient. @spudowiar has the right idea here.)

### Frontend

#### Frontend boot-up

The frontend will bootstrap itself on launch (this should be quick!!)
, and if there's no server available, launch, wait for a `READY`
signal, and then start the frontend.

xtensis-core should have API's to deal with everyday tasks.

## Addons

### Web browser embedding support, via Servo

This might be useful for web developers who want to preview code *in*
xtensis.

## Permission model

Like the Android's permissions model, I have been toying with the idea
of permissions for xtensis.

The permissions model would be along the lines of this:

(This is *not* a definitive list yet)

- `org.xtensis.permissions.buffers.read_buffer`
   This permission grants access to read a buffer's contents.
   At the time of writing, this does *not* grant read access on a
   per-line basis, but that would be something to look into in future.

- `org.xtensis.permissions.buffers.line_range_read`
  Arguments: line_begin, line_end
  
  This would allow 'ranges' of lines to be read by the text editor.
  Probably not _particularly_ useful.

- `org.xtensis.permissions.buffers.edit_buffer`
  This permission grants access to edit a buffer's contents.

  This gives all-area write access to the buffer, and does not
  restrict on a per-line basis.

- `org.xtensis.permissions.modes.minor_hook.add`
  Arguments: `buffer_id`, `hook_name`

  This permission grants access for a plugin to hook into a major
  mode.

  Normally, no plugin should be allowed to hook into a buffer by
  *default* - but this can be globally changed.

To the best of my knowledge, *no* other editor supports a permissions
model, and thus, `xtensis` would be the first editor of its kind to
*ever* implement permissions.
