// ライフタイムは構造体が参照を持つ際にも必要になります。

// TODO: 構造体に関するコンパイルエラーを修正してください。
// 構造体のフィールドとして参照を持つ場合
// ライフタイムを指定する必要がある
// その参照がどの程度の期間有効であるか
// コンパイラに明示する必要がある
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
