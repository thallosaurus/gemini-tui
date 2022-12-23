use std::{
    io::{Read, Write},
    net::TcpStream,
    result,
    sync::Arc,
    thread, str::Bytes,
};

use std::sync::mpsc::channel;

use gemini::Url;
use rustls::{
    Certificate, ClientConfig, RootCertStore, ServerCertVerified, ServerCertVerifier, Session,
    TLSError, DangerousClientConfig, ClientSession,
};
use webpki::DNSNameRef;
struct DummyVerifier {}

impl DummyVerifier {
    fn new() -> Self {
        DummyVerifier {}
    }
}

impl ServerCertVerifier for DummyVerifier {
    fn verify_server_cert(
        &self,
        _: &RootCertStore,
        _: &[Certificate],
        _: DNSNameRef,
        _: &[u8],
    ) -> Result<ServerCertVerified, TLSError> {
        return Ok(ServerCertVerified::assertion());
    }
}

pub struct GeminiError {
    pub error_code: u8,
    pub error_message: String
}

pub fn gemini_request(url: Url) -> Result<gemini::Response, GeminiError>{
    let host = url.host().unwrap();
    let destination = format!("{}:443", &  host);
    let dns_request = host;
    
    let req_enum = RequestType::from(url.clone());
    let request = req_enum.from_enum(url.clone());
    // let request = format!("https://{}/\r\n", &dns_request.to_string());

    //println!("Attempting to visit....{}", destination);

    let mut cfg = rustls::ClientConfig::new();
    let mut config = rustls::DangerousClientConfig { cfg: &mut cfg };
    let dummy_verifier = Arc::new(DummyVerifier::new());
    config.set_certificate_verifier(dummy_verifier);

    let rc_config = Arc::new(cfg);

    let d_copy = dns_request.to_string().clone();
    let dns_name = webpki::DNSNameRef::try_from_ascii_str(&d_copy);

    //let dns_name = String::new();

    match dns_name {
        Ok(_) => {
        }
        Err(e) => return Err(GeminiError { error_code: 0, error_message: String::from("Invalid DNS Name") })
        //Err(_) => {}
    }

    let mut client = rustls::ClientSession::new(&rc_config, dns_name.unwrap());
    let mut socket = TcpStream::connect(destination);

    match socket {
        Ok(_) => {
            //socket = socket.unwrap();
            return send(request, client, socket.unwrap())
        },
        Err(e) => {
            return Err(GeminiError { error_code: 0, error_message: e.to_string() })
        }
    }

    

    //println!("{}", content);
    //rx.send(content.to_string());
}

fn send(request: String, mut client: ClientSession, mut socket: TcpStream) -> Result<gemini::Response, GeminiError> {
    let mut stream = rustls::Stream::new(&mut client, &mut socket);
    stream.write(request.as_bytes()).unwrap();

    while client.wants_read() {
        match client.read_tls(&mut socket) {
            Ok(_) => {},
            Err(e) => {
                return Err(GeminiError {
                    error_code: 0,
                    error_message: e.to_string()
                });
            }
        }
        //.unwrap();
        match client.process_new_packets() {
            Ok(_) => {},
            Err(e) => {
                return Err(GeminiError { error_code: 0, error_message: e.to_string() })
            }
        }
    }
    let mut data = Vec::new();
    let _ = client.read_to_end(&mut data);

    let status = String::from_utf8_lossy(&data);

    let resp = match gemini::parse::parse_response(status.as_bytes()) {
        Ok(r) => r,
        Err(err) => return Err(GeminiError { error_code: 0, error_message: err.input }),
    };
    Ok(resp)


    //println!("{}", status);

    /*client.read_tls(&mut socket).unwrap();
    client.process_new_packets().unwrap();
    let mut data = Vec::new();
    let _ = client.read_to_end(&mut data);

    let content = String::from_utf8_lossy(&data);

    Ok(content.to_string())*/
}

enum RequestType {
    Gemini,
    Https
}

impl RequestType {
    pub fn from_enum(&self, url: Url) -> String {
        match self {
            RequestType::Gemini => {
                let f = format!("gemini://{}{}:1965\r\n", url.host().unwrap(), url.path());
                return f
            },
            RequestType::Https => {
                let f = format!("GET {} HTTP/1.1\r\nHost: {}:443", url.path(), url.host().unwrap());
                return f
            }
        }
    }
}

impl From<Url> for RequestType {
    fn from(value: Url) -> Self {
        match value.scheme() {
            "http*" =>  RequestType::Https,
            "gemini" => RequestType::Gemini,
            _ => RequestType::Gemini,
        }
    }
}