use mlua::{Table, Value};

pub fn pretty_value(value: Value, indent: usize) -> mlua::Result<String> {
    match value {
        Value::Table(table) => pretty_table(table, indent),
        Value::String(s) => Ok(format!("{:?}", s.to_str()?)),
        Value::Nil => Ok("nil".into()),
        Value::Boolean(v) => Ok(v.to_string()),
        Value::Integer(v) => Ok(v.to_string()),
        Value::Number(v) => Ok(v.to_string()),
        Value::Function(_) => Ok("<function>".into()),
        Value::Thread(_) => Ok("<thread>".into()),
        Value::UserData(_) => Ok("<userdata>".into()),
        Value::LightUserData(_) => Ok("<lightuserdata>".into()),
        Value::Error(e) => Ok(format!("<error: {e}>")),
        _ => Ok("<unknown>".into()),
    }
}

pub fn pretty_table(table: Table, indent: usize) -> mlua::Result<String> {
    let mut out = String::from("{\n");

    for pair in table.pairs::<Value, Value>() {
        let (key, value) = pair?;

        out.push_str(&" ".repeat(indent + 2));
        out.push_str(&pretty_value(key, 0)?);
        out.push_str(" = ");
        out.push_str(&pretty_value(value, indent + 2)?);
        out.push_str(",\n");
    }

    out.push_str(&" ".repeat(indent));
    out.push('}');

    Ok(out)
}
