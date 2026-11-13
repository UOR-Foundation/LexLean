//! A deterministic native-core view of inspected kernel types. The structural
//! kernel report remains available when elaborator-only projection labels are
//! absent; no invented declaration name is substituted for such metadata.

use serde::Serialize;
use serde_json::Value;

use crate::ir::core::{CoreBinderInfo, CoreLevel, CoreNode};

#[derive(Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub(super) enum NativeType {
    Available {
        nodes: Vec<CoreNode>,
        root: usize,
        levels: Vec<String>,
    },
    Unrepresentable {
        reason: String,
    },
}

fn keys(value: &Value, expected: &[&str]) -> Result<(), String> {
    let object = value.as_object().ok_or("expected structural object")?;
    if object.len() != expected.len() || expected.iter().any(|key| !object.contains_key(*key)) {
        return Err("unexpected structural fields".to_owned());
    }
    Ok(())
}

fn tuple(value: &Value, length: usize) -> Result<&[Value], String> {
    let values = value.as_array().ok_or("expected structural tuple")?;
    if values.len() != length {
        return Err("unexpected structural tuple length".to_owned());
    }
    Ok(values)
}

fn natural(value: &Value) -> Result<usize, String> {
    value
        .as_u64()
        .and_then(|number| usize::try_from(number).ok())
        .ok_or_else(|| "structural index is outside the native index range".to_owned())
}

fn name(value: &Value) -> Result<String, String> {
    if value.is_null() {
        return Ok(String::new());
    }
    let (tag, numeric) = if value.get("str").is_some() {
        ("str", false)
    } else {
        ("num", true)
    };
    keys(value, &[tag])?;
    let parts = tuple(&value[tag], 2)?;
    let parent = name(&parts[0])?;
    let segment = if numeric {
        let number = parts[1]
            .as_number()
            .ok_or("invalid numeric name component")?
            .to_string();
        if number.is_empty() || !number.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err("invalid numeric name component".to_owned());
        }
        format!("#{number}")
    } else {
        let text = parts[1].as_str().ok_or("invalid string name component")?;
        if text.is_empty() || text.chars().any(char::is_control) {
            return Err(
                "native-core names cannot represent empty or control-character segments".to_owned(),
            );
        }
        text.replace('\\', "\\\\")
            .replace('.', "\\d")
            .replace('#', "\\h")
    };
    Ok(if parent.is_empty() {
        segment
    } else {
        format!("{parent}.{segment}")
    })
}

fn level(value: &Value) -> Result<CoreLevel, String> {
    let object = value.as_object().ok_or("expected universe level")?;
    if object.len() != 1 {
        return Err("unexpected universe fields".to_owned());
    }
    let (tag, argument) = object.iter().next().ok_or("missing universe constructor")?;
    match tag.as_str() {
        "zero" if argument.is_null() => Ok(CoreLevel::Zero),
        "succ" => Ok(CoreLevel::Succ(Box::new(level(argument)?))),
        "max" | "imax" => {
            let pair = tuple(argument, 2)?;
            let pair = Box::new((level(&pair[0])?, level(&pair[1])?));
            Ok(if tag == "max" {
                CoreLevel::Max(pair)
            } else {
                CoreLevel::IMax(pair)
            })
        }
        "param" => Ok(CoreLevel::Param(name(argument)?)),
        _ => Err("unresolved or unknown universe constructor".to_owned()),
    }
}

fn binder(value: &Value) -> Result<CoreBinderInfo, String> {
    match value.as_str() {
        Some("default") => Ok(CoreBinderInfo::Explicit),
        Some("implicit") => Ok(CoreBinderInfo::Implicit),
        Some("strictImplicit") => Ok(CoreBinderInfo::StrictImplicit),
        Some("instImplicit") => Ok(CoreBinderInfo::Instance),
        _ => Err("unknown binder visibility".to_owned()),
    }
}

