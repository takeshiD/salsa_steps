// Database
#[salsa::db]
#[derive(Clone, Default)]
struct RootDatabase {
    storage: salsa::Storage<Self>,
}
#[salsa::db]
impl salsa::Database for RootDatabase {}

// Input Struct
#[salsa::input(debug)]
struct SourceProgram {
    text: String,
}

// Tracked Struct
#[salsa::tracked(debug)]
struct Program<'db> {
    lines: Vec<String>,
}

fn main() {
    let db = RootDatabase::default();
    let program = Program::new(&db, vec!["hello".to_string(), "world".to_string()]);
    println!("{:#?}", program);
}
