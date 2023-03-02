pub mod model;

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
        let res =
            match model::fizzbuzz(params.i1, params.i2, params.limit, params.str1, params.str2) {
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
