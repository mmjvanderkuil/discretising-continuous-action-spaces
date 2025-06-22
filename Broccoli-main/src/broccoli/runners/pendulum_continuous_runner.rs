// use std::fs::create_dir;
use std::io::Write;
use std::{fs::File, path::Path};

// use crate::broccoli::trees::decision_tree::DecisionTree;
use crate::broccoli::{
    broccoli::{Broccoli, BroccoliOutput},
    broccoli_helper_functions::{broccoli_plot, check_predicate_increments},
    environments::{
        environment::{produce_successful_traces, Environment},
        environment_pendulum_continuous::EnvironmentPendulumCont,
    },
    evaluators::environment_evaluator_penc::EnvironmentEvaluatorPenC,
};

pub fn run_pendulum_cont(
    depth: u32,
    num_nodes: u32,
    num_simulation_iterations: u32,
    initial_states: &[Vec<f64>],
    predicate_increments: &[f64],
    use_predicate_reasoning: bool,
    actions: Vec<f64>,
) {
    println!("Starting: Pendulum");

    //let initial_states: Vec<Vec<f64>> = extract_initial_states(initial_states_flattened, 2);
    check_predicate_increments(predicate_increments, 2);
    let clone = actions.clone();
    //construct supporting structs
    let evaluator = EnvironmentEvaluatorPenC::new(
        EnvironmentPendulumCont::new(actions),
        initial_states,
        num_simulation_iterations,
    );
    let copy1: Vec<f64> = clone.clone();
    let string_list: Vec<String> = copy1.iter().map(|n| format!("{:.2}", n)).collect();
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
    
    plot_pendulum(b_output, initial_states,clone);
}

fn plot_pendulum(b_output: BroccoliOutput, initial_states: &[Vec<f64>],actions: Vec<f64>) {
    if let Some(score) = b_output.score {
        if let Some(decision_tree) = &b_output.decision_tree {
            let copy1 = actions.clone();
            // print_result(&b_output, decision_tree.clone(), actions.clone());
            let mut decision_tree_2 = decision_tree.clone();
            let traces = produce_successful_traces(
                &mut EnvironmentPendulumCont::new(actions),
                initial_states,
                decision_tree.clone(),
                b_output.num_environment_calls as u32,
            ).unwrap();
            
        

            for trace in traces.iter().enumerate() {
                let path = Path::new("actions").join(format!("pen_{}.txt", trace.1.len()));
                let mut output = File::create(path).unwrap();
                let copy_actions = copy1.clone();

                //print initial state
                for f in &trace.1[0] {
                    let _ = write!(output, "{}", &format!("{} ", f));
                }

                let mut env = EnvironmentPendulumCont::new(copy_actions);
                env.reset(&trace.1[0]);
                //println!("{}", env.observe_state()[0]);
                //println!("{}", env.observe_state()[1]);

                //print actions
                //let mut i = 0;
                
                for state in trace.1 {
                    let ob_state = env.observe_state();
                    for i in 0..ob_state.len() {
                        if ob_state[i] != state[i] {
                            println!("{}\n{}\n{}", i, ob_state[i], state[i]);
                        }
                        assert!(ob_state[i] == state[i]);
                    }
                    
                    let action = decision_tree_2.get_action(state);

                    let _ = write!(output, "\n{}", &format!("{}", action));

                    env.apply_action(action);

                    //println!("{}, {}", i, action);
                    //println!("{} {}", env.observe_state()[0], env.observe_state()[1]);
                    //println!("{}", env.observe_state()[1]);

                    /*if action == 1 {
                        println!("I AM HERE {}", i);
                    }*/

                    //i += 1;
                }
            }

            for trace in traces.iter().enumerate() {
                for f in 0..trace.1[0].len() {
                    let copy = copy1.clone();
                    let x_values: Vec<f64> = (1..=trace.1.len()).map(|p| p as f64).collect();
                    let x_name = "Time";
                    let y_values: Vec<f64> = trace.1.iter().map(|v| v[f]).collect();
                    let y_name = EnvironmentPendulumCont::new(copy)
                        .environment_info()
                        .feature_name(f);

                    broccoli_plot(
                        &x_values,
                        x_name,
                        &y_values,
                        &y_name,
                        &format!("pen_{}_{}_{}", score, trace.0, y_name),
                    );
                }
            }
        }
    } else {
        println!("No tree found.");
    }
}
// fn print_result(best:&BroccoliOutput, dt: DecisionTree,  actions: Vec<f64>) {
//     let _tmp: Vec<String> = actions.iter().map(|n| format!("{:.2}",n)).collect();
//     let _ = create_dir("ExperimentResults/NoPredicateReason/penc");
//     let title: String = format!("ExperimentResults/try_out_small_pen.txt");


//     let mut contents: String = String::new();
//     let mut file = File::create(title).expect("Problem writing to file?");
    
//     contents += "Uniform Action Discretization\n";
//     contents += &format!("Actions: [{}]\n",_tmp.join(", "));
//     contents += &format!("Best Score: {}\n",best.score.unwrap());
//     contents += &format!("Number of trees considered: {}\n",best.num_trees_considered);
//     contents += &format!("Number of environment calls: {}\n",best.num_environment_calls);
//     contents += &format!("Runtime: {:.2}h {:.2}m {:.2}s \n", best.runtime.as_secs()/3600, (best.runtime.as_secs()%3600)/60, best.runtime.as_secs()%60);
//     contents += &format!("\nOptimal tree:\n{}\n",dt);

//     file.write( contents.as_bytes())
//         .expect("Writing as bytes failed?");
// }

