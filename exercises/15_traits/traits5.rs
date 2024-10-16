trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// TODO: この関数の入出力を変更するだけでコンパイルエラーを修正してください。
// fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
// ↓は↑のシンタックスシュガー
// fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
// ↓whereを使った方法もある(これもシンタックスシュガー)
fn some_func<T>(item: T) -> bool
where
    T: SomeTrait + OtherTrait,
{
    item.some_function() && item.other_function()
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(SomeStruct));
        assert!(some_func(OtherStruct));
    }
}
