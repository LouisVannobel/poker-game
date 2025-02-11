//use crate::card::Card;
use crate::card::{Card, Suit};
use crate::player::Player;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;


pub struct PokerGame { // Structure de données pour une partie de poker
    pub deck: Vec<Card>,
    pub players: Vec<Player>,
    pub pot: u32,
    pub community_cards: Vec<Card>,
    pub current_bet: u32,
    pub last_bettor: Option<usize>,
    pub small_blind: u32,
    pub big_blind: u32,
    pub dealer_position: usize,
}

impl PokerGame { 
    pub fn new(players: Vec<Player>) -> Self { // Crée une nouvelle partie de poker avec des joueurs donnés
        let mut game = Self {
            deck: Vec::new(),
            players,
            pot: 0,
            community_cards: Vec::new(),
            current_bet: 0,
            last_bettor: None,
            small_blind: 5,
            big_blind: 10,
            dealer_position: 0,
        };
        game.reset_deck(); // Initialisation du paquet de cartes
        game
    }

    fn reset_deck(&mut self) { // Réinitialise le paquet de cartes pour un nouveau tour
        let suits: Vec<Suit> = vec![Suit::Hearts, Suit::Spades, Suit::Diamonds, Suit::Clubs];
        let ranks = vec!["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];
        self.deck.clear();
        for suit in &suits {
            for rank in &ranks {
                self.deck.push(Card {
                    rank: rank.to_string(),
                    suit: suit.clone(),
                });
            }
        }
        self.deck.shuffle(&mut rand::thread_rng()); // Mélange du paquet
    }

    pub fn run(&mut self) {  // Lancement de la partie
        println!("Début du jeu avec {} joueurs.", self.players.len());
        while self.players.len() > 1 {
            self.new_round();
            self.advance_dealer();
        }
        println!("Le gagnant est {} avec {} jetons!", self.players[0].name, self.players[0].chips);
    }

    fn advance_dealer(&mut self) { // Passe le rôle de croupier au joueur suivant
        self.dealer_position = (self.dealer_position + 1) % self.players.len();
    }

    pub fn new_round(&mut self) {  // Démarre un nouveau tour de jeu
        println!("+==================== Nouveau tour ====================+");
        self.reset_round();
        self.collect_blinds();
        self.deal_hole_cards();
        self.betting_round("Pré-flop");
        self.deal_community_cards(3);
        self.betting_round("Flop");
        self.deal_community_cards(1);
        self.betting_round("Turn");
        self.deal_community_cards(1);
        self.betting_round("River");
        let winner_index = self.determine_winner();
        println!("| Le gagnant de ce tour est {}", self.players[winner_index].name);
        self.players[winner_index].chips += self.pot;
        self.players.retain(|p| p.chips > 0);
        println!("| Nombre de joueurs restants: {}", self.players.len());
    }

    fn reset_round(&mut self) { // Réinitialise les paramètres pour un nouveau tour
        self.reset_deck();
        self.pot = 0;
        self.community_cards.clear();
        self.current_bet = 0;
        self.last_bettor = None;
        for player in &mut self.players {
            player.current_bet = 0;
            player.is_active = true;
            player.hand.clear();
        }
    }

    fn collect_blinds(&mut self) { // Collecte les blinds pour le tour
        let sb_pos = (self.dealer_position + 1) % self.players.len();
        let bb_pos = (self.dealer_position + 2) % self.players.len();
        self.place_blind(sb_pos, self.small_blind, "small");
        self.place_blind(bb_pos, self.big_blind, "big");
        self.current_bet = self.players[bb_pos].current_bet.max(self.players[sb_pos].current_bet);
    }

    fn place_blind(&mut self, position: usize, amount: u32, blind_type: &str) { // Place une blind pour un joueur donné
        let player = &mut self.players[position];
        let blind_amount = player.chips.min(amount);
        player.chips -= blind_amount;
        player.current_bet = blind_amount;
        self.pot += blind_amount;
        if blind_amount < amount {
            println!("| {} posted {} {} blind (all-in)", player.name, blind_amount, blind_type);
        } else {
            println!("| {} posted {} {}", player.name, blind_amount, blind_type);
        }
    }

    fn deal_hole_cards(&mut self) { // Distribue les cartes cachées aux joueurs
        for _ in 0..2 {
            for player in &mut self.players {
                player.hand.push(self.deck.pop().unwrap());
            }
        }
        for player in &self.players {
            println!("{} a reçu: {} et {}", player.name, player.hand[0].to_string(), player.hand[1].to_string());
        }
    }

    fn deal_community_cards(&mut self, count: usize) { // Distribue des cartes communes pour le tour
        for _ in 0..count {
            if let Some(card) = self.deck.pop() {
                self.community_cards.push(card);
            }
        }
        println!("Cartes communes: [{}]", self.community_cards.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", "));
    }

