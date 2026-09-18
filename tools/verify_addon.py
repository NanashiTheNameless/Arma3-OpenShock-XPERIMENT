"""Check that the unpacked release PBO contains its config and every addon asset."""

import sys
from pathlib import Path


def verify(source: Path, unpacked: Path) -> None:
    prefix = (unpacked / "$PBOPREFIX$").read_text().splitlines()
    if "prefix=A3_OpenShock" not in prefix:
        raise ValueError("PBO prefix must be A3_OpenShock")
    if not (unpacked / "config.bin").read_bytes().startswith(b"\x00raP"):
        raise ValueError("PBO is missing a binarized config.bin")
    expected = {"$PBOPREFIX$", "config.bin"}
    for path in source.rglob("*"):
        if not path.is_file() or path.name in {"config.cpp", "$PBOPREFIX$"}:
            continue
        relative = path.relative_to(source)
        expected.add(relative.as_posix())
        if (unpacked / relative).read_bytes() != path.read_bytes():
            raise ValueError(f"PBO asset differs from source: {relative}")
    actual = {
        path.relative_to(unpacked).as_posix()
        for path in unpacked.rglob("*")
        if path.is_file()
    }
    if actual != expected:
        raise ValueError(f"Unexpected PBO contents: {actual ^ expected}")
    print(f"Verified PBO prefix, binarized config, and {len(expected) - 2} source assets.")


if __name__ == "__main__":
    verify(Path(sys.argv[1]), Path(sys.argv[2]))
