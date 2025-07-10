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
    cargo run--release
    ```


**Note:** Root privileges (`sudo`) are required because the application modifies the network stack (e.g., creating monitor mode interfaces).

## Overview

AngrySniffer aims to provide a graphical user interface as an abstraction layer over `aircrack-ng`, making its powerful command-line tools more convenient to use.

## Roadmap

### v0.1: Current Stage
*   Refactor parts of the codebase for future development.
*   UI tweaks and improvements.
*   Implement failsafes and better error handling.

### v0.2: Planned
*   Add advanced bruteforcing capabilities, writting server based on hashcat that can be used locally or remotely.

### v0.3: Planned
*   Create a `.deb` package for easier distribution and installation on Debian-based systems.

### v0.4: Future
*   (Further developments to be defined)




\*Partially AI generated