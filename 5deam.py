import argparse
import requests

parser = argparse.ArgumentParser(
    prog="5deam",
    description="5beam client")

parser.add_argument("-l", "--level")
args = parser.parse_args()

lvl_id = int(args.level)
response = requests.get("https://5beam.zelo.dev/api/level?id=%i" % lvl_id)
print(response.text)