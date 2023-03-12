use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    

    println!("Server Listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());


                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
                let request = String::from_utf8_lossy(&buffer[..]);

                println!("Request:\n{}", request);

                let path = match get_path(&request) {
                    Some(p) => p,
                    None => continue,

                };

                let response = match serve_file(&path) {
                    Ok(data) => data,
                    Err(_) => "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_owned(),

                };

                stream.write(response.as_bytes()).unwrap();

            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}

fn get_path(request: &str) -> Option<String> {
    let first_line = match request.lines().next() {
        Some(line) => line,
        None => return None,
    };


    let path = first_line.split_whitespace().nth(1)?;
    let path = path.trim_start_matches('/');
    if path.is_empty() {
        Some("index.html".to_owned())
    } else {
        Some(path.to_owned())
    }
}

fn serve_file(path: &str) -> Result<String, std::io::Error> {
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "")),

    };

    let mut response = "HTTP/1.1 200 OK\r\n\r\n".to_owned();

    if is_text_file(path) {
        let mut reader = std::io::BufReader::new(file);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        response.push_str(&buffer);
    } else {
        let mut reader = std::io::BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        response.push_str(&base64::encode(&buffer));


    }

    Ok(response)
}

fn is_text_file(path: &str) -> bool {

    match path.split('.').last() {
        Some(ext) => match ext.to_lowercase().as_str() {
            "txt" | "html" | "css" | "js" => true,
            _ => false,
        },
        None => false,
    }

}