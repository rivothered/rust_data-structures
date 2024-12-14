use structures::stack::Stack;

mod structures;

fn main() {
    println!("Demo stack");
    stack_basic();
    stack_peek();
    stack_iter();
}

fn stack_basic() {
    let mut s = Stack::new();
    s.push(1); s.push(2); s.push(3);

    println!("size: {}, {:?}", s.len(), s);
    println!("pop {:?}, size {}", s.pop().unwrap(), s.len());
    println!("empty: {}, {:?}", s.is_empty(), s);
}

fn stack_peek() {
    let mut s = Stack::new();
    s.push(1); s.push(2); s.push(3);

    println!("{:?}:", s);
    let peek_mut = s.peek_mut();
    if let Some(top) = peek_mut {
        *top = 4;
    }

    println!("top: {:?}", s.peek().unwrap());
    println!("{:?}", s);
}

fn stack_iter() {
    let mut s = Stack::new();
    s.push(1); s.push(2); s.push(3);

    let sum1 = s.iter().sum::<i32>();
    let mut addend = 0;

    for item in s.iter_mut() {
        *item += 1;
        addend += 1;
    }

    let sum2 = s.iter().sum::<i32>();
    println!("{sum1} + {addend} = {sum2}");
}