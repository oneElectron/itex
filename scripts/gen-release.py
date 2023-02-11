#! /usr/bin/env python3
import libgithub as gh
from libgithub import pretty_print as pp

# tag_name = input("Tag name: ")
tag_name = "v1.0.0"

itex_repo = gh.Repo("oneElectron", "test", gh.getAuthToken(), name=tag_name, debuging=True)

data = open("./itex-templates.zip", mode='rb').read()

if not itex_repo.tagExists(tag_name):
  print("Tag", tag_name, "does not exist")
  exit()

if itex_repo.releaseExists(tag_name):
  print("Release already exists!")
  exit()

itex_repo.genRelease(tag_name)

itex_repo.uploadReleaseContent(data, "itex-templates.zip")


# TODO
# publish release | get tag name, then upload release
# zip and upload templates
