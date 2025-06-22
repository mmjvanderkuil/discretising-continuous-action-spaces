echo 'exp2a mcc0'
 ./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.01 --initial-state-values -0.5 0 --predicate-reasoning 1 --actions -0.530 -0.160 0.000 0.030 0.660 0.960  > small_tree_mcc_s=0.txt
echo 'exp2a mcc1'
 ./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.01 --initial-state-values -0.5 0 --predicate-reasoning 1 --actions -0.800 -0.610 -0.190 0.440 0.520 0.810  > small_tree_mcc_s=1.txt
echo 'exp2a mcc2'
 ./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.01 --initial-state-values -0.5 0 --predicate-reasoning 1 --actions -0.900 -0.770 -0.150 0.460 0.720 0.860  > small_tree_mcc_s=2.txt
echo 'exp2a mcc3'
 ./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.01 --initial-state-values -0.5 0 --predicate-reasoning 1 --actions -0.260 -0.160 -0.100 0.060 0.320 0.740  > small_tree_mcc_s=3.txt
echo 'exp2a mcc4'
 ./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.01 --initial-state-values -0.5 0 --predicate-reasoning 1 --actions -0.810 -0.560 -0.050 0.080 0.100 0.420  > small_tree_mcc_s=4.txt
echo 'exp2a mcc5'
 ./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.01 --initial-state-values -0.5 0 --predicate-reasoning 1 --actions -0.790 -0.580 -0.370 0.030 0.270 0.860  > small_tree_mcc_s=5.txt
echo 'exp2a mcc6'
 ./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.01 --initial-state-values -0.5 0 --predicate-reasoning 1 --actions -0.820 -0.520 -0.090 0.660 0.720 0.930  > small_tree_mcc_s=6.txt
echo 'exp2a mcc7'
 ./target/release/broccoli --env mcc --depth 2 --num-nodes 3 --num-iters 1000 --action-policy none --predicate-increment 0.1 0.01 --initial-state-values -0.5 0 --predicate-reasoning 1 --actions -0.730 -0.460 -0.150 0.670 0.690 0.990  > small_tree_mcc_s=7.txt
