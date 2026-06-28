use crate::Binary::format::{BinaryProgram, FunctionInfo, Constant};
use crate::Binary::writer::BinaryWriter;
use crate::Binary::reader::BinaryReader;

#[test]
fn test_writer_reader_roundtrip()
{
    let program = BinaryProgram
    {
        constants: vec![
            Constant::Int(42),
            Constant::String("hello".to_string()),
        ],

        functions: vec![
            FunctionInfo
            {
                parameter_count: 1,
                local_count: 2,
                stack_size: 3,
                code_offset: 0,
                code_size: 10,
            }
        ],

        code: vec![0x01, 0x00, 0x00, 0x00],

        entry_function: 0,
    };

    let writer = BinaryWriter::new();

    let bytes = writer.generate(&program);

    BinaryWriter::save_to_file("test.fsb", &bytes).unwrap();

    let reader = BinaryReader::from_file("test.fsb").unwrap();

    let loaded = reader.load_program();

    assert_eq!(loaded.constants.len(), program.constants.len());

    assert_eq!(loaded.functions.len(), program.functions.len());

    assert_eq!(loaded.code, program.code);
}

#[test]
fn test_writer_output_debug()
{
    let program = create_simple_program();

    let bytes = BinaryWriter::new().generate(&program);

    println!("{:02X?}", &bytes);
}

#[test]
fn test_header_is_valid()
{
    let program = create_simple_program();

    let bytes = BinaryWriter::new().generate(&program);

    assert_eq!(&bytes[0..4], b"FSBC");

    let version = u16::from_le_bytes([bytes[4], bytes[5]]);

    assert_eq!(version, 1);
}

#[test]
fn test_constants_encoding()
{
    let program = BinaryProgram
    {
        constants: vec![
            Constant::Int(10),
            Constant::Float(2.5),
            Constant::String("abc".to_string()),
        ],

        functions: vec![],
        code: vec![],
        entry_function: 0,
    };

    let bytes = BinaryWriter::new().generate(&program);

    // check qu'on a bien des types
    assert!(bytes.contains(&0)); // Int tag
    assert!(bytes.contains(&1)); // Float tag
    assert!(bytes.contains(&2)); // String tag
}

#[test]
fn test_bytecode_written()
{
    let program = BinaryProgram
    {
        constants: vec![],
        functions: vec![],
        code: vec![0x01, 0x02, 0x03],
        entry_function: 0,
    };

    let bytes = BinaryWriter::new().generate(&program);

    assert!(bytes.ends_with(&[0x01, 0x02, 0x03]));
}

#[test]
fn test_offsets_are_consistent()
{
    let program = create_simple_program();

    let bytes = BinaryWriter::new().generate(&program);

    let constant_offset = u32::from_le_bytes([
        bytes[6], bytes[7], bytes[8], bytes[9]
    ]);

    let function_offset = u32::from_le_bytes([
        bytes[10], bytes[11], bytes[12], bytes[13]
    ]);

    assert!(function_offset > constant_offset);
}

fn create_simple_program() -> BinaryProgram
{
    BinaryProgram
    {
        constants: vec![Constant::Int(1)],

        functions: vec![
            FunctionInfo
            {
                parameter_count: 0,
                local_count: 0,
                stack_size: 0,
                code_offset: 0,
                code_size: 1,
            }
        ],

        code: vec![0xFF],

        entry_function: 0,
    }
}