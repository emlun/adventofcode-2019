use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::VecDeque;

const NUM_COMPUTERS: usize = 50;

#[derive(Debug)]
struct Packet {
    x: i64,
    y: i64,
    addr: usize,
}

fn solve_b(template: &IntcodeComputer) -> (i64, i64) {
    let mut computers: Vec<IntcodeComputer> =
        (0..NUM_COMPUTERS).map(|_| template.clone()).collect();
    let mut packet_queues: Vec<VecDeque<Packet>> =
        (0..NUM_COMPUTERS).map(|_| VecDeque::new()).collect();
    let mut input_buffers: Vec<VecDeque<i64>> =
        (0..NUM_COMPUTERS).map(|_| VecDeque::new()).collect();
    let mut output_buffers: Vec<VecDeque<i64>> =
        (0..NUM_COMPUTERS).map(|_| VecDeque::new()).collect();

    let mut nat_buffer: Option<Packet> = None;
    let mut last_nat_y: Option<i64> = None;
    let mut computers_stalled: Vec<usize> = vec![0; NUM_COMPUTERS];

    let mut a_solution: Option<i64> = None;

    for (compi, computer) in computers.iter_mut().enumerate() {
        computer.step(&mut Some(compi as i64));
    }

    loop {
        let network_idle = computers.iter().enumerate().all(|(compi, _)| {
            computers_stalled[compi] > 1
                && packet_queues[compi].is_empty()
                && input_buffers[compi].is_empty()
        });
        if network_idle {
            if let Some(packet) = nat_buffer.as_ref() {
                if Some(packet.y) == last_nat_y {
                    return (a_solution.unwrap(), packet.y);
                } else {
                    last_nat_y = Some(packet.y);
                    input_buffers[0].push_back(packet.x);
                    input_buffers[0].push_back(packet.y);
                }
            } else {
                panic!("Network stalled");
            }
        }

        for (compi, computer) in computers.iter_mut().enumerate() {
            let input = if computer.expects_input() {
                if let Some(i) = input_buffers[compi].pop_front() {
                    computers_stalled[compi] = 0;
                    Some(i)
                } else {
                    if let Some(packet) = packet_queues[compi].pop_front() {
                        computers_stalled[compi] = 0;
                        input_buffers[compi].push_back(packet.y);
                        Some(packet.x)
                    } else {
                        computers_stalled[compi] += 1;
                        None
                    }
                }
            } else {
                None
            };

            if let Some(output) = computer.step(&mut input.or(Some(-1))) {
                computers_stalled[compi] = 0;
                output_buffers[compi].push_back(output);
                if output_buffers[compi].len() >= 3 {
                    let addr = output_buffers[compi].pop_front().unwrap() as usize;
                    let x = output_buffers[compi].pop_front().unwrap();
                    let y = output_buffers[compi].pop_front().unwrap();
                    let packet = Packet { x, y, addr };

                    if addr == 255 {
                        if a_solution.is_none() {
                            a_solution = Some(packet.y);
                        }
                        nat_buffer = Some(packet);
                    } else {
                        packet_queues[addr].push_back(packet);
                    }
                }
            }
        }
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    let (a_solution, b_solution) = solve_b(&computer);
    (a_solution.to_string(), b_solution.to_string())
}
