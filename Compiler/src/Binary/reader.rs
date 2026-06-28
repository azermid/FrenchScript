use std::fs::File;
use std::io::{Read};

use crate::Binary::format::*;

pub struct BinaryReader
{
    pub buffer: Vec<u8>,
    pub cursor: usize,
}

impl BinaryReader
{
    pub fn from_file(path: &str) -> std::io::Result<Self>
    {
        let mut file = File::open(path)?;

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        Ok(Self
        {
            buffer,
            cursor: 0,
        })
    }

    fn read_u8(&mut self) -> u8
    {
        let v = self.buffer[self.cursor];

        self.cursor += 1;

        v
    }

    fn read_u16(&mut self) -> u16
    {
        let bytes =
        [
            self.buffer[self.cursor],
            self.buffer[self.cursor + 1],
        ];

        self.cursor += 2;

        u16::from_le_bytes(bytes)
    }

    fn read_u32(&mut self) -> u32
    {
        let bytes =
        [
            self.buffer[self.cursor],
            self.buffer[self.cursor + 1],
            self.buffer[self.cursor + 2],
            self.buffer[self.cursor + 3],
        ];

        self.cursor += 4;

        u32::from_le_bytes(bytes)
    }

    fn read_i64(&mut self) -> i64
    {
        let mut bytes = [0u8; 8];

        bytes.copy_from_slice(
            &self.buffer[self.cursor..self.cursor + 8]
        );

        self.cursor += 8;

        i64::from_le_bytes(bytes)
    }

    fn read_string(&mut self) -> String
    {
        let len = self.read_u32() as usize;

        let bytes =
            &self.buffer[self.cursor..self.cursor + len];

        self.cursor += len;

        String::from_utf8(bytes.to_vec()).unwrap()
    }

    fn read_header(&mut self) -> BinaryHeader
    {
        let mut magic = [0u8; 4];

        magic.copy_from_slice(
            &self.buffer[self.cursor..self.cursor + 4]
        );

        self.cursor += 4;

        let version = self.read_u16();

        let constant_count = self.read_u32();
        let function_count = self.read_u32();

        let entry_function = self.read_u32();

        BinaryHeader
        {
            magic,
            version,
            constant_count,
            function_count,
            entry_function,
        }
    }

    fn read_constant(&mut self) -> Constant
    {
        let tag = self.read_u8();

        match tag
        {
            0 =>
            {
                Constant::Int(self.read_i64())
            }

            1 =>
            {
                Constant::Float(
                    f64::from_le_bytes(
                        self.buffer[self.cursor..self.cursor+8]
                            .try_into().unwrap()
                    )
                )
            }

            2 =>
            {
                Constant::String(self.read_string())
            }

            _ =>
            {
                panic!("Unknown constant type")
            }
        }
    }

    fn read_constants(&mut self, count: u32) -> Vec<Constant>
    {
        let mut constants = Vec::new();

        for _ in 0..count
        {
            constants.push(self.read_constant());
        }

        constants
    }

    fn read_function(&mut self) -> BinaryFunction
    {
        let name = self.read_string();

        let parameter_count = self.read_u32();

        let local_count = self.read_u32();

        let bytecode_offset = self.read_u32();

        let bytecode_size = self.read_u32();

        BinaryFunction
        {
            name,
            parameter_count,
            local_count,
            bytecode_offset,
            bytecode_size,
        }
    }

    fn read_functions(&mut self, count: u32) -> Vec<BinaryFunction>
    {
        let mut functions = Vec::new();

        for _ in 0..count
        {
            functions.push(self.read_function());
        }

        functions
    }

    fn read_bytecode(&mut self, size: usize) -> Vec<u8>
    {
        let data =
            self.buffer[self.cursor..self.cursor + size]
                .to_vec();

        self.cursor += size;

        data
    }

    pub fn load_program(mut self) -> BinaryProgram
    {
        let header = self.read_header();

        let constants =
            self.read_constants(header.constant_count);

        let functions =
            self.read_functions(header.function_count);

        let bytecode =
            self.read_bytecode(
                self.buffer.len() - self.cursor
            );

        BinaryProgram
        {
            header,
            constants,
            functions,
            bytecode,
        }
    }
}