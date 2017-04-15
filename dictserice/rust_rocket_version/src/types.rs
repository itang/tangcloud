use serde::ser::Serialize;
use rocket_contrib::JSON;


#[derive(Serialize, Debug)]
pub struct Resp<T: Serialize> {
    ok: bool,
    code: i32,
    message: Option<String>,
    data: Option<T>,
}

impl<T: Serialize> Resp<T> {
    pub fn json_ok<S: Into<String>>(code: i32,
                                    message: Option<S>,
                                    data: Option<T>)
                                    -> JSON<Resp<T>> {
        Resp::json(true, code, message, data)
    }

    pub fn json_err<S: Into<String>>(code: i32,
                                     message: Option<S>,
                                     data: Option<T>)
                                     -> JSON<Resp<T>> {
        Resp::json(false, code, message, data)
    }

    pub fn json<S>(ok: bool, code: i32, message: Option<S>, data: Option<T>) -> JSON<Resp<T>>
        where S: Into<String>
    {
        JSON(Resp {
                 ok: ok,
                 code: code,
                 message: message.map(|x| x.into()),
                 data: data,
             })
    }
}


pub type ResultJSONResp<T /*: Serialize*/, E /*: Serialize*/> = Result<JSON<Resp<T>>,
                                                                       JSON<Resp<E>>>;

#[derive(Serialize, Debug)]
pub struct Id<T: Serialize>(pub T);
