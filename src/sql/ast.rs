use bson::Bson;
// use sqlparser::dialect::GenericDialect;
// use sqlparser::parser::Parser;
// use sqlparser::ast::{Statement, Expr, Value};

#[derive(Debug)]
pub enum SQLStatement {
    Insert {
        collection: String,
        document: Bson,
    },
    Select {
        collection: String,
        filter: Option<(String, String)>,
    },
    Commit,
    Rollback,
    Begin,
}

struct FilterExpr {
    field: String,
    op: String, // "=", ">", etc.
    value: bson::Bson,
}

// fn parse_sql_to_ast(sql: &str) -> Result<SQLStatement, Box<dyn std::error::Error>> {
//     let dialect = GenericDialect {};
//     let statements = Parser::parse_sql(&dialect, sql)?;
//     let stmt = &statements[0];

//     match stmt {
//         Statement::Insert { table_name, source, .. } => {
//             let table = table_name.to_string();

//             // parse JSON inside VALUES
//             let values = match source.body.as_ref() {
//                 sqlparser::ast::SetExpr::Values(v) => &v.0,
//                 _ => return Err("Expected VALUES clause".into()),
//             };

//             let json_literal = match &values[0][0] {
//                 Expr::Value(Value::SingleQuotedString(s)) => s,
//                 _ => return Err("Expected JSON as string".into()),
//             };

//             let json_doc: serde_json::Value = serde_json::from_str(json_literal)?;
//             let bson_doc = bson::to_bson(&json_doc)?;

//             Ok(SQLStatement::Insert {
//                 collection: table,
//                 document: bson_doc,
//             })
//         }

//         Statement::Query(q) => {
//             let table = q.body.get_table_name().unwrap_or("unknown").to_string();
//             let where_clause = q.selection.as_ref().map(|expr| {
//                 if let Expr::BinaryOp { left, op: _, right } = expr {
//                     let field = left.to_string();
//                     let value = match &**right {
//                         Expr::Value(Value::SingleQuotedString(s)) => s.clone(),
//                         _ => "???".to_string(),
//                     };
//                     (field, value)
//                 } else {
//                     ("??".to_string(), "??".to_string())
//                 }
//             });

//             Ok(SQLStatement::Select {
//                 collection: table,
//                 filter: where_clause,
//             })
//         }

//         Statement::StartTransaction { .. } => Ok(SQLStatement::Begin),
//         Statement::Commit { .. } => Ok(SQLStatement::Commit),
//         Statement::Rollback { .. } => Ok(SQLStatement::Rollback),

//         _ => Err("Unsupported SQL statement".into()),
//     }
// }
