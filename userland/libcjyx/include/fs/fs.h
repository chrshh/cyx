#include <str/string.h>
#include <sys/stat.h>

int FileExists(String path);
int IsDir(String path);

String ReadFile(String path);
void WriteFile(String path, String in);
void ChangeDir(String path);
