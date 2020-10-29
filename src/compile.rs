use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Loc {
    Imm(i64),
    Reg(usize),
    RegDeref(usize),
    AddrStack(usize), //stack offset
    Addr(usize) //absolute
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
pub struct Compiler {
    instructions: Vec<Instr>,
    labels: HashMap<String, usize>, // mem offsets
    variables: HashMap<String, usize>,
    unlinked: Vec<(String, usize)>,
    var_offset: usize,
    temp_label_num: usize
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            labels: HashMap::new(),
            variables: HashMap::new(),
            unlinked: Vec::new(),
            var_offset: 0,
            temp_label_num: 0
        }
    }

    pub fn compile_program(&mut self, program: &Program) {
        for fun in program.functions.iter() {
            self.labels.insert(fun.name.clone(), self.instructions.len());
            for stmt in fun.body.iter() {
                self.compile_stmt(stmt);
            }
        }
    }

    fn link_program() {
        unimplemented!();
    }
    
    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::ExprStmt(expr) => self.compile_expr(expr),
            Stmt::Assignment(name, expr) => {
                self.compile_expr(expr);
                let var_addr = self.get_var_offset(name);
                self.instructions.push(Instr::Mov(Loc::Reg(0), Loc::AddrStack(var_addr)));
            },
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    self.compile_expr(expr);
                }
                self.instructions.push(Instr::Ret);
            },
            Stmt::Conditional(cond, yes, no) => { //TODO: yes and no should be vecs of stmts, not expression
                self.compile_expr(cond); //eval condition
                self.instructions.push(Instr::Cmp(Loc::Imm(0), Loc::Reg(0))); //set zero flag based on r0
                self.instructions.push(Instr::Jez(Loc::Addr(0))); //jump to false branch if zero flag set

                //true branch
                self.compile_expr(yes);
                
                //false branch
                let temp_label = format!("_{}", self.temp_label_num);
                self.temp_label_num += 1;
                self.unlinked.push((temp_label, self.instructions.len() - 1));
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
                self.instructions.push(Instr::Mov(Loc::Imm(*v as i64), Loc::Reg(0)))
            },
            Expr::Binary(lhs, op, rhs) => {
                self.compile_expr(lhs);
                self.instructions.push(Instr::Mov(Loc::Reg(0), Loc::Reg(1)));
                self.compile_expr(rhs);
                self.instructions.push(Instr::BinOp(Loc::Reg(0), op.clone(), Loc::Reg(1)));
            },
            Expr::Unary(op, rhs) => {
                unimplemented!();
            },
            Expr::Call(name, params) => {
                /*let num_params = params.len();
                for param in params.iter().rev() {
                    self.compile_expr(params);
                    self.instructions.push();
                }*/
                unimplemented!();
            },
            Expr::Symbol(name) => {
                let var_addr = self.get_var_offset(name);
                self.instructions.push(Instr::Mov(Loc::AddrStack(var_addr), Loc::Reg(0)))
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




