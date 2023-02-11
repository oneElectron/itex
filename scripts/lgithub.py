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
    upload_url:str
    debug: bool
    release_name: str
    release_id: int

    def __init__(self, user_name:str, repo_name:str, auth_token:str, name:str, debuging:bool = False):
        self.prefix = "https://api.github.com/repos/" + user_name + "/" + repo_name + "/"
        self.upload_url = "https://uploads.github.com/repos/" + user_name + "/" + repo_name + "/"
        self.token = auth_token
        self.debug = debuging
        self.release_name = name

    def releases(self) -> dict:
        response_content = requests.get(self.prefix + "releases", headers={"Authorization": "Bearer " + self.token}).text
        out_dict = json.loads(response_content)
        return out_dict

    def genRelease(self, tag_name: str):
        release_data = {
            "tag_name": tag_name,
            "name": tag_name,
            "body": "PLACEHOLDER",
            "draft": True,
            "prerelease": False,
            "generate_release_notes": False
        }
        response = requests.post(self.prefix + "releases", json=release_data, headers={"Authorization": "Bearer " + self.token})
        response_json = json.JSONDecoder().decode(response.text)
        self.release_id = int(response_json["id"])
    
    def uploadReleaseContent(self, bin_data:bytes, filename:str):
        requests.post(self.upload_url + "releases/" + str(self.release_id) + "/assets?name=" + filename, data=bin_data, headers={"Authorization": "Bearer " + self.token, "Content-Type": "application/zip"})
    
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

