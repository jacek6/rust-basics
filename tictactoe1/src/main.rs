use std::fmt;
use std::slice::Iter;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Field {
    Oval,
    Cross,
    Nil
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::Oval => write!(f, "O"),
            Field::Cross => write!(f, "X"),
            Field::Nil => write!(f, "-"),
        }
    }
}

#[derive(Copy, Clone)]
struct Board3x3 {
    items: [Field; 9]
}

impl fmt::Display for Board3x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.field_nil_const_to_string())
    }
}

impl Default for Board3x3 {
    fn default() -> Self {
        Board3x3 {items: [Field::Nil; 9]}
    }
}

#[derive(Copy, Clone, PartialEq)]
enum GameResult {
    OvalWins,
    CrossWins,
    NoWinner,
    StillPlaying
}

impl Board3x3 {

    fn make_a_move(&mut self, who_play: Field, pos: usize) -> bool {
        if self.items[pos] == Field::Nil {
            self.items[pos] = who_play;
            return true
        } else {
            return false
        }
    }

    fn get_game_result(& self) -> GameResult {
        fn is_a_winner(items: &[Field; 9], who: Field) -> bool {
            fn is_3_times_same<F>(items: &[Field; 9], who: Field, index_to_pos: F) -> bool
                where F: Fn(usize) -> usize {
                    items[index_to_pos(0)] == who && items[index_to_pos(1)] == who && items[index_to_pos(2)] == who
                }
            let diagonal_match = is_3_times_same(items, who, |index| index*3 + index);
            let diagonal_second_match = is_3_times_same(items, who, |index| (2 - index)*3 + index);
            if diagonal_match || diagonal_second_match {
                return true
            }
            for i in 0..3 {
                let row_match = is_3_times_same(items, who, |index| index + i*3);
                let col_match = is_3_times_same(items, who, |index| index*3 + i);
                if row_match || col_match {
                    return true
                }
            }
            false
        }
        if is_a_winner(&self.items, Field::Oval) {
            return GameResult::OvalWins;
        }
        if is_a_winner(&self.items, Field::Cross) {
            return GameResult::CrossWins;
        }
        for f in self.items {
            if f == Field::Nil {
                return GameResult::StillPlaying;
            }
        }
        GameResult::NoWinner
    }

    fn field_nil_const_to_string(&self) -> String {
        self.custom_to_string(|_i| String::from("-"))
    }

    fn field_nil_number_to_string(&self, start_num: usize) -> String {
        self.custom_to_string(|i| (start_num + i).to_string())
    }

    fn custom_to_string<F>(&self, nil_to_string: F) -> String
    where
        F: Fn(usize) -> String {

        let mut output: String = String::from("");
        let mut pos_num: usize = 0;
        for i in self.items {
            if i == Field::Nil {
                output.push_str(&nil_to_string(pos_num));
            } else {
                output.push_str(&i.to_string());
            }
            output.push_str(" ");
            pos_num += 1;
            if pos_num % 3 == 0 && pos_num > 0 && pos_num < 7 {
                output.push_str("\n");
            }
        }
        output
    }

}

trait Player {
    fn decide_for_move(&self, who_i_am: &Field, current_state: &Board3x3) -> usize;
}

struct HumanConsolePlayer {}

impl Player for HumanConsolePlayer {
    fn decide_for_move(&self, who_i_am: &Field, current_state: &Board3x3) -> usize {
        println!("");
        let start_num = 1;
        let text_state = current_state.field_nil_number_to_string(start_num);
        println!("Dear human, please make a move for '{who_i_am}'\nwith that game state: \n{text_state}\nposition to place {who_i_am}: ");
        let mut line = String::new();
        loop {
            std::io::stdin().read_line(&mut line).expect("Failed to read line");
            let output: usize = match line.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            return output - start_num
        }
    }
}

struct FirstItemPlayer {}

impl Player for FirstItemPlayer {
    fn decide_for_move(&self, who_i_am: &Field, current_state: &Board3x3) -> usize {
        fn decide_for_move_core(current_state: &Board3x3) -> usize {
            for item in current_state.items.into_iter().enumerate() {
                let (i, x): (usize, Field) = item;
                if x == Field::Nil {
                    return i
                }
            }
            panic!("asked to make a move on board with no Nil field")
        }

        let pos = decide_for_move_core(current_state);
        println!("FirstItemPlayer is going to make a move: {who_i_am} on position: {}", pos+1);
        return pos
    }
}

