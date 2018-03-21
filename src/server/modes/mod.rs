//! This is the holding module for the modes in Xt.

/// Major mode module for Xt.
///
/// Major modes define the type of file opened in the editor.
/// They are the analogous equivalent of Vim's &filetype concept.
/// However, each Major mode holds a selection of enabled minor modes.
pub mod major_mode;

/// Minor mode module for Xt.
///
/// Minor modes change the behaviour of a Buffer in Xt. They might
/// introduce syntax checking, spell checking, auto wrapping of text,
/// or auto-completion, but are isolated within the Buffer, and do not
/// affect any other Buffers, **unless** set as a globally
/// workspace-specific minor mode, which will affect all Buffers in a
/// given workspace.
///
/// Minor modes can be associated with a Buffer's instance of a major
/// mode, or not associated. Association is analogous to enabling a
/// minor mode, and dissociation is analogous to disabling a minor mode
pub mod minor_modes;
