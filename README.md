# No Sleep RS 🦀

A lightweight, "voodoo" anti-AFK utility written in Rust. It prevents system sleep and idle status by simulating human-like mouse movements and periodic, non-intrusive keystrokes.

## Features

- **S-Curve Mouse Movement:** Uses trigonometric functions to move the mouse in smooth arcs rather than robotic straight lines.
    
- **Micro-Jitter:** Random $\pm 1$ pixel offsets simulate the natural micro-tremors of a human hand.
    
- **Non-Intrusive Keys:** Periodically "clicks" modifier keys (`Ctrl`, `Alt`, or `Shift`) to signal activity without typing characters into your active windows.
    
- **Multithreaded:** Decoupled mouse and keyboard logic for consistent performance.
    
- **Responsive Exit:** Listen for the `ESC` key to immediately restore terminal settings and stop all activity.
    

## Installation Requirements

### 🐧 Linux

On Linux, this tool interacts with the X11 subsystem via `libxdo`. You must install the development headers for the build to succeed.

**Debian/Ubuntu/Mint:**

```
sudo apt-get update
sudo apt-get install libxdo-dev
```

**Fedora:**

```
sudo dnf install libX11-devel libxdo-devel
```

**Arch Linux:**

```
sudo pacman -S xdotool
```

### 🍎 macOS

MacOS requires explicit user permission for any application that controls the mouse or keyboard.

1. **Build the project:** `cargo build --release`
    
2. **Grant Accessibility Permissions:**
    
    - Open **System Settings** > **Privacy & Security** > **Accessibility**.
        
    - Click the **+** button.
        
    - Navigate to your project folder and select the compiled binary (usually in `target/release/no-sleep-rs`) or your terminal emulator (e.g., iTerm2 or Terminal) if you are running it via `cargo run`.
        
    - Ensure the toggle is turned **ON**.
        

## Technical Architecture

### 1. The S-Curve Algorithm

To avoid simple anti-bot detection, the mouse follows a sine wave trajectory:

$$y(t) = y_{start} + \sin(\pi \cdot t) \cdot 150.0$$

Where $t$ is normalized progress. This creates a human-like "arc" across the screen.

### 2. Concurrency Model

The app uses a **Three-Thread Model** synchronized by an `Arc<AtomicBool>` stop signal:

- **Main Thread:** Handles Terminal Raw Mode and `ESC` detection.
    
- **Mouse Thread:** Manages the S-Curve movement loops.
    
- **Key Thread:** Handles random-interval (10–60s) modifier key presses.
    

## Usage

### Running the tool

```
cargo run
```

### Stopping the tool

Press **`ESC`** at any time while the terminal is focused. The program will:

1. Signal all background threads to stop.
    
2. Disable Terminal Raw Mode (restoring your cursor/input).
    
3. Exit gracefully.
    

> [!WARNING] If you force-kill the process (e.g., `kill -9`), your terminal might remain in "Raw Mode" (weird text input). Simply type `reset` and press Enter to fix it.