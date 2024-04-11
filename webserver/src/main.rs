use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream}, thread, time::Duration,
};

fn main() {
    let tcp_listner = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in tcp_listner.incoming() {
        let tcp_stream = stream.unwrap();

        handle_connection(tcp_stream)
    }
}

fn handle_connection(mut tcp_stream: TcpStream) {
    let buf_reader = BufReader::new(&mut tcp_stream);

    let http_request = buf_reader.lines().next().unwrap().unwrap();

    let (status_code, status_meaning, file) = match http_request.as_str() {
        "GET / HTTP/1.1" => (200, "OK", "hello.html"),
        "GET /hello HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            (200, "OK", "hello_name.html")
        },
        _ => (404, "Not Found", "404.html"),
    };

    let buf = fs::read_to_string(format!("resources/{}", file)).unwrap();
    let len_size = buf.len();
    println!("{buf}");
    let status_line = format!("HTTP/1.1 {status_code} {status_meaning}");
    let http_response = format!("{status_line}\r\nContent-Length: {len_size}\r\n\r\n{buf}");

    tcp_stream.write_all(http_response.as_bytes()).unwrap();
}
