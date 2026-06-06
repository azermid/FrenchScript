mod ErrorHandling;
mod Lexer;
mod Structure;
mod Ast;
mod SemanticParser;

use crate::ErrorHandling::ArgHandling::validate_file; 
use crate::Lexer::LibParse::tokenize_file;
use crate::Lexer::ReadFile::load_files;
use crate::Structure::FileStruct::*;
use crate::Ast::ast_lib::*;
use crate::SemanticParser::SemanticParserLib::SemanticAnalyzer;


use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut paths: Vec<String> = Vec::new();

    if args.len() < 2 {
        println!("Utilisation : {} <nom>", args[0]);
        return;
    }
    args.remove(0);
    for arg in &args {
        match validate_file(arg, "fst") {
            Ok(path) => {
                println!("Le fichier {:?} est prêt à être lu.", path);
                paths.push(path.to_string_lossy().to_string());
            }   
            Err(e) => {
                eprintln!("Erreur lors de la validation : {}", e);
            }
        }
    }
    let source_files :Vec<SourceFile> = load_files(paths);
    
    let tokens = tokenize_file(&source_files[0].path).unwrap();

    let program = parse_all_tokens(tokens, source_files[0].path.clone(), source_files[0].content.clone()).unwrap();

    let mut analyzer = SemanticAnalyzer::new();

    if let Err(e) = analyzer.analyze(&program) {
        eprintln!("Erreur lors de l'analyse sémantique : {:?}", e);
        return;
    }

    println!("Tokens : {:?}", program);
}