use crate::*;


#[derive(Debug, Clone)]
pub enum IRValue {
    Temp(String),
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub enum IRInstruction {

    Load {
        dst: String,
        variable: String,
    },

    Store {
        variable: String,
        src: IRValue,
    },

    Add {
        dst: String,
        left: IRValue,
        right: IRValue,
    },

    Sub {
        dst: String,
        left: IRValue,
        right: IRValue,
    },

    Mul {
        dst: String,
        left: IRValue,
        right: IRValue,
    },

    Div {
        dst: String,
        left: IRValue,
        right: IRValue,
    },

    Call {
        dst: String,
        function: String,
        arguments: Vec<IRValue>,
    },

    Return(IRValue),
}

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub instructions: Vec<IRInstruction>,
}

#[derive(Debug, Clone)]
pub struct IRProgram {
    pub functions: Vec<IRFunction>,
}

pub struct IRGenerator {
    temp_counter: usize,
}

impl IRGenerator {

    pub fn new() -> Self
    {
        Self {
            temp_counter: 0,
        }
    }
    
    fn new_temp(&mut self) -> String
    {
        let name =
            format!("t{}", self.temp_counter);

        self.temp_counter += 1;

        name
    }

    fn generate_call(&mut self,call: &FunctionCall,instructions: &mut Vec<IRInstruction>) -> IRValue
    {
        let mut args =
            Vec::new();

        for arg in &call.arguments {

            args.push(
                self.generate_expression(
                    arg,
                    instructions,
                )
            );
        }

        let temp =
            self.new_temp();

        instructions.push(
            IRInstruction::Call {

                dst:
                    temp.clone(),

                function:
                    call.name.clone(),

                arguments:
                    args,
            }
        );

        IRValue::Temp(temp)
    }

    fn generate_operation(&mut self,op: &Operation,instructions: &mut Vec<IRInstruction>) -> IRValue
    {
        let left =
            self.generate_expression(
                &op.left,
                instructions,
            );

        let right =
            self.generate_expression(
                &op.right,
                instructions,
            );

        let temp =
            self.new_temp();

        match op.operator {

            TokenOperation::Add => {

                instructions.push(
                    IRInstruction::Add {

                        dst:
                            temp.clone(),

                        left,

                        right,
                    }
                );
            }

            TokenOperation::Subtract => {

                instructions.push(
                    IRInstruction::Sub {

                        dst:
                            temp.clone(),

                        left,

                        right,
                    }
                );
            }

            TokenOperation::Multiply => {

                instructions.push(
                    IRInstruction::Mul {

                        dst:
                            temp.clone(),

                        left,

                        right,
                    }
                );
            }

            TokenOperation::Divide => {

                instructions.push(
                    IRInstruction::Div {

                        dst:
                            temp.clone(),

                        left,

                        right,
                    }
                );
            }
        }

        IRValue::Temp(temp)
    }

    fn generate_expression(&mut self,expression: &Expression,instructions: &mut Vec<IRInstruction>) -> IRValue
    {
        match &expression.kind {

            ExpressionKind::Value(value) => {

                match value {

                    Value::Int(v) =>
                        IRValue::Int(*v),

                    Value::Float(v) =>
                        IRValue::Float(*v),

                    Value::String(v) =>
                        IRValue::String(v.clone()),
                }
            }

            ExpressionKind::Variable(name) => {

                let temp =
                    self.new_temp();

                instructions.push(
                    IRInstruction::Load {

                        dst:
                            temp.clone(),

                        variable:
                            name.clone(),
                    }
                );

                IRValue::Temp(temp)
            }

            ExpressionKind::Operation(op) => {

                self.generate_operation(
                    op,
                    instructions,
                )
            }

            ExpressionKind::FunctionCall(call) => {

                self.generate_call(
                    call,
                    instructions,
                )
            }
        }
    }

    fn generate_statement(&mut self,statement: &Statement,instructions: &mut Vec<IRInstruction>)
    {
        match statement {

            Statement::VariableDeclaration(var) => {

                if let Some(expr) = &var.value {

                    let value =
                        self.generate_expression(
                            expr,
                            instructions,
                        );

                    instructions.push(
                        IRInstruction::Store {

                            variable:
                                var.name.clone(),

                            src:
                                value,
                        }
                    );
                }
            }

            Statement::Return(ret) => {

                if let Some(expr) = &ret.value {

                    let value =
                        self.generate_expression(
                            expr,
                            instructions,
                        );

                    instructions.push(
                        IRInstruction::Return(
                            value
                        )
                    );
                }
            }

            Statement::Expression(expr) => {

                self.generate_expression(
                    expr,
                    instructions,
                );
            }
        }
    }

    fn generate_function(&mut self,function: &Function) -> IRFunction
    {
        self.temp_counter = 0;

        let mut instructions =
            Vec::new();

        for statement in &function.body {

            self.generate_statement(
                statement,
                &mut instructions,
            );
        }

        IRFunction {
            name:
                function.name.clone(),

            instructions,
        }
    }

    pub fn generate(&mut self,program: &Program) -> IRProgram
    {
        let mut functions =
            Vec::new();

        for file in &program.files {

            for item in &file.items {

                if let FileItem::Function(func) = item {

                    functions.push(
                        self.generate_function(func)
                    );
                }
            }
        }

        IRProgram {
            functions
        }
    }
}