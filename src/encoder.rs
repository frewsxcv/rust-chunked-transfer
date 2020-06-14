// Copyright 2015 The tiny-http Contributors
// Copyright 2015 The rust-chunked-transfer Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::Result as IoResult;
use std::io::Write;

/// Splits the incoming data into HTTP chunks.
///
/// # Example
///
/// ```
/// use chunked_transfer::Encoder;
/// use std::io::Write;
///
/// let mut decoded = "hello world";
/// let mut encoded: Vec<u8> = vec![];
///
/// {
///     let mut encoder = Encoder::with_chunks_size(&mut encoded, 5);
///     encoder.write_all(decoded.as_bytes());
/// }
///
/// assert_eq!(encoded, b"5\r\nhello\r\n5\r\n worl\r\n1\r\nd\r\n0\r\n\r\n");
/// ```
pub struct Encoder<W>
where
    W: Write,
{
    // where to send the result
    output: W,

    // size of each chunk
    chunks_size: usize,

    // data waiting to be sent is stored here
    buffer: Vec<u8>,

    // Flushes the internal buffer after each write. This might be useful
    // if data should be sent immediately to downstream consumers
    flush_after_write: bool,
}

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn new(output: W) -> Encoder<W> {
        Encoder::with_chunks_size(output, 8192)
    }

    pub fn with_chunks_size(output: W, chunks: usize) -> Encoder<W> {
        Encoder {
            output,
            chunks_size: chunks,
            buffer: Vec::with_capacity(0),
            flush_after_write: false,
        }
    }

    pub fn with_flush_after_write(output: W) -> Encoder<W> {
        Encoder {
            output,
            chunks_size: 8192,
            buffer: Vec::with_capacity(0),
            flush_after_write: true,
        }
    }
}

fn send<W>(output: &mut W, data: &[u8]) -> IoResult<()>
where
    W: Write,
{
    write!(output, "{:x}\r\n", data.len())?;
    output.write_all(data)?;
    write!(output, "\r\n")?;
    Ok(())
}

impl<W> Write for Encoder<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.buffer.write_all(buf)?;

        while self.buffer.len() >= self.chunks_size {
            let rest = {
                let (to_send, rest) = self.buffer.split_at_mut(self.chunks_size);
                send(&mut self.output, to_send)?;
                rest.to_vec()
            };
            self.buffer = rest;
        }
        if self.flush_after_write {
            self.flush()?;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> IoResult<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        send(&mut self.output, &self.buffer)?;
        self.buffer.clear();
        Ok(())
    }
}

impl<W> Drop for Encoder<W>
where
    W: Write,
{
    fn drop(&mut self) {
        self.flush().ok();
        send(&mut self.output, &[]).ok();
    }
}

#[cfg(test)]
mod test {
    use super::Encoder;
    use std::io;
    use std::io::Write;
    use std::str::from_utf8;

    #[test]
    fn test() {
        let mut source = io::Cursor::new("hello world".to_string().into_bytes());
        let mut dest: Vec<u8> = vec![];

        {
            let mut encoder = Encoder::with_chunks_size(dest.by_ref(), 5);
            io::copy(&mut source, &mut encoder).unwrap();
            assert!(!encoder.buffer.is_empty());
        }

        let output = from_utf8(&dest).unwrap();

        assert_eq!(output, "5\r\nhello\r\n5\r\n worl\r\n1\r\nd\r\n0\r\n\r\n");
    }
    #[test]
    fn flush_after_write() {
        let mut source = io::Cursor::new("hello world".to_string().into_bytes());
        let mut dest: Vec<u8> = vec![];

        {
            let mut encoder = Encoder::with_flush_after_write(dest.by_ref());
            io::copy(&mut source, &mut encoder).unwrap();
            // The internal buffer has been flushed.
            assert!(encoder.buffer.is_empty());
        }

        let output = from_utf8(&dest).unwrap();

        assert_eq!(output, "b\r\nhello world\r\n0\r\n\r\n");
    }
}
