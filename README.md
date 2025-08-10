# mdict-rs

This project provides **Python bindings** via PyO3, allowing you to use the fast Rust MDict parser from Python
code.

#### How to build

```bash
# Install maturin for building Python extensions
python3 -m venv .venv && source .venv/bin/activate && pip install maturin

# Build and install the Python package
maturin develop --release
```

#### Usage

```python
import mdict_rs

# Parse from file
mdx = mdict_rs.parse_mdx_file("dictionary.mdx")

# Parse from bytes
with open("dictionary.mdx", "rb") as f:
    data = f.read()
mdx = mdict_rs.parse_mdx_bytes(data)

# Get all entries
records = mdx.items()
for record in records:
    print(f"{record.text}: {record.definition}")

# Get total count
count = mdx.get_entries_count()
print(f"Total entries: {count}")
```

#### Building for PyPI

```bash
# Build wheels for distribution
maturin build --release

# Publish to PyPI (requires API token)
maturin publish
```
[Pypi](https://pypi.org/project/mdict-rs/)

## References
MDX的解析功能和mdx文件规范参考[mdict-analysis](https://bitbucket.org/xwang/mdict-analysis/src/master/)
和文章[MDX/MDD 文件格式解析](http://einverne.github.io/post/2018/08/mdx-mdd-file-format.html)
