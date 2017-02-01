# Concepts and terminology of xtensis

## Background

This document serves as a guide for the concepts & terminology of the
Xtensis project.

## `xtensis/core`

### Buffers

A buffer is called a buffer. This is similar to most text editors,
such as GNU Emacs.

### Files

When talking about a file, it is called a file, just like normal.

### Commands

There are three types of commands (or functions, if you will).

The first one is called a `non-interactive` command. It is designed
for use in packages, and should be kept that way by package standards.

The second one is called a `interactive` command. It is called by the
interactive prompt, similar to GNU Emacs.

The third one is called a `internal` command. It should only be used
for nitty-gritty stuff, and should not used on a day-to-day basis.

### Views

A window is a view *into* a buffer.

It can be used from different 'viewports' of xtensis (We'll come to
that.), and split multiple times.
