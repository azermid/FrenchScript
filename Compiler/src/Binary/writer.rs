use std::fs::File;
use std::io::{BufWriter, Write};

use crate::Binary::format::*;

pub struct BinaryWriter
{
    buffer: Vec<u8>,
}


impl BinaryWriter
{

    pub fn new() -> Self
    {
        Self
        {
            buffer: Vec::with_capacity(4096),
        }
    }


    fn write_bytes(&mut self, bytes: &[u8])
    {
        self.buffer.extend_from_slice(bytes);
    }

    fn write_u8(&mut self, value: u8)
    {
        self.buffer.push(value);
    }


    fn write_u16(&mut self, value: u16)
    {
        self.buffer.extend(value.to_le_bytes());
    }

    fn write_u32(&mut self, value: u32)
    {
        self.buffer.extend(value.to_le_bytes());
    }

    fn write_i64(&mut self, value: i64)
    {
        self.buffer.extend(value.to_le_bytes());
    }

    fn write_f64(&mut self, value: f64)
    {
        self.buffer.extend(value.to_le_bytes());
    }

    fn write_string(&mut self, value: &str)
    {
        self.write_u32(value.len() as u32);

        self.write_bytes(value.as_bytes());
    }

    fn write_constant(&mut self,constant: &Constant)
    {
        match constant
        {
            Constant::Int(v) =>
            {
                self.write_u8(0);
                self.write_i64(*v);
            }

            Constant::Float(v) =>
            {
                self.write_u8(1);
                self.write_f64(*v);
            }

            Constant::String(v) =>
            {
                self.write_u8(2);
                self.write_string(v);
            }
        }
    }

    fn write_constants(&mut self,constants: &[Constant])
    {
        for constant in constants
        {
            self.write_constant(constant);
        }
    }

    fn write_function(&mut self,function: &BinaryFunction)
    {
        self.write_u16(function.parameter_count);

        self.write_u16(function.local_count);

        self.write_u16(function.stack_size);

        self.write_u32(function.code_offset);

        self.write_u32(function.code_size);
    }

    fn write_functions(&mut self,functions: &[BinaryFunction])
    {
        for function in functions
        {
            self.write_function(function);
        }
    }

    pub fn generate(mut self,program: &BinaryProgram) -> Vec<u8>
    {
        self.write_header(program);

        self.write_constants(&program.constants);

        self.write_functions(&program.functions);

        self.write_bytecode(&program.bytecode);

        self.buffer
    }

    pub fn save_to_file(path: &str,bytes: &[u8]) -> std::io::Result<()>
    {
        let file = File::create(path)?;

        let mut writer = BufWriter::new(file);

        writer.write_all(bytes)?;

        writer.flush()?;

        Ok(())
    }

    fn write_header(&mut self,program: &BinaryProgram)
    {
        const HEADER_SIZE: u32 = 38;

        let constant_offset = HEADER_SIZE;

        let constant_size =
            self.compute_constant_pool_size(&program.constants);

        let function_offset =
            constant_offset + constant_size;

        let function_size =
            self.compute_function_table_size(&program.functions);

        let code_offset =
            function_offset + function_size;

        self.write_bytes(&MAGIC);

        self.write_u16(VERSION);

        self.write_u32(program.constants.len() as u32);

        self.write_u32(program.functions.len() as u32);

        self.write_u32(constant_offset);

        self.write_u32(function_offset);

        self.write_u32(code_offset);

        self.write_u32(program.bytecode.len() as u32);

        self.write_u32(program.entry_function);
    }

    fn write_bytecode(&mut self,code: &[u8],)
    {
        self.write_bytes(code);
    }

    fn compute_constant_pool_size(&self,constants: &[Constant]) -> u32
    {
        let mut size = 0u32;

        for constant in constants
        {
            size += match constant
            {
                Constant::Int(_) =>
                {
                    // Type + i64
                    1 + 8
                }

                Constant::Float(_) =>
                {
                    // Type + f64
                    1 + 8
                }

                Constant::String(value) =>
                {
                    // Type + longueur(u32) + données
                    1 + 4 + value.as_bytes().len() as u32
                }
            };
        }

        size
    }

    fn compute_function_table_size(&self,functions: &[BinaryFunction]) -> u32
    {
        const FUNCTION_SIZE: u32 = 14;

        functions.len() as u32 * FUNCTION_SIZE
    }
}