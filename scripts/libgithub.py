import requests
from pathlib import Path
import fileinput
import json

def getAuthToken() -> str:
    github_auth_token_path = Path("../github_auth.token")
    if not github_auth_token_path.is_file():
        print("Could not find the github auth token")
        exit()
    out = next(fileinput.input(files=github_auth_token_path)).removesuffix("\n")
    return out

def pretty_print(input:dict):
    print(json.dumps(input, indent=4))

class Repo:
    token: str
    prefix: str

    def __init__(self, user_name:str, repo_name:str, auth_token:str):
        self.prefix = "https://api.github.com/repos/" + user_name + "/" + repo_name + "/"
        self.token = auth_token

    def releases(self) -> dict:
        response_content = requests.get(self.prefix + "releases", headers={"Authorization": "Bearer " + self.token}).text
        out_dict = json.loads(response_content)
        return out_dict
    
    def tags(self) -> dict:
        response_content = requests.get(self.prefix + "tags", headers={"Authorization": "Bearer " + self.token}).text
        out_dict = json.loads(response_content)
        return out_dict