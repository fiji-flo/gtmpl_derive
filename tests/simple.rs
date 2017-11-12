#[macro_use]
extern crate gtmpl_value;
#[macro_use]
extern crate gtmpl_derive;

use std::any::Any;
use std::sync::Arc;
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
    struct Foo {
        bar: i64,
    }

    let val = Value::from(Foo { bar: 23 });
    if let Value::Object(ref m) = val {
        if let Some(&Value::Number(ref n)) = m.get("bar") {
            return assert_eq!(n.as_i64(), Some(23));
        }
    }
    assert!(false);
}

#[test]
fn test3() {
    fn bar(a: &[Arc<Any>]) -> Result<Arc<Any>, String> {
        Ok(a[0].clone())
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
