import asyncio
from py_async import (
    rust_sleep,
    echo_param,
    print_list,
    get_stock_price,
    StockPriceError,
)


async def main() -> None:
    # Example 1: Sleep
    await rust_sleep()

    # Example 2: Get some data back from Rust
    resp = await echo_param("hello world")
    print(resp)

    # Example 3: Print a list of strings.
    await print_list(["a", "b", "c", "d", "e", "f", "g"])

    # Example 4: Get a stock price and handle errors
    try:
        stock_price = await get_stock_price("AAPL")
        print(stock_price)
    except ValueError as e:
        print(e)
    except StockPriceError:
        print("stock price error")


if __name__ == "__main__":
    asyncio.run(main())
