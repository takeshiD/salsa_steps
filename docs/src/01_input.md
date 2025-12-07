# Input Struct
salsaにおいてのInput Structは計算の全ての出発点になります。
コンパイラや型検査機などの場合、ソースコードをInputとして扱うことが多いです。
単純なStringをフィールドに持った構造体として次のように定義します。

```rust
#[salsa::input]
struct SourceProgram {
    text: String,
}
```

`salsa::input`マクロを使うことで次のことが自動で行われます。
- コンストラクタの生成
    - `fn new(&db, text: String) -> SourceProgram`というコンストラクタが生える
- getterの生成
    - `fn text(&db) -> String`というgetterが生える
- setterの生成
    - `fn set_text(&mut db)`というsetterが生える

getterで値を得る場合次のように使います。
```rust
let source: Stirng = source.text(&db);
```

## 参照を返す
通常のgetterではDatabaseに保存された値がCloneされたものが返されます。
巨大なテキストなどを格納している場合は都合が悪いことがあります。

salsaでは参照を返すように変更することが出来ます。

```rust
#[salsa::input(debug)]
struct SourceProgram {
    #[returns(ref)]
    text: String,
}
```

この場合getterは`&String`を返すようになります。
```rust
let source: &Stirng = source.text(&db);
```


## debug
Input Structは`debug`を付加するとDebugが実装されます。

```rust
#[salsa::input(debug)]
struct SourceProgram {
    text: String,
}
```


# ソースコード全体
```rust
// Database
#[salsa::db]
#[derive(Clone, Default)]
struct RootDatabase {
    storage: salsa::Storage<Self>,
}
#[salsa::db]
impl salsa::Database for RootDatabase {}

// Input Struct
#[salsa::input]
struct SourceProgram {
    text: String,
}

fn main() {
    let db = RootDatabase::default();
    let source = SourceProgram::new(&db, "print 11 + 11".to_string());
    println!("{:#?}", source.0);
    println!("{:#?}", source.text(&db));
    let source = SourceProgram::new(&db, "print 11 + 11".to_string());
    println!("{:#?}", source.0);
    println!("{:#?}", source.text(&db));
    let source = SourceProgram::new(&db, "print 11 + 12".to_string());
    println!("{:#?}", source.0);
    println!("{:#?}", source.text(&db));
}
```
