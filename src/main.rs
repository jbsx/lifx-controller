// extern crate cpal;
// extern crate hound;

// use cpal::StreamConfig;
// use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

// fn main() {
//     let host = cpal::default_host();
//     let device = host.default_input_device().expect("no output device available");
//     let supported_config = device.default_input_config().expect("Failed to get default input config");
//     println!("Default input config: {:?}", supported_config);
//     let config = StreamConfig::from(supported_config);
//     let stream = device.build_input_stream(
//         &config,
//         move |data: &[f32], _: &cpal::InputCallbackInfo| {
//             println!("{:?}lll", &data);
//         },
//         move |err| {
//             println!("{:?}l", &err)
//         },
//     ).unwrap();
//     println!("{:?}", stream.play().unwrap());
// }

use std::{net::UdpSocket, time::Duration, io::Read};
fn main() -> std::io::Result<()>{
    let socket = UdpSocket::bind("192.168.8.132:0")?;
    socket.connect("192.168.8.113:56700")?;

    let buf = [
        0b00110001u8, 0b00000000u8, 0b00000000u8, 0b00010100u8, 0b00000010u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 
        0b11010000u8, 0b01110011u8, 0b11010101u8, 0b01000001u8, 0b10110010u8, 0b01101110u8, 0b00000000u8, 0b00000000u8,
        0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000010u8, 0b00000001u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b01100110u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b01010101u8, 0b01010101u8, 0b11111111u8, 0b11111111u8, 0b11111111u8, 0b11111111u8, 0b10101100u8, 0b00001101u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8
    ];
    
    socket.set_read_timeout(Some(Duration::new(5, 0)))?;
    socket.set_broadcast(true)?;
    socket.send(&buf).expect("couldn't send message");

    println!("Awaiting responses...");
    let mut recv_buff = [0; 100];
    let mut beuf: Vec<String> = vec![];
    while let Ok((n, addr)) = socket.recv_from(&mut recv_buff) {
        println!("{} bytes response from {:?}", n, addr);
        for i in recv_buff.iter(){
            &mut beuf.push(format!("{:#08}", i));            
        }
        println!("{:?}", &beuf);
    }

    Ok(())
}