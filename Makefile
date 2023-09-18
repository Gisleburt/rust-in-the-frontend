build:
	(cd site-perseus && perseus deploy -eo ../docs)
	rg --passthru 'main\(\);' -r '//main();' ./docs/index.html > ./docs/index.tmp && mv ./docs/index.tmp ./docs/index.html
	cp CNAME docs
	cp .nojekyll docs

serve: build
	static-web-server --port 8080 --root ./docs
