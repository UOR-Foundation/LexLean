//! scratch
use lexlean::calculus::rust::{self, Profile};
use lexlean::calculus::{Adt, Expr, Function, Program, Ty, Value};

#[test]
fn scratch_value() {
    let cases: Vec<(Ty, Value)> = vec![
        (Ty::Nat, Value::Nat { value: "7".into() }),
        (
            Ty::Option {
                value: Box::new(Ty::Nat),
            },
            Value::Some {
                value: Box::new(Value::Nat { value: "3".into() }),
            },
        ),
        (
            Ty::List {
                element: Box::new(Ty::Bool),
            },
            Value::List {
                items: vec![Value::Bool { value: true }, Value::Bool { value: false }],
            },
        ),
    ];
    for (ty, value) in cases {
        let program = Program {
            spec: lexlean::calculus::PROGRAM_SPEC.to_owned(),
            adts: Vec::<Adt>::new(),
            functions: vec![Function {
                parameters: vec![],
                types: vec![],
                result: ty.clone(),
                body: Expr::Value { ty, value },
            }],
        };
        let krate = rust::lower(&program, Profile::Std).unwrap();
        println!("{:#?}", krate.items);
    }
}
