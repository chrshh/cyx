.PHONY: all clean cjhlib libcjyx cjsh utils rebuild

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

clean:
	$(MAKE) -C cjhlib clean
	$(MAKE) -C libcjyx clean
	$(MAKE) -C cjsh clean
	$(MAKE) -C cmd clean
