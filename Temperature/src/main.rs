use std::io;
fn main() {
    println!("Temperature Converter!");
    loop {
        let mut choice = String::new();
        println!("Enter 1 to Convert F to C");
        println!("Enter 2 to convert C to F");
        io::stdin().read_line(&mut choice).
            expect("Failed to read line");
        let choice: u32 = match choice.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice != 1 && choice != 2{
            println!("Enter a valid number");
            continue;
        }
        let mut number = String::new();
        println!("Enter number to convert");
        io::stdin().read_line(&mut number).
            expect("Failed to read line");
        let number:f64 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        if choice ==1 {
            ftoc(&number);
        }
        else if choice == 2{
            ctof(&number);
        }
        let mut choice = String::new();
        println!("Do you wish to quit press Q");
        io::stdin().read_line(&mut choice).
            expect("Failed to read line");
        if choice.trim().to_lowercase() == "q" {
            break;
        }

    }
    println!("Good Bye!");
}

fn ftoc(x: &f64){
    println!(" {} degree F is {} degree C", x, ((x - 32.0)*5.0/9.0));
}

fn ctof(x: &f64){
    println!(" {} degree C is {} degree F", x, ((x * 9.0/5.0)+32.0));
}



