import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import polars as pl

def loadDf(path):
    df= pl.read_csv(path)
    return df


def describeData(df):
      # Calculate the mean of specific columns
    average0 = df['timestamp'].mean()
    average1 = df['buy_vol'].mean()
    average2 = df['sell_vol'].mean()
    average3 = df['buy_sell_ratio'].mean()

    print(
        f"Average: {average0},{average1},{average2},{average3}"
    )



