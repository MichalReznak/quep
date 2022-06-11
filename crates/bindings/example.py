import asyncio
import quepy

# First you need to switch to virtualenv where the quepy
# and other dependencies are installed
# Then Just execute this script which will run Quep tool
# and output the result


async def main():
    c = quepy.QuepyConfig(
        a=42
    )

    res = await quepy.run(c)
    print(res)

asyncio.run(main())
