# SubsequenceFinderPy
SubsequenceFinderPy is a python module written in Rust to quickly find shared String (non-contiguous) subsequences. This project is still under active development, but it technically works. Its current implementation is unlikely to be performant.

## Build Instructions
PyO3 uses maturin to create Python modules. First, create and activate Python virtual environment.
```bash
python3 -m venv python/.venv
. ./python/.venv/bin/activate
```
Next, install maturin.
`pip install maturin`
Install the library in your venv.
`maturin develop`

SubsequenceFinder can now be imported in your current python virtual environment.
```python
>>> import subsequence_finder as sf
>>> sf.longest_subsequence_len("test", "tet")
3
>>> sf.longest_subsequence("test", "tet")
'tet'
```
