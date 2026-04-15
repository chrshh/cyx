.PHONY: all clean cjhlib libcjyx utils

all: cjhlib libcjyx utils

cjhlib:
	$(MAKE) -C cjhlib lib

libcjyx: cjhlib
	$(MAKE) -C libcjyx lib

utils: libcjyx
	$(MAKE) -C utils all

clean:
	$(MAKE) -C cjhlib clean
	$(MAKE) -C libcjyx clean
	$(MAKE) -C utils clean
