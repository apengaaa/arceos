use log::debug;

const CREG_MIO_FUNC_SEL: u32 = 0x1000; // MIO功能选择寄存器的地址
const IC_CON: u32 = 0x00;              // I2C控制寄存器地址
const IC_TAR: u32 = 0x04;              // I2C主机地址寄存器地址
const IC_DATA_CMD: u32 = 0x10;         // I2C数据寄存器地址
const IC_RAW_INTR_STAT:u32 = 0x34;     // I2C原始中断状态（真实的中断状态）寄存器
const IC_ENABLE: u32 = 0x6c;           // I2C使能寄存器地址
const IC_STATUS: u32 = 0x70;           // I2C状态寄存器地址

const BASE_ADDR: u32 =  0x2801_6000;   // 设备基地址

// const TX_FIFO_NOT_FULL_MASK: u8 = 0b0000_0010; //  状态位掩码，bit[1] - 发送 FIFO 不满



/// 读取寄存器函数
fn read_reg(addr: u32) -> u32 {
    unsafe { *(BASE_ADDR.wrapping_add(addr) as *const u32) }
}

/// 写入寄存器函数
fn write_reg(addr: u32, value: u32) {
    unsafe { *(BASE_ADDR.wrapping_add(addr) as *mut u32) = value; }
}

/// 配置 I2C 控制器为 Master 模式
pub fn i2c_init() {
    // 选择 I2C 模式
    write_reg(CREG_MIO_FUNC_SEL, 0x00);

    // 禁用 I2C 控制器
    write_reg(IC_ENABLE, 0x00);

    // 设置 I2C 控制参数：主模式、7位地址、标准速率 
    write_reg(IC_CON, 0x65);

    // 设置目标设备地址
    write_reg(IC_TAR, 0x78);

    // 使能 I2C 控制器
    write_reg(IC_ENABLE, 0x01);

    debug!("i2c init success");
}

fn delay(seconds: u64) {
    for i in 1..seconds + 1 {
        fn fibonacci_recursive(n: u64) -> u64 {
            if n == 0 {
                return 0;
            }
            if n == 1 {
                return 1;
            }
            return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2);
        }
        fibonacci_recursive(36 + (i % 2));
    }
}
pub fn i2c_write(data: &[u8]) {
    for (i, &byte) in data.iter().enumerate() {
        // 增加重试机制
        let mut retries = 3;
        while retries > 0 {
            // 等待发送FIFO不满
            while read_reg(IC_STATUS) & (1 << 1) == 0  {
                let mut timeout = 10;
                timeout -= 1;
                if timeout == 0 {
                    debug!("FIFO still full, retrying...");
                    retries -= 1;
                    break;
                }
            }

            // debug!("start send data");

            // 发送数据
            let mut cmd = byte as u32;
            if i == data.len() - 1 {
                cmd |= 1 << 9; // 最后一个字节，加停止信号
            }
          
            write_reg(IC_DATA_CMD, cmd);
            

            // 增加延时
            delay(3);
            

            // 检查是否成功写入
            let status = read_reg(IC_RAW_INTR_STAT);
            if status & (1 << 9) == 0 {
                // 成功写入，打印调试信息
                debug!("Data 0x{:02X} sent successfully", byte);
                break;
            } else {
                // 重试
                retries -= 1;
                debug!("Retry sending data 0x{:02X}, {} retries left", byte, retries);
            }
        }

        if retries == 0 {
            debug!("Failed to send data 0x{:02X}", byte);
            return;
        }
    }
}

pub fn OLED_write_cmd(cmd: &[u8]) {
	
	i2c_write(&[0x3c]);   //从机地址，SA0=0,RW=0 
	
	
	i2c_write(&[0x00]);   //控制字节，Co=0,D/C#=0
	
	
	i2c_write(cmd);   //命令码
	
}

pub fn OLED_write_data(data: &[u8]) {
	
	i2c_write(&[0x3c]);   // 从机地址，SA0=0,RW=0
	
	i2c_write(&[0x40]);   //控制字节，Co=0,D/C#=1
	
	i2c_write(data);   //数据值

}

