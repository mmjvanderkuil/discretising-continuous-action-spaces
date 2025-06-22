use std::io::Write;
use std::{fs::File, path::Path};

use crate::broccoli::trees::decision_tree::DecisionTree;

use super::environment_mountain_car_continuous::EnvironmentMountainCarCont;
use super::environment_pendulum_continuous::EnvironmentPendulumCont;

pub trait Environment {
    fn apply_action(&mut self, action: usize);
    fn observe_state(&self) -> Vec<f64>;
    fn is_at_terminal_state(&self) -> bool;
    fn reset(&mut self, initial_state: &[f64]);
    fn environment_info(&self) -> EnvironmentInfo;
}

#[derive(Clone)]
pub struct EnvironmentInfo {
    //for each state variable, the max and min value it can take
    feature_ranges: Vec<Interval>,
    num_actions: usize,
}

impl EnvironmentInfo {
    pub fn new(ranges: Vec<Interval>, num_actions: usize) -> EnvironmentInfo {
        EnvironmentInfo {
            feature_ranges: ranges,
            num_actions,
        }
    }

    pub fn feature_name(&self, feature_index: usize) -> String {
        self.feature_ranges[feature_index].name.clone()
    }

    pub fn num_actions(&self) -> usize {
        self.num_actions
    }

    pub fn num_features(&self) -> usize {
        self.feature_ranges.len()
    }

    pub fn ranges(&self) -> Vec<Interval> {
        self.feature_ranges.clone()
    }
}

#[derive(Clone)]
pub struct Interval {
    pub name: String,
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn length(&self) -> f64 {
        (self.max - self.min).abs()
    }
}

pub fn run_simulation_until_terminate_state<E: Environment>(
    environment: &mut E,
    initial_state: &[f64],
    controller: &mut DecisionTree,
    max_num_states: u32,
) -> Result<u32, ()> {
    assert!(max_num_states >= 1);

    environment.reset(initial_state);

    if environment.is_at_terminal_state() {
        return Ok(1);
    }

    for i in 0..(max_num_states - 1) {
        let state = environment.observe_state();
        let action = controller.get_action(&state);
        environment.apply_action(action);

        if environment.is_at_terminal_state() {
            return Ok(i + 2);
        }
    }
    Err(())
}

pub fn run_mcc_simulation_until_terminate_state(
    environment: &mut EnvironmentMountainCarCont,
    initial_state: &[f64],
    controller: &mut DecisionTree,
    max_num_states: u32,
) -> Result<f64, ()> {
    assert!(max_num_states >= 1);

    environment.reset(initial_state);

    if environment.is_at_terminal_state() {
        return Ok(100.0);
    }
    let mut total_return: f64 = 0.0;
    for _i in 0..(max_num_states - 1) {
        let state = environment.observe_state();
        let action = controller.get_action(&state);
        environment.apply_action(action);
        total_return += -0.1*(environment.actions[action].powi(2));


        if environment.is_at_terminal_state() {
            total_return += 100.0;
            return Ok(total_return);
        }
    }
    Err(())
}

pub fn run_penc_simulation_until_terminate_state(
    environment: &mut EnvironmentPendulumCont,
    initial_state: &[f64],
    controller: &mut DecisionTree,
    max_num_states: u32,
) -> Result<f64, ()> {
    assert!(max_num_states >= 1);

    environment.reset(initial_state);
    let mut total_return: f64 = 10000.0;
    if environment.is_at_terminal_state() {
        return Ok(total_return);
    }
    
    for _i in 0..(max_num_states - 1) {
        let state = environment.observe_state();
        let action = controller.get_action(&state);
        let old_angle: f64 = environment.angle;
        environment.apply_action(action);
        total_return += -(environment.angle.powi(2) + 
                            0.1*(environment.angle - old_angle).powi(2) +
                            0.01* environment.actions[action]);
        // reward = -(theta^2 + 0.1 * theta_dt^ + 0.001 * torque^2)


        if environment.is_at_terminal_state() {
            // total_return += 0.0;
            return Ok(total_return);
        }
    }
    Err(())
}

pub fn run_simulation<E: Environment>(
    environment: &mut E,
    initial_state: &[f64],
    controller: &mut DecisionTree,
    max_num_states: u32,
) -> u32 {
    assert!(max_num_states >= 1);

    environment.reset(initial_state);

    if environment.is_at_terminal_state() {
        return 1;
    }

    for i in 0..(max_num_states - 1) {
        let state = environment.observe_state();
        let action = controller.get_action(&state);
        environment.apply_action(action);

        if environment.is_at_terminal_state() {
            return i + 2;
        }
    }
    max_num_states
}

pub fn run_successful_simulation_with_trace<E: Environment>(
    environment: &mut E,
    initial_state: &[f64],
    controller: &mut DecisionTree,
    max_num_states: u32,
) -> Result<Vec<Vec<f64>>, ()> {
    assert!(max_num_states >= 1);

    environment.reset(initial_state);

    let mut trace: Vec<Vec<f64>> = vec![initial_state.to_vec()];

    if environment.is_at_terminal_state() {
        return Ok(trace);
    }
    let path = Path::new("actions").join(format!("mcc_{}.txt", trace.len()));
    let mut output = File::create(path).unwrap();

    for _i in 0..(max_num_states - 1) {
        let state = environment.observe_state();
        let action = controller.get_action(&state);
        environment.apply_action(action);
        trace.push(environment.observe_state());
        let _ = write!(output, "\n{}", &format!("{}", action));

        if environment.is_at_terminal_state() {
            return Ok(trace);
        }
    }
    Err(())
}

pub fn run_simulation_with_trace<E: Environment>(
    environment: &mut E,
    initial_state: &[f64],
    controller: &mut DecisionTree,
    max_num_states: u32,
) -> Vec<Vec<f64>> {
    assert!(max_num_states >= 1);

    environment.reset(initial_state);

    let mut trace: Vec<Vec<f64>> = vec![initial_state.to_vec()];

    if environment.is_at_terminal_state() {
        return trace;
    }

    for _i in 0..(max_num_states - 1) {
        let state = environment.observe_state();
        let action = controller.get_action(&state);
        environment.apply_action(action);
        trace.push(environment.observe_state());

        if environment.is_at_terminal_state() {
            return trace;
        }
    }
    trace
}

pub fn produce_successful_traces<E: Environment>(
    environment: &mut E,
    initial_states: &[Vec<f64>],
    mut decision_tree: DecisionTree,
    max_num_states: u32,
) -> Result<Vec<Vec<Vec<f64>>>, ()> {
    assert!(max_num_states >= 1);

    let mut traces: Vec<Vec<Vec<f64>>> = vec![];

    for initial_state in initial_states {
        let result = run_successful_simulation_with_trace(
            environment,
            initial_state,
            &mut decision_tree,
            max_num_states,
        );
        match result {
            Ok(trace) => {
                traces.push(trace);
            }
            Err(_) => return Err(()),
        }
    }
    Ok(traces)
}

pub fn produce_traces<E: Environment>(
    environment: &mut E,
    initial_states: &[Vec<f64>],
    mut decision_tree: DecisionTree,
    max_num_states: u32,
) -> Vec<Vec<Vec<f64>>> {
    assert!(max_num_states >= 1);

    let mut traces: Vec<Vec<Vec<f64>>> = vec![];

    for initial_state in initial_states {
        let trace = run_simulation_with_trace(
            environment,
            initial_state,
            &mut decision_tree,
            max_num_states,
        );

        traces.push(trace);
    }
    traces
}
