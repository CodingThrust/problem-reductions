"""Exercise the release target with real Cargo and a local Git remote."""

import shutil
import subprocess
import tempfile
import tomllib
from pathlib import Path


def run(*args, cwd):
    result = subprocess.run(args, cwd=cwd, text=True, capture_output=True)
    assert result.returncode == 0, result.stdout + result.stderr
    return result.stdout.strip()


def check_release():
    with tempfile.TemporaryDirectory(prefix="release-check-") as tmp:
        root = Path(tmp)
        repo = root / "repo"
        repo.mkdir()
        remote = root / "origin.git"
        run("git", "init", "--bare", str(remote), cwd=root)
        run("git", "init", "-b", "main", cwd=repo)
        run("git", "config", "user.name", "Release Test", cwd=repo)
        run("git", "config", "user.email", "release@example.com", cwd=repo)
        run("git", "remote", "add", "origin", str(remote), cwd=repo)
        shutil.copyfile(Path(__file__).resolve().parents[1] / "Makefile", repo / "Makefile")

        dependencies = {
            "problemreductions": ["problemreductions-macros", "problemreductions-expr"],
            "problemreductions-macros": ["problemreductions-expr"],
            "problemreductions-expr": [],
            "problemreductions-cli": ["problemreductions"],
            "problemreductions-web": ["problemreductions-expr"],
        }
        for name, deps in dependencies.items():
            package = repo if name == "problemreductions" else repo / name
            (package / "src").mkdir(parents=True)
            (package / "src/lib.rs").write_text("pub const READY: bool = true;\n")
            manifest = f'[package]\nname = "{name}"\nversion = "0.1.0"\nedition = "2024"\n'
            if name == "problemreductions":
                members = ", ".join(f'"{n}"' for n in dependencies if n != name)
                manifest += f'\n[workspace]\nmembers = [{members}]\n'
                manifest += '\n[features]\nexample-db = []\n'
            manifest += "\n[dependencies]\n"
            for dep in deps:
                path = dep if name == "problemreductions" else f"../{dep}"
                if dep == "problemreductions":
                    path = ".."
                version = '' if name == "problemreductions-web" else 'version = "0.1.0", '
                manifest += f'{dep} = {{ {version}path = "{path}" }}\n'
            (package / "Cargo.toml").write_text(manifest)
        (repo / ".gitignore").write_text("/target/\n")
        run("cargo", "generate-lockfile", "--offline", cwd=repo)
        run("git", "add", ".", cwd=repo)
        run("git", "commit", "-m", "Initial workspace", cwd=repo)
        run("git", "push", "-u", "origin", "main", cwd=repo)

        run("make", "release", "V=0.2.0", cwd=repo)

        for name, deps in dependencies.items():
            package = repo if name == "problemreductions" else repo / name
            manifest = tomllib.loads((package / "Cargo.toml").read_text())
            assert manifest["package"]["version"] == "0.2.0", name
            for dep in deps:
                if name != "problemreductions-web":
                    assert manifest["dependencies"][dep]["version"] == "0.2.0", (name, dep)
        assert run("git", "status", "--porcelain", cwd=repo) == ""
        commit = run("git", "rev-parse", "HEAD", cwd=repo)
        assert run("git", "rev-parse", "v0.2.0^{}", cwd=remote) == commit
        assert run("git", "rev-parse", "main", cwd=remote) == commit
        lock = tomllib.loads(run("git", "show", "v0.2.0:Cargo.lock", cwd=repo))
        assert all(p["version"] == "0.2.0" for p in lock["package"])


if __name__ == "__main__":
    check_release()
    print("Release versions, lockfile, commit, and remote tag verified.")
