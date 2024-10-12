trait AppendBar {
    fn append_bar(&mut self);
}

// TODO: `AppendBar`を文字列の配列型に移植してください。
// `append_bar`は"Bar"を配列の末尾に追加するメソッドです。
impl AppendBar for Vec<String> {
    fn append_bar(&mut self) {
        self.push("Bar".to_string());
    }
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")];
        foo.append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
