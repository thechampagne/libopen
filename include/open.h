#ifndef __OPEN_H__
#define __OPEN_H__

#ifdef __cplusplus
extern "C" {
#endif

/*
* Open path with the default application without blocking.
* 
* Example:
* * *
* int main()
* {
*     if (open_that("http://rust-lang.org") != 0)
*     {
*         printf("An error occurred when opening http://rust-lang.org\n");
*         return -1;
*     }
*     printf("Opened successfully.");
*     return 0;
* }
* * *
* 
* @param path
* @return 0 on success and non zero value on failure
*/
extern int open_that(const char* path);

/*
* Open path with the given application.
* 
* Example:
* * *
* int main()
* {
*     if (open_with("http://rust-lang.org", "firefox") != 0)
*     {
*         printf("An error occurred when opening http://rust-lang.org\n");
*         return -1;
*     }
*     printf("Opened successfully.");
*     return 0;
* }
* * *
* 
* @param path
* @param app
* @return 0 on success and non zero value on failure
*/
extern int open_with(const char* path, const char* app);

#ifdef __cplusplus
}
#endif

#endif // __OPEN_H__