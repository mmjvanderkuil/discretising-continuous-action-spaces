# Revised experiments




# Mountain Car

Initial states:
+ Velocity zero
+ Position: [-0.6, -0.4]
+ Increments: [0.1, 0.014], [0.1, 0.01], [0.1, 0.014]

Take points -0.6 + i * 0.1, with i \in [0, 10]

Multiple points -> use window of size 5?

cargo run --release -- --env mc --depth 2 --num-iters 1000 --predicate-increment 0.1 0.014 --initial-state-values -0.5 0.0 --predicate-reasoning 1

100 starting points, ten times

use crate::basic_types::Random;


X = [-0.6, 1.2] 0.1
Vel = [-0.07, 0.07] 0.014

10, 20, 30, 40, 50

# Cart-Pole

Initial states:
+ all four states within [-0.05, 0.05]

Take points at the extreme, i.e., either -0.05 or 0.05. This gives 16 points.

stops if cart is outside [-2.4, 2.4], or the angle is outside [-0.2, 0.2]

Multiple points -> cart position 0.0, cart velocity 0.05, pole velocity 0.0, and the pole angle then takes values -0.05 + i * 0.01, i \in [0, 10], then window of size 5

cargo run --release -- --env cp --depth 2 --num-iters 100000 --predicate-increment 0.1 0.1 0.05 0.1 --initial-state-values -0.05 0.05 0.05 0.05 --predicate-reasoning 1

# Pendulum

Initial states:
+ Angle: [-1.0, 1.0]
+ Angular velocity: [-1.0, 1.0]

End state: angle and velocity within [-0.1, 0.1]

Takes points: angle velocity is 0.0, angle is [1.0, 0.9, 0.8, 0.7, 0.6, 0.5, -0.5, -0.6, -0.7, -0.8, -0.9, -1.0]

multiple points: sliding window of size 5

cargo run --release -- --env pen --depth 2 --num-iters 100000 --predicate-increment 0.1 0.1 --initial-state-values 0.5 0.0 --predicate-reasoning 1


State 0: [-0.5, 0.0]
State 1: [-0.517, 0.0]
State 2: [-0.57, 0.0] *
State 3: [-0.464, 0.0]
State 4: [-0.407, 0.0]
State 5: [-0.536, 0.0]
State 6: [-0.445, 0.0]
State 7: [-0.553, 0.0] *
State 8: [-0.492, 0.0]
State 9: [-0.466, 0.0]