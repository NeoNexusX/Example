use imzml::{ImzML, ImzMLReader};
use std::io::{self, Write};

fn main() {
    let data_path = "/Users/neo/Desktop/Example/Pryut/data/example.imzML";
    let parser = ImzMLReader::from_path(data_path).unwrap();

    for error in parser.errors() {
        println!("{:?}", error);
    }

    let imzml: ImzML <_>= parser.into();

    // 必须手动刷新缓冲区，否则提示语可能因为没有换行符而不立即显示
    io::stdout().flush().unwrap(); 
    // 2. 准备一个“桶”来接水 (创建一个可变的空字符串)
    let mut buffer = String::new();
    // 3. 从标准输入 (stdin) 读取一行放入桶里
    // read_line 会把换行符 \n 也读进去，所以输入 "Tom" 实际是 "Tom\n"
    io::stdin().read_line(&mut buffer).expect("读取失败");
    // 4. 清理数据 (去掉末尾的换行符)
    let input = buffer.trim(); 
    println!("Hello, {}!", input);

}
