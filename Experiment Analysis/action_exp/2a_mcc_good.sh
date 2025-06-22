echo 'exp2b mcc 0' 
./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.014 --initial-state-values -0.5 0.0 --predicate-reasoning 1 --actions -1.000000 -1.000000 -0.390993 0.999996 1.000000 1.000000 1.000000 1.000000  > TD3_acts_mcc_s=0.txt
echo 'exp2b mcc 1' 
./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.014 --initial-state-values -0.5 0.0 --predicate-reasoning 1 --actions -1.000000 -1.000000 -0.995421 1.000000 1.000000 1.000000 1.000000 1.000000  > TD3_acts_mcc_s=1.txt
echo 'exp2b mcc 2' 
./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.014 --initial-state-values -0.5 0.0 --predicate-reasoning 1 --actions -1.000000 -0.989419 -0.986496 0.909454 1.000000 1.000000 1.000000 1.000000  > TD3_acts_mcc_s=2.txt
echo 'exp2b mcc 3' 
./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.014 --initial-state-values -0.5 0.0 --predicate-reasoning 1 --actions -1.000000 -1.000000 -0.900458 0.999097 1.000000 1.000000 1.000000 1.000000  > TD3_acts_mcc_s=3.txt
echo 'exp2b mcc 4' 
./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.014 --initial-state-values -0.5 0.0 --predicate-reasoning 1 --actions -1.000000 -1.000000 -0.968604 -0.873097 0.940421 0.999918 1.000000 1.000000  > TD3_acts_mcc_s=4.txt
