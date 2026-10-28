import argparse
import hashlib
import pathlib
import struct
import subprocess
import tempfile
import tomllib

ROOT = pathlib.Path(__file__).resolve().parent
SOURCE = {
    'khmerdict': ('d06150c5e27b1852f89198de7e84f5b71fb9250f0735acd6d0526f343226bed5',
                  '7f56e4358372b19fe7b4aa15bee71e7f06e3d48f407c2c5973e5122043e3a214'),
    'laodict': ('d2c4f10b0e80abaa59ef7dcdac0d1adedb785e2f4f40f4f7ff79f87e36bb55f2',
                '4274ef25980a4d89dc6de38bade1ee3f67239e489178baf35d4bff971badc3a4'),
}


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--gendict', default='gendict')
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    with tempfile.TemporaryDirectory(prefix='.dictionary-generation-', dir=ROOT) as tmp:
        for name, (source_hash, output_hash) in SOURCE.items():
            source = ROOT / (name + '.txt')
            if sha256(source.read_bytes()) != source_hash:
                raise SystemExit(f'{name}: source hash mismatch')
            output = pathlib.Path(tmp) / (name + '.toml')
            subprocess.run([args.gendict, '--uchars', '--toml', str(source), str(output)], check=True)
            data = tomllib.loads(output.read_text())
            if data['trie_type'] != 'uchars' or data['has_values'] or data['transform_type'] != 'none':
                raise SystemExit(f'{name}: unexpected trie format')
            units = data['trie_data']
            packed = struct.pack('<' + 'H' * len(units), *units)
            if sha256(packed) != output_hash:
                raise SystemExit(f'{name}: generated trie hash mismatch')
            target = ROOT / (name + '.u16le')
            if args.write:
                target.write_bytes(packed)
            elif target.read_bytes() != packed:
                raise SystemExit(f'{name}: checked-in trie differs')
            print(f'{name}: {len(packed)} bytes; SHA-256 {output_hash}')


if __name__ == '__main__':
    main()
