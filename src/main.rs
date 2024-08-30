use tun_tap::Iface;
use tun_tap::Mode;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let nic = Iface::new("tun01", Mode::Tun).expect("Could not create tun device");
    let mut buff = [0u8;1504];
    loop {
        let read = nic.recv(&mut buff[..])?;
        eprintln!("read  {} bytes: {:x?}", read, &buff[..read]);
    }
    Ok(())
}
