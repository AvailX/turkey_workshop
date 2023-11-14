import json
import requests


with open('pks.txt') as f:
	pks = [line.rstrip() for line in f]

	doc = {'keys': pks}

	response = requests.api.post('https://ctf.avail.global/', json=doc)

	print(response.text)