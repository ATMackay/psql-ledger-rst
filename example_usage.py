"""
Alex Mackay 2024

A few simple scripted tests for our psql rust server. Requires
psqlledger-rst and postgres to be running as separate processes... TODO
"""
import requests
import json
import time

# Create a database entry with the create_account endpoint
url = "http://localhost:8080/create_account"
body = {"username": "john_doe", "email": "johndoe@email.com"}
response = requests.post(url, json=body)
if response.status_code == 200:
    print(response.json())
else:
    print("Request failed")
    quit()

# Use account_by_id
url = "http://localhost:8080/account_by_id"

# Define the JSON-encoded body
req_body = {"id": 1}

start_time = time.time()
# Make 1000 requests for the account data
for _ in range(1000):

    # Request data from the server
    response = requests.get(url, json=req_body)

    # Check if the request was successful (status code 200)
    if response.status_code == 200:
        print(response.json())
    else:
        print("Request failed")

elapsed_time = time.time() - start_time
print(f"Total elapsed time: {elapsed_time:.2f} seconds")
