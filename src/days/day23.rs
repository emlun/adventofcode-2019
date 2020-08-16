use crate::common::Solution;
use crate::intcode::IntcodeComputer;

const NUM_COMPUTERS: usize = 50;

#[derive(Debug)]
struct Packet {
    x: i64,
    y: i64,
}

fn solve_b(template: &IntcodeComputer) -> (i64, i64) {
    let mut computers: Vec<IntcodeComputer> = (0..NUM_COMPUTERS)
        .map(|i| {
            let mut computer = template.clone();
            computer.input.push_back(i as i64);
            computer
        })
        .collect();

    let mut nat_buffer: Option<Packet> = None;
    let mut last_nat_y: Option<i64> = None;
    let mut computers_stalled: Vec<usize> = vec![0; NUM_COMPUTERS];

    let mut a_solution: Option<i64> = None;

    loop {
        let network_idle = computers
            .iter()
            .enumerate()
            .all(|(compi, _)| computers_stalled[compi] > 1 && computers[compi].input.is_empty());
        if network_idle {
            if let Some(packet) = nat_buffer.as_ref() {
                if Some(packet.y) == last_nat_y {
                    return (a_solution.unwrap(), packet.y);
                } else {
                    last_nat_y = Some(packet.y);
                    computers[0].input.push_back(packet.x);
                    computers[0].input.push_back(packet.y);
                }
            } else {
                panic!("Network stalled");
            }
        }

        for compi in 0..computers.len() {
            let computer = &mut computers[compi];

            if computer.expects_input() {
                if computer.input.is_empty() {
                    computers_stalled[compi] += 1;
                    computer.input.push_back(-1);
                } else {
                    computers_stalled[compi] = 0;
                }
            };

            computer.step();

            if computer.output.len() >= 3 {
                computers_stalled[compi] = 0;
                let addr = computer.output.pop_front().unwrap() as usize;
                let x = computer.output.pop_front().unwrap();
                let y = computer.output.pop_front().unwrap();
                let packet = Packet { x, y };

                if addr == 255 {
                    if a_solution.is_none() {
                        a_solution = Some(packet.y);
                    }
                    nat_buffer = Some(packet);
                } else {
                    computers[addr].input.push_back(packet.x);
                    computers[addr].input.push_back(packet.y);
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
