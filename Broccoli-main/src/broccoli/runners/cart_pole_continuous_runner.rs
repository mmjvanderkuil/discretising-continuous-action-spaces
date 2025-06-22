// use std::fs::create_dir;
use std::io::Write;
use std::{fs::File, path::Path};
// use crate::broccoli::trees::decision_tree::DecisionTree;
use crate::broccoli::{
    broccoli::{Broccoli, BroccoliOutput},
    broccoli_helper_functions::{broccoli_plot, check_predicate_increments},
    environments::{
        environment::{produce_traces, Environment},
        environment_cartpole_continuous::EnvironmentCartPoleCont,
    },
    evaluators::environment_evaluator_maximise_iterations::EnvironmentEvaluatorMaximiseIterations,
};

pub fn run_cart_pole_cont(
    depth: u32,
    num_nodes: u32,
    num_simulation_iterations: u32,
    initial_states: &[Vec<f64>],
    predicate_increments: &[f64],
    use_predicate_reasoning: bool,
    actions: Vec<f64>,
) {
    println!("Starting: Cartpole Continuous");

    // let initial_states: Vec<Vec<f64>> = extract_initial_states(initial_states_flattened, 4);
    check_predicate_increments(predicate_increments, 4);

    //construct supporting structs
    let evaluator = EnvironmentEvaluatorMaximiseIterations::new(
        EnvironmentCartPoleCont::new(actions.clone()),
        initial_states,
        num_simulation_iterations,
    );
    let copy1: Vec<f64> = actions.clone();
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
    plot_cart_pole(b_output, initial_states, num_simulation_iterations, actions);
}

fn plot_cart_pole(
    b_output: BroccoliOutput,
    initial_states: &[Vec<f64>],
    num_simulation_iterations: u32,
    actions: Vec<f64>
) {
    assert!(b_output.score.is_some());
    // let mut dec_tree: Option<DecisionTree> = None;
    if let Some(dt) = &b_output.decision_tree {
        // print_result(&b_output, actions.clone(), dt.clone());
        let dec_tree = Some(dt.clone());
    
    
    let decision_tree = dec_tree.unwrap();
    let mut decision_tree_2 = decision_tree.clone();
    
    
    let score = b_output.score.unwrap();
    let traces = produce_traces(
        &mut EnvironmentCartPoleCont::new(actions.clone()),
        initial_states,
        decision_tree,
        num_simulation_iterations,
    );

    for trace in traces.iter().enumerate() {
        let path = Path::new("actions").join(format!("cp_{}.txt", trace.1.len()));
        let mut output = File::create(path).unwrap();
        //print initial state
        for f in &trace.1[0] {
            let _ = write!(output, "{}", &format!("{} ", f));
        }

        let mut env = EnvironmentCartPoleCont::new(actions.clone());
        env.reset(&trace.1[0]);

        //print actions
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
        }
    }

    for trace in traces.iter().enumerate() {
        for f in 0..trace.1[0].len() {
            let x_values: Vec<f64> = (1..=trace.1.len()).map(|p| p as f64).collect();
            let x_name = "Time";
            let y_values: Vec<f64> = trace.1.iter().map(|v| v[f]).collect();
            let y_name = EnvironmentCartPoleCont::new(actions.clone())
                .environment_info()
                .feature_name(f);

            broccoli_plot(
                &x_values,
                x_name,
                &y_values,
                &y_name,
                &format!("cp_{}_{}_{}", score, trace.0, y_name),
            );
        }
    }
    }
}

// fn print_result(best:&BroccoliOutput,  actions: Vec<f64>, dt: DecisionTree) {
//     let _tmp: Vec<String> = actions.iter().map(|n| format!("{:.2}",n)).collect();
//     let _ = create_dir("ExperimentResults/NoPredicateReason/cpc");
//     let title: String = format!("ExperimentResults/exp1b/cpc_regr.txt");

//     let mut contents: String = String::new();
//     let mut file = File::create(title).expect("Problem writing to file?");

    
//     contents += "Uniform Action Discretization\n";
//     contents += &format!("Actions: [{}]\n",_tmp.join(", "));
//     contents += &format!("Best Score: {:.2}\n", best.score.unwrap());
//     contents += &format!("Number of trees considered: {}\n", best.num_trees_considered);
//     contents += &format!("Number of environment calls: {}\n", best.num_environment_calls);
//     contents += &format!("Runtime: {:.2}h {:.2}m {:.2}s \n", best.runtime.as_secs()/3600, (best.runtime.as_secs()%3600)/60, best.runtime.as_secs()%60);
//     contents += &format!("\nOptimal tree:\n{}\n",dt);


//     file.write( contents.as_bytes())
//         .expect("Writing as bytes failed?");
// }
