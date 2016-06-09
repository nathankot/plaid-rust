default:

.PHONY: gh-pages
gh-pages: target/doc/plaid
	git checkout gh-pages
	rm -rf ./master
	mkdir -p ./master
	cp -r target/doc/* ./master/
	git add ./master
	git commit --allow-empty -m "Update documentation"
	git checkout master

target/doc/plaid:
	@mkdir -p $(@D)
	@cargo doc --no-deps --release
