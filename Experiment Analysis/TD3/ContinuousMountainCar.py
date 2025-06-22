import numpy as np
import gym
from gym import spaces
from gym.utils import seeding
from stable_baselines3 import TD3
from stable_baselines3.common import noise


class MountainCarContinuousEnv(gym.Env):
    """
    Custom MountainCarContinuous-v0 environment matching OpenAI Gym's implementation.
    State: [position, velocity]; action: force ∈ [-1, 1].
    """

    metadata = {'render_modes': ['human'], 'render_fps': 30}

    def __init__(self):
        super().__init__()
        # Environment constants
        self.state = None
        self.min_position = -1.2
        self.max_position = 0.6
        self.max_speed = 0.07
        self.goal_position = 0.45
        self.power = 0.0015  # force multiplier
        self.gravity = 0.0025  # gravity constant
        self.iters = 0

        # Action and observation spaces
        self.action_space = spaces.Box(low=-1.0, high=1.0, shape=(1,), dtype=np.float32)

        low = np.array([self.min_position, -self.max_speed], dtype=np.float32)
        high = np.array([self.max_position, self.max_speed], dtype=np.float32)
        self.observation_space = spaces.Box(low=low, high=high, dtype=np.float32)

        # Initialize state and random seed
        # self.initial_state = [-0.5, 0.0]

        self.seed()

    def seed(self, seed=None):
        """Set random seed for reproducibility."""
        self.np_random, seed = seeding.np_random(seed)
        return [seed]

    def reset(self, *, seed: int | None = None, options: dict | None = None):
        """Reset to a random start in [-0.6, -0.4] with zero velocity [oai_citation:14‡gymlibrary.dev](https://www.gymlibrary.dev/environments/classic_control/mountain_car_continuous/#:~:text=The%20position%20of%20the%20car,is%20always%20assigned%20to%200)."""
        position = self.np_random.uniform(-0.6, -0.4)
        velocity = 0.0
        self.state = np.array([position, velocity], dtype=np.float32)
        self.iters = 0
        return np.array(self.state, dtype=np.float32)

    def step(self, action):
        """Apply action (force) and update state using the environment dynamics."""
        position, velocity = self.state
        force = np.clip(action, -1.0, 1.0)[0]
        # Update dynamics: velocity and position update [oai_citation:15‡gymlibrary.dev](https://www.gymlibrary.dev/environments/classic_control/mountain_car_continuous/#:~:text=velocity%20t%2B1%20%3D%20velocity%20t%2B1,position_%7Bt)
        velocity += force * self.power - self.gravity * np.cos(3 * position)
        # Clip velocity
        velocity = np.clip(velocity, -self.max_speed, self.max_speed)
        position += velocity
        # Enforce position bounds and inelastic collision (velocity=0 at boundaries) [oai_citation:16‡gymlibrary.dev](https://www.gymlibrary.dev/environments/classic_control/mountain_car_continuous/#:~:text=0,0.07%2C%200.07)
        if position < self.min_position:
            position = self.min_position
            velocity = 0.0
        if position > self.max_position:
            position = self.max_position
            velocity = 0.0
        # Check if goal is reached
        done = bool(position >= self.goal_position)
        # Compute reward: -0.1*action^2 each step, plus +100 on reaching goal [oai_citation:17‡gymlibrary.dev](https://www.gymlibrary.dev/environments/classic_control/mountain_car_continuous/#:~:text=A%20negative%20reward%20of%20,negative%20reward%20for%20that%20timestep)
        reward = float(-0.1 * (action[0] ** 2))
        self.iters += 1
        trunc = False
        if done:
            reward += 100.0
            print("\n\n##############\n\t\tREACHED THE GOAL\n##############")
            trunc = True
        if self.iters >= 1000:
            trunc = True
        self.state = np.array([position, velocity], dtype=np.float32)
        return np.array(self.state, dtype=np.float32), reward, done, trunc, {}


if __name__ == "__main__":
    # Create environment and TD3 model
    action_noise = noise.OrnsteinUhlenbeckActionNoise(mean=np.array([0]), sigma=0.25, theta=0.15)

    env = MountainCarContinuousEnv()
    # model = TD3("MlpPolicy", env,
    #             verbose=2,
    #             seed=0,
    #             learning_rate=1e-3,
    #             train_freq=1,
    #             action_noise=action_noise,
    #             batch_size=256,
    #             gradient_steps=1,
    #             policy_kwargs={"net_arch": [400, 300]},
    #             learning_starts=10_000
    #             )

    model = TD3("MlpPolicy", env,
                verbose=2,
                seed=0,
                learning_rate=1e-3,
                gamma=0.99,
                buffer_size=1_000_000,
                train_freq=20,
                tau=0.005,
                action_noise=action_noise,
                batch_size=256,
                gradient_steps=10,
                policy_kwargs={"net_arch": [400, 300]},
                )
    # Train the agent (increase timesteps for better performance)
    model.learn(total_timesteps=300_000, log_interval=10, progress_bar=True)
    print("Done")
    model.save("td3-ContinuousMountainCar")

    del model  # remove old model
