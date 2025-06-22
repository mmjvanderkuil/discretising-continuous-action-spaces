echo 'exp2a penc final' 
./target/release/broccoli --env penc --depth 2 --num-nodes 3 --num-iters 100 --action-policy none --predicate-increment 0.1 0.1 --initial-state-values 0.5 0 --predicate-reasoning 1 --actions -1.290 -1.150 -0.970 -0.780 -0.030 0.100 1.300 1.740 1.910   > small_trees_penc_final.txt
