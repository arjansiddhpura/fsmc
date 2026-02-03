# fsmc: Finite State Machine Compiler

**fsmc** is a lightweight compiler written in Rust that transforms a custom Finite State Machine (FSM) definition language into:
1.  **C Code** (`.c`) - A ready-to-compile implementation of the state machine.
2.  **Graphviz DOT** (`.dot`) - A visualization description of the state machine logic.

This tool is designed to simplify the development of state machines for embedded systems, game logic, or protocol handlers by allowing you to define them in a clean, readable DSL (Domain Specific Language) and automatically generating the boilerplate code and documentation diagrams.

## Features

*   **Simple DSL:** Clean syntax for defining machines, states, and transitions.
*   **Visualization:** Automatically generates `.dot` files to visualize your state machine using Graphviz.
*   **Portable C Output:** Generates standard C code compatible with GCC, Clang, and embedded compilers.
*   **Rust-Powered:** Fast and safe compilation.

## Getting Started

### Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install) (cargo)
*   (Optional) [Graphviz](https://graphviz.org/) (for rendering `.dot` files)
*   (Optional) GCC/Clang (for compiling the generated C code)

### Building

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/your-username/fsmc.git
cd fsmc
cargo build --release
```

The executable will be located in `./target/release/fsmc`.

## Usage

Run the compiler by providing a `.fsm` file as an argument:

```bash
cargo run -- example/lander.fsm
# OR if built:
./target/release/fsmc example/lander.fsm
```

### Output

The compiler produces two files in the same directory as the input:
*   `filename.c`
*   `filename.dot`

## The FSM Language

The input format is simple and intuitive. Here is an example describing a spacecraft landing sequence (`example/lander.fsm`):

```javascript
machine Lander {
    state Cruise {
        on EntryInterface -> AtmosphericEntry;
    }
    state AtmosphericEntry {
        on HeatShieldFailure -> BurnUp;
        on StableDescend -> ParachuteDeploy;
        on Turbulence -> AtmosphericEntry;
    }
    state ParachuteDeploy {
        on ChuteFailure -> BackupChute;
        on LowAltitude -> PoweredDescent;
    }
    state BackupChute {
        on ChuteFailure -> Crash;
        on LowAltitude -> PoweredDescent;
    }
    state PoweredDescent {
        on EngineFailure -> Crash;
        on FuelDepleted -> Landing;
        on Correction -> PoweredDescent;
    }
    state Landing {
        on Touchdown -> Safe;
        on TipOver -> Crash;
    }
    state Safe {
        on Shutdown -> Finished;
    }
    state Crash {
        on RecoveryAttempt -> Cruise;
    }
    state BurnUp {}
    state Finished {}
}
```

![Lander FSM Diagram](example/lander.png)

### Syntax Guide
*   **`machine Name { ... }`**: Defines the FSM container.
*   **`state Name { ... }`**: Defines a state.
*   **`on Event -> NextState;`**: Defines a transition triggered by `Event` leading to `NextState`.

## Workflow Example

1.  **Write your FSM** in `example/lander.fsm`.
2.  **Compile it:**
    ```bash
    cargo run -- example/lander.fsm
    ```
3.  **Visualize the graph** (requires Graphviz):
    ```bash
    dot -Tpng example/lander.dot -o example/lander.png
    open example/lander.png
    ```
4.  **Compile the C code:**
    ```bash
    gcc -o lander example/lander.c
    ./lander
    ```

## Generated C Code Structure

The generated C file is self-contained and typically includes:
*   `typedef enum` definitions for **States**.
*   `state_name`: A helper function to get the string representation of a state.
*   `is_terminal`: A helper function to check if a state is a terminal state.
*   `print_available_events`: A helper function to list valid events for the current state.
*   `next_state`: The core transition function that accepts the current state and an event string, returning the next state.
*   `main`: A driver loop that allows you to interact with the FSM via standard input.

## License

[MIT](LICENSE)
