// Potentially write a fn or struct to validate the original input that the user gives
// Bug in line 107 in main()

/*

Different kinds of primes that could possibly be implemented:
Easy: Mersenne primes, Real Eisenstein primes, Palindrome primes
Medium: Fermat Primes, circular primes
Hard: Cuban Primes, Chen primes, Balanced primes

*/

use std::io;
use std::io::Write;

fn main() {
    //Initialize the program use counter
    let mut program_use_counter = 0;
    
    //Initialize a way to only output the beggining message once
    let mut initial_message_placeholder = true;
    
  loop {
        println!(
        r#"
                                                           ,---,                                                                       
          ____                                          ,`--.' |                 ,-.----.                                              
        ,'  , `.                                        |   :  :                 \    /  \                              ____           
     ,-+-,.' _ |                                        |   |  '                 |   :    \            ,--,           ,'  , `.         
  ,-+-. ;   , ||   ,---.     ,---.                      '   :  |                 |   |  .\ :  __  ,-.,--.'|        ,-+-,.' _ |         
 ,--.'|'   |  ;|  '   ,'\   '   ,'\   .--.--.           ;   |.'.--.--.           .   :  |: |,' ,'/ /||  |,      ,-+-. ;   , ||         
|   |  ,', |  ': /   /   | /   /   | /  /    '     ,---.'---' /  /    '          |   |   \ :'  | |' |`--'_     ,--.'|'   |  || ,---.   
|   | /  | |  ||.   ; ,. :.   ; ,. :|  :  /`./    /     \    |  :  /`./          |   : .   /|  |   ,',' ,'|   |   |  ,', |  |,/     \  
'   | :  | :  |,'   | |: :'   | |: :|  :  ;_     /    /  |   |  :  ;_            ;   | |`-' '  :  /  '  | |   |   | /  | |--'/    /  | 
;   . |  ; |--' '   | .; :'   | .; : \  \    `. .    ' / |    \  \    `.         |   | ;    |  | '   |  | :   |   : |  | ,  .    ' / | 
|   : |  | ,    |   :    ||   :    |  `----.   \'   ;   /|     `----.   \        :   ' |    ;  : |   '  : |__ |   : |  |/   '   ;   /| 
|   : '  |/      \   \  /  \   \  /  /  /`--'  /'   |  / |    /  /`--'  /        :   : :    |  , ;   |  | '.'||   | |`-'    '   |  / | 
;   | |`-'        `----'    `----'  '--'.     / |   :    |   '--'.     /         |   | :     ---'    ;  :    ;|   ;/        |   :    | 
|   ;/                                `--'---'   \   \  /      `--'---'          `---'.|             |  ,   / '---'          \   \  /  
'---'                                             `----'                           `---`              ---`-'                  `----'   
                                                                                       ,----,                                          
         ,--.                                                                        ,/   .`|                                          
       ,--.'|                       ____                                           ,`   .'  :                    ,--,                  
   ,--,:  : |                     ,'  , `.  ,---,                                ;    ;     /                  ,--.'|                  
,`--.'`|  ' :         ,--,     ,-+-,.' _ |,---.'|               __  ,-.        .'___,/    ,'  ,---.     ,---.  |  | :                  
|   :  :  | |       ,'_ /|  ,-+-. ;   , |||   | :             ,' ,'/ /|        |    :     |  '   ,'\   '   ,'\ :  : '                  
:   |   \ | :  .--. |  | : ,--.'|'   |  ||:   : :      ,---.  '  | |' |        ;    |.';  ; /   /   | /   /   ||  ' |                  
|   : '  '; |,'_ /| :  . ||   |  ,', |  |,:     |,-.  /     \ |  |   ,'        `----'  |  |.   ; ,. :.   ; ,. :'  | |                  
'   ' ;.    ;|  ' | |  . .|   | /  | |--' |   : '  | /    /  |'  :  /              '   :  ;'   | |: :'   | |: :|  | :                  
|   | | \   ||  | ' |  | ||   : |  | ,    |   |  / :.    ' / ||  | '               |   |  ''   | .; :'   | .; :'  : |__                
'   : |  ; .':  | : ;  ; ||   : |  |/     '   : |: |'   ;   /|;  : |               '   :  ||   :    ||   :    ||  | '.'|               
|   | '`--'  '  :  `--'   \   | |`-'      |   | '/ :'   |  / ||  , ;               ;   |.'  \   \  /  \   \  / ;  :    ;               
'   : |      :  ,      .-./   ;/          |   :    ||   :    | ---'                '---'     `----'    `----'  |  ,   /                
;   |.'       `--`----'   '---'           /    \  /  \   \  /                                                   ---`-'                 
'---'                                     `-'----'    `----'                                                                           
        "#
    );
    
    //While loop to ensure that 
    while initial_message_placeholder {
    println!("Welcome to my up and coming prime number tool that is under development!\nI'm hoping to continue to develop this into encryption tools, number theory games, and prime number tools in the future.");
    println!("Perhaps I could create something for statistics?");
    initial_message_placeholder = false;
    }
    
        println!("\nChoose an option:");
        println!("1. Prime number validation");
        println!("2. Prime number generation");
        println!("3. Prime number factorization");
        println!("4. Prime subtype validation");
        println!("5. Encryption Generator");
        println!("6. Prime sum puzzle");
        println!("7. About");
        println!("8. Exit");
        
        let mut tool_choice = String::new();
        io::stdin().read_line(&mut tool_choice).expect("Failed to read line");
        let tool_choice: u32 = match tool_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        
    match tool_choice {
        //Loop for the prime number checker
        1 => {
            println!("\n\nEntering the prime number validation tool...");
            println!("Enter quit, exit, or return in order to leave!");
            loop {
            //Initialization of input and repeating output
            let mut input = String::new();
            println!("\nPlease enter a number to calculate whether it's a prime number:");
        
            //Take in input from the user, check for valid input
            io::stdin().read_line(&mut input).expect("Failed to read line");
            
            let input = input.trim();
            if input == "quit" || input == "Quit" || input == "exit" || input == "Exit" || input == "return" || input == "return" {
                println!("\nExiting the prime number validation tool...");
                break;
            }
            
            //If the input is a string that begins with . then this condition while become true before the string error message.
            //Fix later
            if input.contains('.') {
            println!("\nYour input: {}, has numbers in the decimal place! Please try again!", input);
            continue;
            }
            
            let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
            println!("\nYour input: {}, is a word and not a number! Please try again with a valid integer!", input);
            continue;
              }
            };
            
            if input <= 0 {
            println! ("\nYour input: {}, is a negative number which is not an integer! Please try again!", input);
            continue;
            }
        
            //Passed all test cases and is now running through normal logic
            println!("\nYou entered: {}", input);
            println!("Running Calculations...");
    
            //Call the is_prime function and output return from function
            let result = is_prime(input);
            if result {
                println!("{} is a prime number!", input);
            }
            else {
                println!("{} is not a prime number!", input); 
            }
            program_use_counter += 1;
            println!("Total numbers checked so far: {}", program_use_counter);
          }
        }
        
        2 => {
        
            println!("\n\nEntering the prime number generation tool...");
            println!("Enter quit, exit, or return in order to leave!");
            loop {
            //Take in input from the user, check for valid input
            let mut pngi = String::new();
            io::stdin().read_line(mut pngi).expect("Failed to read line");
            
            let pngi = pngi.trim();
            if pngi == "quit" || pngi == "Quit" || pngi == "exit" || pngi == "Exit" || pngi == "return" || pngi == "return" {
                println!("\nExiting the prime number validation tool...");
                break;
            }
            
            //If the input is a string that begins with . then this condition while become true before the string error message.
            //Fix later
            if pngi.contains('.') {
            println!("\nYour input: {}, has numbers in the decimal place! Please try again!", pngi);
            continue;
            }
            
            let pngi: i32 = match pngi.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
            println!("\nYour input: {}, is a word and not a number! Please try again with a valid integer!", pngi);
            continue;
              }
            };
            
            if pngi <= 0 {
            println! ("\nYour input: {}, is a negative number which is not an integer! Please try again!", pngi);
            continue;
            }
        
            //Passed all test cases and is now running through normal logic
            println!("\nYou entered: {}", pngi);
            println!("Running Calculations...");
            
            //Call the seive of eristathanos function
            let pngiResult = sieve_of_eratosthenes(pngi);
            for i in 2..pngiResult+1 {
                if sieve_array[i] {
                    println!("Yes, {}  is a prime number", i);
                }
            }
         }
            
        }
        
        3 => {
            println!("\nThis feature is not yet available but is being worked on. Stay tuned!");
        }
        
        4 => {
            println!("\nThis feature is not yet available but is being worked on. Stay tuned!");
        }
        
        5 => {
            println!("\nThis feature is not yet available but is being worked on. Stay tuned!");
        }
        
        6 => {
            println!("\nThis feature is not yet available but is being worked on. Stay tuned!");
        }
        
        7 => {
            println!(
        
                r#"
         ░▒▓██████▓▒░░▒▓███████▓▒░ ░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓███████▓▒░ 
        ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░     
        ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░     
        ░▒▓████████▓▒░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░     
        ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░     
        ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░     
        ░▒▓█▓▒░░▒▓█▓▒░▒▓███████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░   ░▒▓█▓▒░   
                "#
                );
                        
                    println!("I wanted to try and develop this prime number tool on rust for a couple of different reasons. 
                    Firstly, I hadn't felt like developing something from a programming perspective in what had felt like an eternity. 
                    There wasn't anything that made me interested enough to go and build something for a long time until I recently feel in love with mathematics");
                 loop {
                // Print a prompt and flush stdout
                print!("Enter 'quit' to exit: ");
                io::stdout().flush().expect("Failed to flush stdout");
        
                // Read user input
                let mut input = String::new();
                io::stdin().read_line(&mut input)
                    .expect("Failed to read line");
        
                // Trim whitespace from input
                let input = input.trim();
        
                // Check if the user wants to quit
                if input == "quit" {
                    break; // Exit the loop
                } else {
                    println!("You entered: {}", input);
                    // Continue the loop if user input is not 'quit'
                  }
                 }
        }
        
        8  => {
            println!("\nThank you for checking out my program!");
            return;
        }
        
        //Exit case for invalid input from the user
        _ => {
            break;   
        }
        
   }
  };
  println!("You fucked it. Nice job!");
}

/*
TO:DO ~ create an input function that streamlines all of the input validation
fn input_validation(q: )
*/

//Function to check whether a number is a prime and return as a bool
    fn is_prime(x: i32) -> bool {
        //0 and 1 are not prime numbers
        if x <= 1 {
            return false 
        }
    
        //2 and 3 are prime numbers
        if x <= 3 {
            return true
        }
    
        //If the input is divisible by 2 or 3, then it is a prime number
        if x % 2 == 0 || x % 3 == 0 {
            return false;
        }
    
        //We are looking to eliminate any numbers in 6k+1 & 6k-1.
        //By setting the initial value to 5 (6(1) - 1) and then adding 1 (6(1) + 1) for the condition, we are able to test every possible combination.
        //This can be represented in a proof that I have written down.
        let mut i = 5;
        while i * i <= x {
            if x % i == 0 || x % (i+2) == 0 {
                return false;
            }
        i += 6;
        }
        true
    }
  
    //Function to determine all prime numbers under n
    fn sieve_of_eratosthenes(n: u32) -> Vec<u32> {
    
    // Initialize Sieve Array with all elements initially set to True
    let mut sieve_array = vec![true; n+1];

    // Set arr[0] and arr[1] to false, because 0 and 1 are not prime
    sieve_array[0] =  false;
    sieve_array[1] = false;

    // Mark all even numbers as false, except 2
    for i in (4..n+1).step_by(2) {
        sieve_array[i] = false;
    }

    // Traverse from 3 to square root of n
    // If a number is prime, mark all its multiples except number itself as false
    // Optimization : Check numbers only upto square root of n
    let mut i = 3;
    while i*i <= n+1 {
        if sieve_array[i] {

            // Mark all the multiples except number itself as false
            // Optimization : start from i*i, because smaller multiples are already marked
            // Optimization : use 2*i as step, because  we need to check only odd multiples
            for j in (i*i..n+1).step_by(2*i) {
                sieve_array[j] = false;
            }
        }
        // We do not have to check even numbers.
        // So, we increment i by 2
        i+=2;
    }

    return sieve_array;
    }
    