    fn betting_round(&mut self, stage: &str) { // Gère un tour de mise pour un stade donné
        println!("+==================== {} ====================+", stage);
        self.show_human_advice(stage);
        let num_players = self.players.len();
        let starting_index = match stage {
            "Pré-flop" => (self.dealer_position + 3) % num_players,
            _ => (self.dealer_position + 1) % num_players
        };
        let mut ordered_indices: Vec<usize> = (0..num_players)
            .map(|i| (starting_index + i) % num_players)
            .filter(|&i| self.players[i].is_active && self.players[i].chips > 0)
            .collect();
        let mut last_raiser = None;
        let mut current_bet = self.current_bet;
        loop {
            let mut action_occurred = false;
            let mut new_ordered_indices = Vec::new();
            for &i in &ordered_indices {
                let required = current_bet.saturating_sub(self.players[i].current_bet);
                let bet = self.get_bet(i, required);
                let player = &mut self.players[i];
                
                let actual_bet = bet.min(player.chips);
                
                match actual_bet { // Gestion des actions des joueurs
                    0 if required > 0 => {
                        player.is_active = false;
                        println!("| {} se couche.", player.name);
                    },
                    0 => (),
                    _ => {
                        let total_bet = player.current_bet.saturating_add(actual_bet);
                        player.current_bet = total_bet;
                        player.chips = player.chips.saturating_sub(actual_bet);
                        self.pot = self.pot.saturating_add(actual_bet);
                        
                        if total_bet > current_bet {
                            current_bet = total_bet;
                            last_raiser = Some(i);
                            action_occurred = true;
                            println!("| {} relance à {}.", player.name, total_bet);
                        } else {
                            println!("| {} suit avec {}.", player.name, actual_bet);
                        }
                    }
                }
                if player.is_active && player.chips > 0 && player.current_bet < current_bet {
                    new_ordered_indices.push(i);
                }
            }
            ordered_indices = new_ordered_indices;
            if !action_occurred {
                break;
            }
        }
        self.current_bet = current_bet;
        self.last_bettor = last_raiser;
        println!("| Pot total: {} jetons.", self.pot);
    }

