fn main() {
    let number: u32 = 0x12345678;

    let byte1 = (number >> 24) & 0xff;
    let byte2 = (number >> 16) & 0xff;
    let byte3 = (number >> 8) & 0xff;
    let byte4 = number & 0xff;

    println!("Byte 1: 0x{:02x}", byte1);
    println!("Byte 2: 0x{:02x}", byte2);
    println!("Byte 3: 0x{:02x}", byte3);
    println!("Byte 4: 0x{:02x}", byte4);

    let number_casted: usize = number as usize;

    println!("Hexa to usize: {number_casted}");
}