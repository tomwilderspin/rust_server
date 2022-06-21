use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
  GET,
  POST,
  PUT,
  PATCH,
  HEAD,
  CONNECT,
  DELETE,
  OPTIONS,
  TRACE,
}

impl FromStr for Method {
  type Err = MethodError; 

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "GET" => Ok(Self::GET),
      "POST" => Ok(Self::POST),
      "PUT" => Ok(Self::PUT),
      "PATCH" => Ok(Self::PATCH),
      "HEAD" => Ok(Self::HEAD),
      "CONNECT" => Ok(Self::CONNECT),
      "DELETE" => Ok(Self::DELETE),
      "OPTIONS" => Ok(Self::OPTIONS),
      "TRACE" => Ok(Self::TRACE),
      _ => Err(MethodError),
    }
  }
}

pub struct MethodError; 