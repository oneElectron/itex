#! /usr/bin/env python3
import libgithub as gh
from libgithub import pretty_print as pp

itex_repo = gh.Repo("oneElectron", "test", gh.getAuthToken(), debuging=True)


tag_name = input("Tag name: ")
if not itex_repo.tagExists(tag_name):
  print("Tag", tag_name, "does not exist")
  exit()

if itex_repo.releaseExists(tag_name):
  print("Release already exists!")
  exit()

itex_repo.genRelease(tag_name)


# TODO
# publish release | get tag name, then upload release
# zip and upload templates
