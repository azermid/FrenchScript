mod ReadFile;
use std::fs;

pub fn load_files(paths: Vec<String>) -> Vec<SourceFile>
{
    paths.into_iter().map(|path| {
        let content = fs::read_to_string(&path).unwrap();

        SourceFile {
            path: path.clone(),
            content,
            items: vec![],
        }
    }).collect()
}

