#include "asemics.h"
#include <stdlib.h>
#include <unistd.h>
#include <sys/wait.h>

/* on success opens the chosen file in editor and returns 0 */
/* on failure returns -1 */
int invokePalmer(const char *start_dir) {
    int pipefd[2];
    if (pipe(pipefd) == -1) return -1;

    /* palmer owns the terminal now */
    disableRawMode();

    pid_t pid = fork();

    /* error case, close rw pipes & return */
    if (pid == -1) {
        close(pipefd[0]);
        close(pipefd[1]);
        enableRawMode();
        return -1;
    }

    /*
     * child process, wire stdout to W pipe -> exec palmer
     * palmer draws its UI on /dev/tty and prints only chosen path to stdout
     */
    if (pid == 0) {
        close(pipefd[0]);
        if (dup2(pipefd[1], STDOUT_FILENO) == -1) childExit();
        close(pipefd[1]);

        // XXX Swap this out for prod path later
        const char *palmer_bin = "/home/chris/repositories/cjyx/userland/target/debug/palmer";
        if (start_dir != NULL) {
            execl(palmer_bin, "palmer", "--pick-file", "--cwd", start_dir, (char *)NULL);
        } else {
            execl(palmer_bin, "palmer", "--pick-file", (char *)NULL);
        }
        childExit(); // exec failed
    }

    /*
     * parent process (asemics)
     * drain palmers stdout into a buffer until palmer pipe closes
     */
    close(pipefd[1]);

    char    path[PALMER_MAX_PATH];
    size_t  total = 0;
    ssize_t n;
    while (total < sizeof(path) - 1 &&
           (n = read(pipefd[0], path + total, sizeof(path) - 1 - total)) > 0) {
        total += n;
    }
    close(pipefd[0]);
    path[total] = '\0';

    int status;
    waitpid(pid, &status, 0);

    /* return terminal to asemics */
    enableRawMode();

    if (!WIFEXITED(status) || WEXITSTATUS(status) != 0 || total == 0) { return -1; }

    /* strip newline palmer prints */
    if (path[total - 1] == '\n') path[total - 1] = '\0';

    editorOpen(path);
    editorSetStatusMsg("opened %s", path);
    return 0;
}
