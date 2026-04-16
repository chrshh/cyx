#include <dirent.h>
#include <str/strarray.h>
#include <sys/stat.h>

int FileExists(String path);
int IsDir(String path);

String ReadFile(String path);
void WriteFile(String path, String in);
void ChangeDir(String path);
void PrintEntries(struct dirent *dp, struct stat *sb, int flags);
String FormatPermsOctal(int perm);

StrArr ListDirs(String path);
DIR *OpenDir(char *path);
