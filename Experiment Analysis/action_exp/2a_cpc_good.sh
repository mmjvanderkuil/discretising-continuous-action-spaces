echo 'exp2b cp 0' 
./target/release/broccoli --env cpc --depth 2 --num-nodes 3 --num-iters 10000 --action-policy none --predicate-increment 0.1 0.1 0.05 0.1 --initial-state-values -0.05 0.05 0.05 0.05 --predicate-reasoning 1 --actions -0.272837 -0.183522 -0.002151 -0.000005 0.000000 0.000003 0.000003 0.441593  > TD3_acts_cpc_s=0.txt
echo 'exp2b cp 1' 
./target/release/broccoli --env cpc --depth 2 --num-nodes 3 --num-iters 10000 --action-policy none --predicate-increment 0.1 0.1 0.05 0.1 --initial-state-values -0.05 0.05 0.05 0.05 --predicate-reasoning 1 --actions -0.348442 -0.059043 -0.000005 -0.000005 -0.000004 -0.000002 -0.000000 0.451206  > TD3_acts_cpc_s=1.txt
echo 'exp2b cp 2' 
./target/release/broccoli --env cpc --depth 2 --num-nodes 3 --num-iters 10000 --action-policy none --predicate-increment 0.1 0.1 0.05 0.1 --initial-state-values -0.05 0.05 0.05 0.05 --predicate-reasoning 1 --actions -0.381356 -0.000005 -0.000003 0.000001 0.000003 0.000005 0.000006 0.329289  > TD3_acts_cpc_s=2.txt
echo 'exp2b cp 3' 
./target/release/broccoli --env cpc --depth 2 --num-nodes 3 --num-iters 10000 --action-policy none --predicate-increment 0.1 0.1 0.05 0.1 --initial-state-values -0.05 0.05 0.05 0.05 --predicate-reasoning 1 --actions -0.252881 -0.000007 -0.000005 -0.000004 0.000001 0.000003 0.000006 0.438870  > TD3_acts_cpc_s=3.txt
echo 'exp2b cp 4' 
./target/release/broccoli --env cpc --depth 2 --num-nodes 3 --num-iters 10000 --action-policy none --predicate-increment 0.1 0.1 0.05 0.1 --initial-state-values -0.05 0.05 0.05 0.05 --predicate-reasoning 1 --actions -0.215077 -0.000005 -0.000003 0.000003 0.000004 0.000005 0.000007 0.294656  > TD3_acts_cpc_s=4.txt
