echo 'exp2a penc final' 
./target/release/broccoli --env penc --depth 2 --num-nodes 3 --num-iters 100 --action-policy none --predicate-increment 0.1 0.1 --initial-state-values 0.5 0 --predicate-reasoning 1 --actions -0.970 -0.930 -0.770 -0.550 -0.460 -0.270 0.710 0.780 0.970   > small_trees_penc_final.txt
