echo 'exp2b mcc 0' 
./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.014 --initial-state-values -0.5 0.0 --predicate-reasoning 1 --actions -1.000 -1.000 1.000 1.000 1.000 1.000 1.000 1.000  > TD3_acts_mcc_s=0.txt
echo 'exp2b mcc 1' 
./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.014 --initial-state-values -0.5 0.0 --predicate-reasoning 1 --actions -1.000 -1.000 1.000 1.000 1.000 1.000 1.000 1.000  > TD3_acts_mcc_s=1.txt
echo 'exp2b mcc 2' 
./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.014 --initial-state-values -0.5 0.0 --predicate-reasoning 1 --actions -1.000 -1.000 -1.000 1.000 1.000 1.000 1.000 1.000  > TD3_acts_mcc_s=2.txt
echo 'exp2b mcc 3' 
./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.014 --initial-state-values -0.5 0.0 --predicate-reasoning 1 --actions -1.000 -1.000 1.000 1.000 1.000 1.000 1.000 1.000  > TD3_acts_mcc_s=3.txt
echo 'exp2b mcc 4' 
./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.014 --initial-state-values -0.5 0.0 --predicate-reasoning 1 --actions -1.000 -1.000 -1.000 0.934 1.000 1.000 1.000 1.000  > TD3_acts_mcc_s=4.txt
