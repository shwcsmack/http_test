use std::io::prelude::{Write, Read};
use std::net::TcpStream;
use std::collections::HashMap;
use std::error::Error;

pub fn run() -> Result<(), Box<Error>> {
    //connect and open stream
    let mut stream = TcpStream::connect("httpbin.org:80")?;

    let req = String::from("GET /anything HTTP/1.1\r\nHost: httpbin.org\r\n\r\n");

    println!("Sending request:\n{}", req);
    stream.write(&req.as_bytes())?;

    println!("Sent req geting resp");
    parse_http_response(&mut stream);

    Ok(())
}

fn parse_http_response(stream: &mut TcpStream) {
    let mut line = 0;
    let mut temp: Vec<u8> = Vec::new();
    let mut http_version = String::new();
    let mut status_code = 0;
    let mut reason_phrase = String::new();
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut data_flag = false;
    let mut bytes_read = 0;
    let mut data: Vec<u8> = Vec::new();


    for byte in stream.bytes() {
        let byte = byte.unwrap();
        

        if line == 0 {
            if http_version.is_empty() && byte != 32 {
                temp.push(byte);
                //println!("Working with http-version: {:?}", temp);
            } else if http_version.is_empty() && byte == 32 {
                //println!("I think this is the end of http-version? {:?}", temp);
                let version: Vec<u8> = temp.drain(..).collect();
                http_version = convert_bytes_to_string(version);
                //println!("http-version is now: {}", http_version);
                //println!("Temp is: {:?}", temp);
            } else if status_code == 0 && byte != 32 {
                temp.push(byte);
            } else if status_code == 0 && byte == 32 {
                //println!("I think this is the end of response code? {:?}", temp);
                let code: Vec<u8> = temp.drain(..).collect();
                status_code = convert_bytes_to_int(code);
                //println!("status-code is now: {}", status_code);
                //println!("Temp is: {:?}", temp);
            } else if reason_phrase.is_empty() && byte != 10 {
                temp.push(byte);
            } else if reason_phrase.is_empty() && byte == 10 {
                //println!("I think this is the end of reason-phrase? {:?}", temp);
                let reason: Vec<u8> = temp.drain(..).collect();
                reason_phrase = convert_bytes_to_string(reason);
                //println!("reason-phrase is now: {}", reason_phrase);
                //println!("Temp is: {:?}", temp);
                line += 1;
            }
        } else if data_flag {
            //handle data here
            if bytes_read < headers["Content-Length"].parse::<i32>().unwrap() -1 {
                //println!("Bytes Read: {}", bytes_read);
                temp.push(byte);
                bytes_read += 1;
            } else {
                data = temp.drain(..).collect();
                //println!("Temp is: {:?}", temp);
                break;
            }
            //println!("{}", byte);
        } else {
            if byte != 10 {
                temp.push(byte);
            } else if byte == 10 && temp[0] == 13 {
                //Should be the start of the data
                data_flag = true;
                temp.drain(..);
            } else if byte == 10 && !temp.is_empty()  {
                //println!("I think this is the end of a line? {:?}", temp);
                let line: Vec<u8> = temp.drain(..).collect();
                let line_text = convert_bytes_to_string(line);
                add_header(&mut headers, line_text.clone());
                //println!("{}", line_text);
            }
        }
    }

    println!("{} {} {}", http_version, status_code, reason_phrase);
    println!("{:?}", headers);
    println!("Data:\n{}", convert_bytes_to_string(data));
    
}

fn convert_bytes_to_string(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

fn convert_bytes_to_int(bytes: Vec<u8>) -> u8 {
    let temp_string = convert_bytes_to_string(bytes);
    temp_string.parse::<u8>().unwrap()
}

fn add_header(headers: &mut HashMap<String, String>, mut headerin: String) {
    //take the carrage return off
    if headerin.contains("\r"){
        headerin.pop();
    }
    let headrstr: &str = &*headerin;
    let parts : Vec<&str> = headrstr.split(": ").collect();
    headers.insert(parts[0].to_string(), parts[1].to_string());
    //println!("{} = {}", parts[0], parts[1]);
}

//===============================================================================================
//===============================================================================================
//===============================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}