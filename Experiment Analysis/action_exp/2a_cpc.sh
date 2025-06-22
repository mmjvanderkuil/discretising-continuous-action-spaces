echo 'exp2a cpc0'
 ./target/release/broccoli --env cpc --depth 2 --num-nodes 3 --num-iters 10000 --action-policy none --predicate-increment 0.1 0.1 0.05 0.1 --initial-state-values -0.05 0.05 0.05 0.05 --predicate-reasoning 1 --actions -0.990 -0.470 -0.170 0.430 0.710 0.850  > small_tree_cpc_s=0.txt
echo 'exp2a cpc1'
 ./target/release/broccoli --env cpc --depth 2 --num-nodes 3 --num-iters 10000 --action-policy none --predicate-increment 0.1 0.1 0.05 0.1 --initial-state-values -0.05 0.05 0.05 0.05 --predicate-reasoning 1 --actions -0.710 -0.260 -0.130 0.030 0.340 0.440  > small_tree_cpc_s=1.txt
echo 'exp2a cpc2'
 ./target/release/broccoli --env cpc --depth 2 --num-nodes 3 --num-iters 10000 --action-policy none --predicate-increment 0.1 0.1 0.05 0.1 --initial-state-values -0.05 0.05 0.05 0.05 --predicate-reasoning 1 --actions -0.750 -0.470 -0.400 0.060 0.350 0.570  > small_tree_cpc_s=2.txt
echo 'exp2a cpc3'
 ./target/release/broccoli --env cpc --depth 2 --num-nodes 3 --num-iters 10000 --action-policy none --predicate-increment 0.1 0.1 0.05 0.1 --initial-state-values -0.05 0.05 0.05 0.05 --predicate-reasoning 1 --actions -0.910 -0.280 -0.190 0.220 0.720 0.990  > small_tree_cpc_s=3.txt
echo 'exp2a cpc4'
 ./target/release/broccoli --env cpc --depth 2 --num-nodes 3 --num-iters 10000 --action-policy none --predicate-increment 0.1 0.1 0.05 0.1 --initial-state-values -0.05 0.05 0.05 0.05 --predicate-reasoning 1 --actions -0.970 -0.270 -0.220 0.000 0.240 0.780  > small_tree_cpc_s=4.txt
echo 'exp2a cpc5'
 ./target/release/broccoli --env cpc --depth 2 --num-nodes 3 --num-iters 10000 --action-policy none --predicate-increment 0.1 0.1 0.05 0.1 --initial-state-values -0.05 0.05 0.05 0.05 --predicate-reasoning 1 --actions -0.610 -0.500 -0.390 0.090 0.650 0.880  > small_tree_cpc_s=5.txt
echo 'exp2a cpc6'
 ./target/release/broccoli --env cpc --depth 2 --num-nodes 3 --num-iters 10000 --action-policy none --predicate-increment 0.1 0.1 0.05 0.1 --initial-state-values -0.05 0.05 0.05 0.05 --predicate-reasoning 1 --actions -0.930 -0.520 -0.460 0.410 0.780 0.970  > small_tree_cpc_s=6.txt
echo 'exp2a cpc7'
 ./target/release/broccoli --env cpc --depth 2 --num-nodes 3 --num-iters 10000 --action-policy none --predicate-increment 0.1 0.1 0.05 0.1 --initial-state-values -0.05 0.05 0.05 0.05 --predicate-reasoning 1 --actions -0.770 -0.630 -0.550 0.080 0.710 0.940  > small_tree_cpc_s=7.txt
