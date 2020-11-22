use socketcan;

fn main() {
    println!("Hello, world!");
    let result = socketcan::CANSocket::open("vcan0");
    let socket = result.unwrap();
    let vec = [1, 2, 3];
    let frame_result = socketcan::CANFrame::new(1, &vec, false, true);
    if frame_result.is_ok() {
        let _blah = socket.write_frame(&frame_result.unwrap());       
        println!("Write success!");
    } else {
        println!("Write failure!");
    }
}
