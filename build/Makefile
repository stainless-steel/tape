out := $(OUT_DIR)
jobs := $(NUM_JOBS)
src := $(CARGO_MANIFEST_DIR)/build/libtar

export CFLAGS := $(CFLAGS) -fPIC

check:
	pkg-config libtar || $(MAKE) build

build:
	cd $(out) && $(src)/configure --srcdir=$(src)
	$(MAKE) -C $(out) -j$(jobs)
	cp $(out)/lib/libtar.a $(out)
