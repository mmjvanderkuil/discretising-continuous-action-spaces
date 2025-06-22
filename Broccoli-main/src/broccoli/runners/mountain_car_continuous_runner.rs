
// use std::{fs::File, io::Write};

use crate::broccoli::{
    broccoli::{Broccoli, BroccoliOutput}, broccoli_helper_functions::{broccoli_plot, check_predicate_increments}, environments::{
        environment::produce_successful_traces, environment_mountain_car::EnvironmentMountainCar,
        environment_mountain_car_continuous::EnvironmentMountainCarCont,
    }, evaluators::environment_evaluator_mcc::EnvironmentEvaluatorMCC
};

use crate::broccoli::environments::environment::Environment;

pub fn run_mountain_car_cont(
    depth: u32,
    num_nodes: u32,
    num_simulation_iterations: u32,
    initial_states: &[Vec<f64>],
    predicate_increments: &[f64],
    use_predicate_reasoning: bool,
    actions: Vec<f64>,
) {
    println!("Starting: Mountain Car Continuous");

    //let initial_states: Vec<Vec<f64>> = extract_initial_states(initial_states_flattened, 2);
    check_predicate_increments(predicate_increments, 2);
    let copy1: Vec<f64> = actions.clone();
    //construct supporting structs
    let evaluator: EnvironmentEvaluatorMCC = EnvironmentEvaluatorMCC::new(
        EnvironmentMountainCarCont::new(actions),
        initial_states,
        num_simulation_iterations,
    );

    let string_list: Vec<String> = copy1.iter().map(|n| format!("{:.4}", n)).collect();
    println!("with actions : [{}]", string_list.join(", "));

    //run the main tree algorithm
    let b_output = Broccoli::compute_decision_tree(
        depth,
        num_nodes,
        Box::new(evaluator),
        predicate_increments,
        use_predicate_reasoning,
    );

    //process output
    b_output.print_basic_stats();
    plot_mountain_car(b_output, initial_states,copy1);
}

fn plot_mountain_car(b_output: BroccoliOutput, initial_states: &[Vec<f64>],actions: Vec<f64>) {
    if let Some(score) = b_output.score {
    
        // let decision_tree = b_output.decision_tree.unwrap();
        if let Some(dt) = &b_output.decision_tree {
            // print_result(&b_output, actions.clone(),dt.clone());
            let traces = produce_successful_traces(
                &mut EnvironmentMountainCarCont::new(actions),
                initial_states,
                dt.clone(),
                1000 as u32,
            );
    
            
            for trace in traces.unwrap().iter().enumerate() {
                for f in 0..trace.1[0].len() {
                    let x_values: Vec<f64> = (1..=trace.1.len()).map(|p| p as f64).collect();
                    let x_name = "Time";
                    let y_values: Vec<f64> = trace.1.iter().map(|v| v[f]).collect();
                    let y_name = EnvironmentMountainCar::new()
                        .environment_info()
                        .feature_name(f);
    
                    broccoli_plot(
                        &x_values,
                        x_name,
                        &y_values,
                        &y_name,
                        &format!("mc_{}_{}_{}", score, trace.0, y_name),
                    );
                }
            }
        }
        
        
    } else {
        println!("No tree found.");
    }
}

// fn print_result(best:&BroccoliOutput,  actions: Vec<f64>, dt: DecisionTree) {
//     let _tmp: Vec<String> = actions.iter().map(|n| format!("{:.2}",n)).collect();
//     let title: String = format!("ExperimentResults/try_out_small.txt");

//     let mut contents: String = String::new();
//     let mut file = File::create(title).expect("Problem writing to file?");

    
//     contents += "Large number of actions\n";
//     contents += &format!("Actions: [{}]\n",_tmp.join(", "));
//     contents += &format!("Best Score: {:.2}\n", best.score.unwrap());
//     contents += &format!("Number of trees considered: {}\n", best.num_trees_considered);
//     contents += &format!("Number of environment calls: {}\n", best.num_environment_calls);
//     contents += &format!("Runtime: {:.2}h {:.2}m {:.2}s \n", best.runtime.as_secs()/3600, (best.runtime.as_secs()%3600)/60, best.runtime.as_secs()%60);
//     contents += &format!("\nOptimal tree:\n{}\n",dt);


//     file.write( contents.as_bytes())
//         .expect("Writing as bytes failed?");
// }

