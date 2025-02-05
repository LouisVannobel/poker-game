# Poker Rust Terminal Game

## Description

Un jeu de poker en Rust jouable uniquement dans le terminal avec une interface stylisée. Le jeu prend en charge plusieurs joueurs, incluant des intelligences artificielles avec quatre niveaux de difficulté :

- **Facile**
- **Intermédiaire**
- **Difficile**
- **Extrêmement Difficile**

Chaque joueur commence avec un montant de jetons, qui évolue au fil des paris. Les joueurs qui épuisent leurs jetons sont éliminés.

## Fonctionnalités

- Interface terminal stylisée avec cadres et séparateurs pour une meilleure lisibilité.
- Affichage des cartes avec des symboles immersifs (♥️♠️♦️♣️).
- IA avec quatre niveaux de difficulté simulant différentes stratégies de mise.
- Système de blinds (small blind et big blind).
- Gestion complète des tours de mise (pré-flop, flop, turn, river).
- Conseils stratégiques pour les joueurs humains en fonction de leur main et des pot odds.
- Règle d'élimination pour les joueurs qui perdent tous leurs jetons.

## Prérequis

- **Rust** installé ([installation officielle](https://www.rust-lang.org/tools/install)).
- Un terminal compatible UTF-8 pour afficher les symboles des cartes.

## Installation

Clonez le dépôt et compilez le projet avec Cargo :

```sh
git clone https://github.com/votre-repo/poker-rust-terminal.git
cd poker-rust-terminal
cargo build --release
```

## Utilisation

Lancez le jeu avec :

```sh
cargo run --release
```

Suivez les instructions affichées dans le terminal pour configurer les joueurs et débuter la partie.

## Structure du Projet

```
📂 src/
├── card.rs          # Définition des cartes et symboles
├── main.rs          # Point d'entrée du programme
├── player.rs        # Gestion des joueurs (humains et IA)
├── poker_game.rs    # Mécaniques du jeu et gestion des tours
```

## Exemples de Commandes

1. **Saisir le nombre de joueurs humains et IA**
2. **Sélectionner les niveaux de difficulté pour les IA**
3. **Suivre l'évolution de la partie avec les mises, cartes communes et résultats des tours**


## Licence

Ce projet est sous licence **MIT**.
