import json
import requests


with open('pks.txt') as f:
	pks = [line.rstrip() for line in f]

	doc = {'keys': pks}

	response = requests.api.post('http://localhost:8080/', json=doc)

	print(response.text)