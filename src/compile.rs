use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Loc {
    Imm(i64),
    Reg(usize),
    RegDeref(usize),
    AddrStack(usize), //stack offset
    Label(String) //absolute
}

#[derive(Debug, Clone)]
pub enum Instr {
    BinOp(Loc, Op, Loc),
    UnOp(Op, Loc),
    
    Mov(Loc, Loc),
    
    Cmp(Loc, Loc),

    Jmp(Loc),
    Jez(Loc),
    Call(Loc),
    Ret,
}

#[derive(Debug, Clone)]
pub struct Blob {
    pub instructions: Vec<Instr>,
    pub labels: HashMap<String, usize>
}

#[derive(Debug, Clone)]
pub struct Compiler {
    blob: Blob,
    variables: HashMap<String, usize>,
    unlinked: Vec<(String, usize)>,
    var_offset: usize,
    temp_label_num: usize
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            blob: Blob {
                instructions: Vec::new(),
                labels: HashMap::new()
            },
            variables: HashMap::new(),
            unlinked: Vec::new(),
            var_offset: 0,
            temp_label_num: 0
        }
    }

    pub fn compile_program(mut self, program: &Program) -> Blob {
        for fun in program.functions.iter() {
            self.blob.labels.insert(fun.name.clone(), self.blob.instructions.len());
            for stmt in fun.body.iter() {
                self.compile_stmt(stmt);
            }
        }
        self.blob
    }
    
    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::ExprStmt(expr) => self.compile_expr(expr),
            Stmt::Assignment(name, expr) => {
                self.compile_expr(expr);
                let var_addr = self.get_var_offset(name);
                self.blob.instructions.push(Instr::Mov(Loc::Reg(0), Loc::AddrStack(var_addr)));
            },
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    self.compile_expr(expr);
                }
                // TODO: explicit return address set?
                self.blob.instructions.push(Instr::Ret);
            },
            Stmt::Conditional(cond, yes, no) => { //TODO: yes and no should be vecs of stmts, not expression
                let temp_label = format!("_{}", self.temp_label_num);
                self.temp_label_num += 1;

                self.compile_expr(cond); //eval condition
                self.blob.instructions.push(Instr::Cmp(Loc::Imm(0), Loc::Reg(0))); //set zero flag based on r0
                self.blob.instructions.push(Instr::Jez(Loc::Label(temp_label.clone()))); //jump to false branch if zero flag set

                //true branch
                self.compile_expr(yes);
                
                //false branch
                self.blob.labels.insert(temp_label, self.blob.instructions.len() - 1);
                if let Some(no) = no {
                    self.compile_expr(no);
                }
            },
        }
    }
    
    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Constant(v) => {
                //TODO: Remove this fucking truncation
                self.blob.instructions.push(Instr::Mov(Loc::Imm(*v as i64), Loc::Reg(0)))
            },
            Expr::Binary(lhs, op, rhs) => {
                self.compile_expr(lhs);
                self.blob.instructions.push(Instr::Mov(Loc::Reg(0), Loc::Reg(1)));
                self.compile_expr(rhs);
                self.blob.instructions.push(Instr::BinOp(Loc::Reg(0), op.clone(), Loc::Reg(1)));
            },
            Expr::Unary(op, rhs) => {
                unimplemented!();
            },
            Expr::Call(name, params) => {
                let num_params = params.len();
                for (i, param) in params.iter().enumerate() {
                    self.compile_expr(param);
                    self.blob.instructions.push(Instr::Mov(Loc::Reg(0), Loc::Reg(i+2)));
                }
                self.blob.instructions.push(Instr::Call(Loc::Label(name.clone())));
            },
            Expr::Symbol(name) => {
                let var_addr = self.get_var_offset(name);
                self.blob.instructions.push(Instr::Mov(Loc::AddrStack(var_addr), Loc::Reg(0)));
            },
        }
    }

    fn get_var_offset(&mut self, name: &String) -> usize {
        if self.variables.contains_key(name) {
            *self.variables.get(name).unwrap()
        }
        else {
            self.variables.insert(name.clone(), self.var_offset);
            self.var_offset += 1;

            self.var_offset - 1
        }
    }
}




