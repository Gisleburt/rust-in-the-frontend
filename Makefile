build.site.perseus: install.perseus
	(cd site-perseus && perseus deploy -eo ../docs)
	rg --passthru 'main\(\);' -r '//main();' ./docs/index.html > ./docs/index.tmp && mv ./docs/index.tmp ./docs/index.html
	cp .nojekyll docs

build.demo.dioxus: install.perseus
	(cd demo-dioxus && dx build --release)

build.demo.perseus: install.perseus
	(cd demo-perseus && perseus deploy -e)

build.demo.yew: install.yew
	(cd demo-yew && trunk build)

serve.site.perseus: build.site.perseus install.server
	static-web-server --port 8080 --root ./docs

serve.demo.dioxus: build.demo.dioxus install.server
	static-web-server --port 8081 --root ./demo-dioxus/dist

serve.demo.perseus: build.demo.perseus install.server
	static-web-server --port 8082 --root ./demo-perseus/pkg

serve.demo.yew: build.demo.yew install.server
	static-web-server --port 8083 --root ./demo-yew/dist

install.perseus:
	cargo install perseus-cli ripgrep

install.yew:
	cargo install trunk

install.server:
	cargo install static-web-server
