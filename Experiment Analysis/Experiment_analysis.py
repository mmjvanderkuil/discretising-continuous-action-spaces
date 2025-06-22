from datetime import datetime
from enum import Enum

import numpy as np
import pandas as pd
from dataclasses import dataclass, field


class Environment(Enum):
    mcc = "Mountain Car Continuous"
    penc = "Pendulum Continuous"
    cpc = "Cart Pole Continuous"


@dataclass(order=True)
class BroccoliOutput:
    env: Environment | None
    n_actions: int = field(init=False, repr=False)
    actions: list[float]
    score: float
    n_trees: int
    n_calls: int
    runtime: float
    tree: str

    def __post_init__(self):
        self.n_actions = len(self.actions)

    def toString(self):
        self.n_actions = len(self.actions)
        return (f"Environments     = {self.env.name},\n"
                f"Actions          = {self.actions},\n"
                f"Score            = {self.score},\n"
                f"Number of Trees  = {self.n_trees},\n"
                f"Number of Calls  = {self.n_calls},\n"
                f"Total Runtime    = {self.runtime},\n"
                f"Tree             = {self.tree},\n")


class Node:
    id: int
    left: type
    right: type


class Predicate(Node):
    # index of the state variable
    variable: int
    # Decision value
    value: float


class Action(Node):
    # index of the action in the action array
    index: int
    # actual value in the action space (currently 1-dimensional)
    value: float


def parse_broccoli_output(file: str, env: Environment | str) -> BroccoliOutput:
    lines = []
    with open(file) as f:
        for x in f:
            lines.append(x)

    # print(lines)
    actions = [float(a) for a in lines[1].split('[')[1].split(']')[0].split(',')]
    score = float(lines[2].split(" ")[2])
    n_trees = int(lines[3].split(" ")[4])
    n_calls = int(lines[4].split(" ")[4])
    raw_runtime = lines[5].split(" ")[1:4]
    total = (3600 * int(raw_runtime[0].split("h")[0]) +
             60 * int(raw_runtime[1].split("m")[0]) +
             int(raw_runtime[2].split("s")[0]))
    # print(total)
    tree = [x for x in lines[8:]]
    # print(tree)

    if type(env) == str:
        env = Environment.__dict__[env]

    return BroccoliOutput(env=env,
                          actions=actions,
                          score=score,
                          n_trees=n_trees,
                          n_calls=n_calls,
                          runtime=total,
                          tree=tree)


def parse_broccoli2(file: str, environment: str):
    lines = []
    with open(file) as f:
        for x in f:
            lines.append(x)

    env = Environment.__dict__[environment]
    actions = []
    score = 0
    n_trees = 0
    n_calls = 0
    total = 0
    tree = []


    for i, line in enumerate(lines):

        if line.startswith("with actions :"):
            line = line.split(":")[1]
            for c in "[,]":
                line = line.replace(c, "")
            actions = [float(x) for x in line.split(" ")[1:]]

        elif line.startswith("Score: "):
            score = float(line.split(" ")[1])
            # print(score
        elif line.startswith("Num explicitly considered trees: "):
            n_trees = int(line.split(" ")[-1])
        elif line.startswith("Num environment calls: "):
            n_calls = int(line.split(" ")[-1])
        elif line.startswith("Runtime: "):

            try:
                total = float(line.split(" ")[-1].strip("s\n"))
            except:
                total = float(line.split(" ")[-1].strip("ms\n"))/1000

        elif line.startswith("Tree"):
            tree = lines[i+1:]


    return BroccoliOutput(env=env,
                          actions=actions,
                          score=score,
                          n_trees=n_trees,
                          n_calls=n_calls,
                          runtime=total,
                          tree=tree)


def safe_parse(file: str, environment: str):
    lines = []
    with open(file) as f:
        for x in f:
            lines.append(x)

    env = Environment.__dict__[environment]
    actions = []
    score = 0
    n_trees = 0
    n_calls = 0
    total = 0
    tree = []


    for i, line in enumerate(lines):

        if line.startswith("Actions:"):
            line = line.split(":")[1]
            for c in "[,]":
                line = line.replace(c, "")
            actions = [float(x) for x in line.split(" ")[1:]]

        elif line.startswith("Best Score: "):
            score = float(line.split(" ")[-1])
            # print(score
        elif line.startswith("Number of trees considered: "):
            n_trees = int(line.split(" ")[-1])
        elif line.startswith("Number of environment calls: "):
            n_calls = int(line.split(" ")[-1])
        elif line.startswith("Runtime: "):
            pass
            # try:
            #     total = float(line.split(" ")[-1].strip("s\n"))
            # except:
            #     total = float(line.split(" ")[-1].strip("ms\n"))/1000

        elif line.startswith("Tree"):
            tree = lines[i+1:]


    return BroccoliOutput(env=env,
                          actions=actions,
                          score=score,
                          n_trees=n_trees,
                          n_calls=n_calls,
                          runtime=total,
                          tree=tree)

def create_data_frame(outputs: list[BroccoliOutput]):
    envs = []
    ids = []
    n_actions = []
    scores = []
    n_trees = []
    n_calls = []
    runtimes = []

    for i, outputs in enumerate(outputs):
        envs.append(outputs.env.name)
        ids.append(i)
        n_actions.append(len(outputs.actions))
        scores.append(outputs.score)
        n_trees.append(outputs.n_trees)
        n_calls.append(outputs.n_calls)
        runtimes.append(outputs.runtime)

    d = {"env": envs, "id": ids, "n_actions": n_actions, "score": scores, "n_trees": n_trees, "n_calls": n_calls,
         "runtime": runtimes}
    return d
    # return pd.DataFrame([ids, n_actions, scores, n_trees, n_calls, runtimes],
    #                     columns=["id", "n_actions", "score", "n_trees", "n_calls", "runtime"])


def get_actions(tree: str, actions: list[float]):
    """"
    Args:
        tree: str
        actions: list[float]
    Return:
        actions_full: dict[int, tuple[float, int]]
    """


    # Lines take form of:
    #    1  4    9
    # ['n0: [x_0 >= 0.10] (29)\n',
    #  'n1: [x_0 >= 0.60] (12)\n',
    #  'n2: [x_1 >= 7.20] (17)\n',
    #  'n3: a: 4 (9)\n',
    #  'n4: a: 3 (3)\n',
    #  'n5: a: 4 (4)\n',
    #  'n6: a: 0 (13)\n',
    #  '\n']

    # Step 1: Iterate over tree
    acts_tuple = {}
    for line in tree[:-1]:
        splits = line.split('a')
        if len(splits) < 2:
            continue
        else:
            _a = splits[1].split(" ")
            action_id = int(_a[1])
            calls = int(_a[2].split(")")[0][1:])
            acts_tuple[action_id] = (actions[action_id],calls)
            # print(f"Action {action_id}: {actions[action_id]} with {calls} calls")

    return acts_tuple




if __name__ == "__main__":
    ob = parse_broccoli2(
        "data/exp2/exp2SOT/small_tree_mcc_s=0.txt", "mcc")
    print(ob)
    # print(get_actions(ob.tree, ob.actions))