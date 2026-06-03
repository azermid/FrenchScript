use arg_handling::validate_file;
use ReadFile::load_files;
use std::env;
use Libparse::tokenize_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    Vec<Vector<String>> = Vec::new();
    mut paths: Vec<String> = Vec::new();

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
    let Vec<SourceFile> source_files = load_files(paths);
    
    let tokens = tokenize_file(&source_files[0].path).unwrap();
    
    println!("Tokens : {:?}", tokens);
}