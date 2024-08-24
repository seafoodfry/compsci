.PHONY: all
all: fmt
	~/.poetry/bin/poetry run pytest -vv

.PHONY: fmt
fmt:
	 ~/.poetry/bin/poetry run black max_subarray -v
	 ~/.poetry/bin/poetry run mypy max_subarray

.PHONY: clean
clean:
	rm -rf .mypy_cache .pytest_cache
	find . \( -name '*.pyc' -o -name '__pycache__' \) -not -path './.venv/*' -print
	find . \( -name '*.pyc' -o -name '__pycache__' \) -not -path './.venv/*' -print | xargs rm -rf