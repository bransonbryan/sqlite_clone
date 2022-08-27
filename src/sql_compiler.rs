const SELECT: &str = "select";
const INSERT: &str = "insert";

#[derive(Debug, PartialEq)]
pub enum SqlCompilerState {
    PrepareSuccess,
    UnrecognizedStatement,
}

#[derive(Debug, PartialEq)]
pub enum StatementType {
    Insert,
    Select,
}

#[derive(Debug)]
pub struct Statement {
    statement_type: StatementType,
    query: String,
}

impl Statement {
    pub fn new(buffer: &str) -> Result<Statement, SqlCompilerState> {
        parse_statement_type(buffer)
          .map(|st| Statement { statement_type: st, query: String::from(buffer) })
    }

    pub fn execute(&self) -> Result<SqlCompilerState, bool> {
        todo!()
    }
}

fn parse_statement_type(buffer: &str) -> Result<StatementType, SqlCompilerState> {
    let first_word = buffer.split_whitespace().next().unwrap_or("");

    match first_word.to_lowercase().as_str() {
        SELECT => Ok(StatementType::Select),
        INSERT => Ok(StatementType::Insert),
        _ => Err(SqlCompilerState::UnrecognizedStatement),
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_success() {
        let insert_query = "Insert into";

        let result = Statement::new(insert_query);

        match result {
            Ok(s) => {
                assert_eq!(s.statement_type, StatementType::Insert);
                assert!(s.query == insert_query);
            }
            Err(_) => assert!(false == true)
        }
    }


    #[test]
    fn select_success() {
        let select_query = "SELECT from";

        let result = Statement::new(select_query);

        match result {
            Ok(s) => {
                assert_eq!(s.statement_type, StatementType::Select);
                assert!(s.query == select_query);
            }
            Err(_) => assert!(false == true)
        }
    }

    #[test]
    fn unrecognized_statement() {
        let query = "Hello my name is";

        let result = Statement::new(query);

        match result {
            Ok(_) => assert!(false == true),
            Err(e) => assert_eq!(e, SqlCompilerState::UnrecognizedStatement),
        }
    }
}
