import numpy as np
import gym
from gym import spaces
from gym.utils import seeding
from stable_baselines3 import TD3
from stable_baselines3.common import noise





#############
class PendulumEnv(gym.Env):
    """
    Custom Pendulum-v1 environment matching OpenAI Gym's implementation.
    State: [theta, theta_dot]; action: torque ∈ [-2,2].
    """

    metadata = {'render_modes': ['human'], 'render_fps': 30}

    def __init__(self):
        super().__init__()
        # Physical constants
        self.max_torque = 2.0   # max absolute torque
        self.max_speed = 8.0    # max angular velocity
        self.g = 10.0           # gravity acceleration
        self.m = 1.0            # mass of the pendulum
        self.l = 1.0            # length of the pendulum
        self.dt = 0.05          # time step
        # Action space: single torque in [-2, 2]
        self.action_space = spaces.Box(low=-self.max_torque, high=self.max_torque,
                                       shape=(1,), dtype=np.float32)

        # Observation space: [cos(theta), sin(theta), theta_dot]
        high = np.array([1.0, self.max_speed], dtype=np.float32)
        self.observation_space = spaces.Box(low=-high, high=high, dtype=np.float32)

        # Initialize state and random seed
        self.state = None
        self.initial_state = np.array([0.5, 0.0], np.float32)
        self.iters = 0

        self.seed()

    def seed(self, seed=None):
        """Set random seed for reproducibility."""
        self.np_random, seed = seeding.np_random(seed)
        return [seed]

    def reset(self, *, seed: int | None = None, options:dict | None = None):
        """Reset state with random angle in [-pi,pi] and angular velocity in [-1,1] [oai_citation:6‡github.com](https://github.com/openai/gym/wiki/Pendulum-v1#:~:text=Starting%20State)."""
        high = np.array([np.pi, 1.0])
        low = -high
        self.state = self.np_random.uniform(low=low, high=high).astype(np.float32)
        theta, theta_dot = self.state
        # self.state = self.initial_state
        self.iters = 0
        return np.array(self.state, dtype=np.float32)

    def step(self, action):
        """Apply action (torque) and update state."""
        theta, theta_dot = self.state
        # Clip the torque to the allowed range
        u = np.clip(action, -self.max_torque, self.max_torque)[0]

        # Equations of motion (same as OpenAI Gym) [oai_citation:7‡fossies.org](https://fossies.org/diffs/gym/0.25.2_vs_0.26.0/gym/envs/classic_control/pendulum.py-diff.html#:~:text=angle_normalize%28th%29%20,newth%2C%20newthdot%5D%29%20self.renderer.render_step):
        #   theta_ddot = (3*g/(2*l) * sin(theta) + 3/(m*l^2) * u)
        theta_ddot = (3 * self.g / (2 * self.l) * np.sin(theta) +
                      3.0 / (self.m * self.l ** 2) * u)
        new_theta_dot = theta_dot + theta_ddot * self.dt

        # Clip angular velocity to max speed
        new_theta_dot = np.clip(new_theta_dot, -self.max_speed, self.max_speed)
        new_theta = theta + new_theta_dot * self.dt
        self.state = np.array([new_theta, new_theta_dot], dtype=np.float32)


        # Compute normalized angle for cost (wrap to [-pi,pi])
        angle_norm = ((new_theta + np.pi) % (2 * np.pi)) - np.pi
        # Compute cost: theta_norm^2 + 0.1*theta_dot^2 + 0.001*u^2
        cost = (angle_norm**2
                + 0.1 * (new_theta_dot**2)
                + 0.001 * (u**2))
        reward = -cost
        obs = np.array([new_theta, new_theta_dot], dtype=np.float32)

        done = False  # No terminal state by default


        trunc = False
        self.iters += 1
        if abs(new_theta) < 0.1 and abs(new_theta_dot) < 0.1:
            done = True
            trunc = True

        if self.iters > 999:
            trunc = True

        return obs, reward, done, trunc, {}
#############

if __name__ =="__main__":
    # Create environment and TD3 model
    # env = ContinuousCartPole.ContinuousCartPoleEnv()
    action_noise = noise.NormalActionNoise(mean=np.array([0]), sigma=0.1)


    # Hyperparameters from: https://github.com/DLR-RM/rl-baselines3-zoo/blob/master/hyperparams/td3.yml
    env = PendulumEnv()
    model = TD3("MlpPolicy", env,
                verbose=2,
                gamma=0.98,
                seed=1,
                buffer_size=200_000,
                action_noise=action_noise,
                gradient_steps=1,
                train_freq=1,
                learning_rate=1e-3,
                learning_starts=10_000,
                policy_kwargs={"net_arch": [400,300]}
                )
    # Train the agent (increase timesteps for better performance)
    model.learn(total_timesteps=200_000, log_interval=10, progress_bar=True)
    print("Done")
    model.save("td3-ContinuousPendulum")

    del model  # remove old model