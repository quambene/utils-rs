//! Customized formatter based on `serde_json::ser::PrettyFormatter`.

use serde_json::ser;
use std::io;

/// Formats a JSON value to make it human readable.
#[derive(Clone, Debug)]
pub struct Formatter<'a> {
    current_indent: usize,
    has_value: bool,
    indent: &'a [u8],
}

impl<'a> Formatter<'a> {
    /// Construct a pretty printer formatter that defaults to using two spaces for indentation.
    pub fn new() -> Self {
        Formatter::with_indent(b"  ")
    }

    /// Construct a pretty printer formatter that uses the `indent` string for indentation.
    pub fn with_indent(indent: &'a [u8]) -> Self {
        Formatter {
            current_indent: 0,
            has_value: false,
            indent,
        }
    }
}

impl<'a> Default for Formatter<'a> {
    fn default() -> Self {
        Formatter::new()
    }
}

impl<'a> ser::Formatter for Formatter<'a> {
    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"[")
    }

    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent -= 1;

        if self.has_value {
            writer.write_all(b"")?;
        }

        writer.write_all(b"]")
    }

    #[inline]
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if first {
            writer.write_all(b"")?;
        } else {
            writer.write_all(b",")?;
        }
        Ok(())
    }

    #[inline]
    fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;
        Ok(())
    }

    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"{")
    }

    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent -= 1;

        if self.has_value {
            writer.write_all(b"\n")?;
            indent(writer, self.current_indent, self.indent)?;
        }

        writer.write_all(b"}")
    }

    #[inline]
    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if first {
            writer.write_all(b"\n")?;
        } else {
            writer.write_all(b",\n")?;
        }
        indent(writer, self.current_indent, self.indent)
    }

    #[inline]
    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b": ")
    }

    #[inline]
    fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;
        Ok(())
    }
}

fn indent<W>(wr: &mut W, n: usize, s: &[u8]) -> io::Result<()>
where
    W: ?Sized + io::Write,
{
    for _ in 0..n {
        wr.write_all(s)?;
    }

    Ok(())
}
