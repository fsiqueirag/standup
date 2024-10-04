use colored::*;
use std::io;

fn main() {
    println!("{}", "--------------".bright_magenta());
    println!("{}", "Hola Fabrizio!".bright_magenta());
    println!("{}", "--------------".bright_magenta());

    let mut answers = Vec::new();

    println!("{}", "\nEn qué trabajaste ayer?".bright_green());
    let mut answer = String::new();

    io::stdin().read_line(&mut answer).unwrap();
    let answer = format!("{} {}", "*".magenta(), answer.trim());

    answers.push(answer.to_string());

    loop {
        let mut answer = String::new();

        println!("{}", "\nAlgo más? ".bright_green());

        io::stdin().read_line(&mut answer).unwrap();

        let answer = answer.trim();
        if answer == "no" {
            break;
        }

        let answer = format!("{} {}", "*".magenta(), answer);

        answers.push(answer.to_string());
    }

    let mut todos = Vec::new();

    println!("{}", "\n--------------------------".bright_magenta());
    println!("{}", "\nEn qué vas a trabajar hoy?".bright_green());
    let mut todo = String::new();

    io::stdin().read_line(&mut todo).unwrap();
    let todo = format!("{} {}", "*".magenta(), todo.trim());

    todos.push(todo.to_string());

    loop {
        let mut todo = String::new();

        println!("{}", "\nAlgo más? ".bright_green());

        io::stdin().read_line(&mut todo).unwrap();

        let todo = todo.trim();
        if todo == "no" {
            break;
        }

        let todo = format!("{} {}", "*".magenta(), todo.trim());

        todos.push(todo.to_string());
    }

    println!("{}", "\nHoy trabajaste en:".bright_yellow());
    for answer in answers {
        println!("{}", answer);
    }

    println!("{}", "\nHoy vas a trabajar en:".bright_yellow());
    for todo in todos {
        println!("{}", todo);
    }
}
