use std::result;

macro_rules! operate {
  ($stack:expr, $op:tt) => {{
      if let Some(a) = $stack.pop() {
          if let Some(b) = $stack.pop() {
              $stack.push(Variable {
                  variable: None,
                  value: (b.value $op a.value),
              });
              None
          } else { Some(ProgramError::StackUnderflow) }
      } else { Some(ProgramError::StackUnderflow) }
  }
}}

pub fn interpret(bytecodes: Vec<ByteCode>) -> Result<Variable> {
  let mut stack = Vec::new();
	let mut i = 0;
	while i < bytecodes.len() {
		let op = bytecodes[i];
		if let Some(err) = match op {
			ByteCode::LoadVal(i) => {
				stack.push(Variable {
					variable: None,
					value: i,
				});
				None
			},
			ByteCode::ReadVar(c) => {
				let read_value = stack.iter().rfind(|&&x| x.variable == Some(c));
				if let Some(v) = read_value {
					let var = v.clone();
					stack.push(Variable {
						variable: var.variable,
						value: var.value,
					})
				}
				None
			},
			ByteCode::WriteVar(c) => {
				let loaded_value = stack.pop();
				if let Some(v) = loaded_value {
					stack.push(Variable {
						variable: Some(c),
						value: v.value,
					})
				}
				None
			},
			ByteCode::Add => operate!(stack, +),
			ByteCode::Sub => operate!(stack, -),
			ByteCode::Mul => operate!(stack, *),
			ByteCode::Div => operate!(stack, /),
			ByteCode::Return => break,
			ByteCode::Loop => {
				let loaded_value = stack.pop();
				if let Some(v) = loaded_value {
					let var = v.value;
					if var == 0 {
						i = (&bytecodes[i..bytecodes.len()]).iter().position(|&x| matches!(x, ByteCode::EndLoop)).unwrap() + i;
					}
				}
				None
			},
			ByteCode::EndLoop => {
				i = (&bytecodes[0..i]).iter().rposition(|&x| matches!(x, ByteCode::Loop)).unwrap() - 2;
				None
			},
		} {
			return Err(err);
		}
		i = i + 1;
	}

	if let Some(v) = stack.pop() {
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
    Loop,
    EndLoop,
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
