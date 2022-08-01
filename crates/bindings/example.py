import asyncio
import quepy

# First you need to switch to virtualenv where the quepy
# and other dependencies are installed
# Then Just execute this script which will run Quep tool
# and output the result

# Needs to be in the cwd

async def main():
    c = quepy.QuepyConfig(
        output="Serial",
    )

    res = await quepy.run(c)
    print(res)

asyncio.run(main())
