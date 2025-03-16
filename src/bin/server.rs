use std::future;
use tokio_modbus::{prelude::*, server::rtu::Server};

struct Service;

impl tokio_modbus::server::Service for Service {
    type Request = SlaveRequest<'static>;
    type Response = Response;
    type Exception = ExceptionCode;
    type Future = future::Ready<Result<Self::Response, Self::Exception>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match req.request {
            Request::ReadInputRegisters(_addr, cnt) => {
                let mut registers = vec![0; cnt.into()];
                registers[2] = 0x77;
                future::ready(Ok(Response::ReadInputRegisters(registers)))
            }
            Request::ReadHoldingRegisters(_addr, cnt) => {
                // future::ready(Err(ExceptionCode::IllegalDataAddress))
                let mut registers = vec![0; cnt.into()];
                registers[1] = 0x77;
                future::ready(Ok(Response::ReadHoldingRegisters(registers)))
            }
            _ => unimplemented!(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server");
    let server_builder = tokio_serial::new("/dev/ttyUSB1", 9600);
    let server_serial = tokio_serial::SerialStream::open(&server_builder).unwrap();

    println!("Server listening");
    let server = Server::new(server_serial);
    let service = Service;

    if let Err(err) = server.serve_forever(service).await {
        eprintln!("Server error: {err}");
    }
    Ok(())
}
