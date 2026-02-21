# Simple Tic-Tac-Toe (Rust)

Ein kleines Terminal-Spiel in Rust mit drei Spielmodi:

- **Player vs Player**
- **Player vs Computer (Easy)** - zufällige Züge
- **Player vs Computer (Hard)** - Minimax-Logik

## Voraussetzungen

- Rust (Edition 2021)
- Cargo

## Starten

```bash
cargo run --bin tictactoe
```

## Tests

```bash
cargo test
```

## Steuerung

1. Spielmodus auswählen (`1`, `2` oder `3`)
2. Zug als `Zeile Spalte` eingeben (z. B. `1 2`)
3. Gültige Werte sind `0`, `1`, `2`
