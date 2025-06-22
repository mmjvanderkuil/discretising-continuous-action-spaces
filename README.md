



#### Mountain Car Continuous with actions \[-1.0 -0.5 0.5 1.0]
`cargo run --release -- --env mcc --depth 2 --num-nodes 3 --num-iters 1000 ----predicate-increment 0.1 0.014 --initial-state-values -0.5 0.0 --predicate-reasoning 1 --actions -1.0 -0.5 0.5 1.0`


#### Pendulum Continuous with actions \[-2 0 2]
`cargo run --release -- --env penc --depth 2 --num-nodes 3 --num-iters 1000 --predicate-increment 0.1 0.1 --initial-state-values 0.5 0.0 --predicate-reasoning 1 --actions -2.0  0  2.0`

#### Cart Pole Continuous with actions \[-1 0 1]
`cargo run --release -- --env cpc --depth 2 --num-nodes 3 --num-iters 10000 -- --predicate-increment 0.1 0.1 0.05 0.1 --initial-state-values -0.05 0.05 0.05 0.05 --predicate-reasoning 1 --actions -1 0 1`


## Run Experiments:
- First run:
  `cargo build --release`
- Following commands:
  **Example:**` ./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --predicate-increment 0.1 0.014 --initial-state-values -0.5 0.0 --predicate-reasoning 1 --actions -1.0 -0.5 0.5 1.0`
- Most experiments were saved in shell files. Those can be run with:
  `./experiment_name.sh`
- You can find most of these files in the broccoli folder
