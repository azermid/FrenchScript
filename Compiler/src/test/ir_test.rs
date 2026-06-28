
use crate::*;

#[test]
fn compile() {
    let source_files = load_files(vec!["tests/TestFile1.fst".to_string()]);

    if source_files.is_empty() {
        eprintln!("No source file found");
        return;
    }

    // ===== 1. TOKENIZE =====
    let tokens = match tokenize_file(&source_files[0].path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {:?}", e);
            return;
        }
    };

    // ===== 2. PARSE =====
    let program = match parse_all_tokens(
        tokens,
        source_files[0].path.clone(),
        source_files[0].content.clone(),
    ) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parser error: {:?}", e);
            return;
        }
    };

    // ===== 3. SEMANTIC ANALYSIS =====
    let mut analyzer = SemanticAnalyzer::new();

    if let Err(e) = analyzer.analyze(&program) {
        eprintln!("Semantic error: {:?}", e);
        return;
    }

    // ===== 4. IR GENERATION =====
    let mut ir_gen = IRGenerator::new();
    let ir_program = ir_gen.generate(&program);

    println!("{:#?}", ir_program);
}

pub fn compile_source(path: &str) -> Option<IRProgram> {
    // ===== 1. LOAD FILE =====
    let source_files = load_files(vec![path.to_string()]);

    if source_files.is_empty() {
        eprintln!("No source file found");
        return None;
    }

    // ===== 2. TOKENIZE =====
    let tokens = match tokenize_file(&source_files[0].path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {:?}", e);
            return None;
        }
    };

    // ===== 3. PARSE =====
    let program = match parse_all_tokens(
        tokens,
        source_files[0].path.clone(),
        source_files[0].content.clone(),
    ) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parser error: {:?}", e);
            return None;
        }
    };

    // ===== 4. SEMANTIC =====
    let mut analyzer = SemanticAnalyzer::new();

    if let Err(e) = analyzer.analyze(&program) {
        eprintln!("Semantic error: {:?}", e);
        return None;
    }

    // ===== 5. IR =====
    let mut ir_gen = IRGenerator::new();

    Some(ir_gen.generate(&program))
}

#[test]
fn test_simple_function() {
    let ir = compile_source("tests/TestFile1.fst");

    assert!(ir.is_some());

    let ir = ir.unwrap();

    assert!(!ir.functions.is_empty());

    println!("{:#?}", ir);
}

#[test]
fn test_general_function_exists() {
    let ir = compile_source("tests/TestFile1.fst")
        .expect("Compilation failed");

    let func = ir.functions.iter().find(|f| f.name == "general");

    assert!(func.is_some());
}

#[test]
fn test_function_call() {
    let ir = compile_source("tests/TestFile1.fst")
        .expect("Compilation failed");

    let has_call = ir.functions.iter().any(|func| {
        func.instructions.iter().any(|ins| {
            matches!(ins, IRInstruction::Call { .. })
        })
    });

    assert!(has_call);
}

#[test]
fn test_invalid_file_should_fail() {
    let result = compile_source("tests/invalid_file.fst");

    assert!(result.is_none());
}