.PHONY: all clean static

PKGS = wlroots wayland-server xkbcommon

# The critical fix: added -I. so the compiler checks the local directory for headers
CFLAGS += -I. $(shell pkg-config --cflags $(PKGS)) -DWLR_USE_UNSTABLE
LDFLAGS += $(shell pkg-config --libs $(PKGS))

# Hardcode Debian paths to bypass potential pkg-config failures inside Docker
WAYLAND_PROTOCOLS = /usr/share/wayland-protocols
WAYLAND_SCANNER = wayland-scanner

all: display

# 1. Generate the header from the XML protocol
xdg-shell-protocol.h:
	$(WAYLAND_SCANNER) server-header $(WAYLAND_PROTOCOLS)/stable/xdg-shell/xdg-shell.xml $@

# 2. Force the header to exist BEFORE trying to compile the object file
tinywl.o: tinywl.c xdg-shell-protocol.h
	$(CC) $(CFLAGS) -c -o $@ $<

# 3. Link it all together
display: tinywl.o
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

static: display

clean:
	rm -f display tinywl.o xdg-shell-protocol.h
