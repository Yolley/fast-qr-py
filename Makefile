deps:
	@uv sync --all-groups --all-extras

lint: deps
	@uv run ruff check
	@uv run ty check

test: deps
	@uv run pytest


build: deps
	@uv build
