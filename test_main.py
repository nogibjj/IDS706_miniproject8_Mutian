from main import *
import time





def test_func():
    # Record the current time (before the code block)
    start_time = time.time()

    df = loadDf("taker_buy_sell_volume.csv")
    # Display the DataFrame
    print("DataFrame Contents:")
    print(df)

    assert isinstance(df,pl.DataFrame)
    describeData(df)
    end_time = time.time()

    # Calculate the execution time in milliseconds
    execution_time_ms = (end_time - start_time) * 1000  # Convert to ms

    # Display the execution time in ms
    print(f"Execution Time: {execution_time_ms} ms")
    

test_func()
