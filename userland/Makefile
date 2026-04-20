.PHONY: all clean cjhlib libcjyx cjsh utils rebuild rn

IMAGE = cjyx

all: cjhlib libcjyx cjsh utils

cjhlib:
	$(MAKE) -C cjhlib lib

libcjyx: cjhlib
	$(MAKE) -C libcjyx lib

cjsh: libcjyx
	$(MAKE) -C cjsh

utils: libcjyx
	$(MAKE) -C cmd all

rebuild: clean all

rn:
	docker build -t $(IMAGE) .
	docker run -it --rm $(IMAGE)

dbg:
	docker build -t $(IMAGE)-dbg --build-arg EXTRA_FLAGS="-g -O0" .
	docker run -it --rm $(IMAGE)-dbg /bin/bash

clean:
	$(MAKE) -C cjhlib clean
	$(MAKE) -C libcjyx clean
	$(MAKE) -C cjsh clean
	$(MAKE) -C cmd clean
