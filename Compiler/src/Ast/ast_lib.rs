use crate::*;
use crate::TokenType;
use crate::Span;
use crate::CompilerError;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
    pub file: String,
}

impl Parser
{

    fn peek(&self) -> &Token
    {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token
    {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token
    {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool
    {
        matches!(
            self.peek().token_type,
            TokenType::EOF
        )
    }

    fn check(&self, token: &TokenType) -> bool
    {
        std::mem::discriminant(&self.peek().token_type)
            == std::mem::discriminant(token)
    }

    fn consume(&mut self,expected: TokenType,message: &str,) -> ParseResult<Token>
    {
        if self.check(&expected) {
            return Ok(self.advance().clone());
        }

        Err(
            CompilerError {
                file: self.file.clone(),
                message: message.to_string(),
                span: self.peek().span.clone(),
            }
        )
    }

    fn parse_function(&mut self) -> ParseResult<Function>
    {
        let start_span = self.peek().span.clone();

        self.consume(
            TokenType::Function,
            "fonction"
        )?;

        let name = match self.advance().token_type.clone() {
            TokenType::Identifier(name) => name,

            _ => {
                return Err(
                    CompilerError {
                        file: self.file.clone(),
                        message: "nom de fonction attendu".to_string(),
                        span: self.peek().span.clone(),
                    }
                );
            }
        };

        self.consume(
            TokenType::LParen,
            "'(' attendu"
        )?;

        let parameters =
            self.parse_parameters()?;

        self.consume(
            TokenType::RParen,
            "')' attendu"
        )?;

        // parse le type de retour pas sur d'en avoir un 

        let return_type_parse  = self.parse_type()?;


        self.consume(
            TokenType::LBrace,
            "'{' attendu"
        )?;

        let body =
            self.parse_body()?;

        self.consume(
            TokenType::RBrace,
            "'}' attendu"
        )?;

        Ok(
            Function {
                name,
                parameters,
                return_type: return_type_parse,
                body,
                span: start_span,
            }
        )
    }

    fn parse_parameters(&mut self) -> ParseResult<Vec<Argument>>
    {
        let mut params = Vec::new();

        while !self.check(&TokenType::RParen)
        {
            let arg_span =
                self.peek().span.clone();

            let arg_type =
                self.parse_type()?;

            let name =
                match self.advance().token_type.clone()
            {
                TokenType::Identifier(name) => name,

                _ => {
                    return Err(
                        CompilerError {
                            file: self.file.clone(),
                            message:
                                "nom d'argument attendu"
                                    .to_string(),
                            span:
                                self.peek().span.clone(),
                        }
                    );
                }
            };

            params.push(
                Argument {
                    name,
                    arg_type,
                    span: arg_span,
                }
            );

            if self.check(&TokenType::Comma)
            {
                self.advance();
            }
        }

        Ok(params)
    }

    fn parse_type(&mut self) -> ParseResult<Type>
    {
        let token =
            self.advance().token_type.clone();

        match token {

            TokenType::TypeInt =>
                Ok(Type::Int),

            TokenType::TypeFloat =>
                Ok(Type::Float),

            TokenType::TypeText =>
                Ok(Type::Text),

            TokenType::TypeVoid =>
                Ok(Type::Void),

            _ => Err(
                CompilerError {
                    file: self.file.clone(),
                    message:
                        "type attendu".to_string(),
                    span:
                        self.previous().span.clone(),
                }
            )
        }
    }

    fn parse_body(&mut self) -> ParseResult<Vec<Statement>>
    {
        let mut statements =
            Vec::new();

        while !self.check(&TokenType::RBrace)
            && !self.is_at_end()
        {
            statements.push(
                self.parse_statement()?
            );
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> ParseResult<Statement>
    {
        match &self.peek().token_type {

            TokenType::Return => {
                Ok(
                    Statement::Return(
                        self.parse_return()?
                    )
                )
            }

            TokenType::TypeInt
            | TokenType::TypeFloat
            | TokenType::TypeText => {

                Ok(
                    Statement::VariableDeclaration(
                        self.parse_variable()?
                    )
                )
            }

            _ => {
                let expr =
                    self.parse_expression()?;

                self.consume(
                    TokenType::Semicolon,
                    "';' attendu"
                )?;

                Ok(
                    Statement::Expression(expr)
                )
            }
        }
    }

    fn parse_variable(&mut self) -> ParseResult<Variable>
    {
        let span =
            self.peek().span.clone();

        let var_type =
            self.parse_type()?;

        let name =
            match self.advance().token_type.clone()
        {
            TokenType::Identifier(name) => name,

            _ => {
                return Err(
                    CompilerError {
                        file: self.file.clone(),
                        message:
                            "nom de variable attendu"
                                .to_string(),
                        span:
                            self.peek().span.clone(),
                    }
                );
            }
        };

        let mut value = None;

        if self.check(&TokenType::Assign)
        {
            self.advance();

            value =
                Some(
                    self.parse_expression()?
                );
        }

        self.consume(
            TokenType::Semicolon,
            "';' attendu"
        )?;

        Ok(
            Variable {
                name,
                var_type,
                value,
                span,
            }
        )
    }

    fn parse_return(&mut self) -> ParseResult<ReturnStatement>
    {
        let span =
            self.peek().span.clone();

        self.advance();

        let expr =
            self.parse_expression()?;

        self.consume(
            TokenType::Semicolon,
            "';' attendu"
        )?;

        Ok(
            ReturnStatement {
                value: Some(expr),
                span,
            }
        )
    }

    fn parse_expression(&mut self) -> ParseResult<Expression>
    {
        let mut left =
            self.parse_primary()?;

        while matches!(self.peek().token_type,TokenType::Plus| TokenType::Minus)
        {
            let operator =
                match self.advance().token_type {

                    TokenType::Plus =>
                        TokenOperation::Add,

                    TokenType::Minus =>
                        TokenOperation::Subtract,

                    _ => unreachable!(),
                };

            let right =
                self.parse_primary()?;

            let span =
                left.span.clone();

            left = Expression {
                span: span.clone(),

                kind:
                    ExpressionKind::Operation(
                        Operation {
                            left: Box::new(left),
                            operator,
                            right:
                                Box::new(right),
                            span,
                        }
                    ),
            };
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> ParseResult<Expression>
    {
        let token =
            self.advance().clone();

        match token.token_type {

            TokenType::Number(value) => {

                let number =
                    value.parse::<i64>()
                        .unwrap_or(0);

                Ok(
                    Expression {
                        span:
                            token.span.clone(),

                        kind:
                            ExpressionKind::Value(
                                Value::Int(number)
                            ),
                    }
                )
            }

            TokenType::StringLiteral(value) => {

                Ok(
                    Expression {
                        span:
                            token.span.clone(),

                        kind:
                            ExpressionKind::Value(
                                Value::String(value)
                            ),
                    }
                )
            }

            TokenType::Identifier(name) => {

                if self.check(&TokenType::LParen)
                {
                    return self.parse_function_call(
                        name,
                        token.span.clone()
                    );
                }

                Ok(
                    Expression {
                        span: token.span.clone(),

                        kind:
                            ExpressionKind::Variable(name),
                    }
                )
            }
            _ => Err(
                CompilerError {
                    file: self.file.clone(),
                    message:
                        "expression invalide"
                            .to_string(),
                    span:
                        token.span.clone(),
                }
            )
        }
    }

    pub fn parse(&mut self,path: String,content: String) -> ParseResult<SourceFile>
    {
        let mut items = Vec::new();

        while !self.is_at_end() {
            items.push(self.parse_item()?);
        }

        Ok(
            SourceFile {
                path,
                content,
                items,
            }
        )
    }

    fn parse_function_call(&mut self,name: String,span: Span,) -> ParseResult<Expression>
    {
        self.consume(
            TokenType::LParen,
            "'(' attendu après le nom de fonction"
        )?;

        let mut arguments = Vec::new();

        while !self.check(&TokenType::RParen)
        {
            arguments.push(
                self.parse_expression()?
            );

            if self.check(&TokenType::Comma)
            {
                self.advance();
            }
            else
            {
                break;
            }
        }

        self.consume(
            TokenType::RParen,
            "')' attendu"
        )?;

        Ok(
            Expression {
                span: span.clone(),

                kind: ExpressionKind::FunctionCall(
                    FunctionCall {
                        name,
                        arguments,
                        span,
                    }
                ),
            }
        )
    }

    fn parse_item(&mut self) -> ParseResult<FileItem>
    {
        match &self.peek().token_type {

            TokenType::Function => {
                Ok(
                    FileItem::Function(
                        self.parse_function()?
                    )
                )
            }

            _ => Err(
                CompilerError {
                    file: self.file.clone(),
                    message:
                        "Item inattendu".to_string(),
                    span:
                        self.peek().span.clone(),
                }
            )
        }
    }

    fn parse_source_file(&mut self,path: String,content: String) -> ParseResult<SourceFile>
    {
        let mut items = Vec::new();

        while !self.is_at_end()
        {
            items.push(self.parse_item()?);
        }

        Ok(SourceFile {
            path,
            content,
            items,
        })
    }

}

pub fn parse_all_tokens(tokens: Vec<Token>,file: String,content: String,) -> ParseResult<Program>
{
    let mut parser = Parser {
        tokens,
        current: 0,
        file: file.clone(),
    };

    let mut files = Vec::new();

    let source_file = parser.parse_source_file(file, content)?;

    files.push(source_file);

    Ok(Program { files })
}
