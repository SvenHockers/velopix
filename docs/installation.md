# Installation Guide

## Requirements

* Python 3.9+
* pip

## Installing from PyPI

```bash
pip install velopix
```

## Installing from Source

```bash
git clone https://github.com/SvenHockers/velopix.git
cd velopix
pip install maturin
python -m maturin develop --release
```

For development, it's recommended to use a virtual environment:

```bash
python -m venv env
source env/bin/activate
pip install velopix
```

---

# Developer Guide

## Repository Structure

* `event_model/` – Event data structures and parsers
* `pipelines/` – Predefined pipeline definitions
* `algorithms/` – Reconstruction logic
* `validator/` – Scoring and evaluation
* `tests/` – Unit tests

## Adding a New Algorithm

1. Implement the algorithm logic
2. Expose it through a pipeline class
3. Add configuration or parameters if needed
4. Write tests and validation cases

## Extending the Validator

You can add custom metrics by extending the `EventMetricsCalculator` and updating the reporting logic.

## Testing

Run all tests with:

```bash
pytest
```
