use std::cmp;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: この関数はいつも`Ok`を返すべきではない。

        // other answer
        // if value.is_positive() {
        //     Ok(Self(value as u64))
        // } else if value.is_negative() {
        //     Err(CreationError::Negative)
        // } else {
        //     Err(CreationError::Zero)
        // }

        match value.cmp(&0) {
            cmp::Ordering::Less => Err(CreationError::Negative),
            cmp::Ordering::Equal => Err(CreationError::Zero),
            cmp::Ordering::Greater => Ok(Self(value as u64)),
        }
    }
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
