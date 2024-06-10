use std::io;

pub fn io_select_from_list(options : Vec<String>) -> usize {
    for (i, option) in options.iter().enumerate() {
        println!("{i} : {}", option);
    }

    let len = options.len();

    loop {
        let mut ops = String::new();
        io::stdin().read_line(&mut ops).expect("failed to read line");
        if let Ok(op) = ops.trim().parse::<usize>() {
            if op < len {
                return op
            } else {
                println!("请输入可执行的序号");
            }
        }else {
            println!("输入错误,请输入一个自然数");
        }
    }
}