    fn get_bet(&self, player_index: usize, required: u32) -> u32 { // Obtient la mise d'un joueur pour un tour donné
        let player = &self.players[player_index];
        
        if required > player.chips {
            return 0;
        }
    
        if player.is_human { // Si le joueur est humain, attend une entrée de l'utilisateur
            println!("| {}, vous avez {} jetons. Mise requise: {}. Entrez votre mise (0 pour passer): ", player.name, player.chips, required);
            loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let bet = input.trim().parse::<u32>();
                match bet {
                    Ok(bet) if bet > player.chips => {
                        println!("| Vous n'avez pas assez de jetons. Maximum possible: {}", player.chips);
                    }
                    Ok(bet) if bet >= required => return bet,
                    Ok(0) if required == 0 => return 0,
                    Ok(0) => return 0,
                    Ok(_) => println!("| Mise insuffisante. Minimum requis: {}", required),
                    _ => println!("| Entrée invalide."),
                }
            }
        } else { // Si le joueur est une IA, détermine la mise en fonction de la difficulté
            let mut rng = rand::thread_rng();
            let raise_chance = match player.name.as_str() {
                "IA-Facile" => 0.2,
                "IA-Intermédiaire" => 0.4,
                "IA-Difficile" => 0.6,
                "IA-Extrêmement-Difficile" => 0.8,
                _ => 0.0,
            };
    
            let max_possible_bet = player.chips;
            
            if rng.gen_bool(raise_chance) && player.chips > required { 
                let max_raise = max_possible_bet.saturating_sub(required);
                if max_raise > 0 {
                    required + rng.gen_range(1..=max_raise)
                } else {
                    required
                }
            } else if player.chips >= required {
                required
            } else {
                0
            }
        }
    }

    fn show_human_advice(&self, stage: &str) { // Affiche des conseils pour les joueurs humains
        for (index, player) in self.players.iter().enumerate() {
            if player.is_human && player.is_active {
                let probability = self.calculate_win_probability(player);
                println!("{} a une probabilité de gagner de {:.2}% pour la phase {}.", player.name, probability * 100.0, stage);
                self.provide_advice(player, probability, index, stage);
            }
        }
    }

    fn calculate_win_probability(&self, player: &Player) -> f64 { // Calcule la probabilité de victoire d'un joueur
        let hand_value = self.evaluate_hand(&player.hand, &self.community_cards);
        (hand_value as f64 / 1000.0).min(1.0)
    }

    fn provide_advice(&self, player: &Player, probability: f64, player_index: usize, stage: &str) { // Fournit des conseils pour un joueur donné
        let pot_odds = self.calculate_pot_odds(player);
        let position = self.get_position(player_index);
        let advice = if probability > 0.7 {
            "Excellente main, envisagez de relancer!"
        } else if probability > 0.4 && pot_odds > 2.5 {
            "Main décente avec de bons pot odds, suivez le pari."
        } else if probability > 0.4 && position == "late" {
            "Main décente en position tardive, envisagez de suivre ou relancer."
        } else if pot_odds > 3.0 {
            "Les pot odds sont favorables, envisagez de suivre."
        } else {
            "Main faible, envisagez de passer."
        };
        println!("| Conseils pour {} lors de la {}: {}", player.name, stage, advice);
    }

    fn calculate_pot_odds(&self, player: &Player) -> f64 { // Calcule les pot odds pour un joueur donné
        if self.current_bet == 0 || player.current_bet >= self.current_bet {
            return 0.0;
        }
        let call_amount = self.current_bet - player.current_bet;
        (self.pot as f64) / (call_amount as f64)
    }

    fn get_position(&self, player_index: usize) -> &str { // Détermine la position d'un joueur
        let num_players = self.players.len();
        if player_index < num_players / 3 {
            "early"
        } else if player_index < 2 * num_players / 3 {
            "middle"
        } else {
            "late"
        }
    }

    fn determine_winner(&self) -> usize { // Détermine le gagnant du tour
        let mut best_score = 0;
        let mut winner_index = 0;
        for (i, player) in self.players.iter().enumerate() {
            if !player.is_active { continue; }
            let score = self.evaluate_hand(&player.hand, &self.community_cards);
            if score > best_score {
                best_score = score;
                winner_index = i;
            }
        }
        winner_index
    }

    fn evaluate_hand(&self, hand: &[Card], community: &[Card]) -> u32 { // Évalue la force d'une main de poker
        let mut all_cards = hand.to_vec();
        all_cards.extend_from_slice(community);
        all_cards.sort_by(|a, b| b.rank_value().cmp(&a.rank_value()));
        if let Some(sf_high) = self.is_straight_flush(&all_cards) {
            return if sf_high == 14 { 1000 } else { 900 + sf_high };
        }
        if let Some(quad_value) = self.get_rank_value_of_multiple(&all_cards, 4) {
            return 800 + quad_value;
        }
        if let Some(fh_value) = self.get_full_house_value(&all_cards) {
            return 700 + fh_value;
        }
        if let Some(flush_value) = self.evaluate_flush(&all_cards) {
            return 600 + flush_value;
        }
        if let Some(straight_high) = self.get_straight_highest(&all_cards) {
            return 500 + straight_high;
        }
        if let Some(triple_value) = self.get_rank_value_of_multiple(&all_cards, 3) {
            return 400 + triple_value;
        }
        if let Some(tp_value) = self.get_two_pair_value(&all_cards) {
            return 300 + tp_value;
        }
        if let Some(pair_value) = self.get_rank_value_of_multiple(&all_cards, 2) {
            return 200 + pair_value;
        }
        all_cards.iter().take(5).map(|c| c.rank_value()).sum()
    }

    fn is_straight_flush(&self, cards: &[Card]) -> Option<u32> {   // Vérifie si une main est une quinte flush
        let flush_suit = self.get_flush_suit(cards)?;
        let suited_cards: Vec<&Card> = cards.iter().filter(|c| c.suit == *flush_suit).collect();
        if suited_cards.len() < 5 { return None; }
        let mut ranks: Vec<u32> = suited_cards.iter().map(|c| c.rank_value()).collect();
        ranks.sort();
        ranks.dedup();
        let mut check_ranks = ranks.clone();
        if check_ranks.contains(&14) { check_ranks.push(1); check_ranks.sort(); }
        check_ranks.windows(5).find(|w| w[4] - w[0] == 4)
            .map(|w| if w[4] == 14 && w[0] == 1 { 5 } else { w[4] })
    }

    fn get_flush_suit<'a>(&self, cards: &'a [Card]) -> Option<&'a crate::card::Suit> { // Détermine la couleur d'une quinte flush
        let mut suit_counts = HashMap::new();
        for card in cards {
            *suit_counts.entry(&card.suit).or_insert(0) += 1;
        }
        suit_counts.iter()
            .find(|(_, &count)| count >= 5)
            .map(|(suit, _)| *suit)
    }

    fn evaluate_flush(&self, cards: &[Card]) -> Option<u32> { // Évalue une couleur
        let flush_suit = self.get_flush_suit(cards)?;
        let mut flush_ranks: Vec<u32> = cards.iter()
            .filter(|c| &c.suit == flush_suit)
            .map(|c| c.rank_value())
            .collect();
        flush_ranks.sort_by(|a, b| b.cmp(a));
        Some(flush_ranks.iter().take(5).sum())
    }

    fn get_straight_highest(&self, cards: &[Card]) -> Option<u32> { // Détermine la valeur de la plus haute carte d'une quinte
        let mut ranks: Vec<u32> = cards.iter()
            .map(|c| c.rank_value())
            .collect();
        ranks.sort();
        ranks.dedup();
        let mut check_ranks = ranks.clone();
        if check_ranks.contains(&14) { check_ranks.push(1); check_ranks.sort(); }
        check_ranks.windows(5).rev()
            .find(|w| w[4] - w[0] == 4)
            .map(|w| w[4])
    }

    fn get_full_house_value(&self, cards: &[Card]) -> Option<u32> { // Détermine la valeur d'un full
        let mut rank_counts = HashMap::new();
        for card in cards {
            *rank_counts.entry(card.rank_value()).or_insert(0) += 1;
        }
        let mut triples = rank_counts.iter()
            .filter(|(_, &v)| v >= 3)
            .map(|(&k, _)| k)
            .collect::<Vec<_>>();
        triples.sort_by(|a, b| b.cmp(a));
        let triple_value = triples.first()?;
        let mut pairs = rank_counts.iter()
            .filter(|(k, &v)| v >= 2 && *k != triple_value)
            .map(|(&k, _)| k)
            .collect::<Vec<_>>();
        pairs.sort_by(|a, b| b.cmp(a));
        pairs.first().map(|pair_value| triple_value * 10 + pair_value)
    }


fn get_two_pair_value(&self, cards: &[Card]) -> Option<u32> { // Détermine la valeur de deux paires
    let mut rank_counts = HashMap::new();
    for card in cards {
        *rank_counts.entry(card.rank_value()).or_insert(0) += 1;
    }

    let mut pairs: Vec<u32> = rank_counts.iter()
        .filter(|(_, &count)| count >= 2)
        .map(|(&rank, _)| rank)
        .collect();

    pairs.sort_unstable_by(|a, b| b.cmp(a));

    if pairs.len() >= 2 { // Si deux paires sont trouvées, retourne la valeur de la plus haute paire
        Some(pairs[0] * 15 + pairs[1]) 
    } else {
        None
    }
}

fn get_rank_value_of_multiple(&self, cards: &[Card], count: u32) -> Option<u32> { // Détermine la valeur d'une main avec plusieurs cartes de même rang
    let mut rank_counts = HashMap::new();
    for card in cards {
        *rank_counts.entry(card.rank_value()).or_insert(0) += 1;
    }

    rank_counts.iter() 
        .filter(|(_, &c)| c >= count)
        .map(|(&rank, _)| rank)
        .max()
}

}