fn make_a_single_game(p1: Box<dyn Player>, p2: Box<dyn Player>) {
    fn annouce_winner(who: &str) {
        println!("{who} have win!");
    }
    fn make_single_move(board: &mut Board3x3, p: &Box<dyn Player>, who: &Field, player_name: &str) {
        loop {
            println!("Now {player_name} makes a move.");
            let pos_move = p.decide_for_move(who, &board);
            if board.make_a_move(*who, pos_move) {
                println!("Board after move of {player_name}:\n{board}\n");
                break
            } else {
                println!("Invalid move, please choose valid one.")
            }
        }
    }

    let mut board: Board3x3 = Default::default();
    loop {
        if board.get_game_result() != GameResult::StillPlaying {
            break
        }
        make_single_move(&mut board, &p1, &Field::Oval, "Player 1");
        /* old version without inner function
        let p1_pos_move = p1.decide_for_move(Field::Oval, board);
        board.make_a_move(Field::Oval, p1_pos_move);
        println!("Board after move of Player 1:\n{board}\n");
        */

        if board.get_game_result() != GameResult::StillPlaying {
            break
        }
        make_single_move(&mut board, &p2, &Field::Cross, "Player 2");
    }
    match board.get_game_result() {
        GameResult::OvalWins => annouce_winner("Player 1"),
        GameResult::CrossWins => annouce_winner("Player 2"),
        GameResult::NoWinner => println!("No one win"),
        _ => panic!("should never enter here")
    }

    return ()
}

#[derive(Clone)]
enum PlayerType {
    Human,
    FirstMove
}

impl PlayerType {

    fn get_values() -> Iter<'static, PlayerType> {
        static PLAYER_TYPES: [PlayerType; 2] = [PlayerType::Human, PlayerType::FirstMove];
        PLAYER_TYPES.iter()
    }

    fn ask_for_player_type_in_console(player_title: &str) -> PlayerType {
        loop {
            println!("Please choose type of player: {player_title}");
            for (index, player_type) in PlayerType::get_values().enumerate() {
                println!("{}. {}", index + 1, player_type.get_display_name());
            }
            let mut line = String::from("");
            std::io::stdin().read_line(&mut line).expect("unable to read player type");
            let choosen_player_num: usize = match line.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please provide valid number\n");
                    continue
                }
            };
            if 0 < choosen_player_num && choosen_player_num <= PlayerType::get_values().count() {
                let chosen_player_type = match PlayerType::get_values().nth(choosen_player_num - 1) {
                    Some(o) => o.clone(),
                    None => panic!("Invalid code for value check")
                };
                println!("You have choosen {} for {player_title}\n", chosen_player_type.get_display_name());
                return chosen_player_type
            } else {
                println!("Please choose number in range [1, {}]\n", PlayerType::get_values().count());
            }
        }
    }

    fn get_display_name(&self) -> &'static str {
        match self {
            PlayerType::Human => "Human Player",
            PlayerType::FirstMove => "Computer Player who chooses first empty field",
        }
    }

    fn create_player(&self) -> Box<dyn Player> {
        match self {
            PlayerType::Human => Box::new(HumanConsolePlayer {}),
            PlayerType::FirstMove => Box::new(FirstItemPlayer {}),
        }
    }
}

fn ask_yes_no(question: &str) -> bool {
    loop {
        println!("{question}");
        let mut line = String::from("");
        std::io::stdin().read_line(&mut line).expect("Unable to read line");
        line = line.trim().to_lowercase();
        if line == "y" {
            return true
        } else if line == "n" {
            return false
        }
        println!("Please answer with 'y' or 'n'\n")
    }
}

fn make_a_game() {
    loop {
        let player1_type = PlayerType::ask_for_player_type_in_console("player 1");
        let player2_type = PlayerType::ask_for_player_type_in_console("player 2");

        'chosen_players: loop {
            make_a_single_game(player1_type.create_player(), player2_type.create_player());

            if ! ask_yes_no("Would you like to play again? (y/n): ") {
                return
            }
            if ask_yes_no("Would you like change players? (y/n): ") {
                break 'chosen_players
            }
        }

    }
}

fn main() {
    println!("Welcome in tic tac toe game.\n");
    make_a_game();
    println!("\nThank you for playing. Have a nice day.");
}
