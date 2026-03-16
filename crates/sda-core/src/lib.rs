mod ast;
mod lexer;
mod number;
mod parser;

pub mod eval;
pub mod stdlib;

pub use eval::EvalError;
pub use lexer::LexError;
pub use number::{ExactNum, ParseNumError};
pub use parser::ParseError;

use ast::Expr;
use std::collections::HashMap;
use thiserror::Error;

pub type Env = HashMap<String, Value>;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct SdaRuntime;

impl SdaRuntime {
    #[must_use]
    pub fn name() -> &'static str {
        "sda-core"
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Num(ExactNum),
    Str(String),
    Seq(Vec<Value>),
    Set(Vec<Value>),
    Bag(Vec<Value>),
    Map(Vec<(String, Value)>),
    Prod(Vec<(String, Value)>),
    BagKV(Vec<(Value, Value)>),
    Bind(Box<Value>, Box<Value>),
    Some_(Box<Value>),
    None_,
    Ok_(Box<Value>),
    Fail_(String, String),
    Lambda(String, Box<Expr>, Box<Env>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Num(a), Value::Num(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Seq(a), Value::Seq(b)) => a == b,
            (Value::Set(a), Value::Set(b)) => set_equal(a, b),
            (Value::Bag(a), Value::Bag(b)) => multiset_equal(a, b),
            (Value::Map(a), Value::Map(b)) => kv_extensional_equal(a, b),
            (Value::Prod(a), Value::Prod(b)) => kv_extensional_equal(a, b),
            (Value::BagKV(a), Value::BagKV(b)) => bagkv_equal(a, b),
            (Value::Bind(k1, v1), Value::Bind(k2, v2)) => k1 == k2 && v1 == v2,
            (Value::Some_(a), Value::Some_(b)) => a == b,
            (Value::None_, Value::None_) => true,
            (Value::Ok_(a), Value::Ok_(b)) => a == b,
            (Value::Fail_(c1, m1), Value::Fail_(c2, m2)) => c1 == c2 && m1 == m2,
            (Value::Lambda(_, _, _), _) => false,
            (_, Value::Lambda(_, _, _)) => false,
            _ => false,
        }
    }
}

fn set_equal(left: &[Value], right: &[Value]) -> bool {
    left.len() == right.len()
        && left.iter().all(|left_value| right.iter().any(|right_value| left_value == right_value))
}

