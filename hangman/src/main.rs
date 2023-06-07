use std::io;
use eframe::egui;

fn main() {

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )

    let word = random_word::gen_len(5).unwrap();

    let mut letters: Vec<char> = word.chars().collect();
    let mut guess : Vec<char> = vec![' ';letters.len()];
    let mut guess_as_word: String = guess.iter().collect();

    let mut board: Vec<char>  = vec![' ', '_', '_', '_', '_', '_', '_', '\n',
    '|', ' ', ' ', ' ', ' ', ' ', ' ', '|', '\n',
    '|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '\n',
    '|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '\n',
    '|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '\n',
    '|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '\n'];

    let mut penalty: Vec<char> = vec!['\\', '/', '\\', '/', '|', 'O'];
    let mut penalty_index: Vec<usize> = vec![44, 42, 34, 32, 33, 24];
    
    display_board(&guess, &board);

    while word != guess_as_word{
        println!("Guess a letter!");
        let mut current_guess = String::new();
        io::stdin()
            .read_line(&mut current_guess)
            .expect("Failed to read guess");
        let result: bool = eval_guess(&mut guess, &mut letters, &current_guess.chars().nth(0).unwrap());

        if result == false {
            add_piece(&mut board, &mut penalty, &mut penalty_index);
        }
        display_board(&guess, &board);
        if penalty.is_empty() {
            println!("You Lose");
            return
        }
        guess_as_word = guess.iter().collect();
    }
    println!("You win!");
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

fn display_board(guess: &Vec<char>, board: &Vec<char>) {
    for c in board {
        print!("{}", c);
    }
    println!("{:?}", guess);
}

fn eval_guess(guess: &mut Vec<char>, word: &mut Vec<char>, current_guess: &char) -> bool {
    for (i, c) in word.iter().enumerate() {
        if current_guess == c {
            println!("Your guess was in the word!");
            guess[i] = *c;
            word[i] = ' ';
            return true;
        }
    }
    println!("Sorry that letter is not in the word.");
    false
} 

fn add_piece(board: &mut Vec<char>, penalty: &mut Vec<char>, penalty_index: &mut Vec<usize>) {
    let piece: char = penalty.pop().unwrap();
    if piece == '\\'{
        board.insert(penalty_index.pop().unwrap(), piece);
    } else {
        board[penalty_index.pop().unwrap()] = piece;
    }
}

