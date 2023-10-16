SVDTOOL := svdtools
CHIPTOOL := target/release/chiptool

all: patch crate fmt

svd/PAC55XX.svd.patched: svd/PAC55XX.yaml svd/PAC55XX.svd
	$(SVDTOOL) patch $<

patch: svd/PAC55XX.svd.patched

crate: svd/PAC55XX.svd.patched $(CHIPTOOL)
	$(CHIPTOOL) generate --svd svd/PAC55XX.svd.patched --transform svd/PAC55XX.transforms.yaml
	rm -rf src
	sed -i.orig 's/# ! \[no_std\]/# ! [no_std] # ! [allow(non_camel_case_types)]/g' lib.rs
	form -i lib.rs -o src
	rm lib.rs lib.rs.orig

fmt:
	cargo fmt --package=pac55xx-pac

$(CHIPTOOL):
	cargo build --release --package=chiptool
