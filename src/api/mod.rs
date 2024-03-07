use serde::Serialize;

pub mod message;
pub mod user;

use crate::utils::BDEResult;

#[derive(Serialize)]
pub struct BaseRes<T> {
    code: i16,
    msg: String,
    data: Option<T>,
}

#[derive(Serialize)]
pub struct BoolRes {
    code: i16,
    msg: String,
    data: bool,
}

fn return_base_res<T>(res: BDEResult<T>) -> BaseRes<T> {
    match res {
        Ok(data) => BaseRes {
            code: 200,
            msg: "success".to_string(),
            data: Some(data),
        },
        Err(err) => BaseRes {
            code: 401,
            msg: err.to_string(),
            data: None,
        },
    }
}

fn return_bool_res(res: BDEResult<()>) -> BoolRes {
    match res {
        Ok(()) => BoolRes {
            code: 200,
            msg: "success".to_string(),
            data: true,
        },
        Err(err) => BoolRes {
            code: 401,
            msg: err.to_string(),
            data: false,
        },
    }
}
