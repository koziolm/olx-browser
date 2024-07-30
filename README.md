# 🛒 OLX Browser

## 📜 Overview

OLX Browser is a personal project designed to streamline the process of browsing listings on olx.pl, a popular online marketplace. This lightweight tool features a terminal-based user interface and offers functionality for simple queries, data export, and intelligent GPU listing analysis.

## 📌 Note: This project is for personal use and educational purposes. Please respect OLX's terms of service and use responsibly.
## ✨ Features

- 🖥️ Lightweight terminal UI for easy navigation
- 🔍 Perform simple queries on OLX listings
- 💾 Export data to JSON or CSV formats
- 🧠 Fuzzy string matching for GPU listings to benchmark comparisons
- 🏷️ Identify top deals based on performance and price

## 🎬 Demo

[Coming Soon] A GIF or link to the demo will be inserted here to showcase the application in action.

## 🚀 Installation

To get started with OLX Browser, follow these simple steps:

1. Clone the GitHub repository:
```bash
git clone https://github.com/koziolm/olx-browser.git
```
2. Navigate to the project directory:
```bash
cd olx-browser
```
3. Run the application using Cargo:
```bash
cargo run
```

## 🛠️ Future Improvements

- 📊 Enhance code structure by separating UI, analysis, and scraper components
- 🖼️ Add missing UI elements for data analysis and file export notifications
- 📁 Implement better file handling mechanisms
- 🌐 Expand marketplace coverage beyond olx.pl

## 🤝 Contributing
Contributions are welcome! Feel free to submit pull requests or open issues to improve OLX Browser.
## 📄 License
GNU General Public License v3.0



## 📦 Dependencies

OLX Browser relies on the following Rust crates:

- **crossterm** (v0.27.0): Terminal manipulation library
- **scraper** (v0.19.1): HTML parsing and querying with CSS selectors
- **tui** (v0.19.0): Terminal user interface library
- **reqwest** (v0.11): HTTP client for making requests
- **tokio** (v1): Asynchronous runtime for Rust
- **ratatui** (v0.27.0): Rust library to build rich terminal user interfaces
- **serde_json** (v1.0.120): JSON serialization and deserialization
- **serde** (v1.0.204): Serialization and deserialization framework
- **csv** (v1.3.0): CSV file reading and writing
- **fuzzy-matcher** (v0.3.7): Fuzzy string matching algorithms

To install these dependencies, ensure they are listed in your `Cargo.toml` file: