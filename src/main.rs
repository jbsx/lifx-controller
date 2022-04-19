extern crate cpal;

use cpal::StreamConfig;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() -> std::io::Result<()> {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("no output device available");
    let supported_config = device.default_input_config().expect("Failed to get default input config");
    let config = StreamConfig::from(supported_config);
    println!("Default input config: {:?}", config);
    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], kj: &cpal::InputCallbackInfo| {
            println!("{:?}", cpal::Sample::from(&data.into()));
        },
        move |err| {
            println!("{:?}", &err)
        },
    ).unwrap();
    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(5000));
    Ok(())
}

// use std::{net::UdpSocket, time::Duration};

// #[allow(dead_code)]
// struct ReqInfo{
//     //HEADER
//     size: u16,
//     protocol: u16,
//     addressable: bool,
//     tagged: bool,
//     origin: u8,
//     source: u32,
//     target: String,
//     res_required: bool,
//     ack_required: bool,
//     sequence: u8,
//     pkt_type: u16,
//     // PAYLOAD
//     hue: u16,
//     saturation: u16,
//     brightness: u16,
//     kelvin: u16,
//     duration: u32
// }

// impl ReqInfo{
//     fn new(target: String, kelvin: u16) -> ReqInfo{
//         ReqInfo { 
//             size: 49, 
//             protocol: 1024, 
//             addressable: true, 
//             tagged: false, 
//             origin: 0, 
//             source: 2, 
//             target, 
//             res_required: false, 
//             ack_required: false, 
//             sequence: 1, 
//             pkt_type: 102, 
//             hue: 0, 
//             saturation: 0, 
//             brightness: 65535, 
//             kelvin, 
//             duration: 0
//         }
//     }

//     fn as_bytes(&self) -> [u8; 49]{
//         [0b00110001u8, 0b00000000u8, 0b00000000u8, 0b00010100u8, 0b00000010u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b11010000u8, 0b01110011u8, 0b11010101u8, 0b01000001u8, 0b10110010u8, 0b01101110u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000010u8, 0b00000001u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b01100110u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b11111111u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b11111111u8, 0b11111111u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8]
//     }
// }
// fn main() -> std::io::Result<()>{
//     let socket = UdpSocket::bind("192.168.8.132:0")?;
//     socket.connect("192.168.8.113:56700")?;
//     socket.set_read_timeout(Some(Duration::new(5, 0)))?;

//     let request = ReqInfo::new(String::from("D073D541B26E"), 0);
//     // socket.send(&request.as_bytes()).expect("couldn't send message");
//     println!{"{:?}", u16::to_le_bytes(65535)};

//     // println!("Awaiting responses...");
//     // let mut recv_buff = [0; 100];
//     // let mut beuf: Vec<String> = vec![];
//     // while let Ok((n, addr)) = socket.recv_from(&mut recv_buff) {
//     //     println!("{} bytes response from {:?}", n, addr);
//     //     for i in recv_buff.iter(){
//     //         beuf.push(format!("{:#08}", i));            
//     //     }
//     //     println!("{:?}", &beuf);
//     // }

//     Ok(())
// }