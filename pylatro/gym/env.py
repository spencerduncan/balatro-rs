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
        self._highest_score = 0

        self.observation_space = gym.spaces.Dict(
            {
                "score": gym.spaces.Discrete(1),
                "target": gym.spaces.Discrete(1),
            }
        )

    @property
    def action_space(self):
        actions = self._game.gen_actions()
        return spaces.Discrete(len(actions))

    def _get_obs(self):
        return {"score": self._score, "target": self._target_score}

    def _get_info(self):
        return {"difference": self._target_score - self._score}

    def step(self, action):
        actions = self._game.gen_actions()
        if action < len(actions):
            real_action = actions[action]
            self._game.handle_action(real_action)

        terminated = self._game.is_over
        truncated = terminated

        score = self._game.state.score
        target_score = self._game.state.required_score
        if score > self._highest_score:
            self._highest_score = score
            print(f"new high score: {score}")
        score_diff = score - self._last_score
        # self._last_score = score
        self._target_score = target_score
        # print(f"prev score: {self._last_score}")
        # print(f"current score: {self._game.state.score}")
        # print(f"score diff: {score_diff}")
        # self._score = self._game.state.score
        # self._target_score = self._game.state.required_score

        reward = 0
        if score_diff > 0:
            reward = score_diff / 100
        if terminated:
            reward = 1 if self._game.is_win else 0
            if self._game.is_win:
                print(f"game win: {self._game.state}")
                print(f"score: {self._game.state.score}")
        # print(f"reward: {reward}")

        observation = self._get_obs()
        info = self._get_info()
        return observation, reward, terminated, truncated, info

    def reset(self, seed: Optional[int] = None, options: Optional[dict] = None):
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
