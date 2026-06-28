use std::collections::HashMap;

use crate::Binary::format::*;

#[derive(Debug, Clone)]
pub enum VMValue
{
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub struct Frame
{
    pub locals: HashMap<String, VMValue>,
    pub return_ip: usize,
    pub function_name: String,
}

pub struct VM
{
    pub stack: Vec<VMValue>,
    pub frames: Vec<Frame>,

    pub ip: usize,
    pub bytecode: Vec<u8>,

    pub constants: Vec<Constant>,
    pub functions: Vec<BinaryFunction>,

    pub current_function: usize,
}


impl VM
{
    pub fn new(program: BinaryProgram) -> Self
    {
        Self
        {
            stack: Vec::new(),
            frames: Vec::new(),
            ip: 0,
            bytecode: program.bytecode,
            constants: program.constants,
            functions: program.functions,
        }
    }

    fn push(&mut self, value: VMValue)
    {
        self.stack.push(value);
    }
    
    fn pop(&mut self) -> VMValue
    {
        self.stack.pop().expect("Stack underflow")
    }

    fn read_u8(&self) -> u8
    {
        self.bytecode[self.ip]
    }

    fn advance(&mut self)
    {
        self.ip += 1;
    }

    fn read_i64(&mut self) -> i64
    {
        let start = self.ip;

        let bytes =
        [
            self.bytecode[start],
            self.bytecode[start+1],
            self.bytecode[start+2],
            self.bytecode[start+3],
            self.bytecode[start+4],
            self.bytecode[start+5],
            self.bytecode[start+6],
            self.bytecode[start+7],
        ];

        self.ip += 8;

        i64::from_le_bytes(bytes)
    }

    pub fn run(&mut self)
    {
        loop
        {
            let opcode = self.read_u8();
            self.advance();

            match opcode
            {
                0x01 => self.op_load_const(),
                0x02 => self.op_load_local(),
                0x03 => self.op_store_local(),

                0x06 => self.op_add(),
                0x07 => self.op_sub(),
                0x08 => self.op_mul(),
                0x09 => self.op_div(),

                0x0A => self.op_call(),
                0x0B => self.op_return(),

                0xFF => break,

                _ => panic!("Unknown opcode: {}", opcode),
            }
        }
    }

    fn op_load_const(&mut self)
    {
        let index = self.read_i64() as usize;

        let value = &self.constants[index];

        match value
        {
            Constant::Int(v) =>
                self.push(VMValue::Int(*v)),

            Constant::Float(v) =>
                self.push(VMValue::Float(*v)),

            Constant::String(v) =>
                self.push(VMValue::String(v.clone())),
        }
    }

    fn op_store_local(&mut self)
    {
        let name_len = self.read_i64() as usize;
        let name = format!("arg{}", name_len);

        let value = self.pop();

        let frame = self.frames.last_mut()
            .expect("No frame");

        frame.locals.insert(name, value);
    }

    fn op_load_local(&mut self)
    {
        let name_len = self.read_i64() as usize;
        let name = format!("arg{}", name_len);

        let frame = self.frames.last()
            .expect("No frame");

        let value = frame.locals.get(&name)
            .expect("Undefined local")
            .clone();

        self.push(value);
    }

    fn op_add(&mut self)
    {
        let b = self.pop();
        let a = self.pop();

        match (a, b)
        {
            (VMValue::Int(x), VMValue::Int(y)) =>
                self.push(VMValue::Int(x + y)),

            (VMValue::Float(x), VMValue::Float(y)) =>
                self.push(VMValue::Float(x + y)),

            _ =>
                panic!("Invalid types for ADD"),
        }
    }

    fn op_sub(&mut self)
    {
        let b = self.pop();
        let a = self.pop();

        if let (VMValue::Int(x), VMValue::Int(y)) = (a, b)
        {
            self.push(VMValue::Int(x - y));
        }
    }

    fn op_mul(&mut self)
    {
        let b = self.pop();
        let a = self.pop();

        if let (VMValue::Int(x), VMValue::Int(y)) = (a, b)
        {
            self.push(VMValue::Int(x * y));
        }
    }

    fn op_div(&mut self)
    {
        let b = self.pop();
        let a = self.pop();

        if let (VMValue::Int(x), VMValue::Int(y)) = (a, b)
        {
            self.push(VMValue::Int(x / y));
        }
    }

fn op_call(&mut self)
    {
        let func_index = self.read_i64() as usize;

        let func = self.functions[func_index].clone();

        let arg_count = func.parameter_count as usize;

        let mut args = Vec::new();

        for _ in 0..arg_count
        {
            args.push(self.pop());
        }

        args.reverse();

        let return_ip = self.ip;

        let mut frame = Frame
        {
            locals: HashMap::new(),
            return_ip,
            function_name: func.name.clone(),
        };

        for (i, arg) in args.into_iter().enumerate()
        {
            frame.locals.insert(
                format!("arg{}", i),
                arg
            );
        }

        self.frames.push(frame);

        self.ip = func.bytecode_offset as usize;
    }

    fn op_return(&mut self)
    {
        let ret = self.pop();
        let frame = self.frames.pop().expect("No frame");
        self.ip = frame.return_ip;
        self.push(ret);
    }
}
