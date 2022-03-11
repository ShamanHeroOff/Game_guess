use std::{cmp, io};
use cmp::Ordering;
use rand::Rng;
use io::stdin;
// some execis

fn main (){
    println!("Guess the number!!!");

    //загадываем секретное число
    let secret_number = rand::thread_rng().gen_range(1..100);

    //println!("the is secret number-> {}", secret_number);

    //дабы шанс победы повысился до 100%, фиганём ка цикл провославный
    loop {
        //надо бы прочитать догадку пользователя
        println!("Pleas input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect ("Failed to read line!!!");

        //перед сравнением надо бы привести все к типу u32
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed -> {}", guess);

        //а теперь можно и сравнить, ну и заверщить при победе
        match guess.cmp(&secret_number){
            Ordering::Less => println!("To small"),
            Ordering::Greater => println!("To big"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            }
        }
    }
}