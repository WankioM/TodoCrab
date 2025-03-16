use std::io;

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    loop {
        println!("TodoCrab 🦀");
        println!(" 1. To add a task");
        println!(" 2. List tasks");
        println!(" 3. Delete task");
        println!(" 4. Exit");
        print!("Enter your choice: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice: char = input.trim().chars().next().unwrap_or(' ');

        match choice {
            '1' => {
                println!("Enter a task");
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                tasks.push(task.trim().to_string());
                println!("Task Added!");
            }
            '2' => {
                println!("You need to work on these:");
                for (i, task) in tasks.iter().enumerate() {
                    println!(" {}. {} ", i + 1, task);
                }
            }
            '3' => {
                println!("Enter task number to delete");
                let mut task_number = String::new();
                io::stdin().read_line(&mut task_number).unwrap();
                if let Ok(index) = task_number.trim().parse::<usize>() {
                    if index > 0 && index <= tasks.len() {
                        tasks.remove(index - 1);
                        println!("Task deleted!");
                    } else {
                        println!("Invalid task number!");
                    }
                } else {
                    println!("Invalid Input");
                }
            }
            '4' => {
                println!("Goodbye!");
                break;
            }
            _ => println!("See ya! If this app had a secret formula, Mr. Krabs would have copyrighted it by now!"),
        }
    }
}
