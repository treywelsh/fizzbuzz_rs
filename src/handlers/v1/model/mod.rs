pub mod errors;
pub use errors::Errors;

pub fn fizzbuzz(
    i1: i64,
    i2: i64,
    limit: i64,
    str1: String,
    str2: String,
) -> Result<Vec<String>, Errors> {
    if limit < 1 {
        return Err(Errors::BadParamErr("limit should be greater than 1"));
    }
    if i1 <= 0 {
        return Err(Errors::BadParamErr("i1 should be greater than 0"));
    }
    if i2 <= 0 {
        return Err(Errors::BadParamErr("i2 should be greater than 0"));
    }

    let mut res = vec![String::from("1")];

    let mut tmp = Vec::new();

    for i in 2..=limit {
        if i % i1 == 0 {
            tmp.push(str1.clone());
        }
        if i % i2 == 0 {
            tmp.push(str2.clone());
        }
        if tmp.len() == 0 {
            res.push(format!("{}", i));
        } else {
            res.extend(tmp);
            tmp = Vec::new();
        }
    }

    Ok(res)
}
