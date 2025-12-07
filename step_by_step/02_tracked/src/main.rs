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
    #[tracked]
    lines: Vec<String>,
}

#[salsa::tracked]
fn parse_source(db: &dyn salsa::Database, source: SourceProgram) -> Program<'_> {
    let text = source.text(db);
    let result: Vec<String> = text.as_str().lines().map(String::from).collect();
    Program::new(db, result)
}

fn main() {
    let db = RootDatabase::default();
    {
        let source = SourceProgram::new(&db, "Hello\nWorld\nEveryOne".to_string());
        let program = parse_source(&db, source);
        println!("Source: {:#?}", source);
        println!("Program: {:#?}", program);
    }
    {
        let source = SourceProgram::new(&db, "Hello\nWorld\nEveryOne".to_string());
        let program = parse_source(&db, source);
        println!("Source: {:#?}", source);
        println!("Program: {:#?}", program);
    }
}
