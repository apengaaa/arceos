use arm_pl011::pl011::Pl011Uart;
use std::io::{self};
#[cfg(feature = "axalloc")]
#[cfg(feature = "axstd")]
#[cfg_attr(feature = "axstd", no_mangle)]
#[cfg(all(not(feature = "axstd"), unix))]

macro_rules! print_err {
    ($cmd: literal, $msg: expr) => {
        println!("{}: {}", $cmd, $msg);
    };
    ($cmd: literal, $arg: expr, $err: expr) => {
        println!("{}: {}: {}", $cmd, $arg, $err);
    };
}

type CmdHandler = fn(&str);

const CMD_TABLE: &[(&str, CmdHandler)] = &[
    ("exit", do_exit),
    ("help", do_help),
    ("uname", do_uname),
    ("ldr", do_ldr),
    ("str", do_str),
    ("uart", do_uart),
    ("go", do_go),
    ("moves", do_moves),
    ("test_i2c", test_i2c),
    ("i2c_init",i2c_init),
];

fn do_uname(_args: &str) {
    let arch = option_env!("AX_ARCH").unwrap_or("");
    let platform = option_env!("AX_PLATFORM").unwrap_or("");
    let smp = match option_env!("AX_SMP") {
        None | Some("1") => "",
        _ => " SMP",
    };
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("0.1.0");
    println!(
        "ArceOS {ver}{smp} {arch} {plat}",
        ver = version,
        smp = smp,
        arch = arch,
        plat = platform,
    );
}

fn do_help(_args: &str) {
    println!("Available commands:");
    for (name, _) in CMD_TABLE {
        println!("  {}", name);
    }
}

fn do_exit(_args: &str) {
    println!("Bye~");
    std::process::exit(0);
}

fn do_ldr(args: &str) {
    println!("ldr");

    if args.is_empty() {
        println!("try: ldr ffff0000400fe000 /ldr ffff000040080000 5");
    }

    fn ldr_one(addr: &str, num: i32) -> io::Result<()> {
        if let Ok(parsed_addr) = u64::from_str_radix(addr, 16) {
            let mut address: *const u32 = parsed_addr as *const u32;

            for _ in 0..num {
                let value: u32;
                unsafe {
                    value = *address;
                }
                println!("Value at address {:p}: 0x{:X}", address, value);
                unsafe {
                    address = address.add(1);
                }
            }
        } else {
            println!("Failed to parse address.");
        }
        Ok(())
    }

    let mut iter = args.split_whitespace();
    if let Some(addr) = iter.next() {
        if let Some(num_str) = iter.next() {
            let num: i32 = match num_str.parse() {
                Ok(n) => n,
                _ => 1,
            };
            if let Err(e) = ldr_one(addr, num) {
                println!("ldr {} {}", addr, e);
            }
            return;
        } else {
            let num = 1;
            if let Err(e) = ldr_one(addr, num) {
                println!("ldr {} {}", addr, e);
            }
            return;
        }
    }
}

// use crate::mem::phys_to_virt;
// use core::ptr::{read_volatile, write_volatile};

fn do_str(args: &str) {
    if args.is_empty() {
        println!("try: str ffff0000400fe000 12345678");
    }

    fn str_one(addr: &str, val: &str) -> io::Result<()> {
        if let Ok(parsed_addr) = u64::from_str_radix(addr, 16) {
            let address: *mut u32 = parsed_addr as *mut u32; // 强制转换为合适的指针类型

            if let Ok(parsed_val) = u64::from_str_radix(val, 16) {
                let value: u32 = parsed_val as u32;

                // let ptr = phys_to_virt(parsed_addr.into()).as_mut_ptr() as *mut u32;
                unsafe {
                    *address = value;
                    // write_volatile(address, value);
                    // write_volatile(ptr, value);
                }

                // println!("Write value at address {}: 0x{:X}", addr, value); // 使用输入的地址打印值
            }
        } else {
            println!("Failed to parse address.");
        }

        Ok(())
    }

    let mut split_iter = args.split_whitespace();

    if let Some(addr) = split_iter.next() {
        if let Some(val) = split_iter.next() {
            str_one(addr, val).unwrap(); // 调用 str_one 函数并传递 addr 和 val
        }
    }
}

pub fn run_cmd(line: &[u8]) {
    let line_str = unsafe { core::str::from_utf8_unchecked(line) };
    let (cmd, args) = split_whitespace(line_str);
    if !cmd.is_empty() {
        for (name, func) in CMD_TABLE {
            if cmd == *name {
                func(args);
                return;
            }
        }
        println!("{}: command not found", cmd);
    }
}

