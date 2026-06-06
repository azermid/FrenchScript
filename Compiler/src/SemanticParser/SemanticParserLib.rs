use std::collections::HashMap;

use crate::*;
use crate::CompilerError;

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticAnalyzer {
    functions: HashMap<String, Function>,
}

impl SemanticAnalyzer {

    pub fn new() -> Self
    {
        Self {
            functions: HashMap::new(),
        }
    }

    pub fn analyze(&mut self,program: &Program) -> ParseResult<()>
    {
        self.collect_functions(program)?;

        self.analyze_functions(program)?;

        Ok(())
    }

    fn analyze_function(&self,function: &Function,file: &SourceFile,) -> ParseResult<()>
    {
        let mut variables =
            HashMap::<String, Type>::new();

        for param in &function.parameters {

            variables.insert(
                param.name.clone(),
                param.arg_type.clone(),
            );
        }

        for statement in &function.body {

            self.analyze_statement(
                statement,
                file,
                &mut variables,
            )?;
        }

        Ok(())
    }

    fn analyze_functions(&self,program: &Program,) -> ParseResult<()>
    {
        for file in &program.files {

            for item in &file.items {

                if let FileItem::Function(func) = item {

                    self.analyze_function(
                        func,
                        file,
                    )?;
                }
            }
        }

        Ok(())
    }

    fn analyze_statement(&self,statement: &Statement,file: &SourceFile,variables: &mut HashMap<String, Type>,) -> ParseResult<()>
    {
        match statement {

            Statement::VariableDeclaration(var) => {

                if variables.contains_key(
                    &var.name
                ) {
                    return Err(
                        CompilerError {
                            file: file.path.clone(),
                            message: format!(
                                "Variable '{}' déjà déclarée",
                                var.name
                            ),
                            span: var.span.clone(),
                        }
                    );
                }

                if let Some(expr) = &var.value {

                    let expr_type =
                        self.resolve_expression_type(
                            expr,
                            file,
                            variables,
                        )?;

                    if expr_type != var.var_type {

                        return Err(
                            CompilerError {
                                file: file.path.clone(),
                                message: format!(
                                    "Type attendu {:?} mais {:?} reçu",
                                    var.var_type,
                                    expr_type,
                                ),
                                span: var.span.clone(),
                            }
                        );
                    }
                }

                variables.insert(
                    var.name.clone(),
                    var.var_type.clone(),
                );
            }

            Statement::Return(ret) => {

                if let Some(expr) = &ret.value {

                    self.resolve_expression_type(
                        expr,
                        file,
                        variables,
                    )?;
                }
            }

            Statement::Expression(expr) => {

                self.resolve_expression_type(
                    expr,
                    file,
                    variables,
                )?;
            }
        }

        Ok(())
    }

    fn resolve_expression_type(&self,expression: &Expression,file: &SourceFile,variables: &HashMap<String, Type>,) -> ParseResult<Type>
    {
        match &expression.kind {

            ExpressionKind::Value(value) => {

                match value {

                    Value::Int(_) =>
                        Ok(Type::Int),

                    Value::Float(_) =>
                        Ok(Type::Float),

                    Value::String(_) =>
                        Ok(Type::Text),
                }
            }

            ExpressionKind::Variable(name) => {

                variables
                    .get(name)
                    .cloned()
                    .ok_or(
                        CompilerError {
                            file: file.path.clone(),
                            message: format!(
                                "Variable inconnue '{}'",
                                name
                            ),
                            span: expression.span.clone(),
                        }
                    )
            }

            ExpressionKind::FunctionCall(call) => {

                let function =
                    self.functions
                        .get(&call.name)
                        .ok_or(
                            CompilerError {
                                file: file.path.clone(),
                                message: format!(
                                    "Fonction inconnue '{}'",
                                    call.name
                                ),
                                span: call.span.clone(),
                            }
                        )?;

                if function.parameters.len()
                    != call.arguments.len()
                {
                    return Err(
                        CompilerError {
                            file: file.path.clone(),
                            message: format!(
                                "La fonction '{}' attend {} arguments mais {} reçu(s)",
                                call.name,
                                function.parameters.len(),
                                call.arguments.len()
                            ),
                            span: call.span.clone(),
                        }
                    );
                }

                for (arg, param)
                    in call.arguments.iter()
                        .zip(
                            function.parameters.iter()
                        )
                {
                    let arg_type =
                        self.resolve_expression_type(
                            arg,
                            file,
                            variables,
                        )?;

                    if arg_type != param.arg_type {

                        return Err(
                            CompilerError {
                                file: file.path.clone(),
                                message: format!(
                                    "Argument '{}' invalide",
                                    param.name
                                ),
                                span: arg.span.clone(),
                            }
                        );
                    }
                }

                Ok(
                    function.return_type.clone()
                )
            }

            ExpressionKind::Operation(op) => {

                let left =
                    self.resolve_expression_type(
                        &op.left,
                        file,
                        variables,
                    )?;

                let right =
                    self.resolve_expression_type(
                        &op.right,
                        file,
                        variables,
                    )?;

                if left != right {

                    return Err(
                        CompilerError {
                            file: file.path.clone(),
                            message:
                                "Types incompatibles dans l'opération"
                                    .to_string(),
                            span: op.span.clone(),
                        }
                    );
                }

                Ok(left)
            }
        }
    }
    
    fn collect_functions(&mut self,program: &Program,) -> ParseResult<()>
    {
        for file in &program.files {
            for item in &file.items {
                if let FileItem::Function(func) = item {
                    if self.functions.contains_key(&func.name)
                    {
                        return Err(
                            CompilerError {
                                file: file.path.clone(),
                                message: format!(
                                    "Fonction '{}' déjà déclarée",
                                    func.name
                                ),
                                span: func.span.clone(),
                            }
                        );
                    }
    
                    self.functions.insert(
                        func.name.clone(),
                        func.clone(),
                    );
                }
            }
        }
    
        Ok(())
    }
}
