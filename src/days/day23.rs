use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::VecDeque;

const NUM_COMPUTERS: usize = 50;

struct Packet {
    x: i64,
    y: i64,
    addr: usize,
}

fn solve_a(template: &IntcodeComputer) -> i64 {
    let mut computers: Vec<IntcodeComputer> =
        (0..NUM_COMPUTERS).map(|_| template.clone()).collect();
    let mut packet_queues: Vec<VecDeque<Packet>> =
        (0..NUM_COMPUTERS).map(|_| VecDeque::new()).collect();
    let mut input_buffers: Vec<VecDeque<i64>> =
        (0..NUM_COMPUTERS).map(|_| VecDeque::new()).collect();
    let mut output_buffers: Vec<VecDeque<i64>> =
        (0..NUM_COMPUTERS).map(|_| VecDeque::new()).collect();

    for (compi, computer) in computers.iter_mut().enumerate() {
        computer.step(&mut Some(compi as i64));
    }

    loop {
        for (compi, computer) in computers.iter_mut().enumerate() {
            let input = if computer.expects_input() {
                if let Some(i) = input_buffers[compi].pop_front() {
                    Some(i)
                } else {
                    if let Some(packet) = packet_queues[compi].pop_front() {
                        input_buffers[compi].push_back(packet.y);
                        Some(packet.x)
                    } else {
                        None
                    }
                }
            } else {
                None
            };

            if let Some(output) = computer.step(&mut input.or(Some(-1))) {
                output_buffers[compi].push_back(output);
                if output_buffers[compi].len() >= 3 {
                    let addr = output_buffers[compi].pop_front().unwrap() as usize;
                    let x = output_buffers[compi].pop_front().unwrap();
                    let y = output_buffers[compi].pop_front().unwrap();

                    if addr == 255 {
                        return y;
                    } else {
                        packet_queues[addr].push_back(Packet { x, y, addr });
                    }
                }
            }
        }
    }
}

fn solve_b(_template: &IntcodeComputer) -> i64 {
    0
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    (
        solve_a(&computer).to_string(),
        solve_b(&computer).to_string(),
    )
}
