import gym
import numpy as np
from gym import spaces
import torch
import torch.nn as nn
import torch.nn.functional as F
from ContinuousCartPole import ContinuousCartPoleEnv

class Actor(nn.Module):
    def __init__(self, state_dim, action_dim, action_low, action_high):
        super().__init__()
        # Two hidden layers of size 256 (common default in TD3)
        self.l1 = nn.Linear(state_dim, 256)
        self.l2 = nn.Linear(256, 256)
        self.l3 = nn.Linear(256, action_dim)
        # Compute action scaling factors for [action_low, action_high]
        self.register_buffer('action_scale', torch.tensor((action_high - action_low) / 2.0))
        self.register_buffer('action_bias',  torch.tensor((action_high + action_low) / 2.0))

    def forward(self, x):
        x = F.relu(self.l1(x))
        x = F.relu(self.l2(x))
        # Output in [-1,1] via tanh, then scale to actual range
        action = torch.tanh(self.l3(x))
        return action * self.action_scale + self.action_bias



if __name__=="__main__":
    # Instantiate environments
    envs = {
        'MountainCarContinuous-v0': gym.make('MountainCarContinuous-v0'),
        'Pendulum-v1': gym.make('Pendulum-v1'),
        'ContinuousCartPole': ContinuousCartPoleEnv()
    }

    for name, env in envs.items():
        state_dim = env.observation_space.shape[0]
        action_dim = env.action_space.shape[0]
        action_low = float(env.action_space.low[0])
        action_high = float(env.action_space.high[0])

        # Initialize actor and load pretrained weights (from CleanRL training)
        actor = Actor(state_dim, action_dim, action_low, action_high)
        actor.load_state_dict(torch.load(f'td3_actor_{name}.pth'))
        actor.eval()  # set to evaluation mode (disables dropout/batchnorm if any)

        # Evaluate for a few episodes
        for ep in range(3):
            obs, _ = env.reset()
            done = False
            total_reward = 0.0
            actions = []
            while not done:
                state = torch.as_tensor(obs, dtype=torch.float32).unsqueeze(0)  # batch of 1
                with torch.no_grad():
                    action = actor(state).numpy()[0]  # deterministic action
                obs, reward, done, trunc, _ = env.step(action)
                total_reward += reward
                actions.append(action.item())
            print(f'{name} Episode {ep + 1}: Reward = {total_reward:.2f}')
            print('Actions:', actions)