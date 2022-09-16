use question2::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_arithmetic_written_values() {
        use ByteCode::*;

        let test_arithmetic_written_values = vec![
            LoadVal(0),
            WriteVar('x'),
            LoadVal(10),
            WriteVar('y'),
            ReadVar('y'),
            Loop,
            ReadVar('y'),
            ReadVar('x'),
            Add,
            WriteVar('x'),
            ReadVar('y'),
            LoadVal(1),
            Sub,
            WriteVar('y'),
            EndLoop,
            ReadVar('x'),
            Return,
        ];

        assert_eq!(interpret(test_arithmetic_written_values).unwrap().value, 55, "Wrong Result!");
    }
}
