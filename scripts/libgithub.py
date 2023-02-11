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
    debug: bool

    def __init__(self, user_name:str, repo_name:str, auth_token:str, debuging:bool = False):
        self.prefix = "https://api.github.com/repos/" + user_name + "/" + repo_name + "/"
        self.token = auth_token
        self.debug = debuging

    def releases(self) -> dict:
        response_content = requests.get(self.prefix + "releases", headers={"Authorization": "Bearer " + self.token}).text
        out_dict = json.loads(response_content)
        return out_dict

    def genRelease(self, tag_name: str):
        release_data = {
            "tag_name": tag_name,
            "target_commitish": "main",
            "body": "Description",
            "name": tag_name,
            "draft": True,
            "prerelease": True,
            "generate_release_notes": False,
            "make_latest": False
        }
        json_data = json.JSONEncoder().encode(release_data)
        print(json_data)
        response = requests.post(self.prefix + "releases", json=json_data, headers={"Authorization": "Bearer " + self.token, "Accept": "application/vnd.github+json", "X-GitHub-Api-Version": "2022-11-28"})
    
    def releaseExists(self, release_name) -> bool:
        response_content = requests.get(self.prefix + "releases", headers={"Authorization": "Bearer " + self.token}).text
        data = json.loads(response_content)
        for release in data:
            if release["name"] == release_name:
                return True
        return False
    
    def tags(self) -> dict:
        response_content = requests.get(self.prefix + "tags", headers={"Authorization": "Bearer " + self.token}).text
        out_dict = json.loads(response_content)
        return out_dict
    
    def tagExists(self, tag_name:str) -> bool:
        response_content = requests.get(self.prefix + "tags", headers={"Authorization": "Bearer " + self.token}).text
        data = json.loads(response_content)
        for tag in data:
            if tag["name"] == tag_name:
                return True
        return False

