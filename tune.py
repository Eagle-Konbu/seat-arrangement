import statistics
import os
import joblib
import time
import optuna
import subprocess

def calc_score_and_sigma(t0,t1):
    res = subprocess.check_output(f"./target/release/tmp {t0} {t1}",shell=True)
    # コンマ区切り（score, sigma）で返ってくるので分割
    score, sigma = res.decode().split(",")
    return float(score), float(sigma)

def objective(trial: optuna.trial.Trial):
    start = time.time()

    t1 = trial.suggest_float("t1", 0, 200)
    t2 = trial.suggest_float("t2", 0, 10)

    mean,sigma = calc_score_and_sigma(t1,t2)
    print(f"elapsed: {time.time() - start}")
    return mean, sigma

if __name__ == "__main__":
    study = optuna.create_study(directions=["maximize", "minimize"])
    study.optimize(objective, n_trials=10)
    print(f"{study.best_trials}")