pub fn ssd1306_init() {
    // let init_commands: &[u8] = &[
    //     0xAE, // Display off
    //     0xD5, 0x80, // Set display clock divide ratio/oscillator frequency
    //     0xA8, 0x1F, // Set multiplex ratio (1 to 32). For 128x32, use 0x1F
    //     0xD3, 0x00, // Set display offset
    //     0x40, // Set start line address (0x40 | 0x00)
    //     0x8D, 0x14, // Charge pump setting (0x10 External, 0x14 Internal DC/DC)
    //     0x20, 0x00, // Memory mode
    //     0xA1, // Segment remap
    //     0xC8, // COM output scan direction
    //     0xDA, 0x02, // COM pins hardware configuration. For 128x32, use 0x02
    //     0x81, 0x8F, // Contrast control
    //     0xD9, 0xF1, // Pre-charge period
    //     0xDB, 0x40, // VCOMH Deselect level
    //     0xA4, // Enable display output
    //     0xA6, // Normal display (0xA6 normal, 0xA7 inverse)
    //     0x2E, // Deactivate scroll
    //     0xAF, // Display ON     
    // ];

    // for (i, &cmd) in init_commands.iter().enumerate() {
    //     debug!("Sending command {}: 0x{:02X}", i, cmd);
    //     i2c_write(&[cmd]);
        
    //     for _ in 0..10 {
    //         assert!(true)
    //     }
    // }
    // debug!("SSD1306 initialization commands sent");

    let init_commands: &[u8] = &[
        0x00,
        0xAE, 
        0x40, 
        0xb0,
        0xc8,
        0x81,
        0xff,
        0xa1,
        0xa6,
        0xa8,
        0x1f,
        0xd3,
        0x00,
        0xd5,
        0xf0,
        0xd9,
        0x22,
        0xda,
        0x02,
        0xdb,
        0x49,
        0x8d,
        0x14,
        0xaf,    
    ];

    // for (i, &cmd) in init_commands.iter().enumerate() {
    //     debug!("Sending command {}: 0x{:02X}", i, cmd);
    //     i2c_write(&[cmd]);
        
    //     for _ in 0..10 {
    //         assert!(true)
    //     }
    // }
    i2c_write(init_commands);

    // oled_clear();
        
    debug!("SSD1306 initialization commands sent");
    

	//0.91寸IIC接口OLED初始化
    // delay(1);

	// OLED_write_cmd(&[0xAE]);
    // OLED_write_cmd(&[0x40]);//---set low column address
    // OLED_write_cmd(&[0xB0]);//---set high column address
    // OLED_write_cmd(&[0xC8]);//-not offset
    // OLED_write_cmd(&[0x81]);
    // OLED_write_cmd(&[0xff]);
    // OLED_write_cmd(&[0xa1]);
    // OLED_write_cmd(&[0xa6]);
    // OLED_write_cmd(&[0xa8]);
    // OLED_write_cmd(&[0x1f]);
    // OLED_write_cmd(&[0xd3]);
    // OLED_write_cmd(&[0x00]);
    // OLED_write_cmd(&[0xd5]);
    // OLED_write_cmd(&[0xf0]);
    // OLED_write_cmd(&[0xd9]);
    // OLED_write_cmd(&[0x22]);
    // OLED_write_cmd(&[0xda]);
    // OLED_write_cmd(&[0x02]);
    // OLED_write_cmd(&[0xdb]);
    // OLED_write_cmd(&[0x49]);
    // OLED_write_cmd(&[0x8d]);
    // OLED_write_cmd(&[0x14]);
	// OLED_write_cmd(&[0xaf]); //打开显示
 
   //清除显示
   // debug!("ssd1306 init success")
}


pub fn oled_clear() {
    let init_commands: &[u8] = &[
        0x00,
        0x20, 
        0x00,
        0x21,
        0x00,
        0x7f,
        0x22,
        0x00,
        0x07   
    ];

    i2c_write(init_commands);

    // for (i, &cmd) in init_commands.iter().enumerate() {
    //     debug!("Sending command {}: 0x{:02X}", i, cmd);
    //     i2c_write(&[cmd]);
    // }
    let command: &[u8] = &[0x30,0x40];

    i2c_write(command);

    let commands: &[u8] = &[0xff];
    
    for i in 0..8 {           // 0-7页
        for j in 0..64 {     // 0-127列
            i2c_write(commands);
        }
    }
    
    debug!("SSD1306 clear oled");
}


fn display_text(text: &str) {
    let mut buffer: [u8; 1025] = [0; 1025];
    buffer[0] = 0x40; // Co = 0, D/C# = 1
    
    for (i, &byte) in text.as_bytes().iter().enumerate() {
        buffer[i + 1] = byte;
    }

    i2c_write(&buffer[..text.len() + 1]);
}

// const FONT: [[u8; 5]; 5] = [
//     [0x7C, 0x12, 0x11, 0x12, 0x7C], // H
//     [0x7F, 0x49, 0x49, 0x49, 0x36], // E
//     [0x7F, 0x01, 0x01, 0x01, 0x03], // L
//     [0x7F, 0x01, 0x01, 0x01, 0x03], // L
//     [0x3E, 0x41, 0x41, 0x41, 0x3E], // O
// ];

// pub fn ssd1306_send_command(cmd:u8) {
//     let buffer = [0x00, cmd];
//     i2c_write( &buffer)
// }

// pub fn ssd1306_send_data(data:u8) {
//     let buffer = [0x3c,0x40, data];
//     i2c_write(&buffer)
// }

// pub fn ssd1306_display_hello() {
//     let start_page = 1;
//     let start_col = 0;
    
//     for (i, &letter) in FONT.iter().enumerate() {
//         debug!("commond");
//         ssd1306_send_command(0xB0 + start_page); // Set page address
//         ssd1306_send_command(0x00 + (start_col + i * 6) as u8 & 0x0F); // Set lower column address
//         ssd1306_send_command( 0x10 + ((start_col + i * 6) >> 4) as u8); // Set higher column address

        
//         debug!("data");
//         for &byte in &letter {
//             ssd1306_send_data(byte);
//         }
        
//         ssd1306_send_data(0x00); // Space between characters
//     }

// }

pub fn test() {
    i2c_init();
    debug!("i2c init success");

    ssd1306_init();

    let text = "Hello, Phytium!";
    display_text(text);

    loop {}
}


