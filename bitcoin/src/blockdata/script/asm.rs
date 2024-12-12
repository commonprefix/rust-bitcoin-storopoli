// SPDX-License-Identifier: CC0-1.0

use crate::prelude::Vec;

/// Creates an iterator over instruction words and their position in the file.
pub(crate) fn iter_words(asm: &str) -> impl Iterator<Item = ((usize, usize), &str)> {
    asm.lines().enumerate().flat_map(|(line_idx, line)| {
        let content = line
            .split("#")
            .next()
            .expect("Could not split on #")
            .split("//")
            .next()
            .expect("Could not split on //");
        content
            .split_whitespace()
            .enumerate()
            .map(move |(word_idx, word)| ((line_idx, word_idx), word))
    })
}

/// Try to parse raw hex bytes and push them into the buffer.
pub(crate) fn try_parse_raw_hex(hex: &str, buf: &mut Vec<u8>) -> bool {
    buf.clear();
    let iter = match hex::HexToBytesIter::new(hex) {
        Ok(i) => i,
        Err(_) => return false,
    };
    for item in iter {
        let item = match item {
            Ok(i) => i,
            Err(_) => return false,
        };
        buf.push(item);
    }
    true
}

/// Create a [`ParseAsmError`] with the given position and kind.
pub(crate) fn asm_err(position: (usize, usize), kind: ParseAsmErrorKind) -> ParseAsmError {
    ParseAsmError { position, kind }
}

/// Error from parsing Script ASM.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseAsmError {
    /// The position of the instruction that caused the error.
    ///
    /// The value is (line, word) with word incremented after
    /// every chunk of whitespace.
    pub position: (usize, usize),

    /// The kind of error that occurred.
    pub kind: ParseAsmErrorKind,
}

/// The different kinds of [`ParseAsmError`]s that can occur.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseAsmErrorKind {
    /// ASM ended unexpectedly.
    UnexpectedEof,
    /// We were not able to interpret the instruction.
    UnknownInstruction,
    /// Invalid hexadecimal bytes.
    InvalidHex,
    /// Byte push exceeding the maximum size.
    PushExceedsMaxSize,
    /// ASM contains a byte push with non-minimal size prefix.
    ///
    /// This is not necessarily invalid, but we can't construct such pushes.
    NonMinimalBytePush,
}
