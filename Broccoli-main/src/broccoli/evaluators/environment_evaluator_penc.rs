use crate::broccoli::{
    environments::{environment::{run_penc_simulation_until_terminate_state, Environment, EnvironmentInfo}, environment_pendulum_continuous::EnvironmentPendulumCont},
    trees::decision_tree::DecisionTree,
};

use super::evaluator::Evaluator;

pub struct EnvironmentEvaluatorPenC {
    environment: EnvironmentPendulumCont,
    initial_states: Vec<Vec<f64>>,
    max_num_states: u32,
    best_score: Option<f64>,
    num_environment_calls: usize,
}

impl EnvironmentEvaluatorPenC {
    pub fn new(
        environment: EnvironmentPendulumCont,
        initial_states: &[Vec<f64>],
        max_num_states: u32,
    ) -> EnvironmentEvaluatorPenC {
        EnvironmentEvaluatorPenC {
            environment,
            initial_states: initial_states.to_vec(),
            max_num_states,
            best_score: None,
            num_environment_calls: 0,
        }
    }
}

impl Evaluator for EnvironmentEvaluatorPenC {
    fn environment_info(&self) -> EnvironmentInfo {
        self.environment.environment_info()
    }


    fn evaluate(&mut self, decision_tree: DecisionTree) -> (DecisionTree, Result<f64, ()>) {
        if let Some(optimal_value) = self.best_score {
            if optimal_value < 0.0 {
                return (decision_tree, Err(()));
            }
        }

        let mut controller_global: Option<DecisionTree> = None;
        let mut new_global_score: Option<f64> = None;
        for initial_state in &self.initial_states {
            let mut controller = decision_tree.clone();
            let total_return = run_penc_simulation_until_terminate_state(
                &mut self.environment,
                initial_state,
                &mut controller,
                self.max_num_states,
            );
            self.num_environment_calls += 1;

            match total_return {
                Ok(score) =>{
                    if score <=  self.best_score.unwrap_or(f64::MIN) {
                        // println!("Score <= best score");
                        return (controller, Err(()));
                    } else {
                        // println!("New Best score");
                        match new_global_score {
                            Some(new_global_value) => {
                                if new_global_value > score {
                                    new_global_score = Some(score);
                                }
                            }
                            None => new_global_score = Some(score),
                        }
        
                        match controller_global.as_mut() {
                            Some(_) => {
                                controller_global
                                    .as_mut()
                                    .unwrap()
                                    .merge_threshold_distances(&controller);
                            }
                            None => controller_global = Some(controller),
                        }
                    }
                }
                Err(_) => return (controller, Err(()))
            } 
        }
        (
            controller_global.unwrap(),
            Ok(new_global_score.unwrap() as f64),
        )
    }

    fn register_new_best_score(&mut self, new_best_score: f64) {
        // assert!(broccoli_is_integer(new_best_score));
        // assert!(broccoli_greater_or_equal(new_best_score, 0.0));

        if !self.best_score.is_none(){
            assert!((new_best_score) > self.best_score.unwrap());
        }
        self.best_score = Some(new_best_score);
    }

    fn num_environment_calls(&self) -> usize {
        self.num_environment_calls
    }
}
