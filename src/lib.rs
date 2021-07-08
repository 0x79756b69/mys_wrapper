use mysql::{Params, Pool, PooledConn, Row, Value, from_value};
use mysql::prelude::Queryable;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::{exec};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let exec = exec(
            "mysql://{USER}:{PASSWORD}@localhost:{PORT}/{DBNAME}",
            "SELECT * FROM users WHERE name=? OR name=?",
            vec!["unko".to_string(), "tekou".to_string()]);
        println!("{:?}", exec);
        // let serd = serde_json::to_value(
        //     Response{
        //         body: maps,
        //         status: 200,
        //         description: "This is description.".to_string()
        //     }).unwrap();
        // let json = serde_json::json!(
        //     serd
        // );
    }
}

/// ErrまたはOKか、一連の処理をまとめた関数
pub fn exec<T: AsRef<str>, U: AsRef<str>>(db_url: T, sql:U, params: Vec<String> ) -> Result<Vec<HashMap<String, String>>, String> {
    let r = pool_connect(db_url);
    match r {
        Ok(c) => {
            // let a = connection;
            let params = Params::from(params);
            let r = exec_sql(c, params, sql);
            match r {
                Ok(rows) => {
                    let r = rows_into_hashmaps(rows);
                    match r {
                        Ok(maps) => Ok(maps),
                        Err(s) => Err(s)
                    }
                }
                Err(e) => Err(e)
            }
        },
        Err(e) => {
            // println!("{:?}", e);
            Err(e)
        }
    }
}



/// url: like "mysql://root:PSWD@localhost:3306/example"
/// connect to db using pool
pub fn pool_connect<T: AsRef<str>>(url: T) -> Result<PooledConn, String> {
    match Pool::new(url) {
        Ok(pool) => {
            match pool.get_conn() {
                Ok(s) => Ok(s),
                Err(e) => Err(e.to_string())
            }
        },
        Err(e) => Err(e.to_string())
    }
}

pub fn exec_sql<T: AsRef<str>>(mut connection: PooledConn, params: Params, sql: T) -> Result<Vec<Row>, String>{
    // let params = Params::from(vec!["id","age","age","age", "unko"]);
    let r = connection.prep(sql);
    match r {
        Ok(stmt) => {
            let r2 =  connection.exec(&stmt, params);
            match r2 {
                Ok(rows) => {
                    let result:Vec<Row> = rows;
                    Ok(result)
                },
                Err(e) => Err(e.to_string())
            }
        },
        Err(e) => Err(e.to_string())
    }
    // let results:Vec<Row> = connection.exec(&stmt, params).unwrap();
}

pub fn rows_into_hashmaps(rows: Vec<Row>) -> Result<Vec<HashMap<String, String>>, String>{
    let mut r = Vec::new();
    for result in rows {
        let mut r2 = HashMap::new();
        for column in result.columns_ref().to_owned() {
            // Cells in a row can be indexed by numeric index or by column name
            let column_value = &result[column.name_str().as_ref()];
            let name = column.name_str().to_string();
            let v = column_value.clone();
            let r3 = value_to_string(Value::from(&v));
            match r3 {
                Ok(st_r) => { r2.insert(name, st_r); },
                Err(e) => return Err(e)
            }
        }
        r.push(r2)
    }
    Ok(r)
}



fn value_to_string(unknown_val: Value) -> Result<String, String> {
    return match unknown_val {
        _val @ Value::NULL => {
            // println!("An empty value: {:?}", from_value::<Option<u8>>(Value::from(&val)));
            Ok(String::new())
        },
        _val @ Value::Bytes(..) => {
            // It's non-utf8 bytes, since we already tried to convert it to String
            // println!("Bytes: {:?}", from_value::<Vec<u8>>(Value::from(&val)));
            let r: String = String::from_utf8(from_value::<Vec<u8>>(_val).to_vec()).unwrap();
            Ok(r)
        },
        _val @ Value::Int(..) => {
            // println!("A signed integer: {}", from_value::<i64>(Value::from(&val)));
            let r: String = from_value::<i64>(_val).to_string();
            Ok(r)
        },
        _val @ Value::UInt(..) => {
            // println!("An unsigned integer: {}", from_value::<u64>(val));
            let r: String = from_value::<u64>(_val).to_string();
            Ok(r)
        },
        _val @ Value::Double(..) => {
            // println!("A double precision float value: {}", from_value::<f64>(val));
            let r: String = from_value::<f64>(_val).to_string();
            Ok(r)
        },
        _val @ Value::Date(..) => {
            use mysql::chrono::NaiveDateTime;
            // println!("A date value: {}", from_value::<NaiveDateTime>(val))
            let r: String = from_value::<NaiveDateTime>(_val).to_string();
            Ok(r)
        },
        _val @ Value::Time(..) => {
            // とりあえずミリ秒で。
            use std::time::Duration;
            // println!("A time value: {:?}", from_value::<Duration>(Value::from(&_val)));
            let r = from_value::<Duration>(_val).as_millis().to_string();
            Ok(r)
        },
        // Value::Float(..) => unreachable!("already tried"),
        _ => {
            // println!("変です {:?}", unknown_val);
            Err("Mismatched type".to_string())
        }
    }
}