build.site.perseus: install.perseus
	(cd site-perseus && perseus deploy -eo ../docs)
	rg --passthru 'main\(\);' -r '//main();' ./docs/index.html > ./docs/index.tmp && mv ./docs/index.tmp ./docs/index.html
	cp CNAME docs
	cp .nojekyll docs

build.demo.yew: install.yew
	(cd demo-yew && trunk build)

serve.site.perseus: build.site.perseus install.server
	static-web-server --port 8080 --root ./docs

serve.demo.yew: build.demo.yew
	static-web-server --port 8080 --root ./demo-yew/dist

install.perseus:
	cargo install perseus-cli ripgrep

install.yew:
	cargo install trunk

install.server:
	cargo install static-web-server
