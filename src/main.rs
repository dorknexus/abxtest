use std::process::{Command, Stdio};
use std::collections::HashMap;
use clap::{App, load_yaml};
use std::io;

type FactorialMap = HashMap<i32, f64>;

fn main() {
	let yaml = load_yaml!("cli.yaml");	
	let matches = App::from(yaml).get_matches();

	let mut cmd_a = cmd_from_arg(matches.value_of("COMMAND_A").unwrap());
	let cmd_a_label = matches.value_of("LABEL_A").unwrap();

	let mut cmd_b = cmd_from_arg(matches.value_of("COMMAND_B").unwrap());
	let cmd_b_label = matches.value_of("LABEL_B").unwrap();

	let num_trials = matches.value_of("NUM_TRIALS").unwrap().parse::<i32>().unwrap();
	let mut choose: i32 = 0;
	let mut factorials = FactorialMap::new();

	for trial in 1..=num_trials {
		let expected = exec_rand( &mut cmd_a, &mut cmd_b);
		println!("Which sample was it? ({}/{}): ", cmd_a_label, cmd_b_label);

		let mut input = String::new();	
		io::stdin()
			.read_line(&mut input)
			.expect("Bad input!");
		
		let answer = SampleKind::from(input);
		if answer == expected {
			choose += 1;
		}

		println!("Probability of guessing: {:.2}%", (1.0 - prob_binomial(trial, choose, &mut factorials)) * 100.0);
	}
	
	println!("Trials: {}; Correct: {}", num_trials, choose);
}

fn factorial(n: i32, factorials: &mut FactorialMap) -> f64 {
	match factorials.get(&n) {
		Some(f) => *f,
		None => {
			let f = (1..=n as u64).product::<u64>() as f64;
			factorials.insert(n, f);
			f
		}
	}
}

fn prob_binomial(n: i32, k: i32, f: &mut FactorialMap) -> f64 {
	let mut cum_prob = 0f64;
	for i in 0..=k {
		let _c = factorial(n, f) / (factorial(i, f) * factorial(n-i, f)); 
		cum_prob += _c * 0.5f64.powi(i) * 0.5f64.powi(n - i);
	}
	cum_prob
}

fn cmd_from_arg(args: &str) -> Command {
	let mut cmd = Command::new("/bin/sh");
	cmd.arg("-c").arg(args);
	cmd.stdin(Stdio::null());
	//cmd.stdout(Stdio::null());
	cmd.stderr(Stdio::null());
	cmd
}

#[derive(PartialEq)]
enum SampleKind {
	A,
	B
}

impl From<String> for SampleKind {
	fn from(sample: String) -> Self {
		match sample.to_uppercase().trim() {
			"A" => SampleKind::A,
			"B" => SampleKind::B,
			_ => panic!("Bad Input!")
		}
	}
}

fn exec_rand(cmd_a: &mut Command, cmd_b: &mut Command) -> SampleKind {
	if rand::random() {
		let mut child = cmd_a.spawn().expect("Error running command!");
		child.wait().expect("Command 'A' halted.");
		SampleKind::A
	} else {
		let mut child = cmd_b.spawn().expect("Error running command!");
		child.wait().expect("Command 'B' halted.");
		SampleKind::B
	}
}
