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

fn main() {
    let db = RootDatabase::default();
    let source = SourceProgram::new(&db, "print 11 + 11".to_string());
    println!("Debug: {:#?}", source);
    println!("{:#?}", source.0);
    println!("{:#?}", source.text(&db));
    let source = SourceProgram::new(&db, "print 11 + 11".to_string());
    println!("{:#?}", source.0);
    println!("{:#?}", source.text(&db));
    let source = SourceProgram::new(&db, "print 11 + 12".to_string());
    println!("{:#?}", source.0);
    println!("{:#?}", source.text(&db));
}
