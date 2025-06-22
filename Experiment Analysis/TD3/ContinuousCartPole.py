import math
import numpy as np
import gym
from gym import spaces
from gym.utils import seeding
from stable_baselines3 import TD3


# Define custom continuous-action CartPole environment
class ContinuousCartPoleEnv(gym.Env):
    metadata = {'render.modes': ['human']}

    def __init__(self):
        # Physics constants
        self.gravity = 9.8
        self.masscart = 1.0
        self.masspole = 0.1
        self.total_mass = self.masspole + self.masscart
        self.length = 0.5  # half-length of the pole
        self.polemass_length = self.masspole * self.length
        self.force_mag = 10.0
        self.tau = 0.02  # time step

        # Continuous action in [-1, 1], to be scaled by force_mag [oai_citation:10‡gist.github.com](
        # https://gist.github.com/iandanforth/e3ffb67cf3623153e968f2afdfb01dc8#file-continuous_cartpole-py%23:~:text
        # =self,high%252C%2520high) [oai_citation:11‡gist.github.com](
        # https://gist.github.com/iandanforth/e3ffb67cf3623153e968f2afdfb01dc8#file-continuous_cartpole-py%23:~:text
        # =,stepPhysics%2528force)
        self.min_action = -1.0
        self.max_action = 1.0
        self.action_space = spaces.Box(low=self.min_action, high=self.max_action, shape=(1,), dtype=np.float32)
        # Observation: [cart_pos, cart_vel, pole_angle, pole_vel]
        high = np.array([2.4, 1.0, 0.418, 1.0])
        self.observation_space = spaces.Box(-high, high, dtype=np.float32)

        self.seed()
        self.state = None
        self.steps_beyond_done = None
        self.initial_state = [-0.05, 0.05, 0.05, 0.05]
        self.iter = 0

    def seed(self, seed=None):
        self.np_random, seed = seeding.np_random(seed)
        return [seed]

    def stepPhysics(self, force):
        x, x_dot, theta, theta_dot = self.state
        costheta = math.cos(theta)
        sintheta = math.sin(theta)
        # Apply physics equations from CartPole (Rich Sutton)
        temp = (force + self.polemass_length * theta_dot ** 2 * sintheta) / self.total_mass
        thetaacc = (self.gravity * sintheta - costheta * temp) / (
                self.length * (4.0 / 3.0 - (self.masspole * costheta ** 2) / self.total_mass))
        xacc = temp - (self.polemass_length * thetaacc * costheta) / self.total_mass
        # Update state
        x = x + self.tau * x_dot
        x_dot = x_dot + self.tau * xacc
        theta = theta + self.tau * theta_dot
        theta_dot = theta_dot + self.tau * thetaacc
        return x, x_dot, theta, theta_dot

    def step(self, action):
        # Ensure action is valid
        assert self.action_space.contains(action), f"{action} invalid!"
        # Scale action by force magnitude [oai_citation:12‡gist.github.com](https://gist.github.com/iandanforth/e3ffb67cf3623153e968f2afdfb01dc8#file-continuous_cartpole-py%23:~:text=,stepPhysics%2528force)
        force = self.force_mag * float(action)
        # Update dynamics
        self.state = self.stepPhysics(force)
        x, x_dot, theta, theta_dot = self.state

        # Check for failure
        done = bool((x < -2.4) or (x > 2.4) or (theta < -12 * 2 * math.pi / 360) or (theta > 12 * 2 * math.pi / 360))
        reward = 1.0 if not done else 0.0
        self.iter += 1
        trunc = False
        if not done:
            trunc = self.iter >= 1000

        return np.array(self.state, dtype=np.float32), reward, bool(done), trunc, {}

    def reset(self, *, seed: int | None = None, options: dict | None = None):
        # Small random initial state
        self.state = self.np_random.uniform(low=-0.05, high=0.05, size=(4,))
        self.iter = 0

        #
        self.steps_beyond_done = None
        return np.array(self.state, dtype=np.float32), options

    def render(self, mode='human'):
        pass  # Rendering omitted for brevity


if __name__ == "__main__":
    # Create environment and TD3 model
    env = ContinuousCartPoleEnv()
    model = TD3("MlpPolicy",
                env,
                verbose=1,
                learning_rate=1e-3,
                train_freq=1,
                batch_size=256,
                policy_kwargs={"net_arch": [400, 300]},
                gradient_steps=1,
                learning_starts=10_000)
    # Train the agent (increase timesteps for better performance)
    model.learn(total_timesteps=50_000, log_interval=10, progress_bar=True)
    model.save("td3-ContinuousCartPole")
    del model  # remove old model
