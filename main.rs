 use std::io;
 fn main(){
    loop{
         println!("\n===WELCOME TO PAN ATLANTIC UNIVERSITY CAFETERIA===");// greeting to enter the cafe 
         println!("\n===THIS IS THE CAFE MENUE===");
         println!("\n {:<15}  {:<15}  {:>10}", "CODE","ITEM","PRICE(N)");//this part creates a displayable table to display to the user to ensure radability
         println!("\n {:<15}  {:<15}  {:>10}", "T","TEA","N800");
         println!("\n {:<15}  {:<15}  {:>10}", "C","COFEE","N1,200");
         println!("\n {:<15}  {:<15}  {:>10}", "S","SANDWICH","N2,000");
         println!("\n {:<15}  {:<15}  {:>10}", "J","JUICE","N1,500");


         let mut input1 = String::new();// get the name of the user 
         println!("\n...Enter your name...: ");
         io::stdin().read_line(&mut input1).expect("Failed to read line");


         let code:String = loop{ // to catch the code incase there may be any errors 
            let mut input = String::new();
            println!("...Enter your purchase code from the menue...");
            println!("..Enter CODE: ");
            io::stdin().read_line(&mut input).expect("failed to read line");
            let input = input.trim().to_uppercase();

            if input == "T" || input == "C" || input == "S" || input == "J"{
                break input;
            } 
            else{
                println!("\n...WRONG INPUT!!!...");
                continue;
            }     
        };

         let mut quantity = String::new();// colects the quantity a person would like to have 
         println!("\n ..You are to enter the amount you want to order..");
         println!("..Enter Quantity: ");
         io::stdin().read_line(&mut quantity).expect("Failed to read line");
         let quantity :u32 = quantity.trim().parse().expect("Invalid number");


         let price :u32 = match code.as_str(){ // this line compares the code to the price and assigns the code to a price which returns the price back
            "T"=>800,
            "C"=>1200,
            "S"=>2000,
            "J"=>1500,
            _ => 0,
        };

         let discount = 5; // created the dicount 

         let mut total_discount_cost = 0;// to calculate total dicount in case the order is 5000 above
         let total_cost = quantity * price;
         if total_cost >= 5000{    // incase the total purchase is 5000
            total_discount_cost= total_cost/discount;
         }
         else {
            println!("YOU HAVE NO DISCOUNT");// incase order price is above 5000
         }

         println!("\n=======TOTAL PURCHASE=======");// breakdown of order
         println!("Name : {}",input1);
         println!("Item code : {}",code);
         println!("Unit price : {}",price);
         println!("Total cost : {}",total_cost);
         println!("Total discount if any {}",total_discount_cost);
         println!("Final cost: {}", total_cost - total_discount_cost);


         let mut quit = String::new();// to catch if other users would like to enter
         println!("\nAnymore users !!!(Y/N): ");
         io::stdin().read_line(&mut quit).expect("failed to read line");
         if quit.trim().to_uppercase()=="Y"{
            println!("\n...WELCOME USER...");
            continue;
         }
         else{
            println!("...THANK YOU...{}",input1);
            break;
         }

            
         





    }
   }
