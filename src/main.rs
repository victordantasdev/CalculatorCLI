extern crate termion;
use termion::{color, style};
use core::panic;
use std::io::{stdin, stdout, Write};
use std::process;

fn menu() {
    print!("{}{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), style::Bold, color::Fg(color::Blue));
    println!("--------------------------");
    println!("====> Calculator CLI <====");
    println!("--------------------------");
    print!("{}", style::Reset);
    println!("{}{}Created by victordantasdev{}", style::Italic, color::Fg(color::Cyan), style::Reset);
}

fn read_input(input: &mut String) {
    stdout().flush().expect("Failed to flush!");
    stdin().read_line(input).expect("Failed to read!");
}

fn calculator() {
    menu();

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut op = String::new();

    print!("\nType the first number: ");
    read_input(&mut num1);

    print!("Type the operator [+, -, *, /]: ");
    read_input(&mut op);

    print!("Type the second number: ");
    read_input(&mut num2);


    let num1: f32 = num1.trim().parse().unwrap(); 
    let num2: f32 = num2.trim().parse().unwrap(); 
    let op: char = op.trim().chars().next().unwrap(); 

    let operators = String::from("+-*/");
    if !operators.contains(op) {
        println!("unknown operator!");
        return;
    }

    let result = match op {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => panic!("Error in operator")
    };

    println!("\n{} {} {} = {}\n", num1, op, num2, result);

    let mut option = String::new();

    print!("[1] - Restart calculator;\n[2] - Exit;\nOption: ");
    read_input(&mut option);

    let option: i32 = option.trim().parse().unwrap(); 

    match option {
        1 => main(),
        2 => process::exit(0x0100),
        _ => panic!("Error in operator")
    }
}

fn main() {
    calculator();
}
