use std::io;

fn main() {
    let mut todo_list = vec![];
    loop {
        println!("To-d0 list:");

        for (i,task) in todo_list.iter().enumerate() {
            println!("{}.{}",i+1, task);
        }

        println!("Enter command (add, done,delete, quit):");

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        if command == "add" {
        println!("Enter task:");
        let mut task = String::new();
        io::stdin().read_line(&mut task).unwrap();
        let task = task.trim();
        todo_list.push(task.to_string());

        } else if command == "done" {
            println!("Exiting the program...");
            break;

        } else if command == "delete" {
            println!("Enter the task number to ne deleted:");
            let mut task_num = String::new();
            io::stdin().read_line(&mut task_num).unwrap();
            let task_num = task_num.trim();
            // let task_num = task_num.parse::i<usize>().unwrap();

            match task_num.parse::<usize>(){
                Ok(n) => {
                    if n>0 && n<= todo_list.len() {
                    todo_list.remove(n - 1 );
                    println!("Task number {} has been deleted", n);
                    } else {
                    println!("Invalid task number. please enter a valid number");
                    }
                },

                Err(_) => {
                println!("Invalid task number, please enter a valid number");
                }
            }

        } else if command == "quit" {
            break;
        } else {
            println!("Invalid command");
        }
    }
}
