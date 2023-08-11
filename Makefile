build:
	perseus deploy -e -o docs
	cp -r static/* docs

serve: build
	static-web-server --port 8080 --root ./docs
