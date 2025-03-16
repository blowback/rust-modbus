use tokio_modbus::prelude::*;
use tokio_serial::SerialStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tty_path = "/dev/ttyUSB0";
    let slave = Slave(0x17);

    let builder = tokio_serial::new(tty_path, 9600);
    let port = SerialStream::open(&builder).unwrap();
    println!("Client connected");

    let mut ctx = rtu::attach_slave(port, slave);
    println!("Reading a sensor value");
    let rsp = ctx.read_holding_registers(0x082b, 2).await??;
    println!("Sensor value is {rsp:?}");

    println!("Disconnecting");
    ctx.disconnect().await?;

    Ok(())
}
