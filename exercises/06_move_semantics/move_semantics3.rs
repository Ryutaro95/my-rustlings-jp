// TODO: 行を追加することなく、コンパイルエラーを修正してください。
fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(88);
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let mut vec0 = vec![22, 44, 66];
        fill_vec(&mut vec0);
        assert_eq!(vec0, [22, 44, 66, 88]);
    }
}
