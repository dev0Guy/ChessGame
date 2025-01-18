
# ChessGame

Welcome to **ChessGame**, a Rust-based implementation of a chess game! This project is designed to provide a simple and extensible chess game engine with a focus on clean, modular design and adherence to Rust best practices.

## Features

- **Enum-based Board Representation**:
  - Files (`A` to `H`) and Ranks (`One` to `Eight`) are represented as enums for strong type safety.
- **Chessboard Logic**:
  - Efficient and functional generation of all squares (combinations of files and ranks).
- **Extensibility**:
  - Modular design allows for adding game rules, move validation, and more.
- **Rust Best Practices**:
  - Leverages Rust features like enums, iterators, and pattern matching for clean and idiomatic code.

## Getting Started

### Prerequisites

To build and run this project, you'll need:

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or later recommended)
- A code editor or IDE (e.g., [VS Code](https://code.visualstudio.com/) with Rust Analyzer)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/dev0Guy/ChessGame.git
   cd ChessGame
   ```

2. Switch to the `dev-initlization` branch:

   ```bash
   git checkout dev-initlization
   ```

3. Build the project:

   ```bash
   cargo build
   ```

4. Run the project:

   ```bash
   cargo run
   ```

### Running Tests

To run the unit tests:

```bash
cargo test
```

## Project Structure

```plaintext
ChessGame/
├── src/
│   ├── main.rs        # Entry point of the application
│   ├── board.rs       # Contains logic for board representation
│   ├── enums.rs       # Definitions of File and Rank enums
│   └── lib.rs         # Library module for shared functionality
├── tests/             # Unit and integration tests
├── Cargo.toml         # Project metadata and dependencies
└── README.md          # Project documentation
```

## Usage

### Generating All Board Squares

The project includes a utility to generate all possible chessboard squares (e.g., `A1`, `B2`, etc.):

```rust
fn main() {
    let combinations = all_combinations();
    for (file, rank) in combinations {
        println!("{:?}{:?}", file, rank);
    }
}
```

Run the application to see the output.

### Extending the Game

To add new features like move validation, game state management, or AI, follow these steps:

1. Add relevant logic to the `board.rs` or create new modules as needed.
2. Use `enum` and `match` to handle chess rules cleanly.
3. Write unit tests in the `tests` directory to validate your changes.

## Contributing

Contributions are welcome! If you'd like to contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-name`).
3. Commit your changes (`git commit -m "Add new feature"`).
4. Push to the branch (`git push origin feature-name`).
5. Open a Pull Request.

## License

This project is licensed under the [MIT License](LICENSE). You are free to use, modify, and distribute this software under the terms of the license.

---

Happy coding! If you have any questions or suggestions, feel free to open an issue on the repository. 😊
