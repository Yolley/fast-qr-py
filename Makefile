deps:
	uv sync --all-groups --all-extras

lint:
	uv run ruff check
	uv run ty check

test:
	uv run pytest
