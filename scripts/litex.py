#! /usr/bin/env python3
from typing import List
from pathlib import Path
import fileinput
from dataclasses import dataclass, field


class Version:
    __name: str
    value: List[int]

    def __init__(self, input_str:str):
        self.value = list(map(int, input_str.removeprefix("v").split('.')))
        self.__name = input_str
    
    def to_str(self) -> str:
        return self.__name

    def __lt__(self, b) -> bool:
        for i in range(0, 2):
            if self.value[i] < b.value[i]:
                return True
            if self.value[i] > b.value[i]:
                return False
        return False

    def __gt__(self, b) -> bool:
        for i in range(3):
            if self.value[i] > b.value[i]:
                return True
            if self.value[i] < b.value[i]:
                return False
        return False
    
    def __eq__(self, b) -> bool:
        for i in range(0, 2):
            if self.value[i] != b.value[i]:
                return False
        return True

def get_cargo_version() -> Version:
    cargo_config = Path("./Cargo.toml")
    for line in fileinput.input(cargo_config):
        if not line.startswith("version"):
            continue
        line = line.strip()
        start_index:int = 0
        end_index:int = 0
        for i in range(1, len(line)):
            if line[i] == '"':
                if start_index == 0:
                    start_index = i
                else:
                    end_index = i
            i += 1
        version_str:str = "v1.2.0"
        version = Version(version_str)
        return version

v = get_cargo_version()
print(v.to_str())