fn split_whitespace(str: &str) -> (&str, &str) {
    let str = str.trim();
    str.find(char::is_whitespace)
        .map_or((str, ""), |n| (&str[..n], str[n + 1..].trim()))
}

fn do_go(args: &str) {
    let str_addr1 = "ffff0000fe200004 246c0";
    let str_addr2 = "ffff0000fe2000e4 55000000";
    let str_addr3 = "ffff0000fe201a24 1A";
    let str_addr4 = "ffff0000fe201a28 3";
    let str_addr5 = "ffff0000fe201a2c 70";
    let str_addr6 = "ffff0000fe201a30 301";

    do_str(str_addr1);
    do_str(str_addr2);
    do_str(str_addr3);
    do_str(str_addr4);
    do_str(str_addr5);
    do_str(str_addr6);

    let uart_base = 0xffff_0000_fe20_1a00 as *mut u8;
    let mut uart = Pl011Uart::new(uart_base);

    match args {
        "f" => {
            //前进
            uart.putchar(0xff);
            uart.putchar(0xfc);
            uart.putchar(0x07);
            uart.putchar(0x11);
            uart.putchar(0x01);
            uart.putchar(0x01);
            uart.putchar(0x64);
            uart.putchar(0x00);
            uart.putchar(0x7e);
        }
        "b" => {
            //后退
            uart.putchar(0xff);
            uart.putchar(0xfc);
            uart.putchar(0x07);
            uart.putchar(0x11);
            uart.putchar(0x01);
            uart.putchar(0x02);
            uart.putchar(0x64);
            uart.putchar(0x00);
            uart.putchar(0x7f);
        }
        "s" => {
            //停止
            uart.putchar(0xff);
            uart.putchar(0xfc);
            uart.putchar(0x07);
            uart.putchar(0x11);
            uart.putchar(0x01);
            uart.putchar(0x00);
            uart.putchar(0x00);
            uart.putchar(0x00);
            uart.putchar(0x19);
        }
        "r" => {
            //右转
            uart.putchar(0xff);
            uart.putchar(0xfc);
            uart.putchar(0x07);
            uart.putchar(0x11);
            uart.putchar(0x01);
            uart.putchar(0x06);
            uart.putchar(0x64);
            uart.putchar(0x00);
            uart.putchar(0x83);
        }
        "l" => {
            //左转
            uart.putchar(0xff);
            uart.putchar(0xfc);
            uart.putchar(0x07);
            uart.putchar(0x11);
            uart.putchar(0x01);
            uart.putchar(0x05);
            uart.putchar(0x64);
            uart.putchar(0x00);
            uart.putchar(0x82);
        }
        "w" => {
            //鸣笛
            uart.putchar(0xff);
            uart.putchar(0xfc);
            uart.putchar(0x05);
            uart.putchar(0x02);
            uart.putchar(0x60);
            uart.putchar(0x00);
            uart.putchar(0x67);
        }
        _ => {}
    }
}
fn do_uart(args: &str) {
    match args {
        "5" => {
            let str_addr1 = "ffff0000fe200004 246c0";
            let str_addr2 = "ffff0000fe2000e4 55000000";
            let str_addr3 = "ffff0000fe201a24 1A";
            let str_addr4 = "ffff0000fe201a28 3";
            let str_addr5 = "ffff0000fe201a2c 70";
            let str_addr6 = "ffff0000fe201a30 301";

            do_str(str_addr1);
            do_str(str_addr2);
            do_str(str_addr3);
            do_str(str_addr4);
            do_str(str_addr5);
            do_str(str_addr6);
        }
        _ => {}
    }
}
fn do_moves(args: &str) {
    let str_addr1 = "ffff0000fe200004 246c0";
    let str_addr2 = "ffff0000fe2000e4 55000000";
    let str_addr3 = "ffff0000fe201a24 1A";
    let str_addr4 = "ffff0000fe201a28 3";
    let str_addr5 = "ffff0000fe201a2c 70";
    let str_addr6 = "ffff0000fe201a30 301";

    do_str(str_addr1);
    do_str(str_addr2);
    do_str(str_addr3);
    do_str(str_addr4);
    do_str(str_addr5);
    do_str(str_addr6);

    let uart_base = 0xffff_0000_fe20_1a00 as *mut u8;
    let mut uart = Pl011Uart::new(uart_base);

    let mut iter = args.split_whitespace();
    if let Some(shape) = iter.next() {
        if let Some(num) = iter.next() {
            let mount: i32 = match num.parse() {
                Ok(n) => n,
                _ => 1,
            };

            fn delay(seconds: u64) {
                for i in 1..seconds + 1 {
                    println!("{} ", i);

                    fn fibonacci_recursive(n: u64) -> u64 {
                        if n == 0 {
                            return 0;
                        }
                        if n == 1 {
                            return 1;
                        }
                        return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2);
                    }

                    fibonacci_recursive(34 + (i % 2));
                }
            }
            match shape {
                "s" => {
                    delay(30);
                    for _ in 0..mount {
                        println!("forward");
                        {
                            // 前进
                            uart.putchar(0xff);
                            uart.putchar(0xfc);
                            uart.putchar(0x07);
                            uart.putchar(0x11);
                            uart.putchar(0x01);
                            uart.putchar(0x01);
                            uart.putchar(0x64);
                            uart.putchar(0x00);
                            uart.putchar(0x7e);
                        }
                        delay(4);
                        println!("stop");
                        {
                            // 停止
                            uart.putchar(0xff);
                            uart.putchar(0xfc);
                            uart.putchar(0x07);
                            uart.putchar(0x11);
                            uart.putchar(0x01);
                            uart.putchar(0x00);
                            uart.putchar(0x00);
                            uart.putchar(0x00);
                            uart.putchar(0x19);
                        }
                        delay(1);
                        println!("turn right");
                        {
                            // 右转
                            uart.putchar(0xff);
                            uart.putchar(0xfc);
                            uart.putchar(0x07);
                            uart.putchar(0x11);
                            uart.putchar(0x01);
                            uart.putchar(0x06);
                            uart.putchar(0x64);
                            uart.putchar(0x00);
                            uart.putchar(0x83);
                        }
                        delay(1);
                        println!("stop");
                        {
                            // 停止
                            uart.putchar(0xff);
                            uart.putchar(0xfc);
                            uart.putchar(0x07);
                            uart.putchar(0x11);
                            uart.putchar(0x01);
                            uart.putchar(0x00);
                            uart.putchar(0x00);
                            uart.putchar(0x00);
                            uart.putchar(0x19);
                        }
                        delay(1);
                    }
                    delay(1);
                    {
                        // 停止
                        uart.putchar(0xff);
                        uart.putchar(0xfc);
                        uart.putchar(0x07);
                        uart.putchar(0x11);
                        uart.putchar(0x01);
                        uart.putchar(0x00);
                        uart.putchar(0x00);
                        uart.putchar(0x00);
                        uart.putchar(0x19);
                    }
                }
                "c" => {
                    for _ in 0..mount {
                        delay(10);
                        println!("前进");
                        {
                            //前进
                            uart.putchar(0xff);
                            uart.putchar(0xfc);
                            uart.putchar(0x07);
                            uart.putchar(0x11);
                            uart.putchar(0x01);
                            uart.putchar(0x01);
                            uart.putchar(0x32);
                            uart.putchar(0x00);
                            uart.putchar(0x4c);
                        }
                        delay(1);
                        println!("偏移");
                        {
                            //偏航角PID设置
                            uart.putchar(0xff);
                            uart.putchar(0xfc);
                            uart.putchar(0x0a);
                            uart.putchar(0x14);
                            uart.putchar(0x20);
                            uart.putchar(0x00);
                            uart.putchar(0x20);
                            uart.putchar(0x00);
                            uart.putchar(0x20);
                            uart.putchar(0x00);
                            uart.putchar(0x00);
                            uart.putchar(0x7e);
                        }
                        delay(5);
                    }
                }

                "w" => {
                    for _ in 0..mount {
                        uart.putchar(0xff);
                        uart.putchar(0xfc);
                        uart.putchar(0x05);
                        uart.putchar(0x02);
                        uart.putchar(0x60);
                        uart.putchar(0x00);
                        uart.putchar(0x67);
                        println!("鸣笛");
                    }
                }
                _ => {}
            }
        }
    }
}
fn  i2c_init(_str:&str) {
    let str_addr1 = "ffff000028016000 65";
    let str_addr2 = "ffff000028016004 78";
    let str_addr3 = "ffff000028016014 d4";
    let str_addr4 = "ffff000028016018 f9";
    let str_addr5 = "ffff00002801601c 2a";
    let str_addr6 = "ffff000028016020 4f";
    let str_addr7 = "ffff000028016030 0";
    let str_addr8 = "ffff00002801603c 3";

    do_str(str_addr1);
    do_str(str_addr2);
    do_str(str_addr3);
    do_str(str_addr4);
    do_str(str_addr5);
    do_str(str_addr6);
    do_str(str_addr7);
    do_str(str_addr8);

    println!("i2c init success");

}

fn test_i2c(_str:&str) {
    test()
}