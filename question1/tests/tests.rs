use question1::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_arithmetic_written_values() {
        use ByteCode::*;

        let test_arithmetic_written_values = vec![
            LoadVal(1),
            WriteVar('x'),
            LoadVal(2),
            WriteVar('y'),
            ReadVar('x'),
            LoadVal(1),
            Add,
            ReadVar('y'),
            Mul,
            LoadVal(3),
            Sub,
            Return,
        ];

        assert_eq!(interpret(test_arithmetic_written_values).unwrap().value, 1, "Wrong Result!");
    }
}
