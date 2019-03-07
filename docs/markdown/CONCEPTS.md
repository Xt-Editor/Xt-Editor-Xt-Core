# Concepts and terminology of Xt

## Background

This document serves as a guide for the concepts & terminology of the
Xt project.

## `Xt/core`

### Buffers and files

Pretty much self-explanatory.

### Commands

There are three types of commands (or functions, if you will).

The first one is called a `non-interactive` command. It is designed
for use in packages, and should be kept that way by package standards.

The second one is called a `interactive` command. It is called by the
interactive prompt, similar to GNU Emacs.

The third one is called a `internal` command. It should only be used
for nitty-gritty stuff, and should not used on a day-to-day basis.
