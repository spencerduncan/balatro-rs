import gymnasium as gym
from gymnasium import spaces
import pylatro
from typing import Optional


class BalatroEnv(gym.Env):
    def __init__(self):
        super(BalatroEnv, self).__init__()

        config = pylatro.Config()
        config.ante_end = 1
        self._config = config
        self._game = pylatro.GameEngine(self._config)
        self._score = 0
        self._last_score = 0
        self._target_score = 0

        self.observation_space = gym.spaces.Dict(
            {
                "score": gym.spaces.Discrete(1),
                "target": gym.spaces.Discrete(1),
            }
        )

    @property
    def action_space(self):
        print("action space")
        actions = self._game.gen_moves()
        return spaces.Discrete(len(actions))

    def _get_obs(self):
        return {"score": self._score, "target": self._target_score}

    def _get_info(self):
        return {"difference": self._target_score - self._score}

    def step(self, action):
        print("step")
        moves = self._game.gen_moves()
        real_action = moves[action]
        handle_res = self._game.handle_action(real_action)
        if handle_res is not None:
            print(f"handle response: {handle_res}")

        self._last_score = self._score
        self._score = self._game.state.score
        self._target_score = self._game.state.required_score

        terminated = self._game.is_over
        truncated = terminated

        print(f"terminated: {terminated}")
        print(f"game over: {self._game.is_over}")
        # print(self._game.state)

        # calc reward
        reward = 0
        score_diff = self._score - self._last_score
        if score_diff > 0:
            reward = score_diff
        if terminated:
            reward = 10000 if self._game.is_win else -100
        print(f"reward: {reward}")

        observation = self._get_obs()
        info = self._get_info()
        return observation, reward, terminated, truncated, info

    def reset(self, seed: Optional[int] = None, options: Optional[dict] = None):
        print("reset")
        super().reset(seed=seed)
        self._game = pylatro.GameEngine(self._config)
        self._score = 0
        self._last_score = 0
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
