use crate::either::Either;
use crate::either::Either::{Left, Right};
use curl::easy::{Easy2, Handler, WriteError};
use std::io::Write;

struct Collector<T: Write> {
    bytes: Vec<u8>,
    writer: T,
    should_cache: bool,
}

impl<T: Write> Handler for Collector<T> {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.writer.write_all(data).unwrap();

        if self.should_cache {
            self.bytes.extend(data)
        }

        Ok(data.len())
    }
}

impl<T: Write> Collector<T> {
    fn new(writer: T, should_cache: bool) -> Self {
        Self {
            bytes: Vec::new(),
            writer,
            should_cache,
        }
    }

    fn get_data(&mut self) -> &[u8] {
        &self.bytes
    }
}

pub fn download<T: Write>(url: &str, writer: T, should_cache: bool) -> Option<Either<Vec<u8>, ()>> {
    let mut easy = Easy2::new(Collector::new(writer, should_cache));

    easy.get(true).ok()?;
    easy.url(url).ok()?;
    easy.useragent("curl/8.4.0").unwrap();
    easy.perform().ok()?;

    if should_cache {
        Some(Left(easy.get_mut().get_data().to_owned()))
    } else {
        Some(Right(()))
    }
}
