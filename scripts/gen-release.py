#! /usr/bin/env python3
import lgithub as gh
from pathlib import Path
from zipfile import ZipFile

def zipFolder(p: Path, z:ZipFile):
  for file in p.iterdir():
    z.write(file.relative_to("."))
    if file.is_dir():
      zipFolder(file, z)

tag_name = input("Tag name: ")

itex_repo = gh.Repo("oneElectron", "itex", gh.getAuthToken(), name=tag_name)

if not itex_repo.tagExists(tag_name):
  print("Tag", tag_name, "does not exist")
  exit()

if itex_repo.releaseExists(tag_name):
  print("Release already exists!")
  exit()

# Generate the release
itex_repo.genRelease(tag_name)

# Zip the templates folder
z = ZipFile("itex-templates.zip", mode='w')
zipFolder(Path("./itex-templates"), z)
z.close()

# Upload the templates folder
data = open("./itex-templates.zip", mode='rb').read()
itex_repo.uploadReleaseContent(data, "itex-templates.zip")


# TODO
# publish release | get tag name, then upload release
# zip and upload templates
