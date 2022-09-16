use std::result;

macro_rules! operate {
  ($code:expr, $op:tt) => {{
      if let Some(a) = $code.stack.pop() {
          if let Some(b) = $code.stack.pop() {
              $code.stack.push(Variable {
                  variable: None,
                  value: (b.value $op a.value),
              });
              None
          } else { Some(ProgramError::StackUnderflow) }
      } else { Some(ProgramError::StackUnderflow) }
  }
}}

pub fn interpret(bytecodes: Vec<ByteCode>) -> Result<Variable> {
  let mut code = Program {
      bytecodes,
      stack: Vec::new(),
  };

  for op in code.bytecodes {
      if let Some(err) = match op {
          ByteCode::LoadVal(i) => {
              code.stack.push(Variable {
                  variable: None,
                  value: i,
              });
              None
          },
          ByteCode::ReadVar(c) => {
              let read_value = code.stack.iter().find(|&&x| x.variable == Some(c));
              if let Some(v) = read_value {
                  let var = v.clone();
                  code.stack.push(Variable {
                      variable: var.variable,
                      value: var.value,
                  })
              }
              None
          },
          ByteCode::WriteVar(c) => {
              let loaded_value = code.stack.pop();
              if let Some(v) = loaded_value {
                  code.stack.push(Variable {
                      variable: Some(c),
                      value: v.value,
                  })
              }
              None
          },
          ByteCode::Add => operate!(code, +),
          ByteCode::Sub => operate!(code, -),
          ByteCode::Mul => operate!(code, *),
          ByteCode::Div => operate!(code, /),
          ByteCode::Return => break,
      } {
          return Err(err);
      }
  }

  if let Some(v) = code.stack.pop() {
      Ok(v)
  } else {
      Err(ProgramError::StackUnderflow)
  }
}

#[derive(Copy, Clone)]
pub enum ByteCode {
    LoadVal(i64),
    WriteVar(char),
    ReadVar(char),
    Add,
	  Sub,
    Mul,
	  Div,
    Return,
}

#[derive(Copy, Clone, Debug)]
pub struct Variable {
    pub variable: Option<char>,
    pub value: i64,
}

#[derive(Clone)]
pub struct Program {
    pub bytecodes: Vec<ByteCode>,
    pub stack: Vec<Variable>,
}

#[derive(Debug)]
pub enum ProgramError {
    StackUnderflow,
}

pub type Result<T> = result::Result<T, ProgramError>;
