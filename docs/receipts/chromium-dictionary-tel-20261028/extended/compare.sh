#!/bin/sh
set -eu
exec docker exec -e PYTHONPATH=/usr/lib/python3/dist-packages wegpui-rtl-qualification /work/parity-tools/bin/python /work/full-parity/dictionary-extended/compare.py
