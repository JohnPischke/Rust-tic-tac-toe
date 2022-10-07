

use std::io;

// This is our main function. Here we ask if they want to play Tic-Tac-Toe and start it if they say yes
fn main() {

    println!("Do you want to play Two-Player Tic-Tac-Toe?");

        
        let mut response = String::new();
        

        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");
        
        if response == "yes\r\n"{
            play_game(); // The function that runs the game
        }

} 

// This function runs the game.

fn play_game() {

// We create our mutible location variable.

    let mut a1 = "-";
    let mut a2 = "-";
    let mut a3 = "-";
    let mut b1 = "-";
    let mut b2 = "-";
    let mut b3 = "-";
    let mut c1 = "-";
    let mut c2 = "-";
    let mut c3 = "-";

// We start our gameplay loop here.

    println!("Player 1 is x");
    println!("Player 2 is o");
    while true{

    print_board(a1,a2,a3,b1,b2,b3,c1,c2,c3);

// Here we get input from Player 1 and record it into the right location.

    println!("Player 1 pick your spot");

        
        let mut guess = String::new();
        

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        if guess == "a1\r\n"{
            a1 = "x";
        }
        if guess == "a2\r\n"{
            a2 = "x";
        }
        if guess == "a3\r\n"{
            a3 = "x";
        }
        if guess == "b1\r\n"{
            b1 = "x";
        }
        if guess == "b2\r\n"{
            b2 = "x";
        }
        if guess == "b3\r\n"{
            b3 = "x";
        }
        if guess == "c1\r\n"{
            c1 = "x";
        }
        if guess == "c2\r\n"{
            c2 = "x";
        }
        if guess == "c3\r\n"{
            c3 = "x";
        }
        print_board(a1,a2,a3,b1,b2,b3,c1,c2,c3);

// Here we check if Player 1 is the winner.        

        if (a1 == "x") & (a2 == "x") & (a3 == "x") | (b1 == "x") & (b2 == "x") & (b3 == "x") 
        | (c1 == "x") & (c2 == "x") & (c3 == "x") | (a1 == "x") & (b1 == "x") & (c1 == "x") 
        | (a2 == "x") & (b2 == "x") & (c2 == "x") | (a3 == "x") & (b3 == "x") & (c3 == "x") 
        | (a1 == "x") & (b2 == "x") & (c3 == "x") | (a3 == "x") & (b2 == "x") & (c1 == "x"){

            println!("Player 1 is the winner");
            break;
        }

//Here we get input from player 2 and record it in the right location.
        println!("Player 2 pick your spot");

        
        let mut guess = String::new();
        

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        if guess == "a1\r\n"{
            a1 = "o";
        }
        if guess == "a2\r\n"{
            a2 = "o";
        }
        if guess == "a3\r\n"{
            a3 = "o";
        }
        if guess == "b1\r\n"{
            b1 = "o";
        }
        if guess == "b2\r\n"{
            b2 = "o";
        }
        if guess == "b3\r\n"{
            b3 = "o";
        }
        if guess == "c1\r\n"{
            c1 = "o";
        }
        if guess == "c2\r\n"{
            c2 = "o";
        }
        if guess == "c3\r\n"{
            c3 = "o";
        }
        print_board(a1,a2,a3,b1,b2,b3,c1,c2,c3);

// This checks if player 2 is the winner.
        if (a1 == "o") & (a2 == "o") & (a3 == "o") | (b1 == "o") & (b2 == "o") & (b3 == "o") 
        | (c1 == "o") & (c2 == "o") & (c3 == "o") | (a1 == "o") & (b1 == "o") & (c1 == "o") 
        | (a2 == "o") & (b2 == "o") & (c2 == "o") | (a3 == "o") & (b3 == "o") & (c3 == "o") 
        | (a1 == "o") & (b2 == "o") & (c3 == "o") | (a3 == "o") & (b2 == "o") & (c1 == "o"){

            println!("Player 2 is the winner");
            break;
        }

// This is checking for a draw.
        if (a1 != "-") & (a2 != "-") & (a3 != "-") & (b1 != "-") & (b2 != "-") & (b3 != "-") & (c1 != "-") & (c2 != "-") & (c3 != "-"){
            println!("Draw");
            break;
        }
        }
     }

// This function draws the board to the terminal

fn print_board(a1: &str, a2: &str, a3: &str, b1: &str, b2: &str, b3: &str, c1: &str, c2: &str, c3: &str,) {
    println!(" |a|b|c");
    println!("1|{a1}|{b1}|{c1}");
    println!("2|{a2}|{b2}|{c2}");
    println!("3|{a3}|{b3}|{c3}");
}