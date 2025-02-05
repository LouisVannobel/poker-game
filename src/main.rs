mod poker_game;
mod card;
mod player;

use poker_game::PokerGame;
use player::Player;
use std::io;

fn main() {
    let mut players = Vec::new();

    println!("+==================== Configuration des joueurs ====================+");
    println!("| Entrez le nombre de joueurs humains: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_human_players: usize = input.trim().parse().unwrap_or_else(|_| {
        eprintln!("Entrée non valide, nombre de joueurs humains par défaut: 1");
        1
    });

    for i in 0..num_human_players {
        println!("| Entrez le nom du joueur humain {}: ", i + 1);
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        players.push(Player::new(name.trim().to_string(), true));
    }

    println!("| Entrez le nombre de joueurs IA: ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let num_ai_players: usize = input.trim().parse().unwrap_or(1);

    for i in 0..num_ai_players {
        let difficulty = loop {
            println!("| Choisissez le niveau de difficulté pour l'IA {}: (1) Facile, (2) Intermédiaire, (3) Difficile, (4) Extrêmement Difficile", i + 1);
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            
            match input.trim().parse::<u32>() {
                Ok(level) if level >= 1 && level <= 4 => break level,
                _ => {
                    println!("| Erreur: Veuillez entrer un niveau de difficulté valide (1 à 4)");
                    continue;
                }
            }
        };
    
        let ai_name = format!("IA-{}-{}", i + 1, difficulty);
        players.push(Player::new(ai_name, false));
    }

    println!("+==================== Initialisation du jeu ====================+");
    println!("| Initialisation du jeu avec {} joueurs.", players.len());
    let mut game = PokerGame::new(players);
    game.run();
    println!("+==============================================================+");
}
