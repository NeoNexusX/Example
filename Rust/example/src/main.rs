fn main() {
    {
        let s1 = String::from("ownership test");

        // 验证：打印 s1 指向的堆数据地址
        println!("s1 指向的堆地址: {:p}", s1.as_ptr());
        // 验证：打印 s1 变量本身的栈地址
        println!("s1 变量的栈地址: {:p}", &s1);

        let mut s2 = s1;
        // 此时 s1 已经失效 (Move)，不能再打印 s1 了

        // 验证：打印 s2 指向的堆数据地址 (应该与 s1 的一样)
        println!("s2 指向的堆地址: {:p}", s2.as_ptr());
        // 验证：打印 s2 变量本身的栈地址 (应该与 s1 的不一样)
        println!("s2 变量的栈地址: {:p}", &s2);

        take_ownership(&mut s2);
        println!("{s2}, function out!");
    }
    {
        fn first_word(some_string: & String) -> usize {
            let bytes = some_string.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                println!("Index: {}, Value: {}", i, item);
                if item == b' ' {
                    return i;
                }
            }

            some_string.len()
            
        }
        let s = String::from("hello world");
        let word_index = first_word(&s);
        println!("The first word ends at index: {}", word_index);
    }
}

fn take_ownership(some_string: &mut String) {
    //修改参数为String的引用，不需要返回值
    some_string.push_str(", add somethong");
    println!("{} function inside", some_string);
} // 这里，some_string 离开了作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生
