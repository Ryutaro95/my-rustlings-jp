// テストはコードが意図したように動作しているか確認するために重要です。

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    // TODO: `is_even`をインポートしてください。外部にあるモジュールをインポートするためにワイルドカードを使うことができます。
    use super::is_even;

    #[test]
    fn you_can_assert() {
        // TODO: いくつかの値を入れて、`is_even`の機能を試してみてください。
        assert!(is_even(10));
        assert!(is_even(92));
    }
}
