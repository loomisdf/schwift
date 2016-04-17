use super::schwift_grammar;
use super::{ Value, Statement, Expression };

#[test]
fn test_raw_int() {
    let l = schwift_grammar::int("3").unwrap();
    assert_eq!(l, 3)
}

#[test]
fn test_raw_string() {
    let l = schwift_grammar::string("\"hello!\"").unwrap();
    assert_eq!(l, Value::Str("hello!".to_string()))
}

#[test]
fn test_expression_string() {
    let l = schwift_grammar::expression("\"hello!\"").unwrap();
    assert_eq!(l, Expression::Value(Value::Str("hello!".to_string())));
}

#[test]
fn test_list_instantiation() {
    let l = schwift_grammar::list_instantiation("foobar on a cob").unwrap();
    assert_eq!(l, Statement::ListNew("foobar".to_string()));
}

#[test]
fn test_list_instantiation_statement() {
    let l = schwift_grammar::statement("foobar on a cob").unwrap();
    assert_eq!(l, Statement::ListNew("foobar".to_string()));
}

#[test]
fn test_list_append_statement() {
    let l = schwift_grammar::statement("foobar assimilate 10").unwrap();
    assert_eq!(l, Statement::ListAppend("foobar".to_string(), Expression::Value(Value::Int(10))));

}

#[test]
fn test_list_assign() {
    let l = schwift_grammar::statement("foobar[30] squanch 10").unwrap();
    assert_eq!(l, Statement::ListAssign("foobar".to_string(), Expression::Value(Value::Int(30)),  Expression::Value(Value::Int(10))));
}

#[test]
fn test_printing() {
    let l = schwift_grammar::statement("show me what you got \"Hello\"").unwrap();
    assert_eq!(l, Statement::Print(Expression::Value(Value::Str("Hello".to_string()))));
}

#[test]
fn test_list_length() {
    let l = schwift_grammar::expression("apple squanch").unwrap();
    assert_eq!(l, Expression::ListLength("apple".to_string()));
}

#[test]
fn test_not() {
    let l = schwift_grammar::expression("!foo").unwrap();
    assert_eq!(l, Expression::Not(Box::new(Expression::Variable("foo".to_string()))));
}

#[test]
fn test_while() {
    let l = schwift_grammar::while_loop(r#"while x :<
    show me what you got 30
    >:"#).unwrap();
    assert_eq!(
        l,
        Statement::While(
            Expression::Variable("x".to_string()),
            vec![
                Statement::Print(Expression::Value(Value::Int(30))),
            ],
        )
    );
}

#[test]
fn test_block() {
    let l = schwift_grammar::block(r#":<
show me what you got 10




portal gun x


    >:"#).unwrap();
    assert_eq!(
        l,
        vec![
            Statement::Print(Expression::Value(Value::Int(10))),
            Statement::Input("x".to_string()),
        ]
    );
}

#[test]
fn test_input() {
    let l = schwift_grammar::statement(r"portal gun x").unwrap();
    assert_eq!(
        l,
        Statement::Input("x".to_string())
    );
}
