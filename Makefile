build:
	perseus deploy -eo docs
	cp CNAME docs
	cp .nojekyll docs

serve: build
	static-web-server --port 8080 --root ./docs
