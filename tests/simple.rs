extern crate gtmpl_value;
#[macro_use]
extern crate gtmpl_derive;

use std::any::Any;
use std::sync::Arc;
use std::collections::HashMap;
use gtmpl_value::{Func, Value};

#[test]
fn test1() {
    #[derive(Gtmpl)]
    struct Foo {
        bar: String,
    }

    let val = Value::from(Foo { bar: "2000".to_owned() });
    if let Value::Object(ref m) = val {
        assert_eq!(m.get("bar"), Some(&Value::String("2000".to_owned())));
    } else {
        assert!(false);
    }
}

#[test]
fn test2() {
    #[derive(Gtmpl)]
    struct Bar {
        i: i64,
    }
    #[derive(Gtmpl)]
    struct Foo {
        bar: Bar,
    }

    let val = Value::from(Foo { bar: Bar { i: 23 } });
    if let Value::Object(ref m) = val {
        if let Some(&Value::Object(ref o)) = m.get("bar") {
            if let Some(&Value::Number(ref n)) = o.get("i") {
                return assert_eq!(n.as_i64(), Some(23));
            }
        }
    }
    assert!(false);
}

#[test]
fn test3() {
    fn bar(a: &[Arc<Any>]) -> Result<Arc<Any>, String> {
        Ok(Arc::clone(&a[0]))
    };

    #[derive(Gtmpl)]
    struct Foo {
        bar: Func,
    }

    let val = Value::from(Foo { bar: bar });
    let param: &[Arc<Any>] = &[Arc::new(Value::from(23i64))];
    if let Value::Object(ref m) = val {
        if let Some(&Value::Function(ref f)) = m.get("bar") {
            let res = (f.f)(param).unwrap();
            if let Some(&Value::Number(ref i)) = res.downcast_ref::<Value>() {
                return assert_eq!(i.as_i64(), Some(23));
            }
        }
    }
    assert!(false);
}

#[test]
fn test4() {
    #[derive(Gtmpl)]
    struct Foo {
        field: HashMap<String, u64>,
    }
    let field = [("2000".to_owned(), 23u64)].iter().cloned().collect();
    let val = Value::from(Foo { field });
    if let Value::Object(ref m) = val {
        let map: HashMap<String, u64> = [("2000".to_owned(), 23u64)].iter().cloned().collect();
        let map: Value = map.into();
        if let Value::Map(_) = map {
            assert_eq!(m.get("field"), Some(&map));
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }
}