fn expression(value: &Value, nodes: &mut Vec<CoreNode>) -> Result<usize, String> {
    let object = value.as_object().ok_or("expected kernel expression")?;
    let node = if let Some(constant) = object.get("const") {
        keys(value, &["const", "levels"])?;
        let levels = object["levels"]
            .as_array()
            .ok_or("constant universe list missing")?;
        CoreNode::Const {
            n: name(constant)?,
            u: levels.iter().map(level).collect::<Result<_, _>>()?,
        }
    } else {
        if object.len() != 1 {
            return Err("unexpected expression fields".to_owned());
        }
        let (tag, argument) = object
            .iter()
            .next()
            .ok_or("missing expression constructor")?;
        match tag.as_str() {
            "bvar" => CoreNode::BVar { i: natural(argument)? },
            "sort" => CoreNode::Sort { l: level(argument)? },
            "app" => {
                let pair = tuple(argument, 2)?;
                CoreNode::App { f: expression(&pair[0], nodes)?, x: expression(&pair[1], nodes)? }
            }
            "lam" | "forallE" => {
                keys(argument, &["name", "type", "body", "bi"])?;
                let n = name(&argument["name"])?;
                let b = binder(&argument["bi"])?;
                let t = expression(&argument["type"], nodes)?;
                let v = expression(&argument["body"], nodes)?;
                if tag == "lam" { CoreNode::Lambda { n, b, t, v } } else { CoreNode::Forall { n, b, t, v } }
            }
            "letE" => {
                keys(argument, &["name", "type", "value", "body", "nondep"])?;
                CoreNode::Let {
                    n: name(&argument["name"])?, t: expression(&argument["type"], nodes)?,
                    v: expression(&argument["value"], nodes)?, body: expression(&argument["body"], nodes)?,
                    d: argument["nondep"].as_bool().ok_or("let dependency flag missing")?,
                }
            }
            "lit" => {
                if let Some(number) = argument.get("natVal") {
                    keys(argument, &["natVal"])?;
                    let digits = number.as_number().ok_or("natural literal is not a number")?.to_string();
                    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
                        return Err("natural literal is not a natural number".to_owned());
                    }
                    CoreNode::Nat { v: digits }
                } else {
                    keys(argument, &["strVal"])?;
                    CoreNode::String { v: argument["strVal"].as_str().ok_or("string literal is not text")?.chars().map(u32::from).collect() }
                }
            }
            "proj" => return Err("native-core projection labels require elaborator metadata; the exact kernel projection remains in type".to_owned()),
            _ => return Err("unresolved or unknown kernel expression".to_owned()),
        }
    };
    let index = nodes.len();
    nodes.push(node);
    Ok(index)
}

pub(super) fn convert(ty: &Value, parameters: &[Value]) -> NativeType {
    if ty.is_null() && parameters.is_empty() {
        return NativeType::Unrepresentable {
            reason: "declaration not found in the inspected environment".to_owned(),
        };
    }
    let converted = (|| {
        let levels = parameters.iter().map(name).collect::<Result<Vec<_>, _>>()?;
        let mut nodes = Vec::new();
        let root = expression(ty, &mut nodes)?;
        Ok::<_, String>((nodes, root, levels))
    })();
    match converted {
        Ok((nodes, root, levels)) => NativeType::Available {
            nodes,
            root,
            levels,
        },
        Err(reason) => NativeType::Unrepresentable { reason },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn absent_declaration_has_no_native_goal() {
        let result = serde_json::to_value(convert(&Value::Null, &[])).unwrap();
        assert_eq!(result["status"], "unrepresentable");
        assert_eq!(
            result["reason"],
            "declaration not found in the inspected environment"
        );
        assert!(result.get("nodes").is_none());
        assert!(result.get("root").is_none());
    }

    #[test]
    fn structural_name_components_remain_distinct() {
        assert_eq!(
            name(&json!({"str":[null,"a.b#c\\d"]})).unwrap(),
            "a\\db\\hc\\\\d"
        );
        assert_ne!(
            name(&json!({"str":[null,"#0"]})).unwrap(),
            name(&json!({"num":[null,0]})).unwrap()
        );
    }

    #[test]
    fn binder_visibility_and_large_naturals_are_preserved() {
        let literal: Value =
            serde_json::from_str("{\"lit\":{\"natVal\":184467440737095516160}}").unwrap();
        let ty = json!({"forallE":{"name":null,"bi":"strictImplicit","type":{"sort":{"zero":null}},"body":literal}});
        let NativeType::Available { nodes, root, .. } = convert(&ty, &[]) else {
            panic!("representable type");
        };
        assert!(matches!(&nodes[1], CoreNode::Nat { v } if v == "184467440737095516160"));
        assert!(matches!(
            nodes[root],
            CoreNode::Forall {
                b: CoreBinderInfo::StrictImplicit,
                t: 0,
                v: 1,
                ..
            }
        ));
    }

    #[test]
    fn unresolved_terms_cannot_become_native_candidates() {
        assert!(matches!(
            convert(&json!({"mvar":null}), &[]),
            NativeType::Unrepresentable { .. }
        ));
    }
}
