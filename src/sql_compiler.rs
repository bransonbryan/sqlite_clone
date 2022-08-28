use scanf::sscanf;

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
    row: Row, // Only used by insert TODO refactor to a better model
    query: String,
}

impl Statement {
    pub fn new(buffer: &str) -> Result<Statement, SqlCompilerState> {
        parse_statement_type(buffer)
          .map(|st| Statement { row: Row::new(buffer, &st).unwrap(), statement_type: st, query: String::from(buffer),  })
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

#[derive(Debug, PartialEq)]
pub struct Row {
    id: u32,
    username: String,
    email: String,
}

impl Row {

    pub fn new(buffer: &str, statement_type: &StatementType) -> Result<Row, SqlCompilerState> {
        match statement_type {
            StatementType::Insert => parse_insert(buffer),
            StatementType::Select => Ok(Row {id: 0, username: String::new(), email: String::new()}),
        }
    }
}

fn parse_insert(buffer: &str) -> Result<Row, SqlCompilerState> {
    let mut query_type = String::new();
    let mut id: u32 = 0;
    let mut username: String = String::new();
    let mut user_email: String = String::new();

    let parsed = sscanf!(buffer, "{string} {u32} {string} {string}", query_type, id, username, user_email);

    match parsed {
        Ok(_) => Ok(Row {id: id, username: username, email: user_email,}),
        Err(_) => Err(SqlCompilerState::UnrecognizedStatement),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_success() {
        let insert_query = "Insert 1 bbranson bbranson@gmail.com";

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

    #[test]
    fn create_row() {
        let query = "INSERT 1 bbranson bbranson@gmail.com";

        let result = Row::new(query, &StatementType::Insert);

        let expected = Row {id: 1, username: "bbranson".to_string(), email: "bbranson@gmail.com".to_string()};

        match result {
            Ok(r) => assert_eq!(r, expected),
            Err(_) => assert!(true == false),

        }
    }
}
