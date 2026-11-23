# CPU Scheduling Simulator

## Project Overview

This is a CPU Scheduling Simulator developed in Rust for the Operating Systems (CSC 308) mini-project. It implements three CPU scheduling algorithms: First Come First Served (FCFS), Shortest Job First (SJF), and Round Robin (RR).

The simulator accepts process information through standard input, executes the selected scheduling algorithm, and displays the execution order, waiting time, turnaround time, and average scheduling statistics.

## Implemented Algorithms

- **FCFS (First Come First Served):** Executes processes in their order of arrival. If two processes arrive at the same time, their input order is preserved.
- **SJF (Shortest Job First):** Uses non-preemptive scheduling, selecting the available process with the shortest burst time. Ties are resolved using arrival time and input order.
- **Round Robin (RR):** Executes processes for a fixed time quantum. Processes that have not completed are returned to the ready queue.

## Input Format

The first line specifies the scheduling algorithm:

- `FCFS`
- `SJF`
- `RR Q`, where `Q` is the time quantum.

Each subsequent line contains a process ID, arrival time, and burst time, separated by spaces.

Example:

```text
RR 2
A 0 5
B 1 3
C 2 1
```

## Output Format

The simulator displays:

- The order in which processes are executed.
- Each process's waiting time and turnaround time.
- Average waiting time and average turnaround time, displayed to two decimal places.

Waiting time is calculated as turnaround time minus burst time. Turnaround time is calculated as completion time minus arrival time.

## Technologies and Requirements

The project is implemented in Rust and uses only the Rust standard library. No external crates are required.

## Building and Running

Build the project using:

```bash
cargo build --release
```

Run the program using:

```bash
cargo run --release
```

The program reads input from standard input and writes scheduling results to standard output.

## Testing

The project includes six public test cases (`p01`–`p06`) provided in the `public/` directory.

The public tests can be executed using the supplied Python self-check script. From the scheduler project directory, run:

```powershell
python ..\selfcheck.py --project . --track scheduler --tests ..
```

The script builds the project and compares its output against the expected results for each public test case.

A successful test run should report:

```text
public: 6/6 passed
  PASS p01
  PASS p02
  PASS p03
  PASS p04
  PASS p05
  PASS p06
```