fn multiset_equal(left: &[Value], right: &[Value]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let mut matched = vec![false; right.len()];
    for left_value in left {
        let mut found = false;
        for (index, right_value) in right.iter().enumerate() {
            if !matched[index] && left_value == right_value {
                matched[index] = true;
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }

    true
}

fn kv_extensional_equal(left: &[(String, Value)], right: &[(String, Value)]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    left.iter().all(|(left_key, left_value)| {
        right
            .iter()
            .find(|(right_key, _)| right_key == left_key)
            .is_some_and(|(_, right_value)| left_value == right_value)
    })
}

fn bagkv_equal(left: &[(Value, Value)], right: &[(Value, Value)]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let mut matched = vec![false; right.len()];
    for (left_key, left_value) in left {
        let mut found = false;
        for (index, (right_key, right_value)) in right.iter().enumerate() {
            if !matched[index] && left_key == right_key && left_value == right_value {
                matched[index] = true;
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }

    true
}

#[derive(Debug, Error)]
pub enum SdaError {
    #[error("Lex error: {0}")]
    Lex(#[from] LexError),
    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),
    #[error("Eval error: {0}")]
    Eval(#[from] EvalError),
}

pub fn run(expr: &str, input: serde_json::Value) -> Result<serde_json::Value, SdaError> {
    run_with_input_binding(expr, "input", input)
}

pub fn run_with_input_binding(
    expr: &str,
    binding_name: &str,
    input: serde_json::Value,
) -> Result<serde_json::Value, SdaError> {
    let input_val = from_json(input);
    let expr_normalized = {
        let trimmed = expr.trim_end();
        if trimmed.ends_with(';') {
            trimmed.to_string()
        } else {
            format!("{};", trimmed)
        }
    };
    let tokens = lexer::lex(&expr_normalized)?;
    let program = parser::parse(tokens)?;
    let mut env = Env::new();
    env.insert(binding_name.to_string(), input_val);
    let result = eval::eval_program(&program, &mut env)?;
    Ok(to_json(result.unwrap_or(Value::Null)))
}

pub fn from_json(v: serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => {
            Value::Num(ExactNum::parse_literal(&n.to_string()).expect("serde_json number should parse exactly"))
        }
        serde_json::Value::String(s) => Value::Str(s),
        serde_json::Value::Array(arr) => Value::Seq(arr.into_iter().map(from_json).collect()),
        serde_json::Value::Object(obj) => {
            if let Some(serde_json::Value::String(ty)) = obj.get("$type") {
                match ty.as_str() {
                    "num" => {
                        if let Some(serde_json::Value::String(value)) = obj.get("$value") {
                            return Value::Num(
                                ExactNum::parse_canonical(value)
                                    .expect("canonical numeric wrapper should parse")
                            );
                        }
                    }
                    "set" => {
                        if let Some(serde_json::Value::Array(items)) = obj.get("$items") {
                            return Value::Set(items.iter().cloned().map(from_json).collect());
                        }
                    }
                    "bag" => {
                        if let Some(serde_json::Value::Array(items)) = obj.get("$items") {
                            return Value::Bag(items.iter().cloned().map(from_json).collect());
                        }
                    }
                    "prod" => {
                        if let Some(serde_json::Value::Object(fields)) = obj.get("$fields") {
                            let entries = fields
                                .iter()
                                .map(|(k, v)| (k.clone(), from_json(v.clone())))
                                .collect();
                            return Value::Prod(entries);
                        }
                    }
                    "bagkv" => {
                        if let Some(serde_json::Value::Array(items)) = obj.get("$items") {
                            let pairs = items
                                .iter()
                                .filter_map(|item| {
                                    if let serde_json::Value::Array(pair) = item {
                                        if pair.len() == 2 {
                                            return Some((
                                                from_json(pair[0].clone()),
                                                from_json(pair[1].clone()),
                                            ));
                                        }
                                    }
                                    None
                                })
                                .collect();
                            return Value::BagKV(pairs);
                        }
                    }
                    "bind" => {
                        if let (Some(k), Some(v)) = (obj.get("$key"), obj.get("$val")) {
                            return Value::Bind(
                                Box::new(from_json(k.clone())),
                                Box::new(from_json(v.clone())),
                            );
                        }
                    }
                    "some" => {
                        if let Some(inner) = obj.get("$value") {
                            return Value::Some_(Box::new(from_json(inner.clone())));
                        }
                    }
                    "none" => return Value::None_,
                    "ok" => {
                        if let Some(inner) = obj.get("$value") {
                            return Value::Ok_(Box::new(from_json(inner.clone())));
                        }
                    }
                    "fail" => {
                        let code = obj
                            .get("$code")
                            .and_then(|value| value.as_str())
                            .unwrap_or("")
                            .to_string();
                        let msg = obj
                            .get("$msg")
                            .and_then(|value| value.as_str())
                            .unwrap_or("")
                            .to_string();
                        return Value::Fail_(code, msg);
                    }
                    _ => {}
                }
            }
            Value::Map(obj.into_iter().map(|(k, v)| (k, from_json(v))).collect())
        }
    }
}

pub fn to_json(v: Value) -> serde_json::Value {
    match v {
        Value::Null => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(b),
        Value::Num(n) => n.to_json_value(),
        Value::Str(s) => serde_json::Value::String(s),
        Value::Seq(items) => serde_json::Value::Array(items.into_iter().map(to_json).collect()),
        Value::Set(items) => serde_json::json!({
            "$type": "set",
            "$items": items.into_iter().map(to_json).collect::<Vec<_>>()
        }),
        Value::Bag(items) => serde_json::json!({
            "$type": "bag",
            "$items": items.into_iter().map(to_json).collect::<Vec<_>>()
        }),
        Value::Map(entries) => {
            let mut map = serde_json::Map::new();
            for (k, v) in entries {
                map.insert(k, to_json(v));
            }
            serde_json::Value::Object(map)
        }
        Value::Prod(fields) => {
            let mut map = serde_json::Map::new();
            map.insert(
                "$type".to_string(),
                serde_json::Value::String("prod".to_string()),
            );
            let fields_map: serde_json::Map<String, serde_json::Value> =
                fields.into_iter().map(|(k, v)| (k, to_json(v))).collect();
            map.insert("$fields".to_string(), serde_json::Value::Object(fields_map));
            serde_json::Value::Object(map)
        }
        Value::BagKV(pairs) => serde_json::json!({
            "$type": "bagkv",
            "$items": pairs
                .into_iter()
                .map(|(k, v)| serde_json::json!([to_json(k), to_json(v)]))
                .collect::<Vec<_>>()
        }),
        Value::Bind(k, v) => serde_json::json!({
            "$type": "bind",
            "$key": to_json(*k),
            "$val": to_json(*v)
        }),
        Value::Some_(inner) => serde_json::json!({
            "$type": "some",
            "$value": to_json(*inner)
        }),
        Value::None_ => serde_json::json!({"$type": "none"}),
        Value::Ok_(inner) => serde_json::json!({
            "$type": "ok",
            "$value": to_json(*inner)
        }),
        Value::Fail_(code, msg) => serde_json::json!({
            "$type": "fail",
            "$code": code,
            "$msg": msg
        }),
        Value::Lambda(_, _, _) => serde_json::json!({"$type": "fn"}),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(expr: &str) -> serde_json::Value {
        run(expr, serde_json::Value::Null).expect("run failed")
    }

    fn ri(expr: &str, input: serde_json::Value) -> serde_json::Value {
        run(expr, input).expect("run failed")
    }

    fn rib(expr: &str, binding_name: &str, input: serde_json::Value) -> serde_json::Value {
        run_with_input_binding(expr, binding_name, input).expect("run failed")
    }

    #[test]
    fn test_null_literal() {
        assert_eq!(r("null;"), serde_json::Value::Null);
    }

    #[test]
    fn test_bool_literals() {
        assert_eq!(r("true;"), serde_json::Value::Bool(true));
        assert_eq!(r("false;"), serde_json::Value::Bool(false));
    }

    #[test]
    fn test_num_arithmetic() {
        assert_eq!(r("1 + 2;"), serde_json::json!(3));
        assert_eq!(r("10 - 3;"), serde_json::json!(7));
        assert_eq!(r("4 * 5;"), serde_json::json!(20));
        assert_eq!(r("10 / 2;"), serde_json::json!(5));
        assert_eq!(r("0.1 + 0.2;"), serde_json::json!(0.3));
    }

    #[test]
    fn test_non_terminating_rational_uses_wrapper() {
        assert_eq!(
            r("1 / 3;"),
            serde_json::json!({
                "$type": "num",
                "$value": "1/3"
            })
        );
    }

    #[test]
    fn test_numeric_wrapper_round_trips_exactly() {
        let result = ri(
            "input = 1 / 3;",
            serde_json::json!({
                "$type": "num",
                "$value": "1/3"
            }),
        );
        assert_eq!(result, serde_json::Value::Bool(true));
    }

    #[test]
    fn test_public_run_no_longer_binds_placeholder() {
        let result = run("_;", serde_json::json!({"name": "steve"})).expect("run failed");
        assert_eq!(
            result,
            serde_json::json!({
                "$type": "fail",
                "$code": "t_sda_unbound_placeholder",
                "$msg": "unbound placeholder"
            })
        );
    }

    #[test]
    fn test_custom_input_binding_name() {
        let result = rib(r#"root<"name">;"#, "root", serde_json::json!({"name": "Ada"}));
        assert_eq!(result, serde_json::json!("Ada"));
    }

    #[test]
    fn test_string_concat() {
        assert_eq!(r(r#""hello" ++ " world";"#), serde_json::json!("hello world"));
    }

    #[test]
    fn test_seq_literal() {
        let result = r("seq[1, 2, 3];");
        assert_eq!(result, serde_json::json!([1, 2, 3]));
    }

    #[test]
    fn test_set_literal() {
        let result = r("set{1, 2, 2, 3};");
        if let serde_json::Value::Object(obj) = &result {
            assert_eq!(obj["$type"], serde_json::json!("set"));
            if let serde_json::Value::Array(items) = &obj["$items"] {
                assert_eq!(items.len(), 3);
            }
        }
    }

    #[test]
    fn test_map_literal() {
        let result = r(r#"map{"a" -> 1, "b" -> 2};"#);
        assert_eq!(result, serde_json::json!({"a": 1, "b": 2}));
    }

    #[test]
    fn test_map_equality_is_extensional() {
        assert_eq!(
            r(r#"Map{"a" -> 1, "b" -> 2} = Map{"b" -> 2, "a" -> 1};"#),
            serde_json::Value::Bool(true)
        );
    }

    #[test]
    fn test_bag_equality_is_extensional_with_multiplicity() {
        assert_eq!(
            r(r#"Bag{1, 2, 1} = Bag{1, 1, 2};"#),
            serde_json::Value::Bool(true)
        );
        assert_eq!(
            r(r#"Bag{1, 2, 1} = Bag{1, 2, 2};"#),
            serde_json::Value::Bool(false)
        );
    }

    #[test]
    fn test_prod_equality_is_extensional() {
        assert_eq!(
            r(r#"Prod{a: 1, b: 2} = Prod{b: 2, a: 1};"#),
            serde_json::Value::Bool(true)
        );
    }

    #[test]
    fn test_let_binding() {
        assert_eq!(r("let x = 42; x;"), serde_json::json!(42));
    }

    #[test]
    fn test_lambda_and_call() {
        assert_eq!(r("let f = x => x + 1; f(5);"), serde_json::json!(6));
    }

    #[test]
    fn test_pipe() {
        assert_eq!(r("5 |> _ + 1;"), serde_json::json!(6));
    }

    #[test]
    fn test_comprehension() {
        let result = r("{ x | x in seq[1, 2, 3] };");
        assert_eq!(result, serde_json::json!([1, 2, 3]));
    }

    #[test]
    fn test_comparison() {
        assert_eq!(r("1 < 2;"), serde_json::Value::Bool(true));
        assert_eq!(r("2 > 3;"), serde_json::Value::Bool(false));
        assert_eq!(r("1 = 1;"), serde_json::Value::Bool(true));
        assert_eq!(r("1 != 2;"), serde_json::Value::Bool(true));
    }

    #[test]
    fn test_some_none() {
        let result = r("some(42);");
        assert_eq!(result, serde_json::json!({"$type": "some", "$value": 42}));
        let result2 = r("none;");
        assert_eq!(result2, serde_json::json!({"$type": "none"}));
    }

    #[test]
    fn test_type_of() {
        assert_eq!(r(r#"typeOf(null);"#), serde_json::json!("null"));
        assert_eq!(r(r#"typeOf(42);"#), serde_json::json!("num"));
        assert_eq!(r(r#"typeOf("hello");"#), serde_json::json!("str"));
    }

    #[test]
    fn test_keys_returns_set() {
        let result = r(r#"keys(Map{"b" -> 2, "a" -> 1});"#);
        assert_eq!(
            result,
            serde_json::json!({
                "$type": "set",
                "$items": ["b", "a"]
            })
        );
    }

    #[test]
    fn test_select() {
        let result = ri(r#"input<"name">;"#, serde_json::json!({"name": "Alice"}));
        assert_eq!(result, serde_json::json!("Alice"));
    }

    #[test]
    fn test_placeholder_scoping_pipe() {
        assert_eq!(r("5 |> _ + 1;"), serde_json::json!(6));
    }

    #[test]
    fn test_required_selector_ok() {
        let result = ri(r#"input<"name">!;"#, serde_json::json!({"name": "steve"}));
        assert_eq!(result, serde_json::json!({"$type": "ok", "$value": "steve"}));
    }

    #[test]
    fn test_required_selector_missing() {
        let result = ri(r#"input<"missing">!;"#, serde_json::json!({"name": "steve"}));
        assert_eq!(
            result,
            serde_json::json!({"$type": "fail", "$code": "t_sda_missing_key", "$msg": "missing key"})
        );
    }

    #[test]
    fn test_optional_selector_present() {
        let result = ri(r#"input<"name">?;"#, serde_json::json!({"name": "steve"}));
        assert_eq!(result, serde_json::json!({"$type": "some", "$value": "steve"}));
    }

    #[test]
    fn test_optional_selector_missing() {
        let result = ri(r#"input<"missing">?;"#, serde_json::json!({"name": "steve"}));
        assert_eq!(result, serde_json::json!({"$type": "none"}));
    }

    #[test]
    fn test_null_vs_absence_some_null() {
        let result = ri(r#"input<"x">?;"#, serde_json::json!({"x": null}));
        assert_eq!(result, serde_json::json!({"$type": "some", "$value": null}));
    }

    #[test]
    fn test_null_vs_absence_none() {
        let result = ri(r#"input<"x">?;"#, serde_json::json!({}));
        assert_eq!(result, serde_json::json!({"$type": "none"}));
    }

    #[test]
    fn test_bagkv_duplicate_optional() {
        let result = r(r#"BagKV{"k" -> 1, "k" -> 2}<"k">?;"#);
        assert_eq!(result, serde_json::json!({"$type": "none"}));
    }

    #[test]
    fn test_bagkv_duplicate_required() {
        let result = r(r#"BagKV{"k" -> 1, "k" -> 2}<"k">!;"#);
        assert_eq!(
            result,
            serde_json::json!({"$type": "fail", "$code": "t_sda_duplicate_key", "$msg": "duplicate key"})
        );
    }

    #[test]
    fn test_normalize_unique_ok() {
        let result = r(r#"normalizeUnique(BagKV{"a" -> 1, "b" -> 2});"#);
        assert_eq!(result, serde_json::json!({"$type": "ok", "$value": {"a": 1, "b": 2}}));
    }

    #[test]
    fn test_normalize_unique_fail() {
        let result = r(r#"normalizeUnique(BagKV{"k" -> 1, "k" -> 2});"#);
        assert_eq!(
            result,
            serde_json::json!({"$type": "fail", "$code": "t_sda_duplicate_key", "$msg": "duplicate key"})
        );
    }

    #[test]
    fn test_normalize_first() {
        let result = r(r#"normalizeFirst(BagKV{"k" -> 1, "k" -> 2});"#);
        assert_eq!(result, serde_json::json!({"k": 1}));
    }

    #[test]
    fn test_normalize_last() {
        let result = r(r#"normalizeLast(BagKV{"k" -> 1, "k" -> 2});"#);
        assert_eq!(result, serde_json::json!({"k": 2}));
    }

    #[test]
    fn test_carrier_preservation_seq() {
        let result = ri(r#"{ x | x in input | x > 2 };"#, serde_json::json!([1, 2, 3, 4]));
        assert_eq!(result, serde_json::json!([3, 4]));
    }

    #[test]
    fn test_carrier_preservation_set() {
        let result = r(r#"{ x | x in Set{1, 2, 3} | x > 1 };"#);
        if let serde_json::Value::Object(obj) = &result {
            assert_eq!(obj["$type"], serde_json::json!("set"));
            if let serde_json::Value::Array(items) = &obj["$items"] {
                assert_eq!(items.len(), 2);
            } else {
                panic!("expected $items array");
            }
        } else {
            panic!("expected set object, got {:?}", result);
        }
    }

    #[test]
    fn test_carrier_preservation_bag() {
        let result = r(r#"{ x | x in Bag{1, 2, 2, 3} | x > 1 };"#);
        if let serde_json::Value::Object(obj) = &result {
            assert_eq!(obj["$type"], serde_json::json!("bag"));
            if let serde_json::Value::Array(items) = &obj["$items"] {
                assert_eq!(items.len(), 3);
            } else {
                panic!("expected $items array");
            }
        } else {
            panic!("expected bag object, got {:?}", result);
        }
    }

    #[test]
    fn test_comprehension_with_yield() {
        let result = r("{ yield x * 2 | x in Seq[1, 2, 3] };");
        assert_eq!(result, serde_json::json!([2, 4, 6]));
    }

    #[test]
    fn test_comprehension_with_general_shorthand_projection() {
        let result = r("{ x + 1 | x in Seq[1, 2, 3] };");
        assert_eq!(result, serde_json::json!([2, 3, 4]));
    }

    #[test]
    fn test_bagkv_comprehension_filter_returns_bag_of_bindings() {
        let result = r(r#"{ b | b in BagKV{"a" -> 1, "b" -> 2} | b<key> = "b" };"#);
        assert_eq!(
            result,
            serde_json::json!({
                "$type": "bag",
                "$items": [
                    {"$type": "bind", "$key": "b", "$val": 2}
                ]
            })
        );
    }

    #[test]
    fn test_bagkv_comprehension_projection_returns_bag() {
        let result = r(r#"{ b<val> | b in BagKV{"a" -> 1, "b" -> 2} };"#);
        assert_eq!(
            result,
            serde_json::json!({
                "$type": "bag",
                "$items": [1, 2]
            })
        );
    }

    #[test]
    fn test_title_case_keywords() {
        assert_eq!(r("Seq[1, 2, 3];"), serde_json::json!([1, 2, 3]));
        assert_eq!(r(r#"Map{"a" -> 1};"#), serde_json::json!({"a": 1}));
    }

    #[test]
    fn test_map_rejects_identifier_keys() {
        let err = run("Map{a -> 1};", serde_json::Value::Null).unwrap_err();
        assert!(matches!(err, SdaError::Parse(_)));
    }

    #[test]
    fn test_unbound_placeholder_outside_pipe() {
        use crate::eval::eval_program;
        let tokens = lexer::lex("•;").unwrap();
        let prog = parser::parse(tokens).unwrap();
        let mut env = crate::Env::new();
        let result = eval_program(&prog, &mut env).unwrap();
        assert_eq!(
            result,
            Some(Value::Fail_(
                "t_sda_unbound_placeholder".to_string(),
                "unbound placeholder".to_string(),
            ))
        );
    }

    #[test]
    fn test_as_bag_kv_from_bag_of_bind() {
        use crate::eval::eval_program;
        let tokens = lexer::lex(r#"asBagKV(Bag{Bind("a", 1)});"#).unwrap();
        let prog = parser::parse(tokens).unwrap();
        let mut env = crate::Env::new();
        let result = eval_program(&prog, &mut env).unwrap();
        assert!(matches!(result, Some(Value::Ok_(_))));
    }

    #[test]
    fn test_as_bag_kv_wrong_shape_not_bag() {
        let result = r(r#"asBagKV(Seq[1, 2]);"#);
        assert_eq!(
            result,
            serde_json::json!({"$type": "fail", "$code": "t_sda_wrong_shape", "$msg": "wrong shape"})
        );
    }

    #[test]
    fn test_as_bag_kv_wrong_shape_non_bind_element() {
        let result = r(r#"asBagKV(Bag{1, 2});"#);
        assert_eq!(
            result,
            serde_json::json!({"$type": "fail", "$code": "t_sda_wrong_shape", "$msg": "wrong shape"})
        );
    }
}
