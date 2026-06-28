pub const MAGIC_NUMBER: [u8; 4] = *b"FSBC";
pub const VERSION: u16 = 1;

#[derive(Debug, Clone)]
pub struct BinaryHeader
{
    pub magic: [u8;4],
    pub version: u16,

    pub constant_count: u32,
    pub function_count: u32,

    pub constant_offset: u32,
    pub function_offset: u32,
    pub bytecode_offset: u32,

    pub entry_function: u32,
}

#[derive(Debug, Clone)]
pub enum Constant
{
    Int(i64),

    Float(f64),

    String(String),
}

#[derive(Debug, Clone)]
pub struct BinaryFunction
{
    pub name: String,

    pub parameter_count: u32,

    pub local_count: u32,

    pub bytecode_offset: u32,

    pub bytecode_size: u32,
}

#[derive(Debug, Clone)]
pub struct BinaryProgram
{
    pub header: BinaryHeader,

    pub constants: Vec<Constant>,

    pub functions: Vec<BinaryFunction>,

    pub bytecode: Vec<u8>,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode
{
    LoadConst = 1,

    LoadLocal = 2,

    StoreLocal = 3,

    LoadGlobal = 4,

    StoreGlobal = 5,

    Add = 6,

    Sub = 7,

    Mul = 8,

    Div = 9,

    Call = 10,

    Return = 11,

    Jump = 12,

    JumpIfFalse = 13,

    Equal = 14,

    NotEqual = 15,

    Greater = 16,

    Less = 17,

    Halt = 255,
}
