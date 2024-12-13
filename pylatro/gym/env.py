import gymnasium as gym
from gymnasium import spaces
import pylatro
from typing import Optional


class BalatroEnv(gym.Env):
    def __init__(self):
        super(BalatroEnv, self).__init__()

        self._game = pylatro.GameEngine()
        self._score = 0
        self._target_score = 0

        self.observation_space = gym.spaces.Dict(
            {
                "score": gym.spaces.Discrete(1),
                "target": gym.spaces.Discrete(1),
            }
        )

    @property
    def action_space(self):
        actions = self._game.gen_moves()
        return spaces.Discrete(len(actions))

    def _get_obs(self):
        return {"score": self._score, "target": self._target_score}

    def _get_info(self):
        # print(self._game.state)
        return {"difference": self._target_score - self._score}

    def step(self, action):
        moves = self._game.gen_moves()
        real_action = moves[action]
        self._game.handle_action(real_action)

        terminated = self._game.is_over
        truncated = terminated
        reward = 1 if self._game.is_win else 0
        observation = self._get_obs()
        info = self._get_info()
        return observation, reward, terminated, truncated, info

    def reset(self, seed: Optional[int] = None, options: Optional[dict] = None):
        super().reset(seed=seed)
        self._game = pylatro.GameEngine()
        self._score = 0
        self._target_score = 0
        observation = self._get_obs()
        info = self._get_info()

        return observation, info

    def render(self, mode="human"):
        return


def register():
    gym.register(
        id="gymnasium_env/Balatro-v0",
        entry_point=BalatroEnv,
    )
