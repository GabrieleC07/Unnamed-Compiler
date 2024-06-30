use crate::parser::nodes::*;

pub struct CodeGenerator {
    output: String,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            output: String::new(),
        }
    }

    pub fn generate(&mut self, stmts: Vec<NodeStmt>) -> String {
        self.output.push_str("#include <stdio.h>\n\n");
        self.output.push_str("int main() {\n");
        for stmt in stmts {
            self.visit_stmt(stmt);
        }
        self.output.push_str("}\n");
        self.output.clone()
    }

    fn visit_stmt(&mut self, stmt: NodeStmt) {
        match stmt {
            NodeStmt::Return(expr) => self.visit_return(expr),
            NodeStmt::VarDecl(name, expr) => self.visit_var_decl(name, expr),
        }
    }

    fn visit_return(&mut self, expr: NodeExpr) {
        self.output.push_str("    ");
        self.output.push_str("return ");
        self.visit_expr(expr);
        self.output.push_str(";\n");
    }

    fn visit_var_decl(&mut self, name: String, expr: NodeExpr) {
        self.output.push_str("    ");
        self.output.push_str("int ");
        self.output.push_str(&name);
        self.output.push_str(" = ");
        self.visit_expr(expr);
        self.output.push_str(";\n");
    }

    fn visit_expr(&mut self, expr: NodeExpr) {
        match expr {
            NodeExpr::IntLiteral(value) => {
                self.output.push_str(&value.to_string());
            }
            NodeExpr::Identifier(name) => {
                self.output.push_str(&name);
            }
            NodeExpr::MathOperat(left, operator, right) => {
                self.visit_expr(*left);
                let string_op = match operator {
                    crate::lexer::tokens::Operator::Plus => "+",
                    crate::lexer::tokens::Operator::Minus => "-",
                    crate::lexer::tokens::Operator::Mul => "*",
                    crate::lexer::tokens::Operator::Div => "/",
                };
                self.output.push_str(string_op);
                self.visit_expr(*right);
            }
        }
    }
}