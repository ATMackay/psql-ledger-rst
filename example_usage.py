"""
Alex Mackay 2024

A few simple scripted tests for our psql rust server. Requires
psqlledger-rst and postgres to be running as separate processes... TODO
"""
import requests
import time
import random

def e2e_fetch_accounts(n: int): 
    # Create a database entry with the create_account endpoint
    url = "http://localhost:8080/create-account"
    random_int = random.randint(0, 99999)
    usrname = "exampleuser" + str(random_int)
    email = "user" + str(random_int) + "@example.com"
    body = {"username": usrname, "email": {"String": email, "Valid": True}}
    response = requests.put(url, json=body)
    if response.status_code != 200:
        body = {"username": usrname, "email":  email }
        response = requests.put(url, json=body)
        if response.status_code != 200:
            print("Request failed: ", response.json())
            quit()


    # Use account_by_id
    url = "http://localhost:8080/accounts"


    start_time = time.time()
    # Make 1000 requests for the account data
    for _ in range(n):

        # Request data from the server
        response = requests.get(url)

        # Print error if the request was not successful
        if response.status_code != 200:
            #print(response.json())
        #else:
            print("Request failed: ", response.json())

    elapsed_time = time.time() - start_time
    print(f"Total elapsed time: {elapsed_time:.2f} seconds")
    print(f"Rate: {n/elapsed_time} req/s")


def e2e_health(n: int): 
    # Use account_by_id
    url = "http://localhost:8080/health"

    start_time = time.time()
    # Make 1000 requests for the account data
    for _ in range(n):

        # Request data from the server
        response = requests.get(url)

        # Print error if the request was not successful
        if response.status_code == 200:
            print(response.json())
        else:
            print("Request failed: ", response.json())

    elapsed_time = time.time() - start_time
    print(f"Total elapsed time: {elapsed_time:.2f} seconds")
    print(f"Rate: {n/elapsed_time} req/s")

def e2e_status(n: int): 
    # Use account_by_id
    url = "http://localhost:8080/status"

    start_time = time.time()
    # Make 1000 requests for the account data
    for _ in range(n):

        # Request data from the server
        response = requests.get(url)

        # Print error if the request was not successful
        if response.status_code == 200:
            print(response.json())
        else:
            print("Request failed: ", response.json())
            
    elapsed_time = time.time() - start_time
    print(f"Total elapsed time: {elapsed_time:.2f} seconds")
    print(f"Rate: {n/elapsed_time} req/s")

if __name__ == "__main__":
    e2e_fetch_accounts(100) # Replace with method of your choosing