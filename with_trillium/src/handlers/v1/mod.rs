pub mod errors;
use errors::Errors;

use log::info;
use serde::{Deserialize, Serialize};
use serde_qs as qs;
use trillium::{Conn, Handler, Status};

#[derive(Debug, PartialEq, Deserialize)]
struct QueryParams {
    i1: i64,
    i2: i64,
    limit: i64,
    str1: String,
    str2: String,
}

#[derive(Serialize, Deserialize)]
struct FBResult {
    values: Vec<String>,
}

fn fizzbuzz(i1: i64, i2: i64, limit: i64, str1: String, str2: String) -> Result<FBResult, Errors> {
    if limit < 1 {
        return Err(Errors::BadParamErr("limit should be greater than 1"));
    }
    if i1 <= 0 {
        return Err(Errors::BadParamErr("i1 should be greater than 0"));
    }
    if i2 <= 0 {
        return Err(Errors::BadParamErr("i2 should be greater than 0"));
    }

    let mut res = FBResult {
        values: vec![String::from("1")],
    };

    let mut tmp = Vec::new();

    for i in 2..=limit {
        if i % i1 == 0 {
            tmp.push(str1.clone());
        }
        if i % i2 == 0 {
            tmp.push(str2.clone());
        }
        if tmp.len() == 0 {
            res.values.push(format!("{}", i));
        } else {
            res.values.extend(tmp);
            tmp = Vec::new();
        }
    }

    Ok(res)
}

pub struct FizzBuzz {}

#[trillium::async_trait]
impl Handler for FizzBuzz {
    async fn run(&self, conn: Conn) -> Conn {
        let params: QueryParams = match qs::from_str(conn.querystring()) {
            Ok(p) => p,
            Err(e) => {
                return conn
                    .with_status(Status::BadRequest)
                    .with_body(e.to_string())
                    .halt()
            }
        };

        info!("{:#?}", params);
        let res = match fizzbuzz(params.i1, params.i2, params.limit, params.str1, params.str2) {
            Ok(r) => r,
            Err(e) => {
                return conn
                    .with_status(Status::BadRequest)
                    .with_body(e.to_string())
                    .halt()
            }
        };
        let json = serde_json::to_string(&res).unwrap();
        conn.ok(json)
    }
}
