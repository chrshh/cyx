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
  String self = StringFromLiteral("tests/test_fs.c");
  ASSERT(FileExists(self) == 1, "FileExists returns 1 for existing file");
  FreeString(self);

  String noFile = StringFromLiteral("tests/does_not_exist.xyz");
  ASSERT(FileExists(noFile) == 0, "FileExists returns 0 for missing file");
  FreeString(noFile);

  // A directory is not a regular file
  String dotDir = StringFromLiteral("tests");
  ASSERT(FileExists(dotDir) == 0, "FileExists returns 0 for a directory");
  FreeString(dotDir);

  // --- IsDir ---
  printf("--- IsDir ---\n");
  String testsDir = StringFromLiteral("tests");
  ASSERT(IsDir(testsDir) == 1, "IsDir returns 1 for existing directory");
  FreeString(testsDir);

  String srcDir = StringFromLiteral("src");
  ASSERT(IsDir(srcDir) == 1, "IsDir returns 1 for src directory");
  FreeString(srcDir);

  String notDir = StringFromLiteral("tests/test_fs.c");
  ASSERT(IsDir(notDir) == 0, "IsDir returns 0 for a regular file");
  FreeString(notDir);

  String missingDir = StringFromLiteral("no_such_dir");
  ASSERT(IsDir(missingDir) == 0, "IsDir returns 0 for non-existent path");
  FreeString(missingDir);

  // --- WriteFile + ReadFile ---
  printf("--- WriteFile + ReadFile ---\n");
  String tmpPath = StringFromLiteral("/tmp/cjyx_test_fs.txt");
  String content = StringFromLiteral("hello from cjyx");

  WriteFile(tmpPath, content);
  ASSERT(FileExists(tmpPath) == 1, "WriteFile creates the file");

  String readBack = ReadFile(tmpPath);
  ASSERT(readBack.len == content.len, "ReadFile returns correct length");
  ASSERT(strncmp(readBack.chars, "hello from cjyx", 15) == 0,
         "ReadFile returns correct content");
  FreeString(readBack);

  // Overwrite with different content
  String content2 = StringFromLiteral("overwritten");
  WriteFile(tmpPath, content2);
  String readBack2 = ReadFile(tmpPath);
  ASSERT(readBack2.len == content2.len, "WriteFile overwrites correctly");
  ASSERT(strncmp(readBack2.chars, "overwritten", 11) == 0,
         "ReadFile returns overwritten content");
  FreeString(readBack2);
  FreeString(content2);

  // Clean up tmp file
  unlink(tmpPath.chars);
  FreeString(tmpPath);
  FreeString(content);

  // --- ChangeDir ---
  printf("--- ChangeDir ---\n");
  char origDir[1024];
  getcwd(origDir, sizeof(origDir));

  String tmpDir = StringFromLiteral("/tmp");
  ChangeDir(tmpDir);

  char newDir[1024];
  getcwd(newDir, sizeof(newDir));
  // /tmp may resolve to /private/tmp on macOS
  ASSERT(strstr(newDir, "tmp") != NULL, "ChangeDir changes working directory");
  FreeString(tmpDir);

  // Change back
  String orig = StringFromLiteral(origDir);
  ChangeDir(orig);
  FreeString(orig);

  // --- FormatPermsOctal ---
  printf("--- FormatPermsOctal ---\n");
  // 0755 -> rwxr-xr-x@
  String perm755 = FormatPermsOctal(0755);
  ASSERT(perm755.len == 10, "FormatPermsOctal 755 has correct length");
  ASSERT(strncmp(perm755.chars, "rwxr-xr-x@", 10) == 0,
         "FormatPermsOctal 755 is rwxr-xr-x@");
  FreeString(perm755);

  // 0644 -> rw-r--r--@
  String perm644 = FormatPermsOctal(0644);
  ASSERT(strncmp(perm644.chars, "rw-r--r--@", 10) == 0,
         "FormatPermsOctal 644 is rw-r--r--@");
  FreeString(perm644);

  // 0700 -> rwx------@
  String perm700 = FormatPermsOctal(0700);
  ASSERT(strncmp(perm700.chars, "rwx------@", 10) == 0,
         "FormatPermsOctal 700 is rwx------@");
  FreeString(perm700);

  // 0777 -> rwxrwxrwx@
  String perm777 = FormatPermsOctal(0777);
  ASSERT(strncmp(perm777.chars, "rwxrwxrwx@", 10) == 0,
         "FormatPermsOctal 777 is rwxrwxrwx@");
  FreeString(perm777);

  // --- OpenDir ---
  printf("--- OpenDir ---\n");
  DIR *dp = OpenDir("tests");
  ASSERT(dp != NULL, "OpenDir returns non-NULL for valid directory");
  closedir(dp);

  return 0;
}
