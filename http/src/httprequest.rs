use std::collections::HashMap;
# [derive (Debug ,PartialEq)]
pub enum  Method {
   GET,
   POST,
   UNINITIALIZED,


} 
impl From <&str> for Method {
    fn from (value: &str ) -> Self {
        match value { 
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::UNINITIALIZED,
            
        }
    }
}
#[derive(Debug , PartialEq)]

pub enum Version {
    V1_1,
    V2_0,
    UNINITIALIZED,
}


impl From <&str> for Version {
    fn from (value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::UNINITIALIZED,
        }
    }
 }

#[derive(Debug,PartialEq)]

pub enum Resource {
    Path (String),
}
#[derive(Debug , PartialEq)]
pub struct  HttpRequest{

   pub method: Method,
   pub version: Version,
   pub resource: Resource,
   pub headers: HashMap<String,String>,
   pub msg_body : String,
}

impl From<String> for HttpRequest {

    fn from(req: String) -> Self {
        let mut parsed_method = Method::UNINITIALIZED;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path( "".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines(){
           // process_req_line()
            if line.contains("HTTP") {
                let (method ,resource,version) =process_req_line(line);
                parsed_method =method;
                parsed_version=version;
                parsed_resource=resource;
             //   process_header_line ()
            } else if line.contains(":") { 
                let (key ,value) = process_header_line(line);
                parsed_headers.insert(key, value);
                
            }else if line.len ()== 0 {
                
            }else {
                parsed_msg_body =line;
            }
        }

        HttpRequest {
            method:parsed_method,
            version:parsed_version,
            resource:parsed_resource,
            headers:parsed_headers,
            msg_body:parsed_msg_body.to_string(),
        }
    }
        }
        fn process_req_line(s: &str) ->(Method,Resource,Version) {
            let mut words = s.split_whitespace();
            let method =words.next().unwrap();
            let resource = words.next().unwrap();
            let version =words.next().unwrap();
            (
                method.into(),
                Resource::Path(resource.to_string()),
                version.into(),
            )
          }
        

          fn process_header_line (s: &str) ->(String,String) {
            let mut  header_items = s.split(":");
            let mut key =String::from ("");
            let mut value = "".to_string();
            if let Some(k) = header_items.next(){
                key =k.to_string();
            } 
               if let Some(v) = header_items.next (){
                value = v.to_string()
               }
               (key ,value )

          }


# [cfg(test)]
mod  tests {
    use super::*;
    #[test]
    fn test_method_into (){

        let m: Method = "GET" .into();
        assert_eq!(m,Method::GET);
    }
    #[test]
    fn test_version_into (){
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }
}
   #[test]
   fn test_read_http() {
    
    let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\nHello World!\r\n");
    
    let req : HttpRequest = s.into();

    assert_eq! (Method::GET,req.method);
    assert_eq!(Version::V1_1,req.version);
    assert_eq!(Resource::Path("/greeting".to_string()),req.resource);



    let mut  header_expected:HashMap<String,String> = HashMap::new();
    header_expected.insert("Host".into(), "localhost:3000".into());
    header_expected.insert("User-Agent".into(), "curl/7.64.1".into());
    header_expected.insert("Accept".into(), "*/*".into());


    assert_eq!(Resource::Path("/greeting".to_string()),req.resource);
    
    
   }
