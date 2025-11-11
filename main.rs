use std::io;
fn main(){
    loop{
        println!("---Discount calculation system---");

        let mut name = String::new();
        println!("\n Enter your name");
        io::stdin().read_line(&mut name).expect("failed to read input");

        
            let mut input1 = String::new();
            println!("\nEnter your age ");
            io::stdin().read_line(&mut input1).expect("failed to read line");
            let age :u32 = input1.trim().parse().expect("Not a valid number");

            let mut input2 = String::new();
            println!("\n Enter your membership status(PREMIUM/REGULAR)");
            io::stdin().read_line(&mut input2).expect("Failed to read line");
            let membership =input2.trim().to_uppercase();

            if membership == "PREMIUM" || membership =="REGULAR" {
                println!("\n....Enter your amount (N)....");
            }
            else{
                println!("...Enter a valid membership input");
            }

            let mut input3 = String::new();
            println!("\n...Enter your amount...");
            io::stdin().read_line(&mut input3).expect("Failed to read line");
            let amount : u32 = input3.trim().parse().expect("Invalid input");



            let mut discount = 0;

            // =======Process Calculation=======
            if membership =="PREMIUM" && age>= 60 {
                discount += 20
            }
            else if membership =="PREMIUM" && age >=30 || age <60 {
                discount += 10
            }
            else if membership =="REGULAR" && age >= 60{
                discount += 5
            }
            else{
                println!("NO DISCOUNT");
            }
            let discount_amount = amount * (discount/100);
            let total_amount = amount - discount_amount;



            println!("====SUMMARY====");
            println!("Your status is {}",membership);
            println!("Confirm Your Age {}",age);
            println!("Original Amount:N{:.2}",amount);
            println!("Discount is :N{:.1}%",discount);
            println!("Total amount :N{:.2}",total_amount);

            let mut quit = String::new();
            println!("Do You Want to QUIT (Y/N)");
            io::stdin().read_line(&mut quit).expect("Failed to read line");
            let quit = quit.trim().to_uppercase();

            if quit  == "Y"{
                break;
            }
            else if quit == "N"{
                continue;
            }
            else{
                println!("Invalid input");
                continue;
            }





            
    

    }
}


