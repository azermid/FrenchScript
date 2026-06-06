// ==========================
// POSITION / ERREURS
// ==========================

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub file: String,
    pub message: String,
    pub span: Span,
}

pub type ParseResult<T> = Result<T, CompilerError>;


// ==========================
// TYPES DE BASE
// ==========================

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Text,
    Void,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
}


// ==========================
// OPERATEURS
// ==========================

#[derive(Debug, Clone, PartialEq)]
pub enum TokenOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}


// ==========================
// EXPRESSIONS
// ==========================

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
    Value(Value),
    Variable(String),
    FunctionCall(FunctionCall),
    Operation(Operation),
}


// ==========================
// OPERATION (a + b)
// ==========================

#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub left: Box<Expression>,
    pub operator: TokenOperation,
    pub right: Box<Expression>,
    pub span: Span,
}


// ==========================
// FUNCTION CALL
// ==========================

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expression>,
    pub span: Span,
}


// ==========================
// VARIABLES
// ==========================

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub var_type: Type,
    pub value: Option<Expression>,
    pub span: Span,
}


// ==========================
// ARGUMENTS DE FONCTION
// ==========================

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: String,
    pub arg_type: Type,
    pub span: Span,
}


// ==========================
// STATEMENTS
// ==========================

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VariableDeclaration(Variable),
    Return(ReturnStatement),
    Expression(Expression),
}


// ==========================
// FONCTION
// ==========================

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Argument>,
    pub return_type: Type,
    pub body: Vec<Statement>,
    pub span: Span,
}


// ==========================
// FICHIER SOURCE
// ==========================

#[derive(Debug, Clone, PartialEq)]
pub enum FileItem {
    Function(Function),
    Variable(Variable),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceFile {
    pub path: String,
    pub content: String,
    pub items: Vec<FileItem>,
}


// ==========================
// PROGRAMME GLOBAL
// ==========================

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub files: Vec<SourceFile>,
}


// ==========================
// TOKENS POUR LE LEXER
// ==========================

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType
{
    // Mots-clés
    Function,
    Return,

    TypeInt,
    TypeFloat,
    TypeText,
    TypeVoid,

    // Littéraux
    Identifier(String),
    Number(String),
    StringLiteral(String),

    // Opérateurs
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,

    // Symboles
    LParen,
    RParen,

    LBrace,
    RBrace,

    Comma,
    Semicolon,

    EOF,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}


pub type LexerResult<T> = Result<T, CompilerError>;
