use std::io::{Read, Write};

use crate::{bitreader::BitReader, bitwriter::BitWriter};

///Implementing this trait for a struct lets it be written conveniently with writer.write()
pub trait BitWritable {
    fn write<W: Write>(&self, writer: &mut BitWriter<W>) -> anyhow::Result<()>;
}

///Implementing this trait for a struct lets it be read conveniently with reader.read()
pub trait BitReadable: Sized {
    fn read<R: Read>(reader: &mut BitReader<R>) -> anyhow::Result<Self>;
}

impl BitWritable for bool {
    fn write<W: Write>(&self, writer: &mut BitWriter<W>) -> anyhow::Result<()> {
        writer.write_bits(1, *self as u64)
    }
}

impl BitReadable for bool {
    fn read<R: Read>(reader: &mut BitReader<R>) -> anyhow::Result<Self> {
        Ok(reader.read_bits(1)? == 1)
    }
}

