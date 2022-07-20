# libopen

[![](https://img.shields.io/github/v/tag/thechampagne/libopen?label=version)](https://github.com/thechampagne/libopen/releases/latest) [![](https://img.shields.io/github/license/thechampagne/libopen)](https://github.com/thechampagne/libopen/blob/main/LICENSE)

A **C** library to Open a path or URL with the system-defined program.

### Installation & Setup

#### 1. Clone the repository
```
git clone https://github.com/thechampagne/libopen.git
```
#### 2. Navigate to the root
```
cd libopen
```
#### 3. Build the project
```
cargo build
```

### Example

```c
#include <stdio.h>
#include <open.h>

int main()
{
    if (open_that("http://rust-lang.org") != 0)
    {
        printf("An error occurred when opening http://rust-lang.org\n");
        return -1;
    }
    printf("Opened successfully.");
    return 0;
}
```

### References
 - [open](https://github.com/Byron/open-rs)

### License

This repo is released under the [MIT](https://github.com/thechampagne/libopen/blob/main/LICENSE).
