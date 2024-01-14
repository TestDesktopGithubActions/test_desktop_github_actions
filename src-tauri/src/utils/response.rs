#[derive(serde::Deserialize, serde::Serialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct Response<T> {
    pub code: u32,
    pub message: String,
    pub result: Option<T>,
}

impl<T: serde::Serialize> Response<T> {
    pub fn to_json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

// impl<T> serde::Serialize for Response<T>
// where
//     T: serde::Serialize,
// {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         // use serde::ser::SerializeStruct;
//         use serde::ser::SerializeMap;
//         let mut seq = serializer.serialize_map(Some(3))?;
//         seq.serialize_entry("code", &self.code)?;
//         // seq.serialize_entry("type",&self.typ)?;
//         seq.serialize_entry("message", &self.message)?;

//         // Serialize result as an empty string if it's None, otherwise serialize its value.
//         match &self.result {
//             Some(result) => seq.serialize_entry("result", result)?,
//             None => seq.serialize_entry("result", "{}")?,
//         }
//         seq.end()
//     }
// }

impl<T> From<Result<T, crate::Error>> for Response<T>
where
    T: serde::Serialize + Sized,
{
    fn from(res: Result<T, crate::Error>) -> Self {
        match res {
            Ok(ok) => ok.into(),
            Err(ref err) => {
                let (code, _typ, message) = err.into();
                Response {
                    code,
                    message,
                    result: None,
                }
            }
        }
    }
}

// impl<T> From<Result<T, &crate::Error>> for Response<T>
// where
//     T: serde::Serialize + Sized,
// {
//     fn from(res: Result<T, &crate::Error>) -> Self {
//         match res {
//             Ok(ok) => ok.into(),
//             Err(err) => {
//                 let (code, _typ, message) = (*err).into();
//                 Response {
//                     code,
//                     message,
//                     result: None,
//                 }
//             }
//         }
//     }
// }

impl<T> From<T> for Response<T>
where
    T: serde::Serialize + Sized,
{
    fn from(msg: T) -> Self {
        Self {
            code: 200,
            message: String::new(),
            result: Some(msg),
        }
    }
}

impl From<&crate::Error> for (u32, String, String) {
    fn from(err: &crate::Error) -> Self {
        use crate::Error;
        let (code, typ, message) = match err {
            Error::BadRequest(msg) => (
                msg.get_status_code(),
                "bad request".to_string(),
                msg.to_string(),
            ),
            Error::Http(msg) => (
                msg.get_status_code(),
                "http error".to_string(),
                msg.to_string(),
            ),
            Error::Jwt(jwt_error) => return jwt_error.into(),
            Error::Parse(msg) => (203, "parse error".to_string(), msg.to_string()),
            Error::Reqwest(msg) => (203, "reqwest error".to_string(), msg.to_string()),
            Error::Sqlx(msg) => (203, "sqlx error".to_string(), msg.to_string()),

            Error::QueryTableListFailed(_) => {
                (201, "query table list failed".to_string(), err.to_string())
            }

            Error::CommandChannelSendFailed(_) => (
                201,
                "command channel send failed".to_string(),
                err.to_string(),
            ),
            Error::Database(msg) => (
                msg.get_status_code(),
                "database failed".to_string(),
                msg.to_string(),
            ),
        };
        (code, typ, message)
    }
}

impl From<&crate::JwtError> for (u32, String, String) {
    fn from(err: &crate::JwtError) -> Self {
        use crate::JwtError;
        let (code, typ, message) = match err {
            JwtError::IllegalAccess(_) => (
                err.get_status_code(),
                "illegal access".to_string(),
                err.to_string(),
            ),
            JwtError::TokenExpires => (
                err.get_status_code(),
                "token expires".to_string(),
                err.to_string(),
            ),
        };
        (code, typ, message)
    }
}

impl<T> std::ops::FromResidual<Result<std::convert::Infallible, crate::Error>> for Response<T> {
    fn from_residual(residual: Result<std::convert::Infallible, crate::Error>) -> Self {
        match residual {
            Err(ref err) => {
                let (code, _typ, message) = err.into();
                Response {
                    code,
                    message,
                    result: None,
                }
            }
            Ok(_) => panic!("Infallible"),
        }
    }
}

impl<T> std::ops::FromResidual<Result<std::convert::Infallible, &crate::Error>> for Response<T> {
    fn from_residual(residual: Result<std::convert::Infallible, &crate::Error>) -> Self {
        match residual {
            Err(err) => {
                let (code, _typ, message) = err.into();
                Response {
                    code,
                    message,
                    result: None,
                }
            }
            Ok(_) => panic!("Infallible"),
        }
    }
}

// impl From<crate::JwtError> for (u16, u32, String, String) {
//     fn from(err: crate::JwtError) -> Self {
//         use crate::JwtError;
//         let (code, typ, message) = match err {
//             Error::IllegalAccess(_) => (2001, "illegal access".to_string(), err.to_string()),
//             Error::TokenExpires => (2002, "token expires".to_string(), err.to_string()),
//         };
//         (401, code, typ, message)
//     }
// }
// impl From<crate::service::user::api::error::Error> for (u16, u32, String, String) {
//     fn from(err: crate::service::user::api::error::Error) -> Self {
//         use crate::service::user::api::error::Error;
//         let (code, typ, message) = match err {
//             Error::Service(err) => return err.into(),
//             Error::CardNotExist => (203, "card not exist".to_string(), err.to_string()),
//             Error::IncorrectPassword => (203, "incorrect password".to_string(), err.to_string()),
//             Error::VerifCodeExpired => (203, "verif code expired".to_string(), err.to_string()),
//             Error::TypeVerifCodeExpired => {
//                 (203, "type verif code expired".to_string(), err.to_string())
//             }
//         };
//         (400, code, typ, message)
//     }
// }

// #[cfg(test)]
// mod test {
//     use crate::Error;

//     use super::Response;
//     fn from_ok() -> Response<String> {
//         let str = "success";
//         let t = Result::<_, Error>::Ok(str.to_string()).unwrap();
//         assert_eq!(t, str);
//         Response::default()
//     }
// }
