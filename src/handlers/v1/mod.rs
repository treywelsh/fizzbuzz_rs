pub mod errors;

use log::info;
use serde::Deserialize;
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

pub fn fizzbuzz(
    i1: i64,
    i2: i64,
    limit: i64,
    str1: String,
    str2: String,
) -> Result<String, errors::Errors> {
    if limit < 1 {
        return Err(errors::Errors::BadParamErr(
            "limit should be greater than 1",
        ));
    }
    if i1 <= 0 {
        return Err(errors::Errors::BadParamErr("i1 should be greater than 0"));
    }
    if i2 <= 0 {
        return Err(errors::Errors::BadParamErr("i2 should be greater than 0"));
    }

    let mut res = String::from("1");

    let mut tmp = String::new();

    for i in 2..=limit {
        if i % i1 == 0 {
            tmp.push_str(&str1);
        }
        if i % i2 == 0 {
            tmp.push_str(&str2);
        }
        if tmp.len() == 0 {
            res.push_str(&format!(",{}", i));
        } else {
            res.push(',');
            res.push_str(&tmp);
        }
        tmp.clear();
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
        conn.ok(res)
    }
}
