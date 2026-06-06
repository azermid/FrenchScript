use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::Token;
use crate::TokenType;
use crate::Span;
use crate::LexerResult;
use crate::CompilerError;

pub fn pass_whitespace(chars: &[char], current: &mut usize)
{
    while *current < chars.len()
        && chars[*current].is_whitespace()
    {
        *current += 1;
    }
}


pub fn get_word(chars: &[char], current: &mut usize) -> String
{
    let mut word = String::new();

    while *current < chars.len()
        && (chars[*current].is_alphanumeric()
            || chars[*current] == '_')
    {
        word.push(chars[*current]);
        *current += 1;
    }

    word
}

pub fn get_number(chars: &[char], current: &mut usize) -> String
{
    let mut number = String::new();

    while *current < chars.len()
        && chars[*current].is_numeric()
    {
        number.push(chars[*current]);
        *current += 1;
    }

    number
}

fn get_string(
    chars: &[char],
    current: &mut usize,
) -> Result<String, String>
{
    let mut result = String::new();

    // saute le "
    *current += 1;

    while *current < chars.len()
        && chars[*current] != '"'
    {
        result.push(chars[*current]);
        *current += 1;
    }

    if *current >= chars.len() {
        return Err(
            "String non fermée".to_string()
        );
    }

    *current += 1;

    Ok(result)
}

pub fn get_token(chars: &[char],current: &mut usize,line: usize,file: &str,) -> LexerResult<Token>
{
    pass_whitespace(chars, current);

    if *current >= chars.len() {
        return Ok(
            Token {
                token_type: TokenType::EOF,
                span: Span {
                    line,
                    column: *current,
                    length: 0,
                },
            }
        );
    }

    let start = *current;
    let c = chars[*current];

    if c == '"' {

        let string =
            get_string(chars, current)
                .map_err(|message| {
                    CompilerError {
                        file: file.to_string(),
                        message,
                        span: Span {
                            line,
                            column: start,
                            length: 1,
                        },
                    }
                })?;

        return Ok(Token {
            token_type:
                TokenType::StringLiteral(string),

            span: Span {
                line,
                column: start,
                length: *current - start,
            },
        });
    }

    if c.is_alphabetic() || c == '_' {

        let word = get_word(chars, current);

        let token_type = match word.as_str() {
            "fonction" => TokenType::Function,
            "retourner" => TokenType::Return,

            "entier" => TokenType::TypeInt,
            "decimal" => TokenType::TypeFloat,
            "Texte" => TokenType::TypeText,
            "vide" => TokenType::TypeVoid,

            _ => TokenType::Identifier(word),
        };
        return Ok(
            Token {
                token_type,
                span: Span {
                    line,
                    column: start,
                    length: *current - start,
                },
            }
        );
    }

    if c.is_numeric() {

        let number = get_number(chars, current);

        return Ok(
            Token {
                token_type: TokenType::Number(number),
                span: Span {
                    line,
                    column: start,
                    length: *current - start,
                },
            }
        );
    }

    *current += 1;

    let token_type = match c {

        '(' => TokenType::LParen,
        ')' => TokenType::RParen,

        '{' => TokenType::LBrace,
        '}' => TokenType::RBrace,

        ',' => TokenType::Comma,
        ';' => TokenType::Semicolon,
        '+' => TokenType::Plus,
        '-' => TokenType::Minus,
        '*' => TokenType::Multiply,
        '/' => TokenType::Divide,
        '=' => TokenType::Assign,

        _ => {
            return Err(
                CompilerError {
                    file: file.to_string(),
                    message: format!(
                        "Caractère inattendu '{}'",
                        c
                    ),
                    span: Span {
                        line,
                        column: start,
                        length: 1,
                    },
                }
            );
        }
    };

    Ok(
        Token {
            token_type,
            span: Span {
                line,
                column: start,
                length: 1,
            },
        }
    )
}

pub fn tokenize_line(line_content: &str,line_number: usize,file: &str) -> LexerResult<Vec<Token>>
{
    let chars: Vec<char> =
        line_content.chars().collect();

    let mut current = 0;

    let mut tokens = Vec::new();

    loop {

        let token =
            get_token(
                &chars,
                &mut current,
                line_number,
                file
            )?;

        if token.token_type == TokenType::EOF {
            break;
        }

        tokens.push(token);
    }

    Ok(tokens)
}

pub fn tokenize_file(path: &str) -> LexerResult<Vec<Token>>
{
    let file =
        File::open(path)
            .map_err(|_| CompilerError {
                file: path.to_string(),
                message:
                    "Impossible d'ouvrir le fichier"
                        .to_string(),
                span: Span {
                    line: 0,
                    column: 0,
                    length: 0,
                },
            })?;

    let reader = BufReader::new(file);

    let mut tokens = Vec::new();

    for (line_number, line) in
        reader.lines().enumerate()
    {
        let line =
            line.map_err(|_| CompilerError {
                file: path.to_string(),
                message:
                    "Erreur de lecture"
                        .to_string(),
                span: Span {
                    line: line_number + 1,
                    column: 0,
                    length: 0,
                },
            })?;

        tokens.extend(
            tokenize_line(
                &line,
                line_number + 1,
                path
            )?
        );
    }

    tokens.push(
        Token {
            token_type: TokenType::EOF,
            span: Span {
                line: 0,
                column: 0,
                length: 0,
            },
        }
    );

    Ok(tokens)
}