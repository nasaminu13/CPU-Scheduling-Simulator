use std::io::{self, Read};
use std::collections::VecDeque;

enum Algorithm {
    Fcfs,
    Sjf,
    RoundRobin(u32),
}

struct Process {
    id: String,
    arrival_time: u32,
    burst_time: u32,
    input_order: usize,
    remaining_time: u32,
    completion_time: Option<u32>,
}

fn parse_input(input: &str) -> Result<(Algorithm, Vec<Process>), String> {
    let mut lines = input.lines();

    let first_line = lines
        .next()
        .ok_or("Missing scheduling algorithm")?;

    let algorithm_parts: Vec<&str> = first_line.split_whitespace().collect();

    let algorithm = match algorithm_parts.as_slice() {
        ["FCFS"] => Algorithm::Fcfs,
        ["SJF"] => Algorithm::Sjf,
        ["RR", quantum] => {
            let q: u32 = quantum
                .parse()
                .map_err(|_| "Invalid time quantum")?;

            if q == 0 {
                return Err("Time quantum must be greater than zero".to_string());
            }

            Algorithm::RoundRobin(q)
        }
        _ => return Err("Invalid scheduling algorithm".to_string()),
    };

    let mut processes = Vec::new();

    for (index, line) in lines.enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        if parts.len() != 3 {
            return Err(format!("Invalid process on line {}", index + 2));
        }

        let arrival_time: u32 = parts[1]
            .parse()
            .map_err(|_| format!("Invalid arrival time on line {}", index + 2))?;

        let burst_time: u32 = parts[2]
            .parse()
            .map_err(|_| format!("Invalid burst time on line {}", index + 2))?;

        if burst_time == 0 {
            return Err(format!("Burst time must be positive on line {}", index + 2));
        }

        processes.push(Process {
            id: parts[0].to_string(),
            arrival_time,
            burst_time,
            input_order: processes.len(),
            remaining_time: burst_time,
            completion_time: None,
        });
    }

    Ok((algorithm, processes))
}

fn schedule_fcfs(processes: &mut Vec<Process>) -> Vec<String> {
    let mut indices: Vec<usize> = (0..processes.len()).collect();

    indices.sort_by_key(|&i| {
        (processes[i].arrival_time, processes[i].input_order)
    });

    let mut current_time = 0;
    let mut order = Vec::new();

    for i in indices {
        let process = &mut processes[i];

        if current_time < process.arrival_time {
            current_time = process.arrival_time;
        }

        order.push(process.id.clone());

        current_time += process.burst_time;

        process.completion_time = Some(current_time);
    }

    order
}

fn schedule_sjf(processes: &mut Vec<Process>) -> Vec<String> {
    let mut current_time: u32 = 0;
    let mut completed = 0;
    let mut order = Vec::new();

    while completed < processes.len() {

        let next_process = processes
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                p.completion_time.is_none()
                    && p.arrival_time <= current_time
            })
            .min_by_key(|(_, p)| {
                (p.burst_time, p.arrival_time, p.input_order)
            })
            .map(|(i, _)| i);

        match next_process {
            Some(i) => {
                let process = &mut processes[i];

                order.push(process.id.clone());

                current_time += process.burst_time;

                process.completion_time = Some(current_time);

                completed += 1;
            }

            None => {
                current_time = processes
                    .iter()
                    .filter(|p| p.completion_time.is_none())
                    .map(|p| p.arrival_time)
                    .min()
                    .unwrap();
            }
        }
    }

    order
}

fn schedule_round_robin(
    processes: &mut Vec<Process>,
    quantum: u32,
) -> Vec<String> {
    let mut ready_queue: VecDeque<usize> = VecDeque::new();

    let mut indices: Vec<usize> = (0..processes.len()).collect();
    indices.sort_by_key(|&i| {
        (processes[i].arrival_time, processes[i].input_order)
    });

    let mut next_arrival = 0;
    let mut current_time = 0;
    let mut completed = 0;
    let mut order = Vec::new();

    while completed < processes.len() {
        if ready_queue.is_empty() {
            current_time = current_time.max(
                processes[indices[next_arrival]].arrival_time
            );
        }

        while next_arrival < indices.len()
            && processes[indices[next_arrival]].arrival_time <= current_time
        {
            ready_queue.push_back(indices[next_arrival]);
            next_arrival += 1;
        }

        let i = ready_queue.pop_front().unwrap();
        let run_time = processes[i].remaining_time.min(quantum);
        order.push(processes[i].id.clone());

        current_time += run_time;
        processes[i].remaining_time -= run_time;

        while next_arrival < indices.len()
            && processes[indices[next_arrival]].arrival_time <= current_time
        {
            ready_queue.push_back(indices[next_arrival]);
            next_arrival += 1;
        }

        if processes[i].remaining_time == 0 {
            processes[i].completion_time = Some(current_time);
            completed += 1;
        } else {
            ready_queue.push_back(i);
        }
    }

    order
}

fn print_results(processes: &[Process], order: &[String]) {
    println!("order: {}", order.join(" "));

    let mut total_wait: u64 = 0;
    let mut total_turnaround: u64 = 0;

    for process in processes {
        let completion = process.completion_time.unwrap();

        let turnaround = completion - process.arrival_time;
        let wait = turnaround - process.burst_time;

        total_wait += wait as u64;
        total_turnaround += turnaround as u64;

        println!(
            "{} wait={} turnaround={}",
            process.id, wait, turnaround
        );
    }

    let count = processes.len() as f64;

    let avg_wait = if count == 0.0 {
        0.0
    } else {
        total_wait as f64 / count
    };

    let avg_turnaround = if count == 0.0 {
        0.0
    } else {
        total_turnaround as f64 / count
    };

    println!(
        "avg_wait={:.2} avg_turnaround={:.2}",
        avg_wait, avg_turnaround
    );
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    match parse_input(&input) {
        Ok((algorithm, mut processes)) => {
            match algorithm {
                Algorithm::Fcfs => {
                    let order = schedule_fcfs(&mut processes);
                    print_results(&processes, &order);
                }

                Algorithm::Sjf => {
                    let order = schedule_sjf(&mut processes);
                    print_results(&processes, &order);
                }

                Algorithm::RoundRobin(quantum) => {
                    let order = schedule_round_robin(&mut processes, quantum);
                    print_results(&processes, &order);
                }
            }
        }

        Err(error) => {
            eprintln!("{}", error);
        }
    }
}

