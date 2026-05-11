#include <fcntl.h>
#include <sys/mman.h>
#include <sys/ioctl.h>
#include <linux/fb.h>
#include <unistd.h>
#include <stdint.h>

int main(void) {
  int fd = open("/dev/fb0", O_RDWR);

  struct fb_var_screeninfo vinfo;
  struct fb_fix_screeninfo finfo;
  ioctl(fd, FBIOGET_VSCREENINFO, &vinfo);
  ioctl(fd, FBIOGET_FSCREENINFO, &finfo);

  size_t size = finfo.smem_len;
  uint8_t *fb = mmap(NULL, size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);

  // Fill the screen with a color. Pixel format is typically 32-bit BGRA on efifb.
  for (uint32_t y = 0; y < vinfo.yres; y++) {
    for (uint32_t x = 0; x < vinfo.xres; x++) {
      uint32_t off = y * finfo.line_length + x * (vinfo.bits_per_pixel / 8);
      fb[off + 0] = x & 0xff;       // B
      fb[off + 1] = y & 0xff;       // G
      fb[off + 2] = (x + y) & 0xff; // R
      fb[off + 3] = 0;              // A/pad
    }
  }

  munmap(fb, size);
  close(fd);
  return 0;
}
