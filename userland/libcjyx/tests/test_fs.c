#include <fs/fs.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#define ASSERT(condition, msg)                                                 \
  if (condition) {                                                             \
    printf("PASS✅: %s\n", msg);                                               \
  } else {                                                                     \
    printf("FAIL❌: %s\n", msg);                                               \
  }

int main() {
  printf("=== Filesystem Tests ===\n\n");

  // --- FileExists ---
  printf("--- FileExists ---\n");
  // This test file itself must exist
  String self = StrFromChar("tests/test_fs.c");
  ASSERT(FileExists(self) == 1, "FileExists returns 1 for existing file");
  FreeStr(&self);

  String noFile = StrFromChar("tests/does_not_exist.xyz");
  ASSERT(FileExists(noFile) == 0, "FileExists returns 0 for missing file");
  FreeStr(&noFile);

  // A directory is not a regular file
  String dotDir = StrFromChar("tests");
  ASSERT(FileExists(dotDir) == 0, "FileExists returns 0 for a directory");
  FreeStr(&dotDir);

  // --- IsDir ---
  printf("--- IsDir ---\n");
  String testsDir = StrFromChar("tests");
  ASSERT(IsDir(testsDir) == 1, "IsDir returns 1 for existing directory");
  FreeStr(&testsDir);

  String srcDir = StrFromChar("src");
  ASSERT(IsDir(srcDir) == 1, "IsDir returns 1 for src directory");
  FreeStr(&srcDir);

  String notDir = StrFromChar("tests/test_fs.c");
  ASSERT(IsDir(notDir) == 0, "IsDir returns 0 for a regular file");
  FreeStr(&notDir);

  String missingDir = StrFromChar("no_such_dir");
  ASSERT(IsDir(missingDir) == 0, "IsDir returns 0 for non-existent path");
  FreeStr(&missingDir);

  // --- WriteFile + ReadFile ---
  printf("--- WriteFile + ReadFile ---\n");
  String tmpPath = StrFromChar("/tmp/cjyx_test_fs.txt");
  String content = StrFromChar("hello from cjyx");

  WriteFile(tmpPath, content);
  ASSERT(FileExists(tmpPath) == 1, "WriteFile creates the file");

  String readBack = ReadFile(tmpPath);
  ASSERT(readBack.len == content.len, "ReadFile returns correct length");
  ASSERT(strncmp(readBack.chars, "hello from cjyx", 15) == 0,
         "ReadFile returns correct content");
  FreeStr(&readBack);

  // Overwrite with different content
  String content2 = StrFromChar("overwritten");
  WriteFile(tmpPath, content2);
  String readBack2 = ReadFile(tmpPath);
  ASSERT(readBack2.len == content2.len, "WriteFile overwrites correctly");
  ASSERT(strncmp(readBack2.chars, "overwritten", 11) == 0,
         "ReadFile returns overwritten content");
  FreeStr(&readBack2);
  FreeStr(&content2);

  // Clean up tmp file
  unlink(tmpPath.chars);
  FreeStr(&tmpPath);
  FreeStr(&content);

  // --- ChangeDir ---
  printf("--- ChangeDir ---\n");
  char origDir[1024];
  getcwd(origDir, sizeof(origDir));

  String tmpDir = StrFromChar("/tmp");
  ChangeDir(tmpDir);

  char newDir[1024];
  getcwd(newDir, sizeof(newDir));
  // /tmp may resolve to /private/tmp on macOS
  ASSERT(strstr(newDir, "tmp") != NULL, "ChangeDir changes working directory");
  FreeStr(&tmpDir);

  // Change back
  String orig = StrFromChar(origDir);
  ChangeDir(orig);
  FreeStr(&orig);

  // --- FormatPermsOctal ---
  printf("--- FormatPermsOctal ---\n");
  // 0755 -> rwxr-xr-x@
  String perm755 = FormatPermsOctal(0755);
  ASSERT(perm755.len == 10, "FormatPermsOctal 755 has correct length");
  ASSERT(strncmp(perm755.chars, "rwxr-xr-x@", 10) == 0,
         "FormatPermsOctal 755 is rwxr-xr-x@");
  FreeStr(&perm755);

  // 0644 -> rw-r--r--@
  String perm644 = FormatPermsOctal(0644);
  ASSERT(strncmp(perm644.chars, "rw-r--r--@", 10) == 0,
         "FormatPermsOctal 644 is rw-r--r--@");
  FreeStr(&perm644);

  // 0700 -> rwx------@
  String perm700 = FormatPermsOctal(0700);
  ASSERT(strncmp(perm700.chars, "rwx------@", 10) == 0,
         "FormatPermsOctal 700 is rwx------@");
  FreeStr(&perm700);

  // 0777 -> rwxrwxrwx@
  String perm777 = FormatPermsOctal(0777);
  ASSERT(strncmp(perm777.chars, "rwxrwxrwx@", 10) == 0,
         "FormatPermsOctal 777 is rwxrwxrwx@");
  FreeStr(&perm777);

  // --- OpenDir ---
  printf("--- OpenDir ---\n");
  DIR *dp = OpenDir("tests");
  ASSERT(dp != NULL, "OpenDir returns non-NULL for valid directory");
  closedir(dp);

  return 0;
}
