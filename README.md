# AngrySniffer

This is a small student project trying multithreaded Rust with the Iced GUI library, studying Wi-Fi security and pentesting.

## Prerequisites

*   Linux operating system
*   `aircrack-ng` suite installed
*   Rust programming language and Cargo (for building from source)

## Installation and Usage

If you wish to try it and meet the prerequisites:

1.  Clone the repository:
    ```bash
    git clone <URL>
    cd AngrySniffer/angrysniffer 
    ```
2.  Build:
    ```bash
    cargo run --release
    ```

**Note:** Root privileges (`sudo`) are required because the application modifies the network stack (e.g., creating monitor mode interfaces). Also needs `zenity` and a terminal emulator (set in Settings, or auto-detected).

Long-running tools (airodump / aireplay / aircrack) open in your chosen terminal and appear in the **JOBS** bar — click to select, **×** / **Kill all** to stop.

## Typical workflow

1. **Settings** — set storage directory and local password wordlist.
2. **Kill Services** (optional) — `airmon-ng check kill` so monitor mode works cleanly.
3. **List Interfaces** → pick base iface → **Add Monitor** → bring mon iface **Up**.
4. **Collect Network List** — airodump CSV scan in an external terminal.
5. **Select AP File** — load the CSV; pick a target in the table.
6. **Start Capturing** — focused airodump `.cap` for that BSSID/channel.
7. Optional **Deauth** (needs station MAC) to force a handshake.
8. **Crack Handshake** — finds `{storage}/{essid}-*.cap` and runs `aircrack-ng` with your wordlist; or **Crack Capture File** to pick any `.cap`.

Console lines are tagged: `[info]`, `[ ok ]`, `[err ]`, `[cmd ]`, `[warn]`.

## Overview

AngrySniffer aims to provide a graphical user interface as an abstraction layer over `aircrack-ng`, making its powerful command-line tools more convenient to use.

## Roadmap

### v0.1: Current
*   UI redesign (custom title bar, scrollable controls, AP table).
*   Clearer console logging and validation.
*   Local handshake crack from capture prefix or file picker.

### v0.2: Planned
*   Advanced bruteforcing / hashcat server (local or remote).

### v0.3: Planned
*   `.deb` package for Debian-based systems.

### v0.4: Future
*   Further developments TBD.




\*Partially AI generated