use std::io::{Read, Write};

use crate::{bitreader::BitReader, bitwriter::BitWriter};

pub trait BitWritable {
    fn write<W: Write>(&self, writer: &mut BitWriter<W>) -> anyhow::Result<()>;
}

pub trait BitReadable: Sized {
    fn read<R: Read>(reader: &mut BitReader<R>) -> Self;
}

impl BitWritable for bool {
    fn write<W: Write>(&self, writer: &mut BitWriter<W>) -> anyhow::Result<()> {
        writer.write_bits(1, *self as u64)
    }
}

impl BitReadable for bool {
    fn read<R: Read>(reader: &mut BitReader<R>) -> Self {
        reader.read_bits(1) == 1
    }
}

