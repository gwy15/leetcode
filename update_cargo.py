#! /usr/bin/env python3
from pathlib import Path
import toml

rust_files = list(Path('.').glob('*.rs'))
assert len(rust_files) == 1
rust_file = rust_files[0]
print(rust_file)

cargo_config = toml.load('Cargo.toml')
print(cargo_config)

cargo_config['lib']['path'] = rust_file.name
with open('Cargo.toml', 'w', encoding='utf8') as f:
    toml.dump(cargo_config, f)
