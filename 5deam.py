import requests

lvl_id = 89
response = requests.get("https://5beam.zelo.dev/api/level?id=%i" % lvl_id)
print(response.text)