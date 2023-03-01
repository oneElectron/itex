#! /usr/bin/env python3
import lgithub as gh
from pathlib import Path
from zipfile import ZipFile
import litex as itex
from litex import Version

debug = False

def zipFolder(p: Path, z:ZipFile) -> None:
  for file in p.iterdir():
    z.write(file.relative_to("."))
    if file.is_dir():
      zipFolder(file, z)

def main() -> None:


  if not debug:
    print("Before running the script make sure you have updated the version in cargo.toml and created a new git tag")
    if not input("If you have done this type done: ") == "done":
      exit()

  if not debug:
    tag_name = Version(input("Tag name: "))
  else:
    tag_name = Version("v1.0.0")

  cargo_version = itex.get_cargo_version()
  if cargo_version < tag_name:
    print("The cargo version is not up to date")
    exit()

  if cargo_version > tag_name:
    print("Cargo version is greater than the tag version")
    exit()

  if cargo_version == tag_name:
    print("Cargo is up to date")

  if not debug:
    itex_repo = gh.Repo("oneElectron", "itex", gh.getAuthToken(), name=tag_name.to_str())

  else:
    itex_repo = gh.Repo("oneElectron", "test", gh.getAuthToken(), name=tag_name.to_str())

  if not debug:
    if not itex_repo.tagExists(tag_name.to_str()):
      print("Tag", tag_name.to_str(), "does not exist")
      exit()

    if itex_repo.releaseExists(tag_name.to_str()):
      print("Release already exists!")
      exit()

  # Generate the release
  itex_repo.genRelease(tag_name.to_str())

  # Zip the templates folder
  z = ZipFile("itex-templates.zip", mode='w')
  zipFolder(Path("./itex-templates"), z)
  z.close()

  # Upload the templates folder
  data = open("./itex-templates.zip", mode='rb').read()
  itex_repo.uploadReleaseContent(data, "itex-templates.zip")

  if not debug:
    print("Next steps:")
    print("    - Go to GitHub and finialze the release")
    print("    - Publish the new version to crates.io (cargo publish)")
    print("    - Update homebrew-oneelectron with the new version of itex")

  if debug:
    Path("./itex-templates.zip").unlink()

if __name__ == '__main__':
  main()