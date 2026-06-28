
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

pub const MAGIC: [u8; 4] = *b"FSBC";
pub const VERSION: u16 = 1;

#[derive(Debug, Clone)]
pub struct BinaryHeader
{
    pub magic: [u8; 4],

    pub version: u16,

    pub constant_count: u32,

    pub function_count: u32,

    pub constant_offset: u32,

    pub function_offset: u32,

    pub code_offset: u32,

    pub code_size: u32,

    pub entry_function: u32,
}

#[derive(Debug, Clone)]
pub struct FunctionInfo
{
    pub parameter_count: u16,

    pub local_count: u16,

    pub stack_size: u16,

    pub code_offset: u32,

    pub code_size: u32,
}

#[derive(Debug)]
pub struct BinaryProgram
{
    pub constants: Vec<Constant>,

    pub functions: Vec<FunctionInfo>,

    pub code: Vec<u8>,

    pub entry_function: u32,
}