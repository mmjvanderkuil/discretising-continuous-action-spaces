echo 'exp2a penc 0' 
./target/release/broccoli --env penc --depth 2 --num-nodes 3 --num-iters 100 --action-policy none --predicate-increment 0.1 0.1 --initial-state-values 0.5 0 --predicate-reasoning 1 --actions -1.999 -1.475 -1.475 -1.475 -1.475 -1.475 -1.475 2.000   > TD3_acts_penc_s=0.txt
echo 'exp2a penc 1' 
./target/release/broccoli --env penc --depth 2 --num-nodes 3 --num-iters 100 --action-policy none --predicate-increment 0.1 0.1 --initial-state-values 0.5 0 --predicate-reasoning 1 --actions -1.475 -1.475 -1.475 -1.475 -1.475 -1.475 -1.475 2.000   > TD3_acts_penc_s=1.txt
echo 'exp2a penc 2' 
./target/release/broccoli --env penc --depth 2 --num-nodes 3 --num-iters 100 --action-policy none --predicate-increment 0.1 0.1 --initial-state-values 0.5 0 --predicate-reasoning 1 --actions -2.000 -1.994 -1.475 -1.475 -1.475 -1.475 -1.296 1.676   > TD3_acts_penc_s=2.txt
echo 'exp2a penc 3' 
./target/release/broccoli --env penc --depth 2 --num-nodes 3 --num-iters 100 --action-policy none --predicate-increment 0.1 0.1 --initial-state-values 0.5 0 --predicate-reasoning 1 --actions -1.567 -1.513 -1.478 -1.475 -1.475 -1.475 -1.475 -0.281   > TD3_acts_penc_s=3.txt
echo 'exp2a penc 4' 
./target/release/broccoli --env penc --depth 2 --num-nodes 3 --num-iters 100 --action-policy none --predicate-increment 0.1 0.1 --initial-state-values 0.5 0 --predicate-reasoning 1 --actions -1.475 -1.475 -1.475 -1.475 -1.475 -1.475 -1.475 2.000   > TD3_acts_penc_s=4.txt
