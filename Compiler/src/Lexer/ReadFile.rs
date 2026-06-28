use crate::Structure::FileStruct::*;

use std::fs;

pub fn load_files(paths: Vec<String>) -> Vec<SourceFile>
{
    paths.into_iter().filter_map(|path| {
        match fs::read_to_string(&path) {
            Ok(content) => Some(SourceFile {
                path: path.clone(),
                content,
                items: vec![],
            }),
            Err(_) => None,
        }
    }).collect()
}

