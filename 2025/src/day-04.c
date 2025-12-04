#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <unistd.h>

#define MAX_SIZE 256

static bool grid[MAX_SIZE + 2][MAX_SIZE + 2];  // +2 for padding to avoid bounds checks
static int height = 0;
static int width = 0;

static const int dx[] = {-1, -1, 0, 1, 1, 1, 0, -1};
static const int dy[] = {0, -1, -1, -1, 0, 1, 1, 1};

static void parse(const char *filename) {
    int fd = open(filename, O_RDONLY);
    if (fd < 0) {
        perror("Failed to open file");
        return;
    }

    struct stat st;
    if (fstat(fd, &st) < 0) {
        perror("Failed to stat file");
        close(fd);
        return;
    }

    size_t size = st.st_size;
    if (size == 0) {
        close(fd);
        return;
    }

    const char *data = mmap(NULL, size, PROT_READ, MAP_PRIVATE, fd, 0);
    if (data == MAP_FAILED) {
        perror("Failed to mmap file");
        close(fd);
        return;
    }

    height = 0;
    width = 0;

    const char *p = data;
    const char *end = data + size;

    while (p < end && height < MAX_SIZE) {
        const char *line_start = p;
        while (p < end && *p != '\n') p++;
        int len = p - line_start;
        if (p < end) p++; // skip newline

        if (len == 0) continue;

        if (width == 0) {
            width = len;
        }

        for (int x = 0; x < len && x < MAX_SIZE; x++) {
            grid[height + 1][x + 1] = (line_start[x] == '@');  // +1 offset for padding
        }
        height++;
    }

    munmap((void *)data, size);
    close(fd);
}

static int count_neighbors(int x, int y) {
    int neighbors = 0;
    for (int i = 0; i < 8; i++) {
        int nx = x + dx[i];
        int ny = y + dy[i];
        neighbors += grid[ny][nx];  // padding eliminates bounds checks
    }
    return neighbors;
}

static int get_accessible(int accessible_x[], int accessible_y[]) {
    int count = 0;

    for (int y = 1; y <= height; y++) {
        for (int x = 1; x <= width; x++) {
            if (!grid[y][x]) {
                continue;
            }

            int neighbors = count_neighbors(x, y);
            if (neighbors < 4) {
                accessible_x[count] = x;
                accessible_y[count] = y;
                count++;
            }
        }
    }

    return count;
}

static int part1(void) {
    int accessible_x[MAX_SIZE * MAX_SIZE];
    int accessible_y[MAX_SIZE * MAX_SIZE];
    return get_accessible(accessible_x, accessible_y);
}

static int part2(void) {
    int accessible_x[MAX_SIZE * MAX_SIZE];
    int accessible_y[MAX_SIZE * MAX_SIZE];
    int total = 0;

    int count;
    while ((count = get_accessible(accessible_x, accessible_y)) > 0) {
        total += count;
        for (int i = 0; i < count; i++) {
            grid[accessible_y[i]][accessible_x[i]] = false;
        }
    }

    return total;
}

int main(int argc, char *argv[]) {
    const char *filename = argc > 1 ? argv[1] : "input/2025/day4.txt";

    parse(filename);
    printf("Part 1: %d\n", part1());
    printf("Part 2: %d\n", part2());

    return 